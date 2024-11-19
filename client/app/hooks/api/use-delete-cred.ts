import { useCallback } from "react";
import { auth_api } from "~/shared/api";
import { _fetch } from "~/shared/util/fetch";

export const useDeleteCred = () => {
  const deletCred: (credId: string) => Promise<boolean> = useCallback(
    (credId: string) => {
      return _fetch(`${auth_api.removeKey}?credId=${credId}`).then(
        async (res) => {
          if (res.status === "OK") {
            return true;
          }
          alert(res?.message ?? "An error occurred");
          return false;
        }
      );
    },
    []
  );

  return [deletCred] as const;
};
