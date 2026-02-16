<script>
  import { fetchConfig } from './api.js';
  import { onMount } from 'svelte';

  let name = $state('');
  let link = $state('');

  function isSafeUrl(s) {
    try {
      return ['http:', 'https:'].includes(new URL(s).protocol);
    } catch {
      return false;
    }
  }

  onMount(async () => {
    try {
      const config = await fetchConfig();
      name = config.powered_by_name;
      link = isSafeUrl(config.powered_by_link) ? config.powered_by_link : '';
    } catch {
      // silently ignore config load failure
    }
  });
</script>

{#if name}
  <div class="footer">
    {#if link}
      Powered by <a href={link} target="_blank" rel="noopener noreferrer">{name}</a>
    {:else}
      Powered by {name}
    {/if}
  </div>
{/if}

<style>
  .footer {
    position: fixed;
    bottom: clamp(8px, 2vw, 16px);
    right: clamp(10px, 2vw, 20px);
    font-size: clamp(0.58rem, 1.4vw, 0.75rem);
    font-weight: 500;
    color: rgba(140, 110, 160, 0.5);
    background: rgba(255, 255, 255, 0.55);
    padding: 4px 10px;
    border-radius: 10px;
    z-index: 100;
    -webkit-user-select: none;
    -moz-user-select: none;
    user-select: none;
  }

  .footer a {
    color: rgba(120, 80, 150, 0.6);
    text-decoration: none;
    font-weight: 600;
    transition: color 0.2s ease;
  }

  .footer a:hover {
    color: rgba(100, 60, 130, 0.85);
  }
</style>
