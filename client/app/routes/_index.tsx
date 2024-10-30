import { Button, Field, Input, Text } from "@fluentui/react-components";
import { useEffect } from "react";
import { useNavigate } from "@remix-run/react";
import { auth_api } from "~/shared/api";
import { _fetch, FormDataObj } from "~/shared/util/fetch";

export default function Index() {
  const navigate = useNavigate();

  useEffect(() => {
    const form = document.querySelector("form");
    form?.addEventListener("submit", async (e) => {
      e.preventDefault();
      e.stopImmediatePropagation();
      console.log(form);

      if (e.target) {
        const formData = new FormData(e.target as HTMLFormElement);
        const cred: FormDataObj = {};

        formData.forEach((v, k) => (cred[k] = v));
        _fetch((e.target as HTMLFormElement).action, cred)
          .then((user) => {
            console.log(user);
            navigate("/reauth");
          })
          .catch(console.log);
      }
    });
  }, [navigate]);

  return (
    <div className="flex flex-col h-full items-center">
      <form
        className="w-[300px] bg-gray-100 shadow-md rounded-md px-3 py-5 flex flex-col gap-3"
        method="POST"
        action={auth_api.username}
      >
        <Field label="Username">
          <Input type="text" autoComplete="username webauthn" name="username" />
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
          <Button appearance="primary" type="submit" size="medium">
            NEXT
          </Button>
        </div>
      </form>
    </div>
  );
}
