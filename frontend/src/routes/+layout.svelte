<script>
	let { children } = $props();
	export const ssr = false;
</script>

{@render children()}

<div class="footer">
	<a href="https://git.hlsec.top/7r1n1t3/mithra">
		<img src="https://git.hlsec.top/assets/img/logo.svg" alt="link to repo" class="repo-link">
	</a>
</div>

<style>
	.footer {
		position: fixed;
		bottom: 1rem;
		left: 50%;
		transform: translateX(-50%);
	}

	.repo-link {
		height: 1.5rem;
	}
</style>
