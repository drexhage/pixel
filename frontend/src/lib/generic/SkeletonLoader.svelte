<div class="skeleton-box">
	<div class="skeleton-loader" />
</div>

<style>
	.skeleton-box {
		width: 100%;
		height: 100%;
		overflow: hidden;
		background: var(--color-lightest);
	}

	.skeleton-loader {
		width: 100%;
		height: 100%;
		display: block;
		background: linear-gradient(
				to right,
				var(--color-lightest),
				var(--color-lighter) 50%,
				var(--color-lightest) 90%
			),
			var(--color-stronger);
		background-repeat: repeat-y;
		background-size: 100% 100%;
		background-position: 0 0;
		animation: shine 2s linear infinite;
		transform: translate(-100%, 0);
	}
	@keyframes shine {
		0% {
			transform: translate(-100%, 0);
		}
		100% {
			transform: translate(100%, 0);
		}
	}
</style>
