use std::collections::HashMap;

use crate::{BlendMode, Blender, Image, SoftwareBlender};
use common::{Position, Rectangle};
use image::RgbaImage;
use js_sys::Map;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, ImageData, WebGl2RenderingContext, WebGlFramebuffer, WebGlProgram,
    WebGlShader, WebGlTexture,
};

use super::blender::BlendLayer;

// TODO Code is really rough right now with lots of unwraps

/// WebGlBlender is a [Blender] that relies on WebGl.
/// POC can only perform alpha-blending on full-image blending right now. Uses [SoftwareBlender] as a fallback otherwise.
pub struct WebGlBlender {
    fallback: SoftwareBlender,
    program: WebGlProgram,
    gl_canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    cache: HashMap<usize, WebGlTexture>,
    texture1: WebGlTexture,
    texture2: WebGlTexture,
    ping_pong: Option<(
        (WebGlTexture, WebGlFramebuffer),
        (WebGlTexture, WebGlFramebuffer),
    )>,
}

impl WebGlBlender {
    pub fn new() -> Result<Self, String> {
        let window = web_sys::window().ok_or(String::from("No window"))?;
        let document = window.document().ok_or(String::from("No document"))?;
        let gl_canvas = document
            .create_element("canvas")
            .map_err(|_| String::from("Fail to create canvas"))?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| String::from("No dyn into canvas"))?;
        let options = Map::new();
        options.set(&JsValue::from_str("alpha"), &JsValue::from_bool(true));
        options.set(
            &JsValue::from_str("premultipliedAlpha"),
            &JsValue::from_bool(false),
        );
        let mut gl = gl_canvas
            .get_context_with_context_options("webgl2", &options.into())
            .map_err(|_| String::from("No dyn into"))?
            .ok_or(String::from("No dyn into"))?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| String::from("No dyn into"))?;
        let vs = include_str!("../../res/glsl/base.vert");
        let fs = include_str!("../../res/glsl/alpha.frag");
        let program = create_program(&gl, vs, fs)?;
        let texture1 = create_texture(&mut gl)?;
        let texture2 = create_texture(&mut gl)?;

        let result = WebGlBlender {
            fallback: SoftwareBlender::new(),
            program,
            gl_canvas,
            gl,
            texture1,
            texture2,
            cache: HashMap::new(),
            ping_pong: None,
        };
        Ok(result)
    }

    fn blend_all_fallback(&mut self, destination: &Rectangle, children: Vec<BlendLayer>) -> Image {
        let (width, height) = (destination.size.width, destination.size.height);
        let mut result = Image::new(width, height);
        for (mode, img, pos, alpha, visible, marker) in children {
            if !visible {
                continue;
            }
            result = self.blend(
                mode,
                destination,
                (img, pos, alpha, marker),
                (&result, destination.position, 1.0, None),
            );
        }
        result
    }
}

