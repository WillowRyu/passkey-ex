type Headers = {
  [key: string]: string;
};

export type FormDataObj = {
  [key: string]: FormDataEntryValue;
};

export type Payload = FormData | string | FormDataObj;

export const _fetch = async (path: string, payload: Payload) => {
  const headers: Headers = {
    "X-Requested-With": "XMLHttpRequest",
  };

  if (payload && !(payload instanceof FormData)) {
    headers["Content-Type"] = "application/json";
    payload = JSON.stringify(payload);
  }

  const response = await fetch(path, {
    method: "POST",
    credentials: "include",
    headers: headers,
    body: payload,
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
