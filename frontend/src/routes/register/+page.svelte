<script>
	import Box from   '$lib/Box.svelte';
	import Logo from  '$lib/Logo.svelte';
	import Input from '$lib/Input.svelte';
	import ErrorComp from '$lib/Error.svelte';
	import Notice from '$lib/Notice.svelte';
	import { m } from '$lib/paraglide/messages.js';

	let register_button_pressed = $state(false);
	let notice_message = $state('');
	let error_message = $state('');

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

	function check_arg_valid() {
		if (!passwords_match) {
			error_message = m.passwords_do_not_match();
		} else if (password !== '' && !valid_password) {
			error_message = m.password_minimum_8_characters();
		} else if (!valid_email)  {
			error_message = m.invalid_email();
		} else if (!all_valid) {
			error_message = m.register_credentials_invalid();
		} else {
			error_message = "";
			return true;
		}
	}

	async function register_user() {
		register_button_pressed = true;

		if (!check_arg_valid()) {
			return;
		}

		const response = await fetch('/api/register', {
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

		const body = await response.json();
		if (!response.ok) {
			error_message = body.failure_reason ??
				`Request failed with status ${response.status}`
		}

		if (body.success === true) {
			notice_message = `User ${body.username ?? username} succesfully created.`;
		} else {
			error_message = body.failure_reason ??
				`Internal server error: ${body.failure_reason}`;
		}
	}
</script>

<div class="register">
	<Box>
		<Logo/>
		<h1 class="title">{m.create_account()}</h1>
		<form id="submit-form" class="submit-form">
			<Input bind:value={username} name={m.username()}/>
			<Input bind:value={display_name} name={m.display_name()}/>
			<Input bind:value={email} name={m.email()} type='email'/>
			<Input bind:value={password} name={m.password()} type='Password'/>
			<Input bind:value={confirm_password} name={m.confirm_password()} type='Password'/>

			<div class="sign-buttons">
				<button type="submit" onclick={register_user}>{m.register()}</button>
				<a href="/signin">{m.signin()}</a>
			</div>
		</form>

		{#if register_button_pressed}
			{#if error_message !== ''}
				<ErrorComp content={error_message}/>
			{/if}
			{#if notice_message !== ''}
				<Notice content={notice_message}/>
			{/if}
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
