import { Button, Field, Input, Text } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { auth_api } from "~/shared/api";
import { _fetch, FormDataObj } from "~/shared/util/fetch";
import { useCheckWebAuthAvailable } from "~/hooks/use-check-web-auth-available";

export default function Index() {
  const { checkWebAuthAvailable } = useCheckWebAuthAvailable();
  const [username, setUsername] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);

  const checkWebAuth = async () => {
    const { username } = await checkWebAuthAvailable();
    if (username) {
      setLoading(true);
      setUsername(username);

      setTimeout(() => {
        setLoading(false);
        location.href = "http://localhost:5173/home";
      }, 2000);
    }
  };

  useEffect(() => {
    checkWebAuth();
  }, []);

  useEffect(() => {
    const form = document.querySelector("form");
    form?.addEventListener("submit", async (e) => {
      e.preventDefault();
      e.stopImmediatePropagation();

      if (e.target) {
        const formData = new FormData(e.target as HTMLFormElement);
        const cred: FormDataObj = {};

        formData.forEach((v, k) => (cred[k] = v));
        _fetch((e.target as HTMLFormElement).action, {
          payload: cred,
        })
          .then((res) => {
            if (res?.data?.id) {
              location.href = "http://localhost:5173/reauth";
              return;
            }
            alert(res?.message);
          })
          .catch(console.log);
      }
    });
  }, []);

  return (
    <div className="flex flex-col h-full items-center">
      <form
        className="w-[300px] bg-gray-100 shadow-md rounded-md px-3 py-5 flex flex-col gap-3"
        method="POST"
        action={auth_api.username}
      >
        <Field label="Username">
          <Input
            type="text"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            autoComplete="username webauthn"
            name="username"
            autoFocus
          />
        </Field>

        <Text
          style={{
            color: "gray",
          }}
        >
          Any username can be accepted.
        </Text>

        <div />
        <div className="px-5 flex items-center justify-center">
          <Button
            appearance="primary"
            type="submit"
            size="medium"
            disabled={loading}
          >
            NEXT
          </Button>
        </div>
      </form>
    </div>
  );
}
