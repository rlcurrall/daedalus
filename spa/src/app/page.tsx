import WorkflowEditor from "./WorkflowEditor";

export default async function Home() {
  return (
    <main className="flex min-h-screen w-screen h-screen flex-col items-center justify-between">
      <WorkflowEditor id={123} />
    </main>
  );
}
