import { api } from "../ApiConnection";
import { UserLoginValidator, type UserLogin } from "../types/login";

export async function login(loginData: UserLogin) {
  const loginDataValidator = new UserLoginValidator(loginData);

  const response = await api.post<UserLogin, UserLoginValidator, boolean>(
    "/api/login",
    loginDataValidator,
  );

  return response;
}
