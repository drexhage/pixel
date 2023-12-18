<script>
</script>

<div class="box">
	<slot />
</div>

<style>
	.box {
		display: inline-flex;
		flex-direction: row;
		gap: 0;
	}

	/** global css is used here, assuming the child components are buttons */
	.box :global(.btn) {
		display: inline-block;
		position: relative;
		&:not(:first-child) {
			border-top-left-radius: 0;
			border-bottom-left-radius: 0;
			border-left: 0;
		}
		&:not(:last-child) {
			border-top-right-radius: 0;
			border-bottom-right-radius: 0;
			border-right: 0;
			&::after {
				content: '';
				position: absolute;
				width: 3px;
				right: -3px;
				height: 70%;
				background: var(--color-strong);
				top: 50%;
				transform: translate(-50%, -50%);
			}
		}
	}
</style>
