<script lang="ts">
  import {
    calculateMegaminxGeometry,
    pointsToPath,
    getCenterOfPoints,
  } from "$lib/utils/geometry";
  import {
    STICKER_COLORS,
    CORNER_COLOR_MAP,
    EDGE_COLOR_MAP,
    MEGAMINX_COLORS,
    SELECTION_COLOR,
    HIGHLIGHT_COLOR,
    STROKE_COLOR,
    STROKE_WIDTH,
    STROKE_WIDTH_SELECTED,
  } from "$lib/utils/colors";
  import type { MegaminxState, IgnoreFlags } from "$lib/types/megaminx";

  interface StickerInfo {
    type: "center" | "corner" | "edge";
    cubieIndex: number;
    orientationIndex: number;
  }

  interface Props {
    puzzleState: MegaminxState;
    ignoreFlags: IgnoreFlags;
    onSwapCorners?: (i: number, j: number) => void;
    onRotateCorner?: (index: number, direction: 1 | -1) => void;
    onSwapEdges?: (i: number, j: number) => void;
    onFlipEdge?: (index: number) => void;
    disabled?: boolean;
  }

  let {
    puzzleState,
    ignoreFlags,
    onSwapCorners,
    onRotateCorner,
    onSwapEdges,
    onFlipEdge,
    disabled = false,
  }: Props = $props();

  let containerEl: HTMLDivElement | null = $state(null);
  let dimensions = $state({ width: 300, height: 300 });
  let hoveredSticker: StickerInfo | null = $state(null);
  let selectedSticker: StickerInfo | null = $state(null);
  let isDragging = $state(false);

  $effect(() => {
    if (!containerEl) return;

    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (entry) {
        const size = Math.min(
          entry.contentRect.width,
          entry.contentRect.height
        );
        dimensions = { width: size, height: size };
      }
    });

    observer.observe(containerEl);
    return () => observer.disconnect();
  });

  let geometry = $derived(
    calculateMegaminxGeometry(dimensions.width, dimensions.height)
  );

  function getCornerColor(
    cubieIndex: number,
    orientationIndex: number
  ): string {
    const position = puzzleState.cornerPositions[cubieIndex];
    const orientation = puzzleState.cornerOrientations[cubieIndex];
    const effectiveOrientation = (orientationIndex - orientation + 3) % 3;

    if (ignoreFlags.cornerPositions) {
      if (effectiveOrientation !== 0 || ignoreFlags.cornerOrientations) {
        return MEGAMINX_COLORS.gray;
      }
    }

    if (ignoreFlags.cornerOrientations) {
      return MEGAMINX_COLORS.gray;
    }

    return STICKER_COLORS[CORNER_COLOR_MAP[position][effectiveOrientation]];
  }

  function getEdgeColor(cubieIndex: number, orientationIndex: number): string {
    const position = puzzleState.edgePositions[cubieIndex];
    const orientation = puzzleState.edgeOrientations[cubieIndex];
    const effectiveOrientation = Math.abs(orientationIndex - orientation);

    if (ignoreFlags.edgePositions) {
      if (effectiveOrientation !== 0 || ignoreFlags.edgeOrientations) {
        return MEGAMINX_COLORS.gray;
      }
    }

    if (ignoreFlags.edgeOrientations) {
      return MEGAMINX_COLORS.gray;
    }

    return STICKER_COLORS[EDGE_COLOR_MAP[position][effectiveOrientation]];
  }

  function handleStickerMouseEnter(info: StickerInfo) {
    if (disabled) return;
    hoveredSticker = info;
  }

  function handleStickerMouseLeave() {
    if (disabled) return;
    hoveredSticker = null;
  }

  function handleStickerMouseDown(info: StickerInfo, e: MouseEvent) {
    if (disabled) return;
    e.preventDefault();
    selectedSticker = info;
    isDragging = true;
  }

  function handleMouseUp() {
    if (!isDragging || !selectedSticker || disabled) {
      isDragging = false;
      selectedSticker = null;
      return;
    }

    if (hoveredSticker && selectedSticker.type === hoveredSticker.type) {
      if (selectedSticker.type === "corner") {
        if (selectedSticker.cubieIndex === hoveredSticker.cubieIndex) {
          const direction =
            (hoveredSticker.orientationIndex -
              selectedSticker.orientationIndex +
              3) %
              3 ===
            1
              ? 1
              : -1;
          onRotateCorner?.(selectedSticker.cubieIndex, direction as 1 | -1);
        } else {
          onSwapCorners?.(
            selectedSticker.cubieIndex,
            hoveredSticker.cubieIndex
          );
        }
      } else if (selectedSticker.type === "edge") {
        if (selectedSticker.cubieIndex === hoveredSticker.cubieIndex) {
          onFlipEdge?.(selectedSticker.cubieIndex);
        } else {
          onSwapEdges?.(selectedSticker.cubieIndex, hoveredSticker.cubieIndex);
        }
      }
    }

    isDragging = false;
    selectedSticker = null;
  }

  function isInteractionTarget(info: StickerInfo): boolean {
    if (!selectedSticker || !hoveredSticker || !isDragging) return false;
    return (
      info.type === selectedSticker.type &&
      info.cubieIndex === hoveredSticker.cubieIndex &&
      info.orientationIndex === hoveredSticker.orientationIndex
    );
  }

  let arrowData = $derived.by(() => {
    if (!isDragging || !selectedSticker || !hoveredSticker) return null;
    if (selectedSticker.type !== hoveredSticker.type) return null;

    let startPoints: { x: number; y: number }[];
    let endPoints: { x: number; y: number }[];

    if (selectedSticker.type === "corner") {
      const stickers = geometry.cornerStickers[selectedSticker.cubieIndex];
      const targetStickers = geometry.cornerStickers[hoveredSticker.cubieIndex];
      const orientations = ["top", "left", "right"] as const;
      startPoints = stickers[orientations[selectedSticker.orientationIndex]];
      endPoints = targetStickers[orientations[hoveredSticker.orientationIndex]];
    } else {
      const stickers = geometry.edgeStickers[selectedSticker.cubieIndex];
      const targetStickers = geometry.edgeStickers[hoveredSticker.cubieIndex];
      const orientations = ["top", "bottom"] as const;
      startPoints = stickers[orientations[selectedSticker.orientationIndex]];
      endPoints = targetStickers[orientations[hoveredSticker.orientationIndex]];
    }

    const start = getCenterOfPoints(startPoints);
    const end = getCenterOfPoints(endPoints);
    const angle = Math.atan2(end.y - start.y, end.x - start.x);
    const arrowSize = 10;

    return {
      start,
      end,
      angle,
      arrowSize,
      showReverseArrow:
        selectedSticker.cubieIndex !== hoveredSticker.cubieIndex,
    };
  });

  const EDGE_SIDES = ["top", "bottom"] as const;
  const CORNER_SIDES = ["top", "left", "right"] as const;
