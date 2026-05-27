<script>
	let email = $state('');
	let password = $state('');
	let rememberme = $state(false);
</script>

<main>
    <h1>Mithra</h1>
    <h2>sign-in</h2>

    <form id="signin-form">
      <label>
        email:
        <input bind:value={email} id="email" name="email" type="email" required />
      </label>

      <br /><br />

      <label>
        password:
        <input bind:value={password} id="password" name="password" type="password" required />
      </label>

      <br /><br />

      <label>
        remember me
        <input bind:value={rememberme} id="rememberme" name="rememberme" type="checkbox" required />
      </label>

      <br /><br />

      <button type="submit">register</button>
    </form>
</main>
