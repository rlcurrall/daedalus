import { User } from "next-auth";
import { auth } from "@/auth";
import { Text } from "@/catalyst/text";
import LoginForm from "./login-form";
import { logout } from "./actions";
import { Button } from "@/catalyst/button";

export default async function Login() {
  const session = await auth();
  const user = session?.user;

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      {user ? <UserData user={user} /> : <LoginForm />}
    </main>
  );
}

function UserData({ user }: { user: User }) {
  return (
    <div>
      <Text>{user.name}</Text>
      <Text>{user.email}</Text>
      <form action={logout}>
        <Button type="submit" color="teal">
          Sign out
        </Button>
      </form>
    </div>
  );
}
