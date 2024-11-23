<script lang="ts">
  import { APIReponseCodes } from "$logic/api/ApiConnection";
  import { login } from "$logic/api/routes/login";
  import type { UserLogin } from "$logic/api/types/login";
  import { navigate } from "astro/virtual-modules/transitions-router.js";

  async function onSubmit(e: Event) {
    e.preventDefault();

    const form = new FormData(e.target as HTMLFormElement);
    const loginData: UserLogin = {
      email: form.get("email") as string,
      password: form.get("password") as string,
    };

    const loginResponse = await login(loginData);

    // TODO: Give users feedback when login goes wrong
    switch (loginResponse.status) {
      case APIReponseCodes.OK:
        navigate("/");
      default:
        throw new Error(loginResponse.error);
    }
  }
</script>

<div
  class="flex flex-col rounded-xl p-4 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-base-400"
>
  <form action="post" onsubmit={onSubmit} class="flex flex-col gap-2">
    <label for="email">Email</label>
    <input
      type="text"
      name="email"
      id="email"
      class="border rounded-lg p-2 text-black"
    />

    <label for="password">Password</label>
    <input
      type="password"
      name="password"
      id="password"
      class="border rounded-lg p-2 text-black"
    />

    <button type="submit" class="bg-base-500 rounded-lg p-2">Login</button>
  </form>
  <p>
    Don't have an account? <a href="/register" class="underline text-blue-400"
      >Register</a
    >
  </p>
</div>
