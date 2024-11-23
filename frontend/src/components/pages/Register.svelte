<script lang="ts">
  import { APIReponseCodes } from "$logic/api/ApiConnection";
  import { login } from "$logic/api/routes/login";
  import { register } from "$logic/api/routes/register";
  import type { NewUser } from "$logic/api/types/user";
  import { navigate } from "astro/virtual-modules/transitions-router.js";

  async function onSubmit(e: Event) {
    e.preventDefault();

    const form = new FormData(e.target as HTMLFormElement);
    const registerData: NewUser = {
      email: form.get("email") as string,
      username: form.get("username") as string,
      password: form.get("password") as string,
    };

    const registerResponse = await register(registerData);

    const loginResponse = await login({
      email: registerData.email,
      password: registerData.password,
    })

    // TODO: Give users feedback when login goes wrong
    switch (loginResponse.status) {
      case APIReponseCodes.OK:
        navigate("/");
        break;
      default:
        throw new Error(loginResponse.error);
        break;
    }
  }

  let passwordValue: string = $state("");
  let confirmPasswordValue: string = $state("");

  const passwordMatch = $derived(
    passwordValue === confirmPasswordValue && passwordValue.length > 0
  );
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

    <label for="username">Username</label>
    <input
      type="text"
      name="username"
      id="username"
      class="border rounded-lg p-2 text-black"
    />

    <label for="password">Password</label>
    <input
      bind:value={passwordValue}
      type="password"
      name="password"
      id="password"
      class="border rounded-lg p-2 text-black"
    />

    <label for="password">Confirm Password</label>
    <input
      bind:value={confirmPasswordValue}
      type="password"
      name="password"
      id="password"
      class="border rounded-lg p-2 text-black"
    />

    <button
      type="submit"
      class="bg-base-500 rounded-lg p-2 {passwordMatch
        ? ''
        : 'pointer-events-none opacity-30'}">Register</button
    >
  </form>
</div>