impl Blender for WebGlBlender {
    fn name(&self) -> &'static str {
        "WebGL"
    }

    fn blend(
        &mut self,
        mode: BlendMode,
        destination: &Rectangle,
        (overlay_img, overlay_pos, overlay_alpha, overlay_id): (
            &Image,
            Position,
            f64,
            Option<usize>,
        ),
        (base_img, base_pos, base_alpha, base_id): (&Image, Position, f64, Option<usize>),
    ) -> Image {
        if mode != BlendMode::Alpha {
            return self.fallback.blend(
                mode,
                destination,
                (overlay_img, overlay_pos, overlay_alpha, overlay_id),
                (base_img, base_pos, base_alpha, base_id),
            );
        }
        let (w, h) = destination.size.into();
        let (canvas, gl, program) = (&mut self.gl_canvas, &mut self.gl, &self.program);

        canvas.set_width(w);
        canvas.set_height(h);
        gl.viewport(0, 0, w as i32, h as i32);
        gl.use_program(Some(program));
        let vertices: [f32; 12] = [-1., -1., -1., 1., 1., 1., -1., -1., 1., 1., 1., -1.];

        let vertex_buffer = gl.create_buffer().expect("failed to create buffer");
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let position_location = gl.get_attrib_location(program, "position");
        gl.vertex_attrib_pointer_with_i32(
            position_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(position_location as u32);

        gl.clear_color(1.0, 1.0, 1.0, 0.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        if let Some(texture) = base_id.and_then(|x| self.cache.get(&x)) {
            log::info!("REUSE 1");
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
        } else {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture1));
            load_into(gl, base_img);
        }

        gl.active_texture(WebGl2RenderingContext::TEXTURE1);
        if let Some(texture) = overlay_id.and_then(|x| self.cache.get(&x)) {
            log::info!("REUSE 2");
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
        } else {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture2));
            load_into(gl, overlay_img);
        }

        let base_location = gl.get_uniform_location(program, "baseSampler").unwrap();
        gl.uniform1i(Some(&base_location), 0);

        let active_location = gl.get_uniform_location(program, "activeSampler").unwrap();
        gl.uniform1i(Some(&active_location), 1);

        let location = gl.get_uniform_location(program, "baseAlpha").unwrap();
        gl.uniform1f(Some(&location), base_alpha as f32);

        let location = gl.get_uniform_location(program, "activeAlpha").unwrap();
        gl.uniform1f(Some(&location), overlay_alpha as f32);

        let location = gl.get_uniform_location(program, "basePosition").unwrap();
        gl.uniform2f(Some(&location), base_pos.x as f32, base_pos.y as f32);

        let location = gl.get_uniform_location(program, "activePosition").unwrap();
        gl.uniform2f(Some(&location), overlay_pos.x as f32, overlay_pos.y as f32);

        let location = gl.get_uniform_location(program, "activeSize").unwrap();
        gl.uniform2f(
            Some(&location),
            overlay_img.width() as f32,
            overlay_img.height() as f32,
        );

        let location = gl.get_uniform_location(program, "baseSize").unwrap();
        gl.uniform2f(
            Some(&location),
            base_img.width() as f32,
            base_img.height() as f32,
        );

        let location = gl.get_uniform_location(program, "windowPosition").unwrap();
        gl.uniform2f(
            Some(&location),
            destination.position.x as f32,
            destination.position.y as f32,
        );
        let location = gl.get_uniform_location(program, "windowSize").unwrap();
        gl.uniform2f(
            Some(&location),
            destination.size.width as f32,
            destination.size.height as f32,
        );
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);

        let mut result = vec![0u8; (4 * w * h) as usize];
        gl.read_pixels_with_opt_u8_array(
            0,
            0,
            w as i32,
            h as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&mut result),
        )
        .unwrap();

        let buf = RgbaImage::from_raw(w, h, result).unwrap();
        Image { buf }
    }

    fn blend_damaged_into(
        &mut self,
        mode: BlendMode,
        damage: &Rectangle,
        destination: (&mut Image, Position),
        overlay: (&Image, Position, f64),
        base: (&Image, Position, f64),
    ) {
        self.fallback
            .blend_damaged_into(mode, damage, destination, overlay, base);
    }

    fn blend_damaged(
        &mut self,
        mode: BlendMode,
        base: (&mut Image, Position, f64),
        overlay: (&Image, Position, f64),
        damage: &Rectangle,
    ) {
        self.fallback.blend_damaged(mode, base, overlay, damage);
    }

    fn load(&mut self, marker: usize, img: &Image) {
        let texture = create_texture(&mut self.gl).unwrap();
        let img_data: ImageData = img.into();
        self.gl
            .tex_image_2d_with_u32_and_u32_and_image_data(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &img_data,
            )
            .unwrap();
        self.cache.insert(marker, texture);
    }

    fn clean(&mut self) {
        self.cache = HashMap::new();
        self.ping_pong = None;
    }

    fn blend_all(
        &mut self,
        destination: &Rectangle,
        children: Vec<(BlendMode, &Image, Position, f64, bool, Option<usize>)>,
    ) -> Image {
        if !children.iter().all(|x| x.0 == BlendMode::Alpha) {
            return self.blend_all_fallback(destination, children);
        }
        let (w, h) = destination.size.into();
        let (canvas, gl, program) = (&mut self.gl_canvas, &mut self.gl, &self.program);

        canvas.set_width(w);
        canvas.set_height(h);
        gl.use_program(Some(program));
        let vertices: [f32; 12] = [-1., -1., -1., 1., 1., 1., -1., -1., 1., 1., 1., -1.];

        let vertex_buffer = gl.create_buffer().expect("failed to create buffer");
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let position_location = gl.get_attrib_location(program, "position");
        gl.vertex_attrib_pointer_with_i32(
            position_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(position_location as u32);

        let location = gl.get_uniform_location(program, "windowPosition").unwrap();
        gl.uniform2f(
            Some(&location),
            destination.position.x as f32,
            destination.position.y as f32,
        );
        let location = gl.get_uniform_location(program, "windowSize").unwrap();
        gl.uniform2f(
            Some(&location),
            destination.size.width as f32,
            destination.size.height as f32,
        );

        let level = 0;

        let ((ping_texture, ping_fb), (pong_texture, pong_fb)): (
            (&WebGlTexture, &WebGlFramebuffer),
            (&WebGlTexture, &WebGlFramebuffer),
        ) = if let Some((a, b)) = &self.ping_pong {
            // clean
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&a.1));
            gl.clear_color(0.0, 0.0, 0.0, 0.0);
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&b.1));
            gl.clear_color(0.0, 0.0, 0.0, 0.0);
            ((&a.0, &a.1), (&b.0, &b.1))
        } else {
            let ping_fb = gl.create_framebuffer().unwrap();
            gl.active_texture(WebGl2RenderingContext::TEXTURE0);
            let ping_texture = create_texture(gl).unwrap();
            let base_img = Image::new(w, h);
            load_into(gl, &base_img);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&ping_texture));
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&ping_fb));
            gl.framebuffer_texture_2d(
                WebGl2RenderingContext::FRAMEBUFFER,
                WebGl2RenderingContext::COLOR_ATTACHMENT0,
                WebGl2RenderingContext::TEXTURE_2D,
                Some(&ping_texture),
                level,
            );

            let pong_fb = gl.create_framebuffer().unwrap();
            gl.active_texture(WebGl2RenderingContext::TEXTURE0);
            let pong_texture = create_texture(gl).unwrap();
            let base_img = Image::new(w, h);
            load_into(gl, &base_img);

            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&pong_texture));
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&pong_fb));
            gl.framebuffer_texture_2d(
                WebGl2RenderingContext::FRAMEBUFFER,
                WebGl2RenderingContext::COLOR_ATTACHMENT0,
                WebGl2RenderingContext::TEXTURE_2D,
                Some(&pong_texture),
                level,
            );
            self.ping_pong = Some(((ping_texture, ping_fb), (pong_texture, pong_fb)));
            if let Some((a, b)) = &self.ping_pong {
                ((&a.0, &a.1), (&b.0, &b.1))
            } else {
                panic!("lk")
            }
        };

        gl.viewport(0, 0, w as i32, h as i32);

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let base_location = gl.get_uniform_location(program, "baseSampler").unwrap();
        gl.uniform1i(Some(&base_location), 0);

        let active_location = gl.get_uniform_location(program, "activeSampler").unwrap();
        gl.uniform1i(Some(&active_location), 1);

        let location = gl.get_uniform_location(program, "baseAlpha").unwrap();
        gl.uniform1f(Some(&location), 1.0);

        let location = gl.get_uniform_location(program, "basePosition").unwrap();
        gl.uniform2f(
            Some(&location),
            destination.position.x as f32,
            destination.position.y as f32,
        );

        let location = gl.get_uniform_location(program, "baseSize").unwrap();
        gl.uniform2f(Some(&location), w as f32, h as f32);

        let mut ping_pong = true;

        for (_mode, img, position, alpha, visible, marker) in children {
            if !visible {
                continue;
            }
            if ping_pong {
                gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(ping_fb));
                gl.active_texture(WebGl2RenderingContext::TEXTURE0);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(pong_texture));
            } else {
                gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(pong_fb));
                gl.active_texture(WebGl2RenderingContext::TEXTURE0);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(ping_texture));
            }
            ping_pong = !ping_pong;

            let location = gl.get_uniform_location(program, "activeAlpha").unwrap();
            gl.uniform1f(Some(&location), alpha as f32);

            let location = gl.get_uniform_location(program, "activePosition").unwrap();
            gl.uniform2f(Some(&location), position.x as f32, position.y as f32);

            let location = gl.get_uniform_location(program, "activeSize").unwrap();
            gl.uniform2f(Some(&location), img.width() as f32, img.height() as f32);

            gl.active_texture(WebGl2RenderingContext::TEXTURE1);
            if let Some(texture) = marker.and_then(|x| self.cache.get(&x)) {
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
            } else {
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture2));
                load_into(gl, img);
            }

            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
        }

        let mut result = vec![0u8; (4 * w * h) as usize];
        gl.read_pixels_with_opt_u8_array(
            0,
            0,
            w as i32,
            h as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&mut result),
        )
        .unwrap();

        let buf = RgbaImage::from_raw(w, h, result).unwrap();
        Image { buf }
    }
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn create_program(gl: &WebGl2RenderingContext, vs: &str, fs: &str) -> Result<WebGlProgram, String> {
    let vs = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vs)?;
    let fs = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, fs)?;
    link_program(gl, &vs, &fs)
}

fn create_texture(gl: &mut WebGl2RenderingContext) -> Result<WebGlTexture, String> {
    let texture = gl.create_texture().unwrap();
    gl.active_texture(WebGl2RenderingContext::TEXTURE0);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_S,
        WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_T,
        WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
        WebGl2RenderingContext::LINEAR as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
        WebGl2RenderingContext::LINEAR as i32,
    );
    Ok(texture)
}

fn load_into(gl: &mut WebGl2RenderingContext, img: &Image) {
    let img_data: ImageData = img.into();
    gl.tex_image_2d_with_u32_and_u32_and_image_data(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGBA as i32,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &img_data,
    )
    .unwrap();
}
