<script lang="ts">
	import { current, history } from '../store';

	function retrievePathToRoot(idx: number): number[] {
		if (!$history) return [];
		function retrievePathToRootRec(point: number, acc: number[]) {
			if (point === $history.root) {
				return acc.concat([point]);
			} else {
				return retrievePathToRootRec($history.nodes[point].parent, acc.concat([point]));
			}
		}
		let result = retrievePathToRootRec(idx, []);
		return result;
	}

	let path: number[];
	$: path = retrievePathToRoot($current);
</script>

<span>History:</span>
<ul>
	{#each { length: path.length } as _, i}
		{@const node = $history.nodes[path.length - 1 - i]}
		<li>{node.id}, {node.value.data.type}</li>
	{/each}
</ul>
