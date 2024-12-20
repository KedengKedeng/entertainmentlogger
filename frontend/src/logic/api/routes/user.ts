import { jwtDecode } from "jwt-decode";
import { api, ssrApi } from "../ApiConnection";
import type { User } from "../types/user";
import { getCookie } from "$util/cookie";

export async function getUser(id: string) {
  const response = await ssrApi.get<User>(`/api/v1/user/${id}`);

  return response;
}

export async function getUserFromCookies(cookies: string | null) {
    if (!cookies) return;

    const token = getCookie("token", cookies);
    const decodedJWT = jwtDecode<{id: string}>(token!);

    return await getUser(decodedJWT.id);
}