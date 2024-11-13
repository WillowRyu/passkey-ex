const express = require("express");
const {
  generateRegistrationOptions,
  verifyRegistrationResponse,
  generateAuthenticationOptions,
} = require("@simplewebauthn/server");

const crypto = require("crypto");

if (!global.crypto) {
  global.crypto = crypto.webcrypto;
}

const app = express();
const port = 3001;

app.use(express.json());

app.post("/generate-options", async (req, res) => {
  if (req.body.rpID) {
    try {
      const {
        rpName,
        rpID,
        userID,
        userName,
        userDisplayName,
        attestationType,
        excludeCredentials,
        authenticatorSelection,
      } = req.body;

      const options = await generateRegistrationOptions({
        rpName,
        rpID,
        userID,
        userName,
        userDisplayName: userDisplayName || userName,
        attestationType,
        excludeCredentials,
        authenticatorSelection,
      });

      return res.json(options);
    } catch (error) {
      return res.status(400).json({ error: error.message });
    }
  }

  return res.status(400).json({ error: "Invalid request" });
});

app.post("/verify-credentials", async (req, res) => {
  try {
    const {
      expectedOrigin,
      expectedChallenge,
      expectedRPID,
      response,
      requireUserVerification,
    } = req.body;

    const { verified, registrationInfo } = await verifyRegistrationResponse({
      expectedChallenge,
      expectedOrigin,
      expectedRPID,
      response,
      requireUserVerification,
    });

    if (!verified) {
      throw new Error("User verification failed.");
    }

    return res.json({
      verified,
      registrationInfo: {
        credentialId: registrationInfo.credential.id,
        credentialPublicKey: registrationInfo.credential.publicKey,
      },
    });
  } catch (error) {
    console.error(error);
    return res.status(400).json({ error: error.message });
  }
});

app.post("/generate-auth-options", async (req, res) => {
  try {
    const { rpID, allowCredentials } = req.body;
    const options = await generateAuthenticationOptions({
      rpID,
      allowCredentials,
    });

    return res.json({
      options,
    });
  } catch (error) {
    return res.status(400).json({ error: error.message });
  }
});

app.post("/verify-auth-credentials", async (req, res) => {
  try {
    const {
      response,
      expectedChallenge,
      expectedOrigin,
      expectedRPID,
      authenticator,
    } = req.body;

    const { verified, authenticationInfo } = await verifyAuthenticationResponse(
      {
        response,
        expectedChallenge,
        expectedOrigin,
        expectedRPID,
        authenticator,
        requireUserVerification: false,
      }
    );

    if (!verified) {
      throw new Error("User verification failed.");
    }

    return res.json({
      verified,
      authenticationInfo,
    });
  } catch (error) {
    return res.status(400).json({ error: error.message });
  }
});

app.listen(port, "127.0.0.1", () => {
  console.log(`Example app listening on port ${port}`);
});
