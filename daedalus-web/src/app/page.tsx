"use client";

import {
  Alert,
  AlertActions,
  AlertDescription,
  AlertTitle,
} from "@/catalyst/alert";
import { Button } from "@/catalyst/button";
import { useState } from "react";

export default function Home() {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Button type="button" color="teal" onClick={() => setIsOpen(true)}>
        Open alert
      </Button>

      <Alert open={isOpen} onClose={setIsOpen}>
        <AlertTitle>Are you sure you want to refund this payment?</AlertTitle>
        <AlertDescription>
          The refund will be reflected in the customer&apos;s bank account 2 to
          3 business days after processing.
        </AlertDescription>
        <AlertActions>
          <Button plain onClick={() => setIsOpen(false)}>
            Cancel
          </Button>
          <Button color="teal" onClick={() => setIsOpen(false)}>
            Refund
          </Button>
        </AlertActions>
      </Alert>
    </main>
  );
}
