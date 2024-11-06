const express = require("express");
const { generateRegistrationOptions } = require("@simplewebauthn/server");
const { isoUint8Array } = require("@simplewebauthn/server/helpers");
const crypto = require("crypto");

// Node.js 환경에서 Web Crypto API 설정
if (!global.crypto) {
  global.crypto = crypto.webcrypto;
}

const app = express();
const port = 3001;

app.get("/generate-options", async (req, res) => {
  const options = await generateRegistrationOptions({
    rpName: "SimpleWebAuthn Example",
    rpID: "localhost",
    userID: isoUint8Array.fromUTF8String(
      "Q9iY3jsujMgiviuTrI2If7QEhteNihPKMCC-cN7jQUM"
    ),
    userName: "12345",
    userDisplayName: "12345",
    attestationType: "none",
    excludeCredentials: [],
    authenticatorSelection: {
      authenticatorAttachment: "platform",
      requireResidentKey: true,
    },
    supportedAlgorithmIDs: [-7, -257],
  });

  res.json(options);
});

app.listen(port, "127.0.0.1", () => {
  console.log(`Example app listening on port ${port}`);
});
