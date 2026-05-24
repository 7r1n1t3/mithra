<script lang="ts">
  let message = '';

  async function checkHealth() {
    const res = await fetch('/api/health');
    message = await res.text();
  }
</script>

<button on:click={checkHealth}>
  Check backend
</button>

<p>{message}</p>
