<script lang="ts">
	import SkeletonLoader from './SkeletonLoader.svelte';
</script>

<div class="box">
	<div class="skeleton-image">
		<SkeletonLoader />
	</div>
</div>

<style>
	.skeleton-image {
		border: 1px solid var(--color-stronger);
		border-radius: 5px;
		overflow: hidden;
		min-width: 500px;
		min-height: 500px;
		width: 40vw;
		height: 50vh;
	}

	.box {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
		background-color: var(--color-lightest);
	}
</style>
