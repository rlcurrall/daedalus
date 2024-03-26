import { Dialog, Transition } from "@headlessui/react";
import type React from "react";
import { Children, Fragment } from "react";
import { Button } from "./button";

export function SlideOver({
  children,
  open,
  onClose,
}: {
  children: React.ReactNode;
  open: boolean;
  onClose(open: boolean): void;
}) {
  const title = Children.toArray(children).find(
    (child) => (child as any).type === Title,
  );
  const panel = Children.toArray(children).filter(
    (child) => (child as any).type === Panel,
  );

  return (
    <Transition.Root show={open} appear as={Fragment}>
      <Dialog as="div" onClose={onClose} className="relative z-10">
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-100"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-100"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 flex w-screen justify-center overflow-y-auto bg-zinc-950/25 px-2 py-2 focus:outline-0 sm:px-6 sm:py-8 lg:px-8 lg:py-16 dark:bg-zinc-950/50" />
        </Transition.Child>

        <div className="fixed inset-0 overflow-hidden">
          <div className="absolute inset-0 overflow-hidden">
            <div className="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
              <Transition.Child
                as={Fragment}
                enter="transform transition ease-in-out duration-500 sm:duration-700"
                enterFrom="translate-x-full"
                enterTo="translate-x-0"
                leave="transform transition ease-in-out duration-500 sm:duration-700"
                leaveFrom="translate-x-0"
                leaveTo="translate-x-full"
              >
                <Dialog.Panel className="pointer-events-auto w-screen max-w-2xl bg-white dark:bg-zinc-900">
                  <div className="flex h-full flex-col space-y-6 divide-y divide-zinc-300 overflow-y-auto pt-6 dark:divide-zinc-700">
                    <div className="px-4 sm:px-6">
                      <div className="flex items-center justify-between">
                        <Dialog.Title className="flex items-center text-xl">
                          {title}
                        </Dialog.Title>

                        <div className="ml-3 flex h-7 items-center">
                          <Button plain onClick={() => onClose(false)}>
                            <span className="absolute -inset-2.5" />
                            <span className="sr-only">Close panel</span>
                            <i
                              className="fas fa-xmark text-xl"
                              aria-hidden="true"
                            />
                          </Button>
                        </div>
                      </div>
                    </div>
                    <div className="relative flex-1 grow overflow-y-auto px-4 py-6 sm:px-6">
                      {panel}
                    </div>
                  </div>
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
}

function Title({ children, ...props }: React.ComponentPropsWithoutRef<"div">) {
  return <div {...props}>{children}</div>;
}

function Panel({ children, ...props }: React.ComponentPropsWithoutRef<"div">) {
  return <div {...props}>{children}</div>;
}

SlideOver.Title = Title;
SlideOver.Panel = Panel;
