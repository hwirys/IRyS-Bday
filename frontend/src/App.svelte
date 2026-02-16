<script>
  import Sparkles from './lib/Sparkles.svelte';
  import Header from './lib/Header.svelte';
  import MessageCanvas from './lib/MessageCanvas.svelte';
  import Footer from './lib/Footer.svelte';
  import UserGuide from './lib/UserGuide.svelte';
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
