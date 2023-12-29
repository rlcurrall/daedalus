"use client";

import { useFormState, useFormStatus } from "react-dom";
import { Button } from "@/catalyst/button";
import { Text } from "@/catalyst/text";
import { Input } from "@/catalyst/input";
import {
  Field,
  FieldGroup,
  Fieldset,
  Label,
  Legend,
} from "@/catalyst/fieldset";
import { login } from "./actions";

export default function LoginForm() {
  const { pending } = useFormStatus();
  const [state, formAction] = useFormState(login, { status: "init" });

  const message = {
    init: null,
    success: <p>Your message has been sent successfully!</p>,
    error: (
      <p>
        {"message" in state
          ? state.message
          : "An error occurred. Please try again later."}
      </p>
    ),
  }[state.status];

  return (
    <form action={formAction} className="grid grid-cols-1 gap-y-6">
      <Fieldset>
        <Legend>Login</Legend>
        {state.status === "error" && (
          <Text className="text-red-500">{message}</Text>
        )}
        <FieldGroup>
          <Field>
            <Label>Username</Label>
            <Input
              name="username"
              type="text"
              autoComplete="username"
              required
            />
          </Field>
          <Field>
            <Label>Password</Label>
            <Input
              name="password"
              type="password"
              autoComplete="current-password"
              required
            />
          </Field>
        </FieldGroup>
      </Fieldset>

      <div>
        <Button type="submit" color="teal" disabled={pending}>
          Login
        </Button>
      </div>
    </form>
  );
}
