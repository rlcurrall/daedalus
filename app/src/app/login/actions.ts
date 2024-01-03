"use server";

import { signIn, signOut } from "@/auth";
import { redirect } from "next/navigation";

type FormState =
  | { status: "init" }
  | { status: "error"; message: string }
  | { status: "success" };

export async function login(
  _: FormState,
  formData: FormData
): Promise<FormState> {
  let url: string | null = null;
  let error: Error | null = null;

  try {
    url = await signIn("credentials", {
      username: formData.get("username") as string,
      password: formData.get("password") as string,
      redirect: false,
      redirectTo: "/",
    });
  } catch (e) {
    if (e instanceof Error) error = e;
  }

  if (!url)
    return {
      status: "error",
      message: "An error occurred, please try again",
    };

  if (error) {
    const isAuthError = error.name === "CredentialsSignin";
    const message = isAuthError
      ? "Invalid username or password"
      : "An error occurred, please try again";
    if (!isAuthError)
      console.error(`Error authenticating with credentials: ${error.message}`);
    return { status: "error", message };
  }

  redirect(url);
}

export async function logout() {
  let error: Error = new Error("Unknown error");

  try {
    await signOut();
  } catch (e) {
    if (e instanceof Error) error = e;
  }

  console.log(error);
  if (error?.message === "NEXT_REDIRECT") throw error;

  console.error(`Error logging out: ${error.message}`);
}
