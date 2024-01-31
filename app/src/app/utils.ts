import ElkConstructor, {
  ElkExtendedEdge,
  ElkNode,
} from "elkjs/lib/elk.bundled.js";
import { useLayoutEffect, useRef } from "react";
import { Edge, MarkerType, Node, NodeChange } from "reactflow";

type StateNode = Node<WorkflowState, "state">;
type TransitionNode = Node<WorkflowTransition, "transition">;
export type WorkflowNode = StateNode | TransitionNode;
export type WorkflowEdgeData = {
  transition: WorkflowTransition;
  option?: TransitionOption;
};
export type WorkflowEdge = Edge<WorkflowEdgeData>;

export async function layoutElements(
  nodes: WorkflowNode[],
  edges: WorkflowEdge[]
) {
  const graph: ElkNode = {
    id: "root",
    layoutOptions: {
      "elk.algorithm": "mrtree",
      "elk.direction": "DOWN",
      "elk.spacing.nodeNode": "80",
      "elk.mrtree.edgeRoutingMode": "AVOID_OVERLAP",
      "elk.mrtree.searchOrder": "DFS",
    },
    children: nodes.map(
      (node) =>
        ({
          id: node.id,
          width: node.width ?? 200,
          height: node.height ?? 50,
        } as ElkNode)
    ),
    edges: edges.map(
      (edge) =>
        ({
          id: edge.id,
          sources: [edge.source],
          targets: [edge.target],
        } satisfies ElkExtendedEdge)
    ),
  };

  const elk = new ElkConstructor();
  const { children } = await elk.layout(graph);

  return (
    children?.map(
      ({ id, x = 0, y = 0 }) =>
        ({
          id,
          type: "position",
          position: { x, y },
          positionAbsolute: { x, y },
          dragging: false,
        } satisfies NodeChange)
    ) ?? []
  );
}

export function getNodesFromWorkflow(workflow: Workflow): WorkflowNode[] {
  return workflow.definition.states.flatMap((state) => {
    const transitionNodes: TransitionNode[] = [];

    for (const state of workflow.definition.states)
      for (const transition of state.transitions)
        transitionNodes.push({
          id: transition.id,
          type: "transition",
          position: { x: transition.position.x, y: transition.position.y },
          data: transition,
        });

    return [
      ...transitionNodes,
      {
        id: state.id,
        type: "state",
        position: { x: state.position.x, y: state.position.y },
        data: state,
      },
    ];
  });
}

export function getEdgesFromWorkflow(workflow: Workflow) {
  const edges: WorkflowEdge[] = [];

  for (const state of workflow.definition.states) {
    for (const transition of state.transitions) {
      if ("options" in transition.definition) {
        edges.push({
          id: `opt-${state.id}-to-${transition.id}`,
          source: state.id,
          target: transition.id,
          type: "smoothstep",
          style: { strokeWidth: 2 },
          data: { transition },
        });
        for (const option of transition.definition.options) {
          edges.push({
            id: `opt-${transition.id}-to-${option.target_state_id}`,
            source: transition.id,
            target: option.target_state_id,
            type: "smoothstep",
            label:
              transition.definition.options.length > 1
                ? option.label
                : undefined,
            markerEnd: { type: MarkerType.ArrowClosed },
            style: { strokeWidth: 2 },
            data: { transition, option },
          });
        }
      } else {
        edges.push({
          id: `trn-${state.id}-to-${transition.id}`,
          source: state.id,
          target: transition.id,
          type: "smoothstep",
          style: { strokeWidth: 2 },
          data: { transition },
        });
        edges.push({
          id: `trn-${transition.id}-to-${transition.definition.target_state_id}`,
          source: transition.id,
          target: transition.definition.target_state_id,
          type: "smoothstep",
          markerEnd: { type: MarkerType.ArrowClosed },
          style: { strokeWidth: 2 },
          data: { transition },
        });
      }
    }
  }

  return edges;
}

export function getWorkflowFromNodesAndEdges(
  nodes: WorkflowNode[],
  edges: WorkflowEdge[]
) {
  const states = nodes
    .map((node) => {
      node.data.position = node.position;
      return node;
    })
    .filter<StateNode>((node): node is StateNode => node.type === "state")
    .map((node) => node.data);
  const transitions = nodes
    .map((node) => {
      node.data.position = node.position;
      return node;
    })
    .filter<TransitionNode>(
      (node): node is TransitionNode => node.type === "transition"
    )
    .map((node) => node.data);

  states.forEach((state) => {
    state.transitions.forEach((transition, index) => {
      const updatedTransition = transitions.find(
        (t) => t.id === transition.id
      )!;
      state.transitions[index] = updatedTransition;
    });
  });

  return states;
}

