import { useCallback } from "react";
import { auth_api } from "~/shared/api";
import { _fetch } from "~/shared/util/fetch";

interface UseUpdateRegisterKeyNameVariables {
  credId: string;
  newName: string;
}

export const useUpdateRegisterKeyName = () => {
  const updateRegisterKeyName = useCallback(
    (variables: UseUpdateRegisterKeyNameVariables) => {
      return _fetch(auth_api.renameKey, {
        payload: variables,
      }).then((res) => {
        if (res.data) {
          return res.data;
        }

        alert(res?.message ?? "Failed to update key name");
      });
    },
    []
  );

  return [updateRegisterKeyName] as const;
};
