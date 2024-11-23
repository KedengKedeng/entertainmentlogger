import { DataValidator } from "../DataValidator";

export interface UserLogin {
  email: string;
  password: string;
}
const USER_LOGIN_OBJECT_LENGTH = 2;

export class UserLoginValidator extends DataValidator<UserLogin> {
  validate(): boolean {
    if (
      USER_LOGIN_OBJECT_LENGTH != Object.keys(this.data).length ||
      typeof this.data.email != "string" ||
      typeof this.data.password != "string"
    )
      return false;

    return true;
  }
}
