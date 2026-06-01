<script>
	import Box from   '$lib/Box.svelte';
	import Logo from  '$lib/Logo.svelte';
	import Input from '$lib/Input.svelte';
	import ErrorComp from '$lib/Error.svelte';
	import Notice from '$lib/Notice.svelte';

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
			error_message = 'passwords do not match.';
		} else if (password !== '' && !valid_password) {
			error_message = 'password must contain a minimum of 8 characters.';
		} else if (!valid_email)  {
			error_message = "are you sure that's an email?";
		} else if (!all_valid) {
			error_message = "something seems wrong...";
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

		try {
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
				throw new Error(
					body.failure_reason ??
					`Request failed with status ${response.status}`
				);
			}

			if (body.success === true) {
				notice_message = `User ${body.username ?? username} succesfully created.`;
			} else {
				error_message = body.failure_reason ??
					`Internal server error: ${body.failure_reason}`;
			}
		} catch (error) {
			error_message = error instanceof Error
				? error.message
				: 'Internal server error';
		}
	}
</script>

<div class="register">
	<Box>
		<Logo/>
		<h1 class="title">Create account</h1>
		<form id="submit-form" class="submit-form" method="POST" submit={register_user}>
			<Input bind:value={username} name='Username'/>
			<Input bind:value={display_name} name='Display name'/>
			<Input bind:value={email} name='email' type='email'/>
			<Input bind:value={password} name='password' type='Password'/>
			<Input bind:value={confirm_password} name='confirm password' type='Password'/>

			<div class="sign-buttons">
				<input type="submit" value="Register" onclick={register_user}>
				<a href="/signin">sign in</a>
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
