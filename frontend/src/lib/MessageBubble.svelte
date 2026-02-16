<script>
  import { onDestroy } from 'svelte';

  const { name, message, color, left, duration, swayDuration, scale, onDone } = $props();
  const truncated = $derived(message.length > 50 ? message.slice(0, 50) + '...' : message);

  let expanded = $state(false);
  let dragging = $state(false);
  let didDrag = false;
  let pointerActive = false;
  let dragX = $state(0);
  let dragY = $state(0);
  let startPointerX = 0;
  let startPointerY = 0;
  let startDragX = 0;
  let startDragY = 0;
  let autoReleaseTimer = null;

  function clearAutoRelease() {
    if (autoReleaseTimer) {
      clearTimeout(autoReleaseTimer);
      autoReleaseTimer = null;
    }
  }

  function collapse() {
    clearAutoRelease();
    expanded = false;
  }

  function handleAnimationEnd(e) {
    if (e.animationName === 'floatUp') {
      onDone?.();
    }
  }

  function onPointerDown(e) {
    pointerActive = true;
    dragging = true;
    didDrag = false;
    startPointerX = e.clientX;
    startPointerY = e.clientY;
    startDragX = dragX;
    startDragY = dragY;
    e.currentTarget.setPointerCapture(e.pointerId);
  }

  function onPointerMove(e) {
    if (!dragging) return;
    const dx = e.clientX - startPointerX;
    const dy = e.clientY - startPointerY;
    if (Math.abs(dx) > 4 || Math.abs(dy) > 4) didDrag = true;
    dragX = startDragX + dx;
    dragY = startDragY + dy;
  }

  function onPointerUp() {
    if (!pointerActive) return;
    pointerActive = false;
    dragging = false;
    if (!didDrag) {
      if (expanded) {
        collapse();
      } else {
        expanded = true;
        autoReleaseTimer = setTimeout(collapse, 30000);
      }
    }
  }

  onDestroy(clearAutoRelease);
</script>

<div
  class="bubble-wrap"
  class:dragging
  class:expanded
  style="
    --duration: {duration}s;
    --sway-duration: {swayDuration}s;
    --bubble-scale: {scale};
    left: {left}%;
    translate: {dragX}px {dragY}px;
  "
  onpointerdown={onPointerDown}
  onpointermove={onPointerMove}
  onpointerup={onPointerUp}
  onlostpointercapture={onPointerUp}
  onanimationend={handleAnimationEnd}
>
  <div class="bubble" style="--bg: {color};">
    <svg class="hat" viewBox="0 0 32 9" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M 4,7 C 4,2 24,2 24,7 Q 24,7.5 26,7.5 L 29,7.5 C 30,7.5 30,8.5 29,8.5 L 5,8.5 Q 4,8.5 4,7.5 Z" fill="white" stroke="rgba(0,0,0,0.1)" stroke-width="0.7"/>
    </svg>
    <div class="face">
      <span class="eye"></span>
      <span class="eye"></span>
    </div>
    <p class="msg">{expanded ? message : truncated}</p>
    <span class="author">&mdash; {name}</span>
  </div>
</div>

<style>
  .bubble-wrap {
    position: absolute;
    bottom: -200px;
    padding: clamp(6px, 1.5vw, 10px);
    max-width: 60vw;
    animation:
      floatUp var(--duration) linear forwards,
      sway var(--sway-duration) ease-in-out infinite;
    cursor: grab;
    -webkit-user-select: none;
    -moz-user-select: none;
    user-select: none;
    touch-action: none;
    will-change: transform, opacity;
  }

  .bubble-wrap:hover {
    animation-play-state: paused;
  }

  .bubble-wrap.dragging {
    animation-play-state: paused;
    cursor: grabbing;
    z-index: 200;
  }

  .bubble-wrap.expanded {
    animation-play-state: paused;
    z-index: 150;
  }

  .bubble-wrap.expanded .bubble {
    width: clamp(220px, 60vw, 320px);
    box-shadow: 0 4px 20px rgba(238, 79, 135, 0.12);
  }

  .bubble {
    position: relative;
    width: clamp(155px, 42vw, 220px);
    padding: clamp(14px, 2.8vw, 20px) clamp(16px, 3vw, 22px);
    padding-top: clamp(6px, 1.2vw, 8px);
    background: var(--bg);
    border-radius: 20px;
    border: 1.5px solid rgba(255, 255, 255, 0.5);
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.04);
    transition: transform 0.3s ease, box-shadow 0.3s ease, width 0.25s ease;
  }

  .hat {
    position: absolute;
    bottom: 100%;
    left: 34%;
    width: 36%;
    height: auto;
    pointer-events: none;
  }

  .face {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: clamp(12px, 2.5vw, 18px);
    margin-bottom: clamp(6px, 1.2vw, 10px);
    position: relative;
  }

  .eye {
    display: block;
    width: 4.5px;
    height: 4.5px;
    border-radius: 50%;
    background: #4a3d5c;
  }

  /* Smile */
  .face::after {
    content: '';
    position: absolute;
    bottom: -7px;
    left: 50%;
    transform: translateX(-50%);
    width: 10px;
    height: 5px;
    border: 1.5px solid #4a3d5c;
    border-top: none;
    border-radius: 0 0 10px 10px;
  }

  .bubble-wrap:hover:not(.dragging) .bubble {
    transform: scale(1.04);
    box-shadow: 0 5px 18px rgba(0, 0, 0, 0.06);
    z-index: 50;
  }

  .msg {
    font-size: clamp(0.88rem, 2.5vw, 1.0rem);
    font-weight: 500;
    color: #4a3d5c;
    line-height: 1.55;
    word-wrap: break-word;
    overflow-wrap: break-word;
  }

  .author {
    display: block;
    margin-top: clamp(6px, 1.2vw, 10px);
    font-size: clamp(0.72rem, 1.8vw, 0.82rem);
    font-weight: 700;
    color: #8a7a9e;
    opacity: 0.65;
    letter-spacing: 0.02em;
  }
</style>
