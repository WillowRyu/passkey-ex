import { useCallback } from "react";
import { auth_api } from "~/shared/api";

export const useLogout = () => {
  const logout = useCallback(async () => {
    const response = await fetch(auth_api.logout, {
      method: "GET",
    });

    if (response.status === 200) {
      location.href = "http://localhost:5173";
      return;
    } else {
      alert("Failed to logout");
    }
  }, []);

  return [logout] as const;
};
