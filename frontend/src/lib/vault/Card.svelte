<script>
	import Box from "$lib/Box.svelte";

	let { totp_title, totp_generated } = $props();
</script>

<Box>
	<div class="totp">
		<p class="totp-title">{totp_title}</p>
		<p class="totp-generated">{totp_generated}</p>
	</div>
</Box>

<style>
	.totp {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.totp-title {
		color: white;
	}

	.totp-generated {
		color: grey;
	}
</style>
