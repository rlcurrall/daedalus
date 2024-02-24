"use client";

import {
  ArrowPathIcon,
  MagnifyingGlassMinusIcon,
  MagnifyingGlassPlusIcon,
  SparklesIcon,
  ViewfinderCircleIcon,
  ChevronDoubleRightIcon,
  ChevronDoubleLeftIcon,
} from "@heroicons/react/24/solid";
import { useCallback, useState } from "react";
import ReactFlow, {
  Background,
  BackgroundVariant,
  Connection,
  MarkerType,
  ReactFlowProvider,
  addEdge,
  useEdgesState,
  useNodesState,
  useReactFlow,
} from "reactflow";
import {
  WorkflowNode,
  getEdgesFromWorkflow,
  getNodesFromWorkflow,
  getWorkflow,
  getWorkflowFromNodesAndEdges,
  layoutElements,
  useLayoutEffectOnce,
} from "./utils";
import WorkflowState from "./WorkflowState";
import WorkflowTransition from "./WorkflowTransition";
import clsx from "clsx";

const edgeTypes = {};
const nodeTypes = {
  state: WorkflowState,
  transition: WorkflowTransition,
};

export default function WorkflowEditor({ id }: { id?: number }) {
  return (
    <ReactFlowProvider>
      <InnerWorkflowEditor id={id} />
    </ReactFlowProvider>
  );
}

function InnerWorkflowEditor({ id: _ }: { id?: number }) {
  const { fitView, zoomIn, zoomOut } = useReactFlow();
  const [closed, setClosed] = useState(false);
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onLayout = useCallback(async () => {
    const changes = await layoutElements(nodes as WorkflowNode[], edges);
    onNodesChange(changes);
    globalThis.requestAnimationFrame(() => fitView());
  }, [nodes, edges, onNodesChange, fitView]);

  const fetchWorkflow = useCallback(async () => {
    const workflow = await getWorkflow(0);
    setNodes((nodes) =>
      getNodesFromWorkflow(workflow).map((n) => ({
        width: nodes.find((n2) => n2.id === n.id)?.width,
        height: nodes.find((n2) => n2.id === n.id)?.height,
        ...n,
      }))
    );
    setEdges(getEdgesFromWorkflow(workflow));
    globalThis.requestAnimationFrame(() => fitView());
  }, [fitView, setEdges, setNodes]);

  useLayoutEffectOnce(fetchWorkflow);

  return (
    <div
      style={{
        display: "grid",
        gridTemplateRows: "1fr auto",
        gridTemplateColumns: "auto 1fr",
        height: "100vh",
        width: "100vw",
        margin: 0,
        gridTemplateAreas: `
          "nav graph"
          "nav controls"
        `,
      }}
    >
      <div
        style={{ gridArea: "nav" }}
        className={clsx("p-2 relative", closed ? "w-12" : "w-48")}
      >
        <button
          className="absolute bottom-0 right-0 left-0 flex items-center text-white justify-around p-2 ring-2 ring-inset ring-zinc-700"
          onClick={() => setClosed((o) => !o)}
        >
          {closed ? (
            <ChevronDoubleRightIcon className="w-6 h-6" />
          ) : (
            <ChevronDoubleLeftIcon className="w-6 h-6" />
          )}
          {!closed && (
            <>
              Sidebar
              <span className="w-6" />
            </>
          )}
        </button>
      </div>

      <div
        style={{
          gridArea: "graph",
          backgroundColor: "#fff",
        }}
      >
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={(connection: Connection) => {
            const source = nodes.find((n) => n.id === connection.source);
            const target = nodes.find((n) => n.id === connection.target);

            // The source and target must be valid and of different types
            if (
              !source ||
              !target ||
              (source.type === "transition" && target.type === "transition") ||
              (source.type === "state" && target.type === "state")
            )
              return;

            const targetIsState = target.type === "state";

            setEdges((eds) =>
              addEdge(
                {
                  type: "smoothstep",
                  style: { strokeWidth: 2 },
                  markerEnd: targetIsState
                    ? { type: MarkerType.ArrowClosed }
                    : undefined,
                  ...connection,
                },
                eds
              )
            );
          }}
          edgeTypes={edgeTypes}
          nodeTypes={nodeTypes}
          fitView
        >
          <Background
            color="#21222c"
            className="bg-[#15151d]"
            gap={32}
            variant={BackgroundVariant.Lines}
          />
        </ReactFlow>
      </div>

      <div
        style={{ gridArea: "controls", height: "2.5rem" }}
        className="text-white flex justify-end items-center"
      >
        <div>
          <button onClick={() => zoomIn()} className="w-8 h-8 p-1">
            <MagnifyingGlassPlusIcon />
          </button>
          <button onClick={() => zoomOut()} className="w-8 h-8 p-1">
            <MagnifyingGlassMinusIcon />
          </button>
          <button onClick={() => fitView()} className="w-8 h-8 p-1">
            <ViewfinderCircleIcon />
          </button>
          <button onClick={() => onLayout()} className="w-8 h-8 p-1">
            <SparklesIcon />
          </button>
          <button onClick={() => fetchWorkflow()} className="w-8 h-8 p-1">
            <ArrowPathIcon />
          </button>
        </div>
      </div>
    </div>
  );
}
