<script>
	import Vault from "../+page.svelte";
	import Input from "$lib/Input.svelte";
	import Box from "$lib/Box.svelte";
	import { m } from "$lib/paraglide/messages.js";

	let label = $state("");
	let secret = $state("");

	let error_message = $state("");
	let notice_message = $state("");

	async function register_code() {
		const response = await fetch("/api/register/code", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				label: label,
				secret: secret,
			}),
		});

		const body = await response.json();
		if (!response.ok) {
			error_message =
				body.failure_reason ??
				`Request failed with status ${response.status}`;
		}

		if (body.success === true) {
			notice_message = `code ${label} successfully registered`;
		} else {
			error_message =
				body.failure_reason ??
				`Internal server error: ${body.failure_reason}`;
		}
	}
</script>

<div class="register-code">
	<Box>
		<form id="submit-form" class="submit-form">
			<Input bind:value={label} name={m.label()} />
			<Input bind:value={secret} name={m.secret()} />

			<div class="buttons">
				<button type="submit" onclick={register_code}
					>{m.register_code()}</button
				>
			</div>
		</form>
	</Box>
</div>

<style>
	.register-code {
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
