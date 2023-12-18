<script lang="ts">
	export let level: number;
</script>

<div class="level">
	{#each { length: level } as a, b}
		<div class="delimiter" />
	{/each}
</div>

<style>
	div.level {
		display: flex;
		height: 100%;
	}

	div.delimiter {
		height: 100%;
		width: 10px;
		border-right: 1px solid var(--color-strongest);
	}
</style>
