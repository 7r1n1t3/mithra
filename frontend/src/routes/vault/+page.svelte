<script>
	import Logo from "$lib/Logo.svelte";
	import Card from "$lib/vault/Card.svelte";
	import Button from "$lib/vault/Button.svelte";

	let codes = [
		{
			name: "example",
			code: "123456",
		},
		{
			name: "example",
			code: "123456",
		},
	];
</script>

<div class="logo">
	<Logo />
</div>

<div class="codes">
	{#each codes as code}
		<Card totp_title={code.name} totp_generated={code.code}></Card>
	{/each}
</div>

<div class="new-totp-button">
	<a href="/vault/register-code">
		<Button title="New" />
	</a>
</div>

<style>
	.logo {
		position: fixed;
		top: 1rem;
		left: 1rem;
	}

	.new-totp-button {
		position: fixed;
		top: 1rem;
		right: 1rem;
	}

	.codes {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>
