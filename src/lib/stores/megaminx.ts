import { writable, derived, get } from "svelte/store";
import type {
  MegaminxState,
  IgnoreFlags,
  SolverConfig,
  SolverState,
  AllowedFacesMode,
  MetricType,
  CornerPosition,
  EdgePosition,
  CornerOrientation,
  EdgeOrientation,
} from "$lib/types/megaminx";

const DEFAULT_MEGAMINX_STATE: MegaminxState = {
  cornerPositions: [0, 1, 2, 3, 4] as CornerPosition[],
  cornerOrientations: [0, 0, 0, 0, 0] as CornerOrientation[],
  edgePositions: [0, 1, 2, 3, 4] as EdgePosition[],
  edgeOrientations: [0, 0, 0, 0, 0] as EdgeOrientation[],
};

const DEFAULT_IGNORE_FLAGS: IgnoreFlags = {
  cornerPositions: false,
  edgePositions: false,
  cornerOrientations: false,
  edgeOrientations: false,
};

const DEFAULT_CONFIG: SolverConfig = {
  allowedFaces: "R_U",
  metric: "FTM",
  limitDepth: false,
  maxDepth: 10,
  ignoreFlags: DEFAULT_IGNORE_FLAGS,
};

const DEFAULT_SOLVER_STATE: SolverState = {
  isSearching: false,
  progress: 0,
  status: "Ready",
  solutions: [],
};

export const megaminxState = writable<MegaminxState>({
  ...DEFAULT_MEGAMINX_STATE,
  cornerPositions: [
    ...DEFAULT_MEGAMINX_STATE.cornerPositions,
  ] as CornerPosition[],
  cornerOrientations: [
    ...DEFAULT_MEGAMINX_STATE.cornerOrientations,
  ] as CornerOrientation[],
  edgePositions: [...DEFAULT_MEGAMINX_STATE.edgePositions] as EdgePosition[],
  edgeOrientations: [
    ...DEFAULT_MEGAMINX_STATE.edgeOrientations,
  ] as EdgeOrientation[],
});

export const config = writable<SolverConfig>({ ...DEFAULT_CONFIG });
export const solverState = writable<SolverState>({ ...DEFAULT_SOLVER_STATE });

export const ignoreFlags = derived(config, ($config) => $config.ignoreFlags);
export const isSearching = derived(solverState, ($state) => $state.isSearching);

export function reset() {
  megaminxState.set({
    ...DEFAULT_MEGAMINX_STATE,
    cornerPositions: [
      ...DEFAULT_MEGAMINX_STATE.cornerPositions,
    ] as CornerPosition[],
    cornerOrientations: [
      ...DEFAULT_MEGAMINX_STATE.cornerOrientations,
    ] as CornerOrientation[],
    edgePositions: [...DEFAULT_MEGAMINX_STATE.edgePositions] as EdgePosition[],
    edgeOrientations: [
      ...DEFAULT_MEGAMINX_STATE.edgeOrientations,
    ] as EdgeOrientation[],
  });
}

export function swapCorners(i: number, j: number) {
  megaminxState.update((prev) => {
    const newPositions = [...prev.cornerPositions] as CornerPosition[];
    const newOrientations = [...prev.cornerOrientations] as CornerOrientation[];
    [newPositions[i], newPositions[j]] = [newPositions[j], newPositions[i]];
    [newOrientations[i], newOrientations[j]] = [
      newOrientations[j],
      newOrientations[i],
    ];
    return {
      ...prev,
      cornerPositions: newPositions,
      cornerOrientations: newOrientations,
    };
  });
}

export function rotateCorner(index: number, direction: 1 | -1) {
  megaminxState.update((prev) => {
    const newOrientations = [...prev.cornerOrientations] as CornerOrientation[];
    newOrientations[index] = ((newOrientations[index] + direction + 3) %
      3) as CornerOrientation;
    return { ...prev, cornerOrientations: newOrientations };
  });
}

export function swapEdges(i: number, j: number) {
  megaminxState.update((prev) => {
    const newPositions = [...prev.edgePositions] as EdgePosition[];
    const newOrientations = [...prev.edgeOrientations] as EdgeOrientation[];
    [newPositions[i], newPositions[j]] = [newPositions[j], newPositions[i]];
    [newOrientations[i], newOrientations[j]] = [
      newOrientations[j],
      newOrientations[i],
    ];
    return {
      ...prev,
      edgePositions: newPositions,
      edgeOrientations: newOrientations,
    };
  });
}

export function flipEdge(index: number) {
  megaminxState.update((prev) => {
    const newOrientations = [...prev.edgeOrientations] as EdgeOrientation[];
    newOrientations[index] = ((newOrientations[index] + 1) %
      2) as EdgeOrientation;
    return { ...prev, edgeOrientations: newOrientations };
  });
}

export function setAllowedFaces(mode: AllowedFacesMode) {
  config.update((prev) => ({ ...prev, allowedFaces: mode }));
}

export function setMetric(metric: MetricType) {
  config.update((prev) => ({ ...prev, metric }));
}

export function setLimitDepth(limit: boolean) {
  config.update((prev) => ({ ...prev, limitDepth: limit }));
}

export function setMaxDepth(depth: number) {
  config.update((prev) => ({
    ...prev,
    maxDepth: Math.max(1, Math.min(50, depth)),
  }));
}

export function setIgnoreFlag(flag: keyof IgnoreFlags, value: boolean) {
  config.update((prev) => ({
    ...prev,
    ignoreFlags: { ...prev.ignoreFlags, [flag]: value },
  }));
}

export function startSolve() {
  solverState.set({
    isSearching: true,
    progress: 0,
    status: "Initializing search...",
    solutions: [],
  });
}

export function cancelSolve() {
  solverState.update((prev) => ({
    ...prev,
    isSearching: false,
    status: "Cancelled",
  }));
}

export function addSolution(solution: string) {
  solverState.update((prev) => ({
    ...prev,
    solutions: [...prev.solutions, solution],
  }));
}

export function updateProgress(progress: number, status: string) {
  solverState.update((prev) => ({ ...prev, progress, status }));
}
