const express = require("express");
const {
  generateRegistrationOptions,
  verifyRegistrationResponse,
} = require("@simplewebauthn/server");
const { isoUint8Array } = require("@simplewebauthn/server/helpers");
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

      console.log(req.body, "req.body");

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

      console.log(options, "options");
      return res.json(options);
    } catch (error) {
      console.error(error);
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

    console.log({
      verified,
      registrationInfo: {
        credentialId: registrationInfo.credential.id,
        credentialPublicKey: registrationInfo.credential.publicKey,
      },
    });

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

app.listen(port, "127.0.0.1", () => {
  console.log(`Example app listening on port ${port}`);
});
