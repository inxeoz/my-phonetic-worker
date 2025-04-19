import init, { PhoneticConverter } from '../pkg/phonetic.js';
import wasmModule from '../pkg/phonetic_bg.wasm'; // Direct import, no bindings
import index_html from '../public/index.html'

let wasmInitialized = false;
let converter;

// Ensure WASM is initialized
async function ensureWasmInitialized() {
	if (!wasmInitialized) {
		await init(wasmModule);
		wasmInitialized = true;
	}
}


async function handleRequest(request) {
	const { pathname } = new URL(request.url);

	const headers = {
		'Access-Control-Allow-Origin': '*', // Allow all origins, or specify your domain like 'http://localhost:5000'
		'Access-Control-Allow-Methods': 'POST, OPTIONS', // Allow specific methods
		'Access-Control-Allow-Headers': 'Content-Type, Authorization', // Allow specific headers
		'Access-Control-Allow-Credentials': 'true' // Allow credentials if needed
	};

	// Handle OPTIONS preflight request (browser sends this before actual request)
	if (request.method === 'OPTIONS') {
		return new Response(null, {
			status: 204, // No content
			headers: headers
		});
	}

	// Handle POST request
	if (pathname === '/api/convert' && request.method === 'POST') {
		try {
			const body = await request.json();
			const text = body.text;

			if (!text) {
				return new Response(
					JSON.stringify({ error: 'Missing \'text\' field' }),
					{
						status: 400,
						headers: { 'Content-Type': 'application/json', ...headers }
					}
				);
			}

			const result = await convertPhonetic(text);

			return new Response(
				JSON.stringify(result),
				{
					status: 200,
					headers: { 'Content-Type': 'application/json', ...headers }
				}
			);
		} catch (err) {
			return new Response(
				JSON.stringify({ error: 'Failed to process the request', details: err.message }),
				{
					status: 500,
					headers: { 'Content-Type': 'application/json', ...headers }
				}
			);
		}
	}

	return new Response(index_html, {
		status: 200,
		headers: {
			"Content-Type": "text/html",
			"Access-Control-Allow-Origin": "*",
		},
	});

}


async function convertPhonetic(text) {


	await ensureWasmInitialized();

	// Reuse the same converter instance
	if (!converter) {
		converter = new PhoneticConverter();
	}

	return converter.convert(text);

	// // Ensure the WASM module is initialized before using it
	// await ensureWasmInitialized();
	//
	// // Create a new instance of the PhoneticConverter
	// const converter = new PhoneticConverter();
	//
	// // Perform the phonetic conversion and return the result
	// return converter.convert(text);
}

// Export the Worker fetch handler
export default {
	fetch: handleRequest
};
