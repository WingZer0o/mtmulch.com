<template>
  <div class="tuner-page">
    <div class="tuner-card">
      <p class="label-chip">Tool</p>
      <h1>Guitar Tuner</h1>
      <p class="sub">Standard Tuning · EADGBe</p>

      <canvas ref="canvasEl" class="tuner-canvas" width="520" height="320" />

      <div class="strings-row">
        <div
          v-for="s in STRINGS"
          :key="s.note"
          class="s-chip"
          :class="{ 'is-active': nearestString?.note === s.note && hasSignal }"
        >
          {{ s.note }}
        </div>
      </div>

      <!-- Device selector — visible once permission is granted -->
      <div v-if="devices.length > 0" class="device-row">
        <label class="device-label" for="audio-input">Input</label>
        <select
          id="audio-input"
          class="device-select"
          :value="selectedDeviceId"
          :disabled="isListening"
          @change="onDeviceChange"
        >
          <option v-for="d in devices" :key="d.deviceId" :value="d.deviceId">
            {{ d.label || `Microphone ${d.deviceId.slice(0, 8)}` }}
          </option>
        </select>
      </div>

      <button class="start-btn" :class="{ 'is-on': isListening }" @click="toggle">
        <span class="btn-dot" :class="{ 'is-on': isListening }" />
        {{ isListening ? 'Stop' : 'Start Tuner' }}
      </button>

      <p v-if="errMsg" class="err-msg">{{ errMsg }}</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onUnmounted } from 'vue'

const STRINGS = [
  { note: 'E2', freq: 82.41 },
  { note: 'A2', freq: 110.00 },
  { note: 'D3', freq: 146.83 },
  { note: 'G3', freq: 196.00 },
  { note: 'B3', freq: 246.94 },
  { note: 'E4', freq: 329.63 },
]

const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']

// Canvas geometry
const CW = 520
const CH = 320
const CX = 260   // horizontal center
const CY = 308   // arc center (slightly below canvas so gauge curves upward)
const R = 225    // outer arc radius
const R_TRK = 24 // track stroke width
const R_NDL = 212 // needle length

const canvasEl = ref(null)
const isListening = ref(false)
const errMsg = ref('')
const hasSignal = ref(false)
const nearestString = ref(null)
const devices = ref([])
const selectedDeviceId = ref('')

let audioCtx = null
let analyser = null
let micStream = null
let rafId = null
let pcmBuf = null

let smoothAngle = Math.PI * 1.5  // 270° = pointing up = in tune
let lastNote = ''
let lastCents = 0
let silenceFrames = 0

function deg(d) { return d * Math.PI / 180 }

function centsToAngle(cents) {
  return deg(270 + Math.max(-50, Math.min(50, cents)) * 1.2)
}

function freqToInfo(freq) {
  const noteNum = 12 * Math.log2(freq / 440) + 69
  const rounded = Math.round(noteNum)
  const cents = (noteNum - rounded) * 100
  const name = NOTE_NAMES[((rounded % 12) + 12) % 12]
  const octave = Math.floor(rounded / 12) - 1
  return { note: `${name}${octave}`, cents }
}

function findNearestString(freq) {
  let best = null
  let bestSemitones = Infinity
  for (const s of STRINGS) {
    const semitones = Math.abs(12 * Math.log2(freq / s.freq))
    if (semitones < bestSemitones) {
      bestSemitones = semitones
      best = s
    }
  }
  return bestSemitones < 1.0 ? best : null
}

