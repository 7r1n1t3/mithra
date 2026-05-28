<script>
	let { value = $bindable(''), name, type='text', required=true } = $props();
</script>

<main>
	<input bind:value={value} id={name} placeholder={name} class="input-field" {name} {type} {required} />
</main>

<style>
	.input-field {
		background-color: var(--color-background);
		border: 1px solid var(--color-muted-text);
		border-radius: 4px;
	}
</style>
