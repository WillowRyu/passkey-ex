### Passkey Example

This project demonstrates the use of **Passkeys** and is composed of the following components:

- **Rust Server**
- **Node Server**
- **Remix Client**

---

### Getting Started

To run this project locally, follow the steps below.

#### 1. Start the Local Database

Use Docker to run the local database.

```bash
pnpm db:up
```

#### 2. Run the Servers

Start both the Rust and Node servers.

```bash
pnpm server:rust
pnpm server:node
```

#### 3. Start the Client

Finally, start the Remix client.

```bash
pnpm client:dev
```

---

Now, your application should be up and running!