function autoCorrelate(buf, sampleRate) {
  const SIZE = buf.length
  let rms = 0
  for (let i = 0; i < SIZE; i++) rms += buf[i] * buf[i]
  rms = Math.sqrt(rms / SIZE)
  if (rms < 0.012) return -1

  const MIN_LAG = Math.floor(sampleRate / 1200)
  const MAX_LAG = Math.min(Math.ceil(sampleRate / 60), Math.floor(SIZE / 2) - 2)

  let bestNsdf = 0
  let bestLag = -1

  for (let lag = MIN_LAG; lag <= MAX_LAG; lag++) {
    let corr = 0, normA = 0, normB = 0
    const len = SIZE - lag
    for (let i = 0; i < len; i++) {
      corr += buf[i] * buf[i + lag]
      normA += buf[i] * buf[i]
      normB += buf[i + lag] * buf[i + lag]
    }
    const norm = normA + normB
    const nsdf = norm > 0 ? 2 * corr / norm : 0
    if (nsdf > bestNsdf) {
      bestNsdf = nsdf
      bestLag = lag
    }
  }

  if (bestLag === -1 || bestNsdf < 0.78) return -1
  return sampleRate / bestLag
}

function drawFrame() {
  const ctx = canvasEl.value?.getContext('2d')
  if (!ctx) return
  ctx.clearRect(0, 0, CW, CH)

  const START = deg(210)
  const END = deg(330)

  // Background track
  ctx.beginPath()
  ctx.arc(CX, CY, R, START, END)
  ctx.strokeStyle = '#1a1d27'
  ctx.lineWidth = R_TRK + 8
  ctx.lineCap = 'butt'
  ctx.stroke()

  ctx.beginPath()
  ctx.arc(CX, CY, R, START, END)
  ctx.strokeStyle = '#0f1117'
  ctx.lineWidth = R_TRK
  ctx.stroke()

  // Colored zones
  const zones = [
    { from: -50, to: -25, color: '#ef4444' },
    { from: -25, to: -10, color: '#f59e0b' },
    { from: -10, to:  10, color: '#22c55e' },
    { from:  10, to:  25, color: '#f59e0b' },
    { from:  25, to:  50, color: '#ef4444' },
  ]
  ctx.lineWidth = R_TRK - 8
  ctx.lineCap = 'butt'
  for (const z of zones) {
    ctx.beginPath()
    ctx.arc(CX, CY, R, centsToAngle(z.from), centsToAngle(z.to))
    ctx.strokeStyle = z.color + '55'
    ctx.stroke()
  }

  // Tick marks
  for (let c = -50; c <= 50; c += 5) {
    const isMajor = c % 25 === 0
    const isMinor = c % 10 === 0
    const a = centsToAngle(c)
    const rInner = isMajor ? R - 36 : isMinor ? R - 28 : R - 20
    ctx.beginPath()
    ctx.moveTo(CX + rInner * Math.cos(a), CY + rInner * Math.sin(a))
    ctx.lineTo(CX + (R - R_TRK / 2 + 4) * Math.cos(a), CY + (R - R_TRK / 2 + 4) * Math.sin(a))
    ctx.strokeStyle = isMajor ? '#4b5563' : '#2a2d3e'
    ctx.lineWidth = isMajor ? 2 : 1
    ctx.stroke()
  }

  // Tick labels
  ctx.font = '10px JetBrains Mono, monospace'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'
  ctx.fillStyle = '#4b5563'
  for (const c of [-50, -25, 0, 25, 50]) {
    const a = centsToAngle(c)
    const lr = R + 16
    const label = c === 0 ? '0' : c > 0 ? `+${c}` : `${c}`
    ctx.fillText(label, CX + lr * Math.cos(a), CY + lr * Math.sin(a))
  }

  // Needle color
  const inTune = hasSignal.value && Math.abs(lastCents) < 8
  const nearTune = hasSignal.value && Math.abs(lastCents) < 25
  const needleColor = !hasSignal.value
    ? '#374151'
    : inTune ? '#22c55e'
    : nearTune ? '#f59e0b'
    : '#ef4444'

  if (hasSignal.value) {
    ctx.shadowColor = needleColor
    ctx.shadowBlur = 14
  }

  ctx.beginPath()
  ctx.moveTo(CX, CY)
  ctx.lineTo(CX + R_NDL * Math.cos(smoothAngle), CY + R_NDL * Math.sin(smoothAngle))
  ctx.strokeStyle = needleColor
  ctx.lineWidth = hasSignal.value ? 2.5 : 1.5
  ctx.lineCap = 'round'
  ctx.stroke()
  ctx.shadowBlur = 0

  ctx.beginPath()
  ctx.arc(CX, CY, 8, 0, 2 * Math.PI)
  ctx.fillStyle = needleColor
  if (hasSignal.value) { ctx.shadowColor = needleColor; ctx.shadowBlur = 16 }
  ctx.fill()
  ctx.shadowBlur = 0

  ctx.beginPath()
  ctx.arc(CX, CY, 3, 0, 2 * Math.PI)
  ctx.fillStyle = '#e2e8f0'
  ctx.fill()

  ctx.textAlign = 'center'
  ctx.shadowBlur = 0

  if (hasSignal.value) {
    ctx.font = 'bold 58px JetBrains Mono, monospace'
    ctx.fillStyle = '#e2e8f0'
    ctx.textBaseline = 'alphabetic'
    ctx.fillText(lastNote, CX, 162)

    const cLabel = lastCents >= 0 ? `+${Math.round(lastCents)}¢` : `${Math.round(lastCents)}¢`
    ctx.font = '16px JetBrains Mono, monospace'
    ctx.fillStyle = needleColor
    ctx.textBaseline = 'top'
    ctx.fillText(cLabel, CX, 170)
  } else {
    ctx.font = '18px JetBrains Mono, monospace'
    ctx.fillStyle = '#374151'
    ctx.textBaseline = 'middle'
    ctx.fillText(isListening.value ? 'Play a note…' : 'Press Start', CX, 148)
  }
}

