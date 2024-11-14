import type { DataValidator } from "./DataValidator";

export enum APIReponseCodes {
  OK = 200,
  BadRequest = 400,
  Unauthorized = 401,
  NotFound = 404,
  MissingFields = 422,
  InternalServerError = 500,
}

export interface APIResponse<T> {
  data?: T;
  status: APIReponseCodes;
  error?: string;
}

export class APIConnection {
  private url: string;

  constructor(url: string) {
    this.url = url;
  }

  public async get<ReturnType>(path: string): Promise<APIResponse<ReturnType>> {
    const response = await fetch(this.url + path, {
      method: "GET",
      headers: {
        allowCredentials: "true",
      },
    });

    if (!response.ok) {
      return {
        status: response.status,
        error: await response.text(),
      };
    }

    const responseData = await response.json();

    return {
      status: APIReponseCodes.OK,
      data: responseData,
    };
  }

  public async post<
    InputType,
    InputTypeClass extends DataValidator<InputType>,
    ReturnType,
  >(path: string, data: InputTypeClass): Promise<APIResponse<ReturnType>> {
    if (!data.validate())
      return {
        status: APIReponseCodes.MissingFields,
        error: "Missing fields",
      };

    const response = await fetch(this.url + path, {
      method: "POST",
      headers: {
        allowCredentials: "true",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      return {
        status: response.status,
        error: await response.text(),
      };
    }

    const responseData = await response.json();

    return {
      status: APIReponseCodes.OK,
      data: responseData,
    };
  }

  public async patch<
    InputType,
    InputTypeClass extends DataValidator<InputType>,
    ReturnType,
  >(path: string, data: InputTypeClass): Promise<APIResponse<ReturnType>> {
    if (!data.validate())
      return {
        status: APIReponseCodes.MissingFields,
        error: "Missing fields",
      };

    const response = await fetch(this.url + path, {
      method: "PATCH",
      headers: {
        allowCredentials: "true",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      return {
        status: response.status,
        error: await response.text(),
      };
    }

    const responseData = await response.json();

    return {
      status: APIReponseCodes.OK,
      data: responseData,
    };
  }

  public async delete<ReturnType>(path: string): Promise<APIResponse<ReturnType>> {
    const response = await fetch(this.url + path, {
      method: "DELETE",
      headers: {
        allowCredentials: "true",
      },
    });

    if (!response.ok) {
      return {
        status: response.status,
        error: await response.text(),
      };
    }

    const responseData = await response.json();

    return {
      status: APIReponseCodes.OK,
      data: responseData,
    };
  }
}
