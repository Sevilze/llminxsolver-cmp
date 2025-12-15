export const MEGAMINX_COLORS = {
	yellow: '#E1E100',
	red: '#C80000',
	orange: '#E16400',
	green: '#00C800',
	pink: '#FF9696',
	blue: '#000096',
	gray: '#808080'
} as const;

export const STICKER_COLORS = [
	MEGAMINX_COLORS.yellow, // 0 - top face
	MEGAMINX_COLORS.red, // 1
	MEGAMINX_COLORS.orange, // 2
	MEGAMINX_COLORS.green, // 3
	MEGAMINX_COLORS.pink, // 4
	MEGAMINX_COLORS.blue // 5
] as const;

export const CORNER_COLOR_MAP: [number, number, number][] = [
	[0, 3, 4], // Corner 0
	[0, 4, 5], // Corner 1
	[0, 5, 1], // Corner 2
	[0, 1, 2], // Corner 3
	[0, 2, 3] // Corner 4
];

export const EDGE_COLOR_MAP: [number, number][] = [
	[0, 1], // Edge 0
	[0, 2], // Edge 1
	[0, 3], // Edge 2
	[0, 4], // Edge 3
	[0, 5] // Edge 4
];

export const SELECTION_COLOR = 'rgba(128, 0, 0, 0.9)';
export const HIGHLIGHT_COLOR = 'rgba(255, 0, 0, 0.15)';
export const STROKE_COLOR = '#000';
export const STROKE_WIDTH = 1.5;
export const STROKE_WIDTH_SELECTED = 3;
