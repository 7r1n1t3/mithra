<script>
	import Box from '$lib/Box.svelte';
	import Logo from  '$lib/Logo.svelte';
	import Input from  '$lib/Input.svelte';
	import ErrorComp from '$lib/Error.svelte';
	import Notice from '$lib/Notice.svelte';
	import { m } from '$lib/paraglide/messages.js';

	let error_message = $state('');
	let notice_message = $state('');
	let email_address = $state('');
	let password = $state('');

	let valid_email_pattern = /^\S+@\S+\.\S+$/ ;
	let valid_email = $derived(valid_email_pattern.test(email_address));

	function check_arg_valid() {
		if (!valid_email)  {
			error_message = m.invalid_email();
		} else {
			error_message = "";
			return true;
		}
	}

	async function signin() {
		if (!check_arg_valid()) {
			return;
		}

		const response = await fetch('/api/signin', {
		  method: 'POST',
		  headers: {
			'Content-Type': 'application/json'
		  },
		  body: JSON.stringify({
			  email_address: email_address,
			  password: password
		  })
		});

		const body = await response.json();
		if (!response.ok) {
			error_message = body.failure_reason ??
				`Request failed with status ${response.status}`
		}

		if (body.success === true) {
			notice_message = m.successful_signin();
		} else {
			error_message = body.failure_reason ??
				`Internal server error: ${body.failure_reason}`;
		}
	}
</script>

<div class="signin">
	<Box>
		<Logo/>
		<h1 class="title">{m.signin()}</h1>
		<form id="signin-form">
			<Input bind:value={email_address} name={m.email()} type='email'/>
			<Input bind:value={password} name={m.password()} type='Password'/>
		</form>
		<div class="sign-buttons">
			<button type="submit" onclick={signin}>{m.signin()}</button>
			<a href="/register">{m.register()}</a>
		</div>

		{#if error_message !== ''}
			<ErrorComp content={error_message}/>
		{/if}
		{#if notice_message !== ''}
			<Notice content={notice_message}/>
		{/if}
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
