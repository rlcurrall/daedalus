import NextAuth from "next-auth";
import credentials from "next-auth/providers/credentials";

export const {
  handlers: { GET, POST },
  auth,
  signIn,
  signOut,
} = NextAuth({
  secret: process.env.AUTH_SECRET ?? "secret",
  providers: [
    credentials({
      credentials: {
        username: { label: "Username", type: "text" },
        password: { label: "Password", type: "password" },
      },
      async authorize(credentials) {
        let user = null;
        if (credentials.username === "test" && credentials.password === "test")
          user = { id: "1", name: "Test User", email: "asdf@test.com" };

        await new Promise((resolve) => setTimeout(resolve, 1000));

        return user;
      },
    }),
  ],
});
