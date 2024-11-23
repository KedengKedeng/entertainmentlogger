import { api } from "../ApiConnection";
import { NewUserValidator, type NewUser, type User } from "../types/user";

export async function register(registerData: NewUser) {
  const registerDataValidator = new NewUserValidator(registerData);

  const response = await api.post<NewUser, NewUserValidator, User>(
    "/api/v1/user",
    registerDataValidator,
  );

  return response;
}
