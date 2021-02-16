const fs = require("fs")
const lexoffice = require("./pkg/lexoffice_js.js");

const client = lexoffice.client(fs.readFileSync("/home/tox/.lexoffice", "utf-8"));

(async function() {
	let request = client.contact();

	let page = await request.page(0);
})();