export async function getWorkflow(id: number) {
  await new Promise((resolve) => setTimeout(resolve, 300));

  return {
    id: 69,
    tenant_id: 12,
    name: "New Workflow",
    definition: {
      initial_state: "018cdc83-8406-7eed-9139-6f2d57602053",
      states: [
        {
          id: "018cdc83-8406-7eed-9139-6f2d57602053",
          name: "Request Submitted",
          description: "The request has been submitted",
          entry_actions: [
            {
              id: "91e59229-d9c0-4671-9c3f-98a0af79cfd8",
              name: "Auto Assign",
              definition: {
                type: "AutoAssign",
              },
            },
            {
              id: "f32a21f4-757d-402d-9991-a9d0b749b3de",
              name: "Email",
              definition: {
                type: "Email",
                template_id: 1,
                email: "test@example.com",
              },
            },
          ],
          exit_actions: [
            {
              id: "4dd858f8-7b4c-4ee7-96ac-74ffe2c72e6e",
              name: "Email",
              definition: {
                type: "Email",
                template_id: 1,
                email: "test@test.com",
              },
            },
          ],
          transitions: [
            {
              id: "018cdc87-602a-7345-b9a0-de094ef68e8f",
              name: "Automatic",
              definition: {
                type: "Automatic",
                target_state_id: "018cdc88-0142-7f0d-908f-a4dc33960e27",
              },
              position: {
                x: 335,
                y: 320,
              },
            },
          ],
          position: {
            x: 259.5,
            y: 40,
          },
        },
        {
          id: "018cdc88-0142-7f0d-908f-a4dc33960e27",
          name: "Triage",
          description: "The request is being triaged",
          entry_actions: [],
          exit_actions: [],
          transitions: [
            {
              id: "018cdc88-9ad8-74ee-999d-09177a0dbb3f",
              name: "Triage completed",
              definition: {
                type: "Manual",
                options: [
                  {
                    label: "Not Covered",
                    target_state_id: "018cdc8b-3817-7269-a644-31342c47985c",
                    data: [
                      {
                        type: "Comment",
                      },
                      {
                        type: "Date",
                        label: "Inspection Date",
                      },
                    ],
                  },
                  {
                    label: "Covered",
                    target_state_id: "018cdc8b-1df8-75be-8f74-9b3386aa938c",
                    data: [
                      {
                        type: "Comment",
                      },
                    ],
                  },
                ],
              },
              position: {
                x: 307.5,
                y: 661.5,
              },
            },
          ],
          position: {
            x: 280.5,
            y: 454,
          },
        },
        {
          id: "018cdc8b-3817-7269-a644-31342c47985c",
          name: "Closed",
          description: "The request is closed",
          entry_actions: [
            {
              id: "018cdc8d-ac19-7d23-be7b-61d28d60a71d",
              name: "Email",
              definition: {
                type: "Email",
                template_id: 1,
                email: "asdf@test.com",
              },
            },
          ],
          exit_actions: [],
          transitions: [],
          position: {
            x: -131.5,
            y: 803,
          },
        },
        {
          id: "018cdc8b-1df8-75be-8f74-9b3386aa938c",
          name: "Inspection Scheduled",
          entry_actions: [],
          exit_actions: [],
          position: {
            x: 637,
            y: 803,
          },
          transitions: [
            {
              id: "018cdc91-1d03-78da-8e33-4f80b0193888",
              name: "Inspection Completed",
              position: {
                x: 634,
                y: 1015.5,
              },
              definition: {
                type: "Manual",
                options: [
                  {
                    label: "Inspection Completed",
                    target_state_id: "018cdc91-8b61-76a6-988f-4f1657181cab",
                    data: [
                      {
                        type: "Comment",
                      },
                    ],
                  },
                ],
              },
            },
          ],
        },
        {
          id: "018cdc91-8b61-76a6-988f-4f1657181cab",
          name: "Review",
          entry_actions: [],
          exit_actions: [],
          position: {
            x: 637.5,
            y: 1157,
          },
          transitions: [
            {
              id: "018cdcbf-6653-786d-b8c7-fd32d0581239",
              name: "Manager Approved",
              position: {
                x: 139.5,
                y: 1369.5,
              },
              definition: {
                type: "Approval",
                approver_id: 1,
                options: [
                  {
                    label: "Approved",
                    target_state_id: "018cdc8b-3817-7269-a644-31342c47985c",
                    data: [],
                  },
                  {
                    label: "Denied",
                    target_state_id: "74dc5e29-8a83-4b5a-8174-dc743f650c0d",
                    data: [],
                  },
                ],
              },
            },
            {
              id: "91ffaf21-a635-4814-a310-15db102f805d",
              name: "Homeowner Complaint",
              position: {
                x: 632,
                y: 1369.5,
              },
              definition: {
                type: "Approval",
                approver_id: 1,
                options: [
                  {
                    label: "Approved",
                    target_state_id: "018cdc8b-3817-7269-a644-31342c47985c",
                    data: [],
                  },
                  {
                    label: "Denied",
                    target_state_id: "74dc5e29-8a83-4b5a-8174-dc743f650c0d",
                    data: [],
                  },
                ],
              },
            },
          ],
        },
        {
          id: "74dc5e29-8a83-4b5a-8174-dc743f650c0d",
          name: "Failed",
          position: {
            x: 296,
            y: 1511,
          },
          entry_actions: [],
          exit_actions: [],
          transitions: [],
        },
      ],
    },
    created_at: new Date(),
    updated_at: new Date(),
  } satisfies Workflow;
}

export function useLayoutEffectOnce(callback: () => void) {
  const hasRun = useRef(false);
  useLayoutEffect(() => {
    if (!hasRun.current) {
      callback();
      hasRun.current = true;
    }
  }, [callback]);
}
