import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { MegaminxState, SolverConfig } from '$lib/types/megaminx';

export interface ProgressEvent {
	eventType: string;
	message: string;
	progress: number;
}

export interface SolverConfigPayload {
	allowedFaces: string;
	metric: string;
	limitDepth: boolean;
	maxDepth: number;
	ignoreCornerPositions: boolean;
	ignoreEdgePositions: boolean;
	ignoreCornerOrientations: boolean;
	ignoreEdgeOrientations: boolean;
}

export interface MegaminxStatePayload {
	cornerPositions: number[];
	cornerOrientations: number[];
	edgePositions: number[];
	edgeOrientations: number[];
}

function toSolverConfigPayload(config: SolverConfig): SolverConfigPayload {
	return {
		allowedFaces: config.allowedFaces,
		metric: config.metric,
		limitDepth: config.limitDepth,
		maxDepth: config.maxDepth,
		ignoreCornerPositions: config.ignoreFlags.cornerPositions,
		ignoreEdgePositions: config.ignoreFlags.edgePositions,
		ignoreCornerOrientations: config.ignoreFlags.cornerOrientations,
		ignoreEdgeOrientations: config.ignoreFlags.edgeOrientations
	};
}

function toMegaminxStatePayload(state: MegaminxState): MegaminxStatePayload {
	return {
		cornerPositions: [...state.cornerPositions],
		cornerOrientations: [...state.cornerOrientations],
		edgePositions: [...state.edgePositions],
		edgeOrientations: [...state.edgeOrientations]
	};
}

export function isTauri(): boolean {
	return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export async function solveMegaminx(config: SolverConfig, state: MegaminxState): Promise<string[]> {
	const configPayload = toSolverConfigPayload(config);
	const statePayload = toMegaminxStatePayload(state);

	return await invoke<string[]>('solve', {
		config: configPayload,
		megaminxState: statePayload
	});
}

export async function cancelSolve(): Promise<void> {
	await invoke('cancel_solve');
}

export async function onSolverProgress(
	callback: (data: ProgressEvent) => void
): Promise<UnlistenFn> {
	return await listen<ProgressEvent>('solver:progress', (event) => {
		callback(event.payload);
	});
}

export async function onSolutionFound(callback: (solution: string) => void): Promise<UnlistenFn> {
	return await listen<string>('solver:solution', (event) => {
		callback(event.payload);
	});
}

export async function onSolveComplete(callback: () => void): Promise<UnlistenFn> {
	return await listen('solver:complete', () => {
		callback();
	});
}
