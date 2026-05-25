<script lang="ts">
  import Logo from './lib/Logo.svelte';
  import Register from './pages/Register.svelte';
</script>

<div class="app">
	<Logo />

	<main class="body">
		<Register />
	</main>

	<p>Mithra <a href="https://git.hlsec.top/7r1n1t3/mithra">repo</a></p>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, sans-serif;
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
