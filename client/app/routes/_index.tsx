import { Button, Input } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { auth_api } from "~/shared/api";
import { _fetch, FormDataObj } from "~/shared/util/fetch";
import { useCheckWebAuthAvailable } from "~/hooks/use-check-web-auth-available";
import "../styles/fade-in-text.css";
import { BottomTextUi } from "~/shared/components/bottom-text-ui.component";

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

  const text = `Please enter any name.\nIf the name exists, you will be logged in with it.\nIf it does not exist, a new account will be created.`;

  return (
    <div className="flex flex-col h-full items-center bg-transparent justify-between">
      <div className="h-[10%]" />
      <form
        className="w-[300px] shadow-lg rounded-md px-3 py-5 flex flex-col gap-3 bg-blue-900/80"
        method="POST"
        action={auth_api.username}
      >
        <Input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoComplete="username webauthn"
          name="username"
          autoFocus
        />

        <div />
        <div className="px-5 flex items-center justify-center">
          <Button
            type="submit"
            size="medium"
            className="hover:shadow-xl"
            disabled={loading}
            style={{
              borderColor: "#1e40af",
              background: "#1e40af",
              color: "white",
            }}
          >
            NEXT
          </Button>
        </div>
      </form>

      <BottomTextUi text={text} />
    </div>
  );
}
