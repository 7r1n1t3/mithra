<script lang="ts">
	import Logo from  '../../lib/Logo.svelte';
	import Error from '../../lib/Error.svelte';
	import Input from '../../lib/Input.svelte';

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

<div class="app">
	<Logo />

	<main class="body">

		<h2>Create account</h2>

		<form id="submit-form">
			<Input bind:value={username} name='Username'/>
			<br/>
			<Input bind:value={display_name} name='Display name'/>
			<br/>
			<Input bind:value={email} name='email' type='email'/>
			<br/>
			<Input bind:value={password} name='password' type='Password'/>
			<br/>
			<Input bind:value={confirm_password} name='username' type='Password'/>
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

	<p>Mithra <a href="https://git.hlsec.top/7r1n1t3/mithra">repo</a></p>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, sans-serif;
	align-items: "center";
  }

  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .body {
    flex: 1;
    padding: clamp(1rem, 3vw, 3rem);
  }
</style>
