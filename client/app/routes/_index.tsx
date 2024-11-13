import { Button, Field, Input, Text } from "@fluentui/react-components";
import { useEffect } from "react";
import { useNavigate } from "@remix-run/react";
import { auth_api } from "~/shared/api";
import { _fetch, FormDataObj } from "~/shared/util/fetch";
import { base64url } from "~/shared/util/base64_url";

export default function Index() {
  const navigate = useNavigate();

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
              return navigate("/reauth");
            }

            alert(res?.message);
          })
          .catch(console.log);
      }
    });
  }, [navigate]);

  const authenticate = async () => {
    const res = await _fetch(auth_api.signinRequest);
    const { data: options } = res;

    options.challenge = base64url.decode(options.challenge);
    options.allowCredentials = [];

    const cred = await navigator.credentials.get({
      publicKey: options,
      mediation: "conditional",
    });

    const credential: {
      id?: string;
      rawId?: string;
      type?: string;
    } = {};

    if (cred) {
      credential.id = cred.id;
      credential.rawId = cred.id; // Pass a Base64URL encoded ID string.
      credential.type = cred.type;

      const clientDataJSON = base64url.encode(cred?.response.clientDataJSON);
      const authenticatorData = base64url.encode(
        cred?.response.authenticatorData
      );
      const signature = base64url.encode(cred?.response.signature);
      const userHandle = base64url.encode(cred?.response.userHandle);

      credential.response = {
        clientDataJSON,
        authenticatorData,
        signature,
        userHandle,
      };

      return await _fetch(auth_api.signinResponse, {
        payload: credential,
      });
    }
  };

  const webauthnAvailable = async () => {
    if (
      window.PublicKeyCredential &&
      !!PublicKeyCredential.isConditionalMediationAvailable
    ) {
      try {
        const cma = await PublicKeyCredential.isConditionalMediationAvailable();
        if (cma) {
          const user = await authenticate();
          if (user) {
            console.log(user);
          }
        }
      } catch (e) {
        console.log(e);
      }
    }
  };

  useEffect(() => {}, []);

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
            autoComplete="username webauthn"
            name="username"
            autoFocus
          />
        </Field>

        <input
          type="password"
          style={{ visibility: "hidden" }}
          name="password"
          autoComplete="current-password"
          autoFocus
        />

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

      <Button
        appearance="primary"
        type="button"
        size="medium"
        onClick={webauthnAvailable}
      >
        Authenen
      </Button>
    </div>
  );
}
