import { Button, Field, Input, Text } from "@fluentui/react-components";
import { useEffect } from "react";
import { auth_api } from "~/shared/api";
import { _fetch, FormDataObj } from "~/shared/util/fetch";

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
        _fetch((e.target as HTMLFormElement).action, cred)
          .then((res) => {
            console.log(res);
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
        action={auth_api.password}
      >
        <input hidden type="text" autoComplete="username" />
        <Field label="Password">
          <Input
            type="password"
            autoComplete="current-password"
            name="password"
          />
        </Field>

        <div />
        <div className="px-5 flex items-center justify-center">
          <Button appearance="primary" type="submit" size="medium">
            SIGN-IN
          </Button>
        </div>

        <Text
          style={{
            color: "gray",
          }}
        >
          password will be ignored in this demo.
        </Text>
      </form>
    </div>
  );
}
