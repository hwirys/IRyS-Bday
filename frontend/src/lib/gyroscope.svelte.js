// Shared reactive gyroscope state for tilt-based "snow globe" effect
// Uses a single $state object so the export reference is never reassigned

export const gyro = $state({
  tiltX: 0,
  tiltY: 0,
  supported: false,
  active: false,
});

const MAX_GAMMA = 30;
const MAX_BETA = 30;
const MAX_OFFSET_X = 25;
const MAX_OFFSET_Y = 15;
const SMOOTHING = 0.15;

let rawGamma = 0;
let rawBeta = 0;
let animFrameId = null;
let listening = false;

function clamp(val, min, max) {
  return Math.max(min, Math.min(max, val));
}

function onDeviceOrientation(event) {
  if (event.gamma != null) rawGamma = event.gamma;
  if (event.beta != null) rawBeta = event.beta;
}

function updateLoop() {
  const normalizedX = clamp(rawGamma / MAX_GAMMA, -1, 1);
  const normalizedY = clamp((rawBeta - 45) / MAX_BETA, -1, 1);

  const targetX = normalizedX * MAX_OFFSET_X;
  const targetY = normalizedY * MAX_OFFSET_Y;
  gyro.tiltX = gyro.tiltX + (targetX - gyro.tiltX) * SMOOTHING;
  gyro.tiltY = gyro.tiltY + (targetY - gyro.tiltY) * SMOOTHING;

  if (Math.abs(gyro.tiltX) < 0.1) gyro.tiltX = 0;
  if (Math.abs(gyro.tiltY) < 0.1) gyro.tiltY = 0;

  animFrameId = requestAnimationFrame(updateLoop);
}

function startListening() {
  if (listening) return;
  listening = true;
  window.addEventListener('deviceorientation', onDeviceOrientation);
  animFrameId = requestAnimationFrame(updateLoop);
  gyro.active = true;
}

function stopListening() {
  if (!listening) return;
  listening = false;
  window.removeEventListener('deviceorientation', onDeviceOrientation);
  if (animFrameId) cancelAnimationFrame(animFrameId);
  animFrameId = null;
  gyro.active = false;
  gyro.tiltX = 0;
  gyro.tiltY = 0;
}

export async function initGyroscope() {
  if (!('DeviceOrientationEvent' in window)) {
    gyro.supported = false;
    return () => {};
  }

  if (typeof DeviceOrientationEvent.requestPermission === 'function') {
    gyro.supported = true;
    return () => stopListening();
  }

  gyro.supported = true;
  startListening();

  return new Promise((resolve) => {
    setTimeout(() => {
      if (rawGamma === 0 && rawBeta === 0) {
        gyro.supported = false;
        stopListening();
      }
      resolve(() => stopListening());
    }, 1000);
  });
}

export async function requestGyroPermission() {
  if (typeof DeviceOrientationEvent.requestPermission !== 'function') {
    startListening();
    return true;
  }

  try {
    const permission = await DeviceOrientationEvent.requestPermission();
    if (permission === 'granted') {
      startListening();
      return true;
    }
    gyro.supported = false;
    return false;
  } catch {
    gyro.supported = false;
    return false;
  }
}
