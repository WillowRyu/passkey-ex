import { useEffect, useState } from "react";
import { auth_api } from "~/shared/api";
import { _fetch, FormDataObj } from "~/shared/util/fetch";
import { useCheckWebAuthAvailable } from "~/hooks/use-check-web-auth-available";
import "../styles/fade-in-text.css";
import "../styles/form-style.css";
import { BottomTextUi } from "~/shared/components/bottom-text-ui.component";
import { CustomInput } from "~/shared/components/custom-input.component";

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
    } else {
      setLoading(false);
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

  const text = `If the name exists, you will be logged in with it.\nIf it does not exist, a new account will be created.`;

  return (
    <div className="flex flex-col h-full items-center justify-between w-full background-style">
      <div className="h-[10%]" />
      <form
        className="w-[300px] shadow-lg rounded-md px-3 py-5 flex flex-col custom-form"
        method="POST"
        action={auth_api.username}
      >
        <CustomInput
          autoFocus
          autoComplete="username webauthn"
          name="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder=""
          title="Your Name"
          subText="Please enter any name."
        />
        <div className="px-5 flex items-center justify-center">
          <button
            className="custom-form-button min-w-[100px] flex gap-2 items-center justify-center"
            disabled={loading}
          >
            <span>NEXT</span>
            {loading && <span className="loadingSpinner" />}
          </button>
        </div>
      </form>

      <BottomTextUi text={text} />
    </div>
  );
}
