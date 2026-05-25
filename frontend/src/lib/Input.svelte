<script>
	let { value = $bindable(''), name, type='text', required=true } = $props();
</script>

<main>
    <label>
    	{name}:
        <input bind:value={value} id={name} {name} {type} {required} />
    </label>
</main>
