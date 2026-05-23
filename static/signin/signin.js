const form = document.getElementById("submit-form");
const output = document.getElementById("output");

form.addEventListener("submit", async (event) => {
  event.preventDefault();

  const payload = {
    email: document.getElementById("email").value,
    password: document.getElementById("password").value,
  };

  const response = await fetch("/api/signin", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  const data = await response.json();
  output.textContent = JSON.stringify(data, null, 2);
});