function tick() {
  if (!analyser) return
  analyser.getFloatTimeDomainData(pcmBuf)
  const freq = autoCorrelate(pcmBuf, audioCtx.sampleRate)

  if (freq > 0) {
    const info = freqToInfo(freq)
    lastNote = info.note
    lastCents = info.cents
    hasSignal.value = true
    nearestString.value = findNearestString(freq)
    silenceFrames = 0
  } else {
    silenceFrames++
    if (silenceFrames > 40) {
      hasSignal.value = false
      nearestString.value = null
    }
  }

  const target = hasSignal.value ? centsToAngle(lastCents) : deg(270)
  smoothAngle = smoothAngle * 0.82 + target * 0.18

  drawFrame()
  rafId = requestAnimationFrame(tick)
}

async function enumerateDevices() {
  const all = await navigator.mediaDevices.enumerateDevices()
  const inputs = all.filter(d => d.kind === 'audioinput')
  devices.value = inputs

  // Auto-select Focusrite/Scarlett if present, otherwise keep current or pick first
  const scarlett = inputs.find(d =>
    /focusrite|scarlett/i.test(d.label)
  )
  if (scarlett) {
    selectedDeviceId.value = scarlett.deviceId
  } else if (!selectedDeviceId.value || !inputs.find(d => d.deviceId === selectedDeviceId.value)) {
    selectedDeviceId.value = inputs[0]?.deviceId ?? ''
  }
}

async function startStream(deviceId) {
  const constraints = {
    audio: deviceId ? { deviceId: { exact: deviceId } } : true,
    video: false,
  }
  micStream = await navigator.mediaDevices.getUserMedia(constraints)
  audioCtx = new AudioContext()
  analyser = audioCtx.createAnalyser()
  analyser.fftSize = 4096
  pcmBuf = new Float32Array(analyser.fftSize)
  const source = audioCtx.createMediaStreamSource(micStream)
  source.connect(analyser)
}

async function toggle() {
  if (isListening.value) {
    stop()
  } else {
    await start()
  }
}

async function start() {
  errMsg.value = ''
  try {
    // First call with no deviceId to trigger permission prompt
    await startStream(null)
    // Now labels are available — enumerate and pick best device
    await enumerateDevices()
    // If a better device was found, restart with it
    if (selectedDeviceId.value) {
      teardownStream()
      await startStream(selectedDeviceId.value)
    }
    isListening.value = true
    rafId = requestAnimationFrame(tick)
  } catch (e) {
    errMsg.value = 'Microphone access denied. Please allow microphone permissions and try again.'
  }
}

