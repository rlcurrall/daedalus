"use client";

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
import WorkflowState from "./WorkflowState";
import WorkflowTransition from "./WorkflowTransition";
import {
  WorkflowNode,
  getEdgesFromWorkflow,
  getNodesFromWorkflow,
  getWorkflow,
  getWorkflowFromNodesAndEdges,
  layoutElements,
  useLayoutEffectOnce,
} from "./utils";

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
      })),
    );
    setEdges(getEdgesFromWorkflow(workflow));
    globalThis.requestAnimationFrame(() => fitView());
  }, [fitView, setEdges, setNodes]);

  useLayoutEffectOnce(fetchWorkflow);

  return (
    <div className="flex h-screen w-screen flex-col ">
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
              eds,
            ),
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

      <div className="flex items-center justify-end bg-zinc-900 text-white">
        <div>
          <button onClick={() => zoomIn()} className="h-8 w-8 p-1">
            <i aria-hidden className="fas fa-search-plus" />
          </button>
          <button onClick={() => zoomOut()} className="h-8 w-8 p-1">
            <i aria-hidden className="fas fa-search-minus" />
          </button>
          <button onClick={() => fitView()} className="h-8 w-8 p-1">
            <i aria-hidden className="fas fa-compress" />
          </button>
          <button onClick={() => onLayout()} className="h-8 w-8 p-1">
            <i aria-hidden className="fas fa-magic" />
          </button>
          <button onClick={() => fetchWorkflow()} className="h-8 w-8 p-1">
            <i aria-hidden className="fas fa-sync" />
          </button>
          <button
            className="h-8 w-8 p-1"
            onClick={() =>
              console.log(
                getWorkflowFromNodesAndEdges(nodes as WorkflowNode[], edges),
              )
            }
          >
            <i aria-hidden className="fas fa-save" />
          </button>
        </div>
      </div>
    </div>
  );
}
