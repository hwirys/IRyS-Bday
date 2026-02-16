<script>
  import { onMount } from 'svelte';

  const STORAGE_KEY = 'irys-bday-guide-seen';

  let show = $state(false);

  onMount(() => {
    if (!localStorage.getItem(STORAGE_KEY)) {
      show = true;
    }
  });

  function dismiss() {
    show = false;
    localStorage.setItem(STORAGE_KEY, '1');
  }

  function open(e) {
    e.stopPropagation();
    show = true;
  }
</script>

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="guide-overlay" onclick={dismiss}>
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="guide-card" onclick={(e) => e.stopPropagation()}>
      <h2 class="guide-title">Welcome!</h2>
      <p class="guide-sub">Here's how to enjoy the birthday messages</p>

      <div class="tips">
        <div class="tip">
          <span class="tip-icon">
            <svg viewBox="0 0 24 24" fill="none" width="20" height="20">
              <path d="M12 9V12M12 15H12.01M7.8 21H16.2C17.88 21 18.72 21 19.362 20.673C19.927 20.385 20.385 19.927 20.673 19.362C21 18.72 21 17.88 21 16.2V7.8C21 6.12 21 5.28 20.673 4.638C20.385 4.073 19.927 3.615 19.362 3.327C18.72 3 17.88 3 16.2 3H7.8C6.12 3 5.28 3 4.638 3.327C4.073 3.615 3.615 4.073 3.327 4.638C3 5.28 3 6.12 3 7.8V16.2C3 17.88 3 18.72 3.327 19.362C3.615 19.927 4.073 20.385 4.638 20.673C5.28 21 6.12 21 7.8 21Z" stroke="#ee4f87" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
          </span>
          <div>
            <span class="tip-label">Tap a bubble</span>
            <span class="tip-desc">to read the full message</span>
          </div>
        </div>

        <div class="tip">
          <span class="tip-icon">
            <svg viewBox="0 0 24 24" fill="none" width="20" height="20">
              <path d="M9.5 14.5L3 21M14.333 9.667L21 3M7 3H3V7M21 17V21H17" stroke="#9b6bb0" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </span>
          <div>
            <span class="tip-label">Drag a bubble</span>
            <span class="tip-desc">to move it around</span>
          </div>
        </div>

        <div class="tip">
          <span class="tip-icon">
            <svg viewBox="0 0 24 24" fill="none" width="20" height="20">
              <path d="M9 5H7C5.89543 5 5 5.89543 5 7V19C5 20.1046 5.89543 21 7 21H17C18.1046 21 19 20.1046 19 19V7C19 5.89543 18.1046 5 17 5H15M9 5C9 6.10457 9.89543 7 11 7H13C14.1046 7 15 6.10457 15 5M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5M12 11V17M9 14H15" stroke="#64c8dc" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
          </span>
          <div>
            <span class="tip-label">Message list button</span>
            <span class="tip-desc">to browse all messages at once</span>
          </div>
        </div>

        <div class="tip">
          <span class="tip-icon">
            <svg viewBox="0 0 24 24" fill="none" width="20" height="20">
              <rect x="3" y="5" width="18" height="14" rx="3" stroke="#ffb844" stroke-width="1.5"/>
              <polygon points="10,8.5 16.5,12 10,15.5" fill="#ffb844" opacity="0.7"/>
            </svg>
          </span>
          <div>
            <span class="tip-label">Video button</span>
            <span class="tip-desc">to watch the IRyStocrats birthday video</span>
          </div>
        </div>
      </div>

      <button class="guide-close" onclick={dismiss}>Got it!</button>
    </div>
  </div>
{/if}

{#if !show}
  <button class="help-btn" onclick={open} aria-label="Help">
    <svg viewBox="0 0 24 24" fill="none" width="14" height="14">
      <path d="M9.09 9C9.3251 8.33167 9.78915 7.76811 10.4 7.40913C11.0108 7.05016 11.7289 6.91894 12.4272 7.03871C13.1255 7.15849 13.7588 7.52152 14.2151 8.06353C14.6713 8.60553 14.9211 9.29152 14.92 10C14.92 12 11.92 13 11.92 13M12 17H12.01M22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12Z" stroke="rgba(238,79,135,0.5)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </button>
{/if}

<style>
  .guide-overlay {
    position: fixed;
    inset: 0;
    z-index: 250;
    background: rgba(0, 0, 0, 0.2);
    display: flex;
    justify-content: center;
    align-items: center;
    animation: fadeIn 0.3s ease-out;
  }

  .guide-card {
    width: min(380px, 88vw);
    padding: clamp(20px, 4vw, 30px) clamp(22px, 4.5vw, 32px);
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-radius: 20px;
    border: 1.5px solid rgba(255, 255, 255, 0.7);
    box-shadow: 0 8px 40px rgba(238, 79, 135, 0.1);
    animation: cardPop 0.35s ease-out;
  }

  .guide-title {
    font-family: 'Zen Maru Gothic', sans-serif;
    font-size: clamp(1.2rem, 4vw, 1.5rem);
    font-weight: 700;
    text-align: center;
    background: linear-gradient(135deg, #ee4f87, #9b6bb0);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    color: transparent;
  }

  .guide-sub {
    text-align: center;
    font-size: clamp(0.75rem, 2vw, 0.88rem);
    color: #a888c0;
    margin-top: 4px;
    margin-bottom: clamp(16px, 3vw, 24px);
  }

  .tips {
    display: flex;
    flex-direction: column;
    gap: clamp(10px, 2vw, 14px);
  }

  .tip {
    display: flex;
    align-items: center;
    gap: clamp(10px, 2vw, 14px);
  }

  .tip-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.8);
    border: 1px solid rgba(238, 79, 135, 0.1);
  }

  .tip-label {
    display: block;
    font-size: clamp(0.82rem, 2.2vw, 0.92rem);
    font-weight: 700;
    color: #4a3d5c;
  }

  .tip-desc {
    display: block;
    font-size: clamp(0.7rem, 1.8vw, 0.8rem);
    color: #8a7a9e;
    margin-top: 1px;
  }

  .guide-close {
    display: block;
    width: 100%;
    margin-top: clamp(18px, 3.5vw, 26px);
    padding: clamp(10px, 2vw, 13px);
    border: none;
    border-radius: 14px;
    background: linear-gradient(135deg, #ee4f87, #9b6bb0);
    color: white;
    font-family: 'Nunito', sans-serif;
    font-size: clamp(0.85rem, 2.2vw, 0.95rem);
    font-weight: 700;
    letter-spacing: 0.03em;
    cursor: pointer;
    transition: opacity 0.2s ease, transform 0.2s ease;
  }

  .guide-close:hover {
    opacity: 0.9;
    transform: scale(1.02);
  }

  /* Help button (bottom-left) */
  .help-btn {
    position: fixed;
    bottom: clamp(12px, 2.5vw, 20px);
    left: clamp(12px, 2.5vw, 20px);
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: 1.5px solid rgba(238, 79, 135, 0.15);
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(238, 79, 135, 0.06);
    transition: background 0.2s ease, box-shadow 0.2s ease;
    animation: fadeIn 0.3s ease-out;
  }

  .help-btn:hover {
    background: rgba(255, 255, 255, 0.85);
    box-shadow: 0 3px 12px rgba(238, 79, 135, 0.12);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes cardPop {
    from { opacity: 0; transform: scale(0.92); }
    to { opacity: 1; transform: scale(1); }
  }
</style>
