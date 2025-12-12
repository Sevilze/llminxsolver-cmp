interface Point {
  x: number;
  y: number;
}

export function pointAt(center: Point, distance: number, angle: number): Point {
  return {
    x: center.x + distance * Math.cos(angle),
    y: center.y + distance * Math.sin(angle),
  };
}

export function lerp(p1: Point, p2: Point, fraction: number): Point {
  return {
    x: p1.x * (1 - fraction) + p2.x * fraction,
    y: p1.y * (1 - fraction) + p2.y * fraction,
  };
}

export function distance(p1: Point, p2: Point): number {
  const dx = p2.x - p1.x;
  const dy = p2.y - p1.y;
  return Math.sqrt(dx * dx + dy * dy);
}

export function lineIntersection(
  x1: number,
  y1: number,
  x2: number,
  y2: number,
  x3: number,
  y3: number,
  x4: number,
  y4: number
): Point {
  const det = (a: number, b: number, c: number, d: number) => a * d - b * c;
  const denom = det(x1 - x2, y1 - y2, x3 - x4, y3 - y4);
  const det12 = det(x1, y1, x2, y2);
  const det34 = det(x3, y3, x4, y4);
  return {
    x: det(det12, x1 - x2, det34, x3 - x4) / denom,
    y: det(det12, y1 - y2, det34, y3 - y4) / denom,
  };
}

export interface MegaminxGeometry {
  centerPoints: Point[];
  innerCorners: Point[];
  middleCorners: Point[];
  outerCorners: Point[];
  middleEdgesLeft: Point[];
  middleEdgesRight: Point[];
  edgeStickers: { top: Point[]; bottom: Point[] }[];
  cornerStickers: { top: Point[]; left: Point[]; right: Point[] }[];
}

export function calculateMegaminxGeometry(
  width: number,
  height: number,
  padding: number = 10
): MegaminxGeometry {
  const halfWidth = width / 2;
  const halfHeight = height / 2;
  const outerRadius = Math.min(halfHeight, halfWidth) - padding;
  const middleRadius = (3 * outerRadius) / 4;
  const innerRadius = outerRadius / 3;
  const center: Point = { x: halfWidth, y: halfHeight };

  const innerCorners: Point[] = [];
  const middleCorners: Point[] = [];
  const outerCorners: Point[] = [];
  const centerPoints: Point[] = [];

  for (let i = 0; i < 5; i++) {
    const angle = -Math.PI / 2 + (i / 5) * Math.PI * 2;
    innerCorners.push(pointAt(center, innerRadius, angle));
    middleCorners.push(pointAt(center, middleRadius, angle));
    outerCorners.push(pointAt(center, outerRadius, angle));
    centerPoints.push(pointAt(center, innerRadius, angle));
  }

  const middleEdgesLeft: Point[] = [];
  const middleEdgesRight: Point[] = [];

  for (let i = 0; i < 5; i++) {
    const prevCorner = (i + 4) % 5;
    const nextCorner = (i + 1) % 5;

    const intersectionRight = lineIntersection(
      innerCorners[prevCorner].x,
      innerCorners[prevCorner].y,
      innerCorners[i].x,
      innerCorners[i].y,
      middleCorners[i].x,
      middleCorners[i].y,
      middleCorners[nextCorner].x,
      middleCorners[nextCorner].y
    );
    middleEdgesRight[i] = intersectionRight;

    const intersectionLeft = lineIntersection(
      innerCorners[i].x,
      innerCorners[i].y,
      innerCorners[nextCorner].x,
      innerCorners[nextCorner].y,
      middleCorners[prevCorner].x,
      middleCorners[prevCorner].y,
      middleCorners[i].x,
      middleCorners[i].y
    );
    middleEdgesLeft[prevCorner] = intersectionLeft;
  }

  const edgeStickers: { top: Point[]; bottom: Point[] }[] = [];
  const cornerStickers: { top: Point[]; left: Point[]; right: Point[] }[] = [];

  for (let i = 0; i < 5; i++) {
    const prevCorner = (i + 4) % 5;
    const nextCorner = (i + 1) % 5;

    const fraction =
      distance(middleEdgesLeft[prevCorner], innerCorners[i]) /
      distance(middleEdgesLeft[prevCorner], middleEdgesRight[nextCorner]);

    const leftOuterCorner = lerp(
      outerCorners[i],
      outerCorners[nextCorner],
      fraction
    );
    const rightOuterCorner = lerp(
      outerCorners[i],
      outerCorners[prevCorner],
      fraction
    );
    const leftOuterEdge = lerp(
      outerCorners[nextCorner],
      outerCorners[i],
      fraction
    );

    // Edge stickers (index matches the edge between corner i and corner i+1)
    const edgeIndex = (i + 3) % 5;
    if (!edgeStickers[edgeIndex]) {
      edgeStickers[edgeIndex] = { top: [], bottom: [] };
    }

    edgeStickers[edgeIndex].top = [
      innerCorners[i],
      innerCorners[nextCorner],
      middleEdgesLeft[i],
      middleEdgesRight[i],
    ];

    edgeStickers[edgeIndex].bottom = [
      leftOuterCorner,
      middleEdgesRight[i],
      middleEdgesLeft[i],
      leftOuterEdge,
    ];

    // Corner stickers
    cornerStickers[i] = {
      top: [
        innerCorners[i],
        middleEdgesLeft[prevCorner],
        middleCorners[i],
        middleEdgesRight[i],
      ],
      left: [
        middleCorners[i],
        middleEdgesRight[i],
        leftOuterCorner,
        outerCorners[i],
      ],
      right: [
        middleCorners[i],
        middleEdgesLeft[prevCorner],
        rightOuterCorner,
        outerCorners[i],
      ],
    };
  }

  return {
    centerPoints,
    innerCorners,
    middleCorners,
    outerCorners,
    middleEdgesLeft,
    middleEdgesRight,
    edgeStickers,
    cornerStickers,
  };
}

export function pointsToPath(points: Point[]): string {
  if (points.length === 0) return "";
  return (
    points
      .map(
        (p, i) => `${i === 0 ? "M" : "L"} ${p.x.toFixed(2)} ${p.y.toFixed(2)}`
      )
      .join(" ") + " Z"
  );
}

export function getCenterOfPoints(points: Point[]): Point {
  const sum = points.reduce((acc, p) => ({ x: acc.x + p.x, y: acc.y + p.y }), {
    x: 0,
    y: 0,
  });
  return { x: sum.x / points.length, y: sum.y / points.length };
}
