use baum::{Cursor, Tree};
use common::{Position, Rectangle};
use imagine::{Blender, Image};

use crate::{
    error::EngineError,
    layer::{Layer, LayerFlag},
    Engine,
};

pub fn remove_layer(
    blender: &mut Box<dyn Blender>,
    content: &mut Tree<Layer>,
    idx: usize,
) -> Result<(), EngineError> {
    let parent = content.get_parent(idx).map_err(EngineError::from)?;
    content.remove_entry(idx).map_err(EngineError::from)?;
    propagate_changes_up(blender, content, parent)?;
    Ok(())
}

/// Merges ghost layer with base
pub fn merge_ghost(
    blender: &mut Box<dyn Blender>,
    content: &mut Tree<Layer>,
    idx: usize,
) -> Result<(), EngineError> {
    let mut cursor = Cursor::new(content, idx).map_err(EngineError::from)?;
    let layer = cursor.value_mut();
    if let (Some(ghost), Some(zombie)) = (&layer.ghost, &layer.zombie) {
        log::debug!("Merging ghost of {}", layer.name);
        layer.img = blender.blend(
            ghost.mode,
            &layer.rectangle(),
            (&ghost.img, layer.attr.pos, ghost.alpha as f64, None),
            (&zombie, layer.attr.pos, 1.0, None),
        );
        layer.ghost = None;
        layer.zombie = None;
        Ok(())
    } else {
        Err(EngineError::application_error(
            "Can't merge non-existing ghost",
        ))
    }
}

/// Propagates the changes at the layer of the index `changed` up the content tree.
/// The index must be valid.
pub fn propagate_changes_up(
    blender: &mut Box<dyn Blender>,
    content: &mut Tree<Layer>,
    changed: usize,
) -> Result<(), EngineError> {
    let mut cursor = Cursor::new(content, changed).map_err(EngineError::from)?;
    // merge ghost of current layer if needed
    let layer = cursor.value_mut();

    if layer.flag == LayerFlag::Pixel {
        log::debug!("Propagate changes from '{}'", layer.name);

        // ghost
        if let (Some(ghost), Some(zombie)) = (&layer.ghost, &layer.zombie) {
            layer.img = blender.blend(
                ghost.mode,
                &layer.rectangle(),
                (&ghost.img, layer.attr.pos, ghost.alpha as f64, None),
                (&zombie, layer.attr.pos, layer.attr.alpha as f64, None),
            );
        }
        cursor.go_up();
        let changed = cursor.index();
        propagate_changes_up(blender, content, changed)
    } else {
        let rectangle = &layer.rectangle();
        let arg = cursor
            .children()
            .iter()
            .map(|(child, idx)| {
                (
                    child.attr.mode,
                    &child.img,
                    child.attr.pos,
                    child.attr.alpha as f64,
                    child.visible,
                    Some(*idx),
                )
            })
            .collect();
        let result = blender.blend_all(rectangle, arg);
        let layer = cursor.value_mut();
        layer.img = result;
        if cursor.is_on_root() {
            Ok(())
        } else {
            cursor.go_up();
            let changed = cursor.index();
            propagate_changes_up(blender, content, changed)
        }
    }
}

pub fn propagate_damage(
    blender: &mut Box<dyn Blender>,
    content: &mut Tree<Layer>,
    changed: usize,
    damage: &Rectangle,
) -> Result<(), EngineError> {
    let (layer, children) = unsafe { get_layer_with_children(content, changed) };
    let dest = &mut layer.img;

    // clean damaged
    let relative_damage = damage - &layer.attr.pos;
    dest.clean(&relative_damage);

    for child in children {
        if !child.visible {
            continue;
        }
        blender.blend_damaged(
            child.attr.mode,
            (dest, layer.attr.pos, layer.attr.alpha as f64),
            (&child.img, child.attr.pos, child.attr.alpha as f64),
            damage,
        )
    }

    if let (Some(ghost), Some(zombie)) = (&mut layer.ghost, &mut layer.zombie) {
        blender.blend_damaged_into(
            ghost.mode,
            damage,
            (dest, layer.attr.pos),
            (&ghost.img, layer.attr.pos, ghost.alpha as f64),
            (zombie, layer.attr.pos, 1.0),
        )
    }

    let parent_idx = {
        let mut g = Cursor::new(content, changed).map_err(EngineError::from)?;
        g.go_up();
        g.index()
    };

    if changed == content.root {
        Ok(())
    } else {
        propagate_damage(blender, content, parent_idx, damage)
    }
}

pub fn add_layer(
    state: &mut Engine,
    parent: usize,
    position: &Option<Position>,
    content: Image,
    name: Option<String>,
) -> Result<usize, EngineError> {
    // get default values for size and position of a layer
    let position = match position {
        Some(position) => *position,
        None => Position { x: 0, y: 0 },
    };
    // create layer
    let mut layer = Layer::from_content(content);
    layer.attr.pos = position;
    let mut cursor = Cursor::new(&mut state.content, parent).map_err(EngineError::from)?;
    let current = cursor.value();
    if current.flag == LayerFlag::Pixel {
        Err(EngineError::user_error(
            "Can't create sub layer on pixel layer",
        ))
    } else {
        let idx = cursor.add_child_and_go_down(layer);
        let layer = cursor.value_mut();
        let default_name = format!("Layer # {}", idx).to_string();
        layer.name = (name).unwrap_or(default_name).clone().to_string();
        Ok(idx)
    }
}

pub fn spawn_layer(
    session: &mut Engine,
    id: usize,
    reference_idx: isize,
) -> Result<(), EngineError> {
    if reference_idx.is_positive() {
        let res = session.content.get_value(reference_idx.unsigned_abs());
        if let Ok(layer) = res {
            if layer.flag == LayerFlag::Group {
                return move_layer(session, id, -reference_idx);
            }
        }
    }
    move_layer(session, id, reference_idx)
}

pub fn move_layer(session: &mut Engine, id: usize, move_idx: isize) -> Result<(), EngineError> {
    session
        .content
        .move_node(id, move_idx)
        .map_err(EngineError::from)
}

unsafe fn get_layer_with_children(
    content: &mut Tree<Layer>,
    idx: usize,
) -> (&mut Layer, Vec<&Layer>) {
    let nodes = &mut content.nodes;
    let ptr = nodes.as_mut_ptr();
    let node = ptr.add(idx).as_mut().unwrap();
    let children: Vec<&Layer> = node
        .children
        .iter()
        .map(|x| nodes.get(*x).unwrap())
        .map(|node| &node.value)
        .collect();
    let layer = &mut node.value;
    (layer, children)
}
