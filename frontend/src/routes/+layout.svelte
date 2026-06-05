<script>
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { locales, localizeHref } from '$lib/paraglide/runtime';
	import { setLocale } from '$lib/paraglide/runtime';

	let { children } = $props();

	let locale = $state('en');
</script>

{@render children()}

<div class="left-footer">
	<a href="https://git.hlsec.top/7r1n1t3/mithra">
		<img
			src="https://git.hlsec.top/assets/img/logo.svg"
			alt="link to repo"
			class="repo-link"
		/>
	</a>
</div>
<div class="right-footer">
	
	<select name="locale" id="locale" bind:value={locale} onchange={() => setLocale(locale)}>
	  <option value="en">English</option>
	  <option value="de">German</option>
	  <option value="fr">French</option>
	  <option value="ar">Arabic</option>
	  <option value="ja">Japanese</option>
	</select>
</div>

<div style="display:none">
	{#each locales as locale (locale)}
		<a
			href={resolve(localizeHref(page.url.pathname, { locale }))}
		>{locale}</a>
	{/each}
</div>

<style>
	.left-footer {
		position: fixed;
		bottom: 1rem;
		left: 1rem;
	}

	.right-footer {
		position: fixed;
		bottom: 1rem;
		right: 1rem;
	}

	.repo-link {
		height: 1.5rem;
	}
</style>
