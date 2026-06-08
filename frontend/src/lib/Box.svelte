<script>
	let { width = "50%", height = "30%", children } = $props();
</script>

<div class="box" style:width style:height>
	{@render children?.()}
</div>

<style>
	.box {
		border: 0.5px solid var(--color-muted-text);
		border-radius: 12px;
		padding: 30px;
		margin: 20px;

		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>
