<script lang="ts">
	import Button from './Button.svelte';
	import Separator from './Separator.svelte';

	export let show = false;
	export let title = '';
	export let isError = true;
</script>

{#if show}
	<div class="overlay" on:click={() => (show = false)} on:keydown>
		<div class="popup" on:click={(e) => e.stopImmediatePropagation()} on:keydown>
			<header class="popup-header">
				<span>
					{#if isError}
						<i class="fa fa-exclamation-triangle" />
					{/if}
					{title}
				</span>
				<Button on:click={() => (show = false)}>
					<i class="fa fa-times" />
				</Button>
			</header>
			<Separator />
			<slot />
		</div>
	</div>
{/if}

<style>
	.overlay {
		display: flex;
		align-items: center;
		justify-items: center;
		justify-content: center;
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		/** TODO */
		background-color: rgba(var(--color-lightest), 0.2);
		z-index: 1;
	}
	.popup {
		min-width: 400px;
		min-height: 400px;
		max-width: 90vw;
		border: 1px solid var(--color-stronger);
		border-radius: 5px;
		background-color: var(--color-lightest);
		position: relative;
		opacity: 100%;
	}

	.popup-header {
		display: flex;
		align-items: center;
		justify-items: center;
		gap: 5px;
		padding-top: 5px;
		padding-bottom: 5px;
		padding-right: 5px;
		padding-left: 15px;
		justify-content: space-between;
	}
</style>
