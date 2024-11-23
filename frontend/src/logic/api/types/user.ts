import { DataValidator } from "../DataValidator";

export interface User {
  id: string;
  created: string;
  last_online: string;

  email: string;

  username: string;
  profile_picture?: string;
  bio: string;
}

export interface NewUser {
  email: string;
  password: string;
  username: string;
}
const NEW_USER_OBJECT_LENGTH = 3;

export class NewUserValidator extends DataValidator<NewUser> {
  validate(): boolean {
    if (
      NEW_USER_OBJECT_LENGTH != Object.keys(this.data).length ||
      typeof this.data.email != "string" ||
      typeof this.data.password != "string" ||
      typeof this.data.username != "string"
    )
      return false;

    return true;
  }
}
