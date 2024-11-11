import { auth_api } from "~/shared/api";
import { base64url } from "~/shared/util/base64_url";
import { _fetch } from "~/shared/util/fetch";

export const useCreateCredential = () => {
  const createCred = async () => {
    const response = await _fetch(auth_api.registerRequest);
    const { data: options } = response;

    console.log(options, "options");

    options.user.id = base64url.decode(options.user.id);
    options.challenge = base64url.decode(options.challenge);

    if (options.excludeCredentials) {
      for (let cred of options.excludeCredentials) {
        cred.id = base64url.decode(cred.id);
      }
    }

    options.authenticatorSelection = {
      authenticatorAttachment: "platform",
      requireResidentKey: true,
    };

    const cred = (await navigator.credentials.create({
      publicKey: options,
    })) as any;

    const credential: any = {};
    credential.id = cred?.id;
    credential.rawId = cred?.id; // Pass a Base64URL encoded ID string.
    credential.type = cred?.type;

    if (cred?.authenticatorAttachment) {
      credential.authenticatorAttachment = cred.authenticatorAttachment;
    }

    const clientDataJSON = base64url.encode(cred.response.clientDataJSON);
    const attestationObject = base64url.encode(cred.response.attestationObject);

    const transports = cred.response.getTransports
      ? cred.response.getTransports()
      : [];

    credential.response = {
      clientDataJSON,
      attestationObject,
      transports,
    };

    return await _fetch(auth_api.registerResponse, credential);
  };

  return { createCred };
};
