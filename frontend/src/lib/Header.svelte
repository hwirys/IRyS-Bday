<script>
  import { fetchMessages } from './api.js';

  let visible = $state(true);
  let showList = $state(false);
  let showVideo = $state(false);
  let allMessages = $state([]);
  let tooltipStyle = $state('');
  let tooltipText = $state('');
  let tooltipVisible = $state(false);

  async function openMessageList(e) {
    e.stopPropagation();
    hideTooltip();
    if (allMessages.length === 0) {
      try {
        allMessages = await fetchMessages();
      } catch {
        console.error('Failed to load messages');
        return;
      }
    }
    showList = true;
  }

  function closeList(e) {
    e.stopPropagation();
    showList = false;
  }

  function openVideo(e) {
    e.stopPropagation();
    hideTooltip();
    showVideo = true;
  }

  function closeVideo(e) {
    e.stopPropagation();
    showVideo = false;
  }

  function stopProp(e) {
    e.stopPropagation();
  }

  function showTooltip(e, text = 'See every messages') {
    const rect = e.currentTarget.getBoundingClientRect();
    tooltipText = text;
    tooltipVisible = true;
    requestAnimationFrame(() => {
      const tip = document.querySelector('.fixed-tooltip');
      if (!tip) return;
      const tipW = tip.offsetWidth;
      let x = rect.left + rect.width / 2 - tipW / 2;
      let y = rect.bottom + 8;
      if (x + tipW > window.innerWidth - 8) x = window.innerWidth - tipW - 8;
      if (x < 8) x = 8;
      tooltipStyle = `left:${x}px;top:${y}px`;
    });
  }

  function hideTooltip() {
    tooltipVisible = false;
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="header-wrap" onclick={() => { hideTooltip(); visible = false; }}>
    <div class="cake">
      <!-- 5 Candles -->
      <div class="candles">
        {#each [0, 1, 2, 3, 4] as i}
          <div class="candle" style="--i: {i};">
            <div class="flame-wrap">
              <svg class="flame" viewBox="0 0 40 64" fill="none">
                <ellipse cx="20" cy="50" rx="10" ry="6" fill="rgba(255,180,60,0.15)" />
                <path d="M20,4 C24,14 30,28 28,40 C26,50 14,50 12,40 C10,28 16,14 20,4 Z"
                      fill="url(#fo-{i})" />
                <path d="M20,12 C23,20 27,30 25,38 C23,44 17,44 15,38 C13,30 17,20 20,12 Z"
                      fill="url(#fm-{i})" opacity="0.85" />
                <path d="M20,22 C22,28 23,33 22,37 C21,40 19,40 18,37 C17,33 18,28 20,22 Z"
                      fill="rgba(255,248,220,0.95)" />
                <ellipse cx="20" cy="6" rx="1.5" ry="2" fill="rgba(255,255,240,0.7)" />
                <defs>
                  <linearGradient id="fo-{i}" x1="0.5" y1="0" x2="0.5" y2="1">
                    <stop offset="0%" stop-color="#ffe880" />
                    <stop offset="40%" stop-color="#ffb844" />
                    <stop offset="75%" stop-color="#ff6a3d" />
                    <stop offset="100%" stop-color="#ee4f87" />
                  </linearGradient>
                  <linearGradient id="fm-{i}" x1="0.5" y1="0" x2="0.5" y2="1">
                    <stop offset="0%" stop-color="#fff4b8" />
                    <stop offset="50%" stop-color="#ffcc55" />
                    <stop offset="100%" stop-color="#ff8844" />
                  </linearGradient>
                </defs>
              </svg>
            </div>
            <div class="candle-stick"></div>
          </div>
        {/each}
      </div>

      <!-- Cake top tier (frosting edge) -->
      <div class="frosting">
        <svg class="frosting-svg" viewBox="0 0 200 16" preserveAspectRatio="none">
          <path d="M0,16 C10,0 20,0 30,12 C40,0 50,0 60,12 C70,0 80,0 90,12 C100,0 110,0 120,12 C130,0 140,0 150,12 C160,0 170,0 180,12 C190,0 200,0 200,16 Z"
                fill="rgba(255,255,255,0.85)" />
        </svg>
      </div>

      <!-- Cake body -->
      <div class="cake-body">
        <svg class="deco left-deco" viewBox="0 0 24 32" fill="none">
          <polygon points="12,2 22,12 12,30 2,12" fill="url(#dg-l)" opacity="0.3" />
          <line x1="12" y1="2" x2="12" y2="30" stroke="rgba(255,255,255,0.5)" stroke-width="0.5" />
          <line x1="2" y1="12" x2="22" y2="12" stroke="rgba(255,255,255,0.4)" stroke-width="0.5" />
          <defs>
            <linearGradient id="dg-l" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stop-color="#ee4f87" />
              <stop offset="100%" stop-color="#9b6bb0" />
            </linearGradient>
          </defs>
        </svg>
        <svg class="deco right-deco" viewBox="0 0 24 32" fill="none">
          <polygon points="12,2 22,12 12,30 2,12" fill="url(#dg-r)" opacity="0.3" />
          <line x1="12" y1="2" x2="12" y2="30" stroke="rgba(255,255,255,0.5)" stroke-width="0.5" />
          <line x1="2" y1="12" x2="22" y2="12" stroke="rgba(255,255,255,0.4)" stroke-width="0.5" />
          <defs>
            <linearGradient id="dg-r" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stop-color="#9b6bb0" />
              <stop offset="100%" stop-color="#ee4f87" />
            </linearGradient>
          </defs>
        </svg>

        <h1>Happy Birthday, IRyS!</h1>
        <p class="subtitle">A constellation of wishes from IRyStocrats!</p>

        <div class="bottom-accent">
          <span class="accent-line"></span>
          <svg class="accent-diamond" viewBox="0 0 10 10" fill="none">
            <rect x="5" y="1" width="4" height="4" transform="rotate(45, 5, 5)"
                  fill="rgba(238, 79, 135, 0.3)" />
          </svg>
          <span class="accent-line"></span>
        </div>

        <p class="dismiss-hint">click anywhere to dismiss</p>
      </div>

      <!-- Cake bottom tier -->
      <div class="cake-base"></div>
    </div>

    <!-- Buttons below cake -->
    <div class="cake-btns">
      <button class="msg-list-btn cake-pos" onclick={openMessageList}
              onmouseenter={showTooltip} onmouseleave={hideTooltip}
              aria-label="See every messages">
        <svg viewBox="0 0 20 22" fill="none" width="16" height="16">
          <rect x="2" y="1" width="16" height="20" rx="2" stroke="rgba(238,79,135,0.5)" stroke-width="1.5" fill="rgba(255,255,255,0.6)" />
          <line x1="5.5" y1="6" x2="14.5" y2="6" stroke="rgba(238,79,135,0.35)" stroke-width="1.2" stroke-linecap="round" />
          <line x1="5.5" y1="10" x2="14.5" y2="10" stroke="rgba(238,79,135,0.35)" stroke-width="1.2" stroke-linecap="round" />
          <line x1="5.5" y1="14" x2="11" y2="14" stroke="rgba(238,79,135,0.35)" stroke-width="1.2" stroke-linecap="round" />
        </svg>
      </button>
      <button class="msg-list-btn cake-pos" onclick={openVideo}
              onmouseenter={(e) => showTooltip(e, 'See IRyStocrats video!')} onmouseleave={hideTooltip}
              aria-label="See IRyStocrats video!">
        <svg viewBox="0 0 20 20" fill="none" width="16" height="16">
          <rect x="1" y="3" width="18" height="14" rx="3" stroke="rgba(238,79,135,0.5)" stroke-width="1.5" fill="rgba(255,255,255,0.6)" />
          <polygon points="8,6.5 14.5,10 8,13.5" fill="rgba(238,79,135,0.45)" />
        </svg>
      </button>
    </div>
  </div>
{/if}

<!-- Top-right buttons when cake is hidden -->
{#if !visible}
  <div class="top-btns">
    <button class="icon-btn" onclick={() => { hideTooltip(); visible = true; }}
            onmouseenter={(e) => showTooltip(e, 'Birthday Cake!')} onmouseleave={hideTooltip}
            aria-label="Show cake">
      <svg viewBox="0 0 24 24" fill="none" width="16" height="16">
        <rect x="9" y="2" width="6" height="14" rx="3" fill="#f8c8d8" stroke="rgba(238,79,135,0.4)" stroke-width="1" />
        <path d="M12,0 C13,3 14,5 12,8 C10,5 11,3 12,0 Z" fill="#ffcc55" opacity="0.8" />
        <rect x="2" y="14" width="20" height="10" rx="3" fill="rgba(255,255,255,0.8)" stroke="rgba(238,79,135,0.25)" stroke-width="1" />
      </svg>
    </button>
    <button class="icon-btn" onclick={openMessageList}
            onmouseenter={showTooltip} onmouseleave={hideTooltip}
            aria-label="See every messages">
      <svg viewBox="0 0 20 22" fill="none" width="16" height="16">
        <rect x="2" y="1" width="16" height="20" rx="2" stroke="rgba(238,79,135,0.5)" stroke-width="1.5" fill="rgba(255,255,255,0.6)" />
        <line x1="5.5" y1="6" x2="14.5" y2="6" stroke="rgba(238,79,135,0.35)" stroke-width="1.2" stroke-linecap="round" />
        <line x1="5.5" y1="10" x2="14.5" y2="10" stroke="rgba(238,79,135,0.35)" stroke-width="1.2" stroke-linecap="round" />
        <line x1="5.5" y1="14" x2="11" y2="14" stroke="rgba(238,79,135,0.35)" stroke-width="1.2" stroke-linecap="round" />
      </svg>
    </button>
    <button class="icon-btn" onclick={openVideo}
            onmouseenter={(e) => showTooltip(e, 'See IRyStocrats video!')} onmouseleave={hideTooltip}
            aria-label="See IRyStocrats video!">
      <svg viewBox="0 0 20 20" fill="none" width="16" height="16">
        <rect x="1" y="3" width="18" height="14" rx="3" stroke="rgba(238,79,135,0.5)" stroke-width="1.5" fill="rgba(255,255,255,0.6)" />
        <polygon points="8,6.5 14.5,10 8,13.5" fill="rgba(238,79,135,0.45)" />
      </svg>
    </button>
  </div>
{/if}

<!-- Full message list overlay -->
{#if showList}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="overlay" onclick={closeList}>
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="msg-panel" onclick={stopProp}>
      <div class="panel-header">
        <h2>All Messages</h2>
        <button class="close-btn" onclick={closeList} aria-label="Close">&times;</button>
      </div>
      <div class="panel-body">
        {#each allMessages as msg, i}
          <div class="msg-card" style="--card-bg: {['var(--pastel-pink)','var(--pastel-lavender)','var(--pastel-light-blue)','var(--pastel-peach)','var(--pastel-rose)','var(--pastel-lilac)','var(--pastel-sky)','var(--pastel-coral)','var(--pastel-butter)','var(--pastel-mint)'][i % 10]};">
            <p class="msg-text">{msg.message}</p>
            <span class="msg-author">&mdash; {msg.name}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<!-- Video overlay -->
{#if showVideo}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="overlay" onclick={closeVideo}>
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="video-panel" onclick={stopProp}>
      <div class="panel-header">
        <h2>IRyStocrats Video</h2>
        <button class="close-btn" onclick={closeVideo} aria-label="Close">&times;</button>
      </div>
      <div class="video-body">
        <div class="video-wrapper">
          <iframe
            src="https://www.youtube.com/embed/pAKdNZfDm-c"
            title="IRyStocrats Birthday Video"
            frameborder="0"
            allow="encrypted-media; picture-in-picture"
            allowfullscreen
            sandbox="allow-scripts allow-same-origin allow-presentation"
          ></iframe>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Fixed tooltip (positioned via JS) -->
{#if tooltipVisible}
  <div class="fixed-tooltip" style={tooltipStyle}>{tooltipText}</div>
{/if}

<style>
  .header-wrap {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100vh;
    height: 100dvh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    z-index: 100;
    pointer-events: auto;
    cursor: pointer;
    animation: cakeFadeIn 0.4s ease-out;
  }

  .cake {
    pointer-events: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    max-width: 90vw;
  }

  /* Candles row */
  .candles {
    display: flex;
    justify-content: center;
    gap: clamp(16px, 4vw, 28px);
    margin-bottom: -2px;
    z-index: 2;
  }

  .candle {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .flame-wrap {
    transform-origin: bottom center;
    animation:
      flameSway 2s ease-in-out calc(var(--i) * 0.4s) infinite,
      flameBreath 1.2s ease-in-out calc(var(--i) * 0.25s) infinite;
    will-change: transform;
  }

  .flame {
    width: clamp(16px, 4vw, 24px);
    height: clamp(28px, 6.5vw, 42px);
    filter: drop-shadow(0 0 6px rgba(255, 180, 60, 0.4));
  }

  .candle-stick {
    width: clamp(5px, 1.2vw, 7px);
    height: clamp(18px, 4vw, 28px);
    border-radius: 2px 2px 0 0;
    background: linear-gradient(180deg, #f8c8d8, #ee4f87);
    box-shadow: 0 0 4px rgba(238, 79, 135, 0.15);
  }

  /* Frosting scallop edge */
  .frosting {
    width: 100%;
    position: relative;
    z-index: 1;
    line-height: 0;
  }

  .frosting-svg {
    width: 100%;
    height: clamp(8px, 2vw, 14px);
    display: block;
  }

  /* Main cake body */
  .cake-body {
    position: relative;
    text-align: center;
    width: 100%;
    padding: clamp(20px, 3.5vw, 32px) clamp(36px, 7vw, 80px);
    background: rgba(255, 255, 255, 0.75);
    -webkit-backdrop-filter: blur(8px);
    backdrop-filter: blur(8px);
    border: none;
    box-shadow:
      0 4px 20px rgba(238, 79, 135, 0.08),
      0 0 40px rgba(238, 79, 135, 0.04);
  }

  .cake-body::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(
      180deg,
      transparent,
      rgba(100, 200, 220, 0.12) 20%,
      transparent 50%,
      rgba(155, 107, 176, 0.12) 80%,
      transparent
    );
    z-index: -1;
    -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor;
    mask-composite: exclude;
    padding: 1.5px;
  }

  /* Cake base platform */
  .cake-base {
    width: calc(100% + clamp(16px, 3vw, 28px));
    height: clamp(10px, 2vw, 16px);
    background: rgba(255, 255, 255, 0.7);
    border-radius: 0 0 clamp(10px, 2vw, 14px) clamp(10px, 2vw, 14px);
    box-shadow: 0 4px 12px rgba(238, 79, 135, 0.06);
  }

  /* Diamond decorations */
  .deco {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    width: clamp(12px, 3vw, 20px);
    height: auto;
    animation: twinkle 3.5s ease-in-out infinite;
  }

  .left-deco { left: clamp(10px, 2vw, 18px); }
  .right-deco { right: clamp(10px, 2vw, 18px); animation-delay: 1.8s; }

  h1 {
    font-family: 'Zen Maru Gothic', sans-serif;
    font-size: clamp(1.5rem, 6vw, 3rem);
    font-weight: 700;
    letter-spacing: 0.02em;
    background: linear-gradient(135deg, #64c8dc 0%, #ee4f87 30%, #9b6bb0 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    color: transparent;
    white-space: nowrap;
  }

  .subtitle {
    margin-top: clamp(6px, 1.2vw, 10px);
    font-size: clamp(0.85rem, 2.5vw, 1.1rem);
    font-weight: 500;
    color: #a888c0;
    letter-spacing: 0.04em;
  }

  .bottom-accent {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: clamp(8px, 2vw, 14px);
    margin-top: clamp(12px, 2.5vw, 20px);
  }

  .accent-line {
    width: clamp(28px, 7vw, 48px);
    height: 1.5px;
    background: linear-gradient(90deg, transparent, rgba(238, 79, 135, 0.25), transparent);
  }

  .accent-diamond {
    width: clamp(10px, 2.5vw, 14px);
    height: clamp(10px, 2.5vw, 14px);
    animation: twinkle 4s ease-in-out 0.5s infinite;
  }

  .dismiss-hint {
    margin-top: clamp(10px, 2vw, 16px);
    font-size: clamp(0.55rem, 1.4vw, 0.68rem);
    font-weight: 500;
    color: rgba(170, 140, 190, 0.5);
    letter-spacing: 0.06em;
  }

  /* ---- Message list button (on cake) ---- */
  .msg-list-btn {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: clamp(36px, 8vw, 44px);
    height: clamp(36px, 8vw, 44px);
    border: 1.5px solid rgba(238, 79, 135, 0.2);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    box-shadow: 0 2px 10px rgba(238, 79, 135, 0.06);
    transition: background 0.2s ease, box-shadow 0.2s ease;
  }

  .msg-list-btn:hover {
    background: rgba(255, 255, 255, 0.85);
    box-shadow: 0 4px 16px rgba(238, 79, 135, 0.12);
  }

  .cake-btns {
    display: flex;
    gap: 8px;
    justify-content: center;
    margin-top: clamp(10px, 2vw, 16px);
    pointer-events: auto;
  }

  .msg-list-btn.cake-pos {
    pointer-events: auto;
  }

  /* Fixed tooltip */
  .fixed-tooltip {
    position: fixed;
    z-index: 300;
    white-space: nowrap;
    font-size: clamp(0.58rem, 1.3vw, 0.68rem);
    font-weight: 600;
    color: #8a6a9e;
    background: rgba(255, 255, 255, 0.92);
    padding: 3px 8px;
    border-radius: 6px;
    box-shadow: 0 1px 6px rgba(0, 0, 0, 0.06);
    pointer-events: none;
    animation: tooltipIn 0.15s ease-out;
  }

  @keyframes tooltipIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  /* ---- Top-right buttons (cake hidden) ---- */
  .top-btns {
    position: fixed;
    top: clamp(12px, 2.5vw, 20px);
    right: clamp(12px, 2.5vw, 20px);
    z-index: 100;
    display: flex;
    gap: 8px;
    animation: cakeFadeIn 0.3s ease-out;
  }

  .icon-btn {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: clamp(36px, 8vw, 44px);
    height: clamp(36px, 8vw, 44px);
    border: 1.5px solid rgba(238, 79, 135, 0.2);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    box-shadow: 0 2px 10px rgba(238, 79, 135, 0.06);
    transition: background 0.2s ease, box-shadow 0.2s ease;
  }

  .icon-btn:hover {
    background: rgba(255, 255, 255, 0.85);
    box-shadow: 0 4px 16px rgba(238, 79, 135, 0.12);
  }


  /* ---- Message list overlay ---- */
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 200;
    background: rgba(0, 0, 0, 0.25);
    display: flex;
    justify-content: center;
    align-items: center;
    animation: cakeFadeIn 0.25s ease-out;
  }

  .msg-panel {
    width: min(500px, 90vw);
    max-height: 80vh;
    max-height: 80dvh;
    display: flex;
    flex-direction: column;
    background: rgba(255, 255, 255, 0.88);
    border-radius: 20px;
    border: 1.5px solid rgba(255, 255, 255, 0.7);
    box-shadow: 0 8px 40px rgba(238, 79, 135, 0.1);
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: clamp(14px, 3vw, 20px) clamp(18px, 4vw, 24px);
    border-bottom: 1px solid rgba(238, 79, 135, 0.1);
  }

  .panel-header h2 {
    font-family: 'Zen Maru Gothic', sans-serif;
    font-size: clamp(1rem, 3vw, 1.3rem);
    font-weight: 700;
    background: linear-gradient(135deg, #ee4f87, #9b6bb0);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    color: transparent;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 8px;
    background: transparent;
    font-size: 1.4rem;
    color: #b8a0c0;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .close-btn:hover {
    background: rgba(238, 79, 135, 0.08);
  }

  .panel-body {
    overflow-y: auto;
    padding: clamp(12px, 2.5vw, 18px);
    display: flex;
    flex-direction: column;
    gap: clamp(8px, 1.5vw, 12px);
  }

  .msg-card {
    padding: clamp(10px, 2vw, 14px) clamp(12px, 2.5vw, 16px);
    background: var(--card-bg);
    border-radius: 14px;
    border: 1px solid rgba(255, 255, 255, 0.5);
  }

  .msg-text {
    font-size: clamp(0.78rem, 2vw, 0.88rem);
    font-weight: 500;
    color: #4a3d5c;
    line-height: 1.55;
    word-wrap: break-word;
    overflow-wrap: break-word;
  }

  .msg-author {
    display: block;
    margin-top: clamp(4px, 1vw, 8px);
    font-size: clamp(0.65rem, 1.6vw, 0.74rem);
    font-weight: 700;
    color: #8a7a9e;
    opacity: 0.65;
  }

  /* ---- Video overlay panel ---- */
  .video-panel {
    width: min(720px, 92vw);
    max-height: 90vh;
    max-height: 90dvh;
    display: flex;
    flex-direction: column;
    background: rgba(255, 255, 255, 0.88);
    border-radius: 20px;
    border: 1.5px solid rgba(255, 255, 255, 0.7);
    box-shadow: 0 8px 40px rgba(238, 79, 135, 0.1);
    overflow: hidden;
  }

  .video-body {
    padding: clamp(8px, 2vw, 16px);
  }

  .video-wrapper {
    position: relative;
    width: 100%;
    padding-top: 56.25%;
    border-radius: 12px;
    overflow: hidden;
  }

  .video-wrapper iframe {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border: none;
    border-radius: 12px;
  }

  /* Animations */
  @keyframes flameSway {
    0%, 100% { transform: rotate(0deg); }
    25% { transform: rotate(2.5deg); }
    75% { transform: rotate(-2.5deg); }
  }

  @keyframes flameBreath {
    0%, 100% { transform: scaleY(1) scaleX(1); }
    30% { transform: scaleY(1.08) scaleX(0.92); }
    60% { transform: scaleY(0.93) scaleX(1.06); }
  }

  @keyframes cakeFadeIn {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }
</style>
