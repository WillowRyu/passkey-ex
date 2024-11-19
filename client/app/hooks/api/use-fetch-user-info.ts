import { useCallback } from "react";
import { ResponseData, UserInfo } from "~/models/response_model";
import { auth_api } from "~/shared/api";
import { _fetch, redirectHome } from "~/shared/util/fetch";

export const useFetchUserInfo = () => {
  const fetchUserInfo: () => Promise<UserInfo | undefined> = useCallback(() => {
    return _fetch(auth_api.userinfo).then((res: ResponseData<UserInfo>) => {
      if (res.data) {
        const { data } = res;
        return data;
      }

      redirectHome(res?.message ?? "Failed to fetch user info");
    });
  }, []);

  return [fetchUserInfo] as const;
};
