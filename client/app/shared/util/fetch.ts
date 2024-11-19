import { Navigate } from "@remix-run/react";

type Headers = {
  [key: string]: string;
};

export type FormDataObj = {
  [key: string]: FormDataEntryValue;
};

export type Payload = FormData | string | FormDataObj;

interface fetchOptions<T> {
  payload?: Payload | T;
}

export const _fetch = async <T>(path: string, options?: fetchOptions<T>) => {
  const headers: Headers = {
    "X-Requested-With": "XMLHttpRequest",
  };

  let payload = options?.payload ?? "";

  if (payload && !(payload instanceof FormData)) {
    headers["Content-Type"] = "application/json";
    payload = JSON.stringify(payload);
  }

  const response = await fetch(path, {
    method: "POST",
    credentials: "include",
    headers: headers,
    body: payload as BodyInit,
  });

  if (response.status === 200) {
    return response.json();
  }

  if (response.status >= 500) {
    const result = await response.json();
    throw new Error(result);
  }

  return response.json();
};

export const redirectHome = (message: string) => {
  alert(message);
  Navigate({
    to: "/",
    replace: true,
  });
};
