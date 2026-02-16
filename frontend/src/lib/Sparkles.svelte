<script>
  import { onMount } from 'svelte';

  let particles = $state([]);

  onMount(() => {
    const count = 15;
    const items = [];
    for (let i = 0; i < count; i++) {
      items.push({
        id: i,
        left: Math.random() * 100,
        top: Math.random() * 100,
        size: 2 + Math.random() * 2.5,
        delay: Math.random() * 6,
        duration: 3 + Math.random() * 5,
        type: Math.random() > 0.82 ? 'diamond' : 'dot',
      });
    }
    particles = items;
  });
</script>

<div class="sparkles">
  {#each particles as p (p.id)}
    <span
      class="particle {p.type}"
      style="
        left: {p.left}%;
        top: {p.top}%;
        width: {p.size}px;
        height: {p.size}px;
        --delay: {p.delay}s;
        --dur: {p.duration}s;
      "
    ></span>
  {/each}
</div>

<style>
  .sparkles {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    height: 100dvh;
    z-index: 1;
    pointer-events: none;
    overflow: hidden;
  }

  .particle {
    position: absolute;
    animation: twinkle var(--dur) ease-in-out var(--delay) infinite;
    will-change: opacity;
  }

  .dot {
    border-radius: 50%;
    background: rgba(255, 230, 240, 0.55);
    box-shadow: 0 0 4px rgba(255, 200, 220, 0.25);
  }

  .diamond {
    border-radius: 1px;
    transform: rotate(45deg);
    background: rgba(238, 79, 135, 0.22);
    box-shadow: 0 0 4px rgba(238, 79, 135, 0.12);
    animation: diamondFloat var(--dur) ease-in-out var(--delay) infinite;
  }

  @keyframes diamondFloat {
    0%, 100% { opacity: 0.1; transform: rotate(45deg) translateY(0); }
    50% { opacity: 0.4; transform: rotate(45deg) translateY(-4px); }
  }
</style>
