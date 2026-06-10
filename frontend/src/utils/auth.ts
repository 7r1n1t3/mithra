import { betterAuth } from "better-auth";
import { passkey } from "@better-auth/passkey";
import { passkeyClient } from "@better-auth/passkey/client";
import { genericOAuth } from "better-auth/plugins";
import { Pool } from "pg";

export const auth = betterAuth({
  database: new Pool({
    // TODO: find a way to reference name of container here
    host: "db",
    user: process.env.POSTGRES_USER,
  }),
  emailAndPassword: {
    enabled: true,
  },
  plugins: [
    passkey(),
    passkeyClient(),
    genericOAuth({
      config: [
        {
          providerId: process.env.OIDC_PROVIDER_ID as string,
          clientId: process.env.OIDC_CLIENT_ID as string,
          clientSecret: process.env.OIDC_CLIENT_SECRET as string,
          discoveryUrl:
            "https://auth.example.com/.well-known/openid-configuration",
        },
      ],
    }),
  ],
});
