import { useEffect } from "react";
import { auth_api } from "~/shared/api";
import "../styles/fade-in-text.css";
import "../styles/form-style.css";
import { CustomInput } from "~/shared/components/custom-input.component";
import { _fetch, FormDataObj } from "~/shared/util/fetch";
import { BottomTextUi } from "~/shared/components/bottom-text-ui.component";

export default function Reauth() {
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
              location.href = "http://localhost:5173/home";
              return;
            }

            alert(res?.message);
          })
          .catch((err) => alert(err.message));
      }
    });
  }, []);

  const text = `Password will be ignored in this demo.`;

  return (
    <div className="flex flex-col h-full items-center justify-between w-full background-style">
      <div />
      <form
        className="w-[300px] shadow-lg rounded-md px-3 py-5 flex flex-col custom-form"
        method="POST"
        action={auth_api.password}
      >
        <CustomInput
          autoFocus
          autoComplete="current-password"
          name="password"
          placeholder="Input Password"
          type="password"
          title="Your Password"
          subText="Please enter your password"
        />

        <input hidden type="text" autoComplete="username" />

        <div className="px-5 flex items-center justify-center">
          <button className="custom-form-button min-w-[100px] flex gap-2 items-center justify-center">
            <span>SIGN-IN</span>
          </button>
        </div>
      </form>
      <BottomTextUi text={text} />
    </div>
  );
}
