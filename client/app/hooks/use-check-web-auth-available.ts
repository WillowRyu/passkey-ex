import { auth_api } from "~/shared/api";
import { base64url } from "~/shared/util/base64_url";
import { _fetch } from "~/shared/util/fetch";

type CredntialModel = {
  id?: string;
  rawId?: string;
  type?: string;
  response?: {
    clientDataJSON?: string;
    authenticatorData?: string;
    signature?: string;
    userHandle?: string;
  };
};

export const useCheckWebAuthAvailable = () => {
  const authenticate = async () => {
    const res = await _fetch(auth_api.signinRequest);
    const { data: options } = res;

    options.challenge = base64url.decode(options.challenge);
    options.allowCredentials = [];

    const cred: CredntialModel | null = await navigator.credentials.get({
      publicKey: options,
      mediation: "conditional",
    });

    const credential: CredntialModel = {};

    if (cred?.response) {
      credential.id = cred.id;
      credential.rawId = cred.id;
      credential.type = cred.type;

      const clientDataJSON = base64url.encode(
        cred?.response.clientDataJSON as Iterable<number>
      );
      const authenticatorData = base64url.encode(
        cred?.response.authenticatorData as Iterable<number>
      );
      const signature = base64url.encode(
        cred?.response.signature as Iterable<number>
      );
      const userHandle = base64url.encode(
        cred?.response.userHandle as Iterable<number>
      );

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

  const checkWebAuthAvailable: () => Promise<{
    username: string;
  }> = async () => {
    if (
      window.PublicKeyCredential &&
      !!PublicKeyCredential.isConditionalMediationAvailable
    ) {
      try {
        const cma = await PublicKeyCredential.isConditionalMediationAvailable();
        if (cma) {
          const user = await authenticate();
          const userName = user?.data?.username as string;
          return { username: userName ?? "" };
        }
      } catch {
        alert("WebAuthn is not available");
      }
    }

    return {
      username: "",
    };
  };

  return { checkWebAuthAvailable };
};
