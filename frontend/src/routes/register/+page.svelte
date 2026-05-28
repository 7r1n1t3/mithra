<script>
	import Box from   '$lib/Box.svelte';
	import Logo from  '$lib/Logo.svelte';
	import Error from '$lib/Error.svelte';
	import Input from '$lib/Input.svelte';

	let register_button_pressed = $state(false);

	let username = $state('');
	let display_name = $state('');
	let email = $state('');
	let password = $state('');
	let confirm_password = $state('');

	let valid_username = $derived(username.trim().length > 0);
	let valid_display_name = $derived(display_name.trim().length > 0);
	let valid_email_pattern = /^\S+@\S+\.\S+$/ ;
	let valid_email = $derived(valid_email_pattern.test(email));
	let valid_password = $derived(password.length >= 8);
	let passwords_match = $derived(password === confirm_password);

	let all_valid = $derived(
		valid_username &&
		valid_display_name &&
		valid_email &&
		valid_password &&
		passwords_match
	);

	function register_user() {
		register_button_pressed = true;
		if (all_valid) {
			fetch('/api/register', {
			  method: 'POST',
			  headers: {
				'Content-Type': 'application/json'
			  },
			  body: JSON.stringify({
				  username: username,
				  display_name: display_name,
				  email: email,
				  password: password
			  })
			});
		}
	}
</script>

<div class="register">
	<Box>
		<Logo/>
		<h1 class="title">Create account</h1>
		<form id="submit-form" class="submit-form">
			<Input bind:value={username} name='Username'/>
			<Input bind:value={display_name} name='Display name'/>
			<Input bind:value={email} name='email' type='email'/>
			<Input bind:value={password} name='password' type='Password'/>
			<Input bind:value={confirm_password} name='confirm password' type='Password'/>

		</form>

		<div class="sign-buttons">
			<button type="submit" onclick={register_user}>register</button>
			<a href="/signin">sign in</a>
		</div>

		{#if !passwords_match}
			<Error content='passwords do not match.' />
		{:else if password !== '' && !valid_password}
			<Error content='password must contain a minimum of 8 characters.' />
		{:else if !valid_email && register_button_pressed} 
			<Error content="are you sure that's an email?" />
		{:else if !all_valid && register_button_pressed} 
			<Error content="something seems wrong..." />
		{/if}
	</Box>
</div>

<style>
	.register {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 1rem;
	}

	.submit-form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 1rem;
	}
</style>
