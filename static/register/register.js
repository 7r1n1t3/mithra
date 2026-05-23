const form = document.getElementById("submit-form");
const output = document.getElementById("output");
const password = document.getElementById("password").value;
const confirm_password = document.getElementById("confirm-password").value;

if (password != confirm_password) {
	output.setHTML('<p style="color: red;">passwords do not match</p>');
}

form.addEventListener("submit", async (event) => {
	event.preventDefault();

	const payload = {
		username: document.getElementById("username").value,
		display-name: document.getElementById("display-name").value,
		email: document.getElementById("email").value,
		password: document.getElementById("password").value,
	};

	const response = await fetch("/api/register", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(payload),
	});

	const data = await response.json();
	output.textContent = JSON.stringify(data, null, 2);
});
