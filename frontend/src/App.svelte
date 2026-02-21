<script>
  import { onMount } from 'svelte';
  import Sparkles from './lib/Sparkles.svelte';
  import Header from './lib/Header.svelte';
  import MessageCanvas from './lib/MessageCanvas.svelte';
  import Footer from './lib/Footer.svelte';
  import UserGuide from './lib/UserGuide.svelte';
  import { initGyroscope } from './lib/gyroscope.svelte.js';

  onMount(() => {
    let cleanup = () => {};
    initGyroscope().then(fn => { cleanup = fn; });
    return () => cleanup();
  });
</script>

<div class="app-container">
  <Sparkles />
  <MessageCanvas />
  <Header />
  <Footer />
  <UserGuide />
</div>

<style>
  .app-container {
    width: 100vw;
    height: 100vh;
    height: 100dvh;
    position: relative;
    overflow: hidden;
  }
</style>