function teardownStream() {
  if (rafId) { cancelAnimationFrame(rafId); rafId = null }
  if (micStream) { micStream.getTracks().forEach(t => t.stop()); micStream = null }
  if (audioCtx) { audioCtx.close(); audioCtx = null }
  analyser = null
}

function stop() {
  teardownStream()
  isListening.value = false
  hasSignal.value = false
  nearestString.value = null
  silenceFrames = 0
  smoothAngle = deg(270)
  drawFrame()
}

async function onDeviceChange(e) {
  selectedDeviceId.value = e.target.value
  if (isListening.value) {
    // Restart stream on the new device without losing the listening state
    teardownStream()
    try {
      await startStream(selectedDeviceId.value)
      rafId = requestAnimationFrame(tick)
    } catch {
      errMsg.value = 'Failed to switch to selected device.'
      isListening.value = false
    }
  }
}

onUnmounted(stop)
</script>

<style scoped>
.tuner-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem 1rem;
  background: var(--bg);
}

.tuner-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 2.5rem 2rem 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.25rem;
  max-width: 580px;
  width: 100%;
}

.label-chip {
  font-family: var(--mono);
  font-size: 0.7rem;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--accent);
  border: 1px solid var(--accent);
  padding: 0.2rem 0.7rem;
  border-radius: 999px;
  margin: 0;
}

h1 {
  font-family: var(--mono);
  font-size: 1.8rem;
  font-weight: 700;
  color: var(--text);
  margin: 0;
}

.sub {
  font-family: var(--mono);
  font-size: 0.85rem;
  color: var(--text-muted);
  margin: -0.5rem 0 0;
}

.tuner-canvas {
  width: 100%;
  max-width: 520px;
  height: auto;
  display: block;
  border-radius: 10px;
}

.strings-row {
  display: flex;
  gap: 0.6rem;
  flex-wrap: wrap;
  justify-content: center;
}

.s-chip {
  font-family: var(--mono);
  font-size: 0.8rem;
  font-weight: 600;
  padding: 0.35rem 0.75rem;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-muted);
  transition: all 0.15s ease;
  cursor: default;
  user-select: none;
}

.s-chip.is-active {
  border-color: #22c55e;
  color: #22c55e;
  background: #22c55e18;
  box-shadow: 0 0 10px #22c55e44;
}

/* Device selector */
.device-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
}

.device-label {
  font-family: var(--mono);
  font-size: 0.75rem;
  color: var(--text-muted);
  white-space: nowrap;
  flex-shrink: 0;
}

.device-select {
  flex: 1;
  font-family: var(--mono);
  font-size: 0.8rem;
  color: var(--text);
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 0.45rem 0.75rem;
  outline: none;
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%238892a4' d='M6 8L1 3h10z'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  padding-right: 2rem;
  transition: border-color 0.15s;
}

.device-select:hover:not(:disabled) {
  border-color: var(--accent);
}

.device-select:focus {
  border-color: var(--accent);
}

.device-select:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Buttons */
.start-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-family: var(--mono);
  font-size: 0.9rem;
  font-weight: 600;
  padding: 0.65rem 1.6rem;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text);
  cursor: pointer;
  transition: all 0.15s ease;
}

.start-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: #7c6af718;
}

.start-btn.is-on {
  border-color: #ef4444;
  color: #ef4444;
  background: #ef444418;
}

.start-btn.is-on:hover {
  background: #ef444430;
}

.btn-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-muted);
  transition: background 0.15s ease;
  flex-shrink: 0;
}

.btn-dot.is-on {
  background: #ef4444;
  box-shadow: 0 0 6px #ef4444;
  animation: pulse 1.2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.err-msg {
  font-family: var(--mono);
  font-size: 0.8rem;
  color: #ef4444;
  text-align: center;
  margin: 0;
}
</style>
