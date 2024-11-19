import { useCallback } from "react";
import { CredentialsResponse } from "~/models/credentials_model";
import { auth_api } from "~/shared/api";
import { _fetch, redirectHome } from "~/shared/util/fetch";

export const useFetchKeys = () => {
  const fetchKeys: () => Promise<Array<CredentialsResponse>> = useCallback(
    () =>
      _fetch(auth_api.getkeys).then((res) => {
        if (res.data) {
          return res.data;
        }
        redirectHome(res?.message ?? "Failed to fetch keys");
      }),
    []
  );

  return [fetchKeys] as const;
};
