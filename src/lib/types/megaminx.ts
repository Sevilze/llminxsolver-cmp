export type CornerPosition = 0 | 1 | 2 | 3 | 4;
export type EdgePosition = 0 | 1 | 2 | 3 | 4;
export type CornerOrientation = 0 | 1 | 2;
export type EdgeOrientation = 0 | 1;

export interface MegaminxState {
	cornerPositions: CornerPosition[];
	cornerOrientations: CornerOrientation[];
	edgePositions: EdgePosition[];
	edgeOrientations: EdgeOrientation[];
}

export interface IgnoreFlags {
	cornerPositions: boolean;
	edgePositions: boolean;
	cornerOrientations: boolean;
	edgeOrientations: boolean;
}

export type AllowedFacesMode =
	| 'R_U'
	| 'R_U_L'
	| 'R_U_F'
	| 'R_U_bL'
	| 'R_U_bR'
	| 'R_U_L_F'
	| 'R_U_L_F_bL';

export type MetricType = 'FTM' | 'QTM';

export interface SolverConfig {
	allowedFaces: AllowedFacesMode;
	metric: MetricType;
	limitDepth: boolean;
	maxDepth: number;
	ignoreFlags: IgnoreFlags;
}

export interface SolverState {
	isSearching: boolean;
	progress: number;
	status: string;
	solutions: string[];
}

export type StickerType = 'center' | 'corner' | 'edge';

export interface Sticker {
	type: StickerType;
	cubieIndex: number;
	orientationIndex: number;
	points: { x: number; y: number }[];
}

export interface DragState {
	active: boolean;
	sourceSticker: Sticker | null;
	targetSticker: Sticker | null;
}

export interface ScoredSolution {
	algorithm: string;
	mcc: number;
	moveCount: number;
}
