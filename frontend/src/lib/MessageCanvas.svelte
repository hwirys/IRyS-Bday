<script>
  import { onMount } from 'svelte';
  import { fetchMessages } from './api.js';
  import MessageBubble from './MessageBubble.svelte';

  const PASTEL_COLORS = [
    'var(--pastel-pink)',
    'var(--pastel-lavender)',
    'var(--pastel-light-blue)',
    'var(--pastel-peach)',
    'var(--pastel-mint)',
    'var(--pastel-rose)',
    'var(--pastel-lilac)',
    'var(--pastel-sky)',
    'var(--pastel-coral)',
    'var(--pastel-butter)',
  ];

  let messages = $state([]);
  let bubbles = $state([]);
  let nextId = 0;

  const COLUMNS = 6;
  let positionQueue = [];
  let messageQueue = [];

  function nextLeft() {
    if (positionQueue.length === 0) {
      const colWidth = 80 / COLUMNS;
      for (let i = 0; i < COLUMNS; i++) {
        positionQueue.push(5 + colWidth * i + Math.random() * colWidth);
      }
      for (let i = positionQueue.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [positionQueue[i], positionQueue[j]] = [positionQueue[j], positionQueue[i]];
      }
    }
    return positionQueue.pop();
  }

  function randomBetween(min, max) {
    return min + Math.random() * (max - min);
  }

  function getMaxBubbles() {
    return window.innerWidth < 768 ? 20 : 40;
  }

  function nextMessage() {
    if (messageQueue.length === 0) {
      messageQueue = [...messages];
      for (let i = messageQueue.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [messageQueue[i], messageQueue[j]] = [messageQueue[j], messageQueue[i]];
      }
    }
    return messageQueue.pop();
  }

  function spawnBubble() {
    if (messages.length === 0) return;
    if (bubbles.length >= getMaxBubbles()) return;
    const msg = nextMessage();
    const bubble = {
      id: nextId++,
      name: msg.name,
      message: msg.message,
      color: PASTEL_COLORS[Math.floor(Math.random() * PASTEL_COLORS.length)],
      left: nextLeft(),
      duration: randomBetween(12, 22),
      swayDuration: randomBetween(3, 6),
      scale: randomBetween(0.85, 1.15),
    };
    bubbles = [...bubbles, bubble];
  }

  function removeBubble(id) {
    bubbles = bubbles.filter(b => b.id !== id);
  }

  onMount(async () => {
    try {
      messages = await fetchMessages();
    } catch {
      console.error('Failed to load messages');
      return;
    }

    // Initial burst of 10 bubbles with staggered timing
    for (let i = 0; i < 10; i++) {
      setTimeout(() => spawnBubble(), i * 300);
    }

    // Spawn a new bubble every ~1.2 seconds
    let interval = setInterval(spawnBubble, 1200);

    // Pause spawning when tab is hidden; clean up and resume when visible
    function onVisibilityChange() {
      if (document.hidden) {
        clearInterval(interval);
        interval = null;
      } else {
        // Remove all accumulated bubbles to avoid a burst of animations
        bubbles = [];
        // Re-seed with a small batch
        for (let i = 0; i < 5; i++) {
          setTimeout(() => spawnBubble(), i * 300);
        }
        interval = setInterval(spawnBubble, 1200);
      }
    }
    document.addEventListener('visibilitychange', onVisibilityChange);

    return () => {
      clearInterval(interval);
      document.removeEventListener('visibilitychange', onVisibilityChange);
    };
  });
</script>

<div class="canvas">
  {#each bubbles as bubble (bubble.id)}
    <MessageBubble
      name={bubble.name}
      message={bubble.message}
      color={bubble.color}
      left={bubble.left}
      duration={bubble.duration}
      swayDuration={bubble.swayDuration}
      scale={bubble.scale}
      onDone={() => removeBubble(bubble.id)}
    />
  {/each}
</div>

<style>
  .canvas {
    position: fixed;
    inset: 0;
    overflow: hidden;
    pointer-events: none;
  }

  .canvas :global(.bubble-wrap) {
    pointer-events: auto;
  }
</style>
