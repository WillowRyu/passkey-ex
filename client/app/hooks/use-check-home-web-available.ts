export const useCheckHomeWebAvailable = () => {
  const checkHomeWebAvailable = async () => {
    if (window.PublicKeyCredential) {
      try {
        const result = await Promise.all([
          PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable(),
          PublicKeyCredential.isConditionalMediationAvailable(),
        ]);

        console.log(result, "result");

        if (result.every((r) => r === true)) {
          return true;
        }
      } catch {
        return false;
      }
    }

    return false;
  };

  return {
    checkHomeWebAvailable,
  };
};
