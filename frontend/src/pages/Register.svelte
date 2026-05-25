<script lang="ts">
	import Error from '../lib/Error.svelte';
	import Input from '../lib/Input.svelte';

	let username = $state('');
	let display_name = $state('');
	let email = $state('');
	let password = $state('');
	let confirm_password = $state('');

	let valid_password = $derived(password === confirm_password);
	let valid_email_pattern = /^\S+@\S+\.\S+$/ ;
	let valid_email = $derived(valid_email_pattern.test(email));

	let all_valid = $derived(valid_email && valid_password);
	let all_empty = $derived(username === '' && display_name === '' && email === '' && password === '');

	function register_user() {
		if (all_valid) {
			fetch('/api/register', {
			  method: 'POST',
			  headers: {
				'Content-Type': 'application/json'
			  },
			  body: JSON.stringify({ key: 'value' })
			});
		}
	}
</script>

<main>
    <h1>Mithra</h1>
    <h2>Create account</h2>

    <form id="submit-form">
		<Input bind:value={username} name='username'/>
		<br/>
		<Input bind:value={display_name} name='display_name'/>
		<br/>
		<Input bind:value={email} name='email' type='email'/>
		<br/>
		<Input bind:value={password} name='password' type='password'/>
		<br/>
		<Input bind:value={confirm_password} name='username' type='password'/>
		<br/>

		{#if !valid_password}
			<Error content='passwords do not match' />
		{:else if !valid_email && email !== ''} 
			<Error content="are you sure that's an email" />
		{:else if !all_valid && !all_empty} 
			<Error content="something seems wrong..." />
		{/if}

    </form>

	<button type="submit" onclick={register_user}>register</button>
	<a href="/signin">sign in</a>
	<br />
</main>