</script>

<div
  bind:this={containerEl}
  class="w-full aspect-square min-w-[200px] flex items-center justify-center"
  onmouseup={handleMouseUp}
  onmouseleave={handleMouseUp}
  role="application"
  aria-label="Megaminx puzzle editor"
>
  <svg
    width={dimensions.width}
    height={dimensions.height}
    viewBox="0 0 {dimensions.width} {dimensions.height}"
    class="select-none"
    style="cursor: {disabled ? 'default' : isDragging ? 'grabbing' : 'pointer'}"
  >
    <!-- Center pentagon -->
    <path
      d={pointsToPath(geometry.centerPoints)}
      fill={MEGAMINX_COLORS.yellow}
      stroke={STROKE_COLOR}
      stroke-width={STROKE_WIDTH}
    />

    <!-- Edge stickers -->
    {#each geometry.edgeStickers as edge, cubieIndex}
      <g>
        {#each EDGE_SIDES as side, orientationIndex}
          {@const info = {
            type: "edge" as const,
            cubieIndex,
            orientationIndex,
          }}
          {@const isSelected =
            selectedSticker?.type === "edge" &&
            selectedSticker?.cubieIndex === cubieIndex &&
            selectedSticker?.orientationIndex === orientationIndex}
          {@const isHovered =
            hoveredSticker?.type === "edge" &&
            hoveredSticker?.cubieIndex === cubieIndex &&
            hoveredSticker?.orientationIndex === orientationIndex}
          {@const isTarget = isInteractionTarget(info)}

          <g>
            {#if (isTarget || (isSelected && isDragging)) && selectedSticker}
              <path
                d={pointsToPath(
                  geometry.edgeStickers[cubieIndex][
                    selectedSticker.orientationIndex === 0 ? "top" : "bottom"
                  ]
                )}
                fill={HIGHLIGHT_COLOR}
                stroke="none"
              />
            {/if}
            <path
              d={pointsToPath(edge[side])}
              fill={getEdgeColor(cubieIndex, orientationIndex)}
              stroke={isSelected ? SELECTION_COLOR : STROKE_COLOR}
              stroke-width={isSelected ? STROKE_WIDTH_SELECTED : STROKE_WIDTH}
              onmouseenter={() => handleStickerMouseEnter(info)}
              onmouseleave={handleStickerMouseLeave}
              onmousedown={(e) => handleStickerMouseDown(info, e)}
              class={isHovered && !disabled ? "brightness-110" : ""}
              role="button"
              tabindex="-1"
            />
          </g>
        {/each}
      </g>
    {/each}

    <!-- Corner stickers -->
    {#each geometry.cornerStickers as corner, cubieIndex}
      <g>
        {#each CORNER_SIDES as side, orientationIndex}
          {@const info = {
            type: "corner" as const,
            cubieIndex,
            orientationIndex,
          }}
          {@const isSelected =
            selectedSticker?.type === "corner" &&
            selectedSticker?.cubieIndex === cubieIndex &&
            selectedSticker?.orientationIndex === orientationIndex}
          {@const isHovered =
            hoveredSticker?.type === "corner" &&
            hoveredSticker?.cubieIndex === cubieIndex &&
            hoveredSticker?.orientationIndex === orientationIndex}
          {@const isTarget = isInteractionTarget(info)}

          <g>
            {#if (isTarget || (isSelected && isDragging)) && selectedSticker}
              {@const sideKey = CORNER_SIDES[selectedSticker.orientationIndex]}
              <path
                d={pointsToPath(geometry.cornerStickers[cubieIndex][sideKey])}
                fill={HIGHLIGHT_COLOR}
                stroke="none"
              />
            {/if}
            <path
              d={pointsToPath(corner[side])}
              fill={getCornerColor(cubieIndex, orientationIndex)}
              stroke={isSelected ? SELECTION_COLOR : STROKE_COLOR}
              stroke-width={isSelected ? STROKE_WIDTH_SELECTED : STROKE_WIDTH}
              onmouseenter={() => handleStickerMouseEnter(info)}
              onmouseleave={handleStickerMouseLeave}
              onmousedown={(e) => handleStickerMouseDown(info, e)}
              class={isHovered && !disabled ? "brightness-110" : ""}
              role="button"
              tabindex="-1"
            />
          </g>
        {/each}
      </g>
    {/each}

    <!-- Interaction arrows -->
    {#if arrowData}
      <g pointer-events="none">
        <line
          x1={arrowData.start.x}
          y1={arrowData.start.y}
          x2={arrowData.end.x}
          y2={arrowData.end.y}
          stroke={SELECTION_COLOR}
          stroke-width={3}
          stroke-dasharray="6,8"
        />
        <polygon
          points="{arrowData.end.x},{arrowData.end.y}
            {arrowData.end.x -
            arrowData.arrowSize *
              Math.cos(arrowData.angle - Math.PI / 6)},{arrowData.end.y -
            arrowData.arrowSize * Math.sin(arrowData.angle - Math.PI / 6)}
            {arrowData.end.x -
            arrowData.arrowSize *
              Math.cos(arrowData.angle + Math.PI / 6)},{arrowData.end.y -
            arrowData.arrowSize * Math.sin(arrowData.angle + Math.PI / 6)}"
          fill={SELECTION_COLOR}
        />
        {#if arrowData.showReverseArrow}
          <polygon
            points="{arrowData.start.x},{arrowData.start.y}
              {arrowData.start.x +
              arrowData.arrowSize *
                Math.cos(arrowData.angle - Math.PI / 6)},{arrowData.start.y +
              arrowData.arrowSize * Math.sin(arrowData.angle - Math.PI / 6)}
              {arrowData.start.x +
              arrowData.arrowSize *
                Math.cos(arrowData.angle + Math.PI / 6)},{arrowData.start.y +
              arrowData.arrowSize * Math.sin(arrowData.angle + Math.PI / 6)}"
            fill={SELECTION_COLOR}
          />
        {/if}
      </g>
    {/if}
  </svg>
</div>
