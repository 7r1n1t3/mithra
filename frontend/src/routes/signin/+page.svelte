<script>
	import Box from '$lib/Box.svelte';
	import Logo from  '$lib/Logo.svelte';
	import Input from  '$lib/Input.svelte';

	let email = $state('');
	let password = $state('');
</script>

<div class="signin">
	<Box>
		<Logo/>
		<h1 class="title">Sign-in</h1>
		<form id="signin-form">
			<Input bind:value={email} name='email' type='email'/>
			<Input bind:value={password} name='password' type='Password'/>
		</form>
		<div class="sign-buttons">
			<button type="submit">sign-in</button>
			<a href="/register">register</a>
		</div>
	</Box>
</div>

<style>
	.signin {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 1rem;
	}

	#signin-form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 1rem;
	}
</style>
