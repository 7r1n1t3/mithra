import { getTextDirection } from '$lib/paraglide/runtime';
import { paraglideMiddleware } from '$lib/paraglide/server';

/** @type {import('@sveltejs/kit').Handle} */ const handleParaglide = ({ event, resolve }) => paraglideMiddleware(event.request, ({ request, locale }) => {
	event.request = request;

	return resolve(event, {
		transformPageChunk: ({ html }) => html.replace('%paraglide.lang%', locale).replace('%paraglide.dir%', getTextDirection(locale))
	});
});

export /** @type {import('@sveltejs/kit').Handle} */ const handle = handleParaglide;
