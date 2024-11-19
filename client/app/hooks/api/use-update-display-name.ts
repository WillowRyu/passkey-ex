import { useCallback } from "react";
import { ResponseData, UserInfo } from "~/models/response_model";
import { auth_api } from "~/shared/api";
import { _fetch } from "~/shared/util/fetch";

export const useUpdateDisplayName = () => {
  const updateDisplayName: (
    newDisplayName: string
  ) => Promise<UserInfo | undefined> = useCallback((newDisplayName: string) => {
    return _fetch(auth_api.update_dispaly_name, {
      payload: {
        display_name: newDisplayName,
      },
    }).then((res: ResponseData<UserInfo>) => {
      if (res?.data?.displayname) {
        return res.data;
      }

      alert(res?.message ?? "Failed to update display name");
      return;
    });
  }, []);

  return [updateDisplayName] as const;
};
