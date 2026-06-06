<template>
  <div class="recordings-wrapper">
    <div class="recordings-card">
      <p class="label">Audio</p>
      <h1>Recordings</h1>
      <p class="subtitle">Record your guitar and save as FLAC, stored in your account.</p>

      <!-- Capture controls -->
      <div class="section">
        <h2>Capture</h2>

        <div class="field">
          <label class="field-label">Input Device</label>
          <div class="row-group">
            <select v-model="selectedDevice" :disabled="recording" class="field-select">
              <option v-for="d in devices" :key="d.deviceId" :value="d.deviceId">{{ d.label }}</option>
            </select>
            <button class="btn-ghost" :disabled="recording" @click="enumerate">Refresh</button>
          </div>
        </div>

        <div class="field">
          <label class="field-label">Channels</label>
          <select v-model="channelMode" :disabled="recording" class="field-select">
            <option value="mono1">Mono — Input 1 (instrument)</option>
            <option value="mono2">Mono — Input 2 (XLR mic)</option>
            <option value="stereo">Stereo — both inputs</option>
          </select>
        </div>

        <div class="btn-row">
          <button class="btn-record" :disabled="!flacReady || recording" @click="startRecording">● Record</button>
          <button class="btn-stop" :disabled="!recording" @click="stopRecording">■ Stop</button>
        </div>

        <div class="field">
          <label class="field-label">Input Level</label>
          <progress :value="inputLevel" max="1" class="level-bar"></progress>
        </div>

        <p class="status-msg">{{ status }}</p>
      </div>

      <!-- Effects chain -->
      <div class="section section-border">
        <h2>Effects</h2>
        <FxDistortion />
        <FxEq />
        <FxCompressor />
        <FxDelay />
        <FxReverb />
        <FxTremolo />
        <FxChorus />
        <FxWah />
      </div>

      <!-- Pending (recorded locally, not yet uploaded) -->
      <div v-if="pending.length > 0" class="section section-border">
        <h2>New Recordings</h2>
        <div v-for="(p, i) in pending" :key="i" class="rec-row">
          <div class="rec-meta">
            <span class="rec-name">{{ p.name }}</span>
            <span class="muted">{{ formatBytes(p.blob.size) }}</span>
          </div>
          <div class="rec-acts">
            <a :href="p.url" :download="p.name" class="btn-ghost small">⬇ Save</a>
            <button
              class="btn-primary small"
              :disabled="p.uploading || p.uploaded"
              @click="uploadPending(i)"
            >
              {{ p.uploading ? 'Uploading…' : p.uploaded ? '✓ Saved' : 'Upload' }}
            </button>
          </div>
          <p v-if="p.error" class="field-error">{{ p.error }}</p>
        </div>
      </div>

      <!-- Saved recordings from server -->
      <div v-if="loadError" class="section section-border">
        <p class="state-msg error">{{ loadError }}</p>
      </div>
      <div v-else-if="saved.length > 0" class="section section-border">
        <h2>Your Recordings</h2>
        <div v-for="rec in saved" :key="rec.id" class="rec-row">
          <div class="rec-meta">
            <span class="rec-name">{{ rec.filename }}</span>
            <span class="muted">{{ formatBytes(rec.size_bytes) }} · {{ formatDate(rec.created_at) }}</span>
          </div>
          <div class="custom-player">
            <button
              class="play-btn"
              :disabled="getPlayer(rec.id).loading"
              @click="togglePlay(rec.id)"
            >
              <span v-if="getPlayer(rec.id).loading" class="play-icon">⋯</span>
              <span v-else-if="getPlayer(rec.id).playing" class="play-icon">⏸</span>
              <span v-else class="play-icon">▶</span>
            </button>
            <span class="time-label">{{ formatTime(getPlayer(rec.id).currentTime) }}</span>
            <div class="seek-wrap">
              <input
                type="range"
                class="seek-slider"
                :min="0"
                :max="getPlayer(rec.id).duration || 0"
                :value="getPlayer(rec.id).currentTime"
                step="0.01"
                :disabled="!getPlayer(rec.id).loaded"
                :style="`--pct: ${getPlayer(rec.id).duration ? (getPlayer(rec.id).currentTime / getPlayer(rec.id).duration * 100).toFixed(2) : 0}%`"
                @input="handleSeekInput(rec.id, +($event.target as HTMLInputElement).value)"
                @change="handleSeekChange(rec.id, +($event.target as HTMLInputElement).value)"
              />
            </div>
            <span class="time-label">{{ formatTime(getPlayer(rec.id).duration) }}</span>
          </div>
          <div class="rec-acts">
            <button class="btn-ghost small" @click="downloadRecording(rec.id, rec.filename)">⬇ Download</button>
            <button class="btn-danger small" @click="deleteRecording(rec.id)">Delete</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, provide, onMounted, onUnmounted } from 'vue'
import { apiFetch } from '../../api'
import { showSnackBar } from '../../components/subjects/snack-bar-subject'
import FxDistortion from './components/FxDistortion.vue'
import FxEq         from './components/FxEq.vue'
import FxCompressor from './components/FxCompressor.vue'
import FxDelay      from './components/FxDelay.vue'
import FxReverb     from './components/FxReverb.vue'
import FxTremolo    from './components/FxTremolo.vue'
import FxChorus     from './components/FxChorus.vue'
import FxWah        from './components/FxWah.vue'
import { useDistortionFx, distortionFxKey } from './composables/useDistortionFx'
import { useEqFx,         eqFxKey         } from './composables/useEqFx'
import { useCompressorFx, compressorFxKey } from './composables/useCompressorFx'
import { useDelayFx,      delayFxKey      } from './composables/useDelayFx'
import { useReverbFx,     reverbFxKey     } from './composables/useReverbFx'
import { useTremoloFx,    tremoloFxKey    } from './composables/useTremoloFx'
import { useChorusFx,     chorusFxKey     } from './composables/useChorusFx'
import { useWahFx,        wahFxKey        } from './composables/useWahFx'

// ---------- Reactive state ----------

const devices = ref<{ deviceId: string; label: string }[]>([])
const selectedDevice = ref('')
const channelMode = ref<'mono1' | 'mono2' | 'stereo'>('mono1')
const flacReady = ref(false)
const recording = ref(false)
const inputLevel = ref(0)
const status = ref('Loading FLAC encoder…')

interface PendingRecording {
  name: string
  blob: Blob
  url: string
  uploading: boolean
  uploaded: boolean
  error: string
}

interface SavedRecording {
  id: string
  filename: string
  size_bytes: number
  created_at: string
}

const pending = ref<PendingRecording[]>([])
const saved = ref<SavedRecording[]>([])
const loadError = ref('')

// ---------- Effects composables ----------

const distortionFx  = useDistortionFx()
const eqFx          = useEqFx()
const compressorFx  = useCompressorFx()
const delayFx       = useDelayFx()
const reverbFx      = useReverbFx()
const tremoloFx     = useTremoloFx()
const chorusFx      = useChorusFx()
const wahFx         = useWahFx()

provide(distortionFxKey,  distortionFx)
provide(eqFxKey,          eqFx)
provide(compressorFxKey,  compressorFx)
provide(delayFxKey,       delayFx)
provide(reverbFxKey,      reverbFx)
provide(tremoloFxKey,     tremoloFx)
provide(chorusFxKey,      chorusFx)
provide(wahFxKey,         wahFx)

// ---------- Non-reactive audio state (changes on every audio frame) ----------

let audioCtx: AudioContext | null = null
let mediaStream: MediaStream | null = null
let workletNode: AudioWorkletNode | null = null
let sourceNode: MediaStreamAudioSourceNode | null = null

let encoder = 0
let flacChunks: Uint8Array[] = []
let captureMode: 'mono1' | 'mono2' | 'stereo' = 'mono1'
let channels = 0
let sampleRate = 0

// ---------- AudioWorklet (defined inline, loaded via blob URL) ----------

const WORKLET_CODE = `
class CaptureProcessor extends AudioWorkletProcessor {
  process(inputs) {
    const input = inputs[0];
    if (input && input.length > 0 && input[0].length > 0) {
      const block = input.map(ch => Float32Array.from(ch));
      this.port.postMessage(block);
    }
    return true;
  }
}
registerProcessor('capture-processor', CaptureProcessor);
`
const workletUrl = URL.createObjectURL(
  new Blob([WORKLET_CODE], { type: 'application/javascript' })
)

// ---------- FLAC readiness ----------

function onFlacReady() {
  flacReady.value = true
  status.value = 'Ready. Select a device and press Record.'
  enumerate()
}

// ---------- Device enumeration ----------

async function enumerate() {
  try {
    const tmp = await navigator.mediaDevices.getUserMedia({ audio: true })
    tmp.getTracks().forEach(t => t.stop())
  } catch (e: any) {
    status.value = 'Microphone permission denied: ' + e.message
    return
  }
  const all = await navigator.mediaDevices.enumerateDevices()
  devices.value = all
    .filter(d => d.kind === 'audioinput')
    .map(d => ({
      deviceId: d.deviceId,
      label: d.label || 'Input ' + d.deviceId.slice(0, 6),
    }))
  const scarlett = devices.value.find(d => /scarlett|focusrite/i.test(d.label))
  if (scarlett) selectedDevice.value = scarlett.deviceId
  else if (devices.value.length) selectedDevice.value = devices.value[0].deviceId
}

// ---------- Recording ----------

async function startRecording() {
  if (!flacReady.value || recording.value) return
  flacChunks = []
  encoder = 0
  channels = 0
  captureMode = channelMode.value

  try {
    mediaStream = await navigator.mediaDevices.getUserMedia({
      audio: {
        deviceId: selectedDevice.value ? { exact: selectedDevice.value } : undefined,
        channelCount: 2,
        echoCancellation: false,
        autoGainControl: false,
        noiseSuppression: false,
      },
    })
  } catch (e: any) {
    status.value = 'Could not open device: ' + e.message
    return
  }

  audioCtx = new AudioContext()
  sampleRate = audioCtx.sampleRate
  await audioCtx.audioWorklet.addModule(workletUrl)

  sourceNode = audioCtx.createMediaStreamSource(mediaStream)
  workletNode = new AudioWorkletNode(audioCtx, 'capture-processor')
  workletNode.port.onmessage = (e) => handleAudioBlock(e.data)

  let prev: AudioNode = sourceNode
  prev = distortionFx.connect(audioCtx, prev)
  prev = eqFx.connect(audioCtx, prev)
  prev = compressorFx.connect(audioCtx, prev)
  prev = tremoloFx.connect(audioCtx, prev)
  prev = chorusFx.connect(audioCtx, prev)
  prev = delayFx.connect(audioCtx, prev)
  prev = reverbFx.connect(audioCtx, prev)
  prev = wahFx.connect(audioCtx, prev)
  prev.connect(workletNode)
  workletNode.connect(audioCtx.destination)

  recording.value = true
  status.value = `Recording… (${sampleRate} Hz, ${captureMode})`
}

function initEncoder(numChannels: number) {
  const Flac = (window as any).Flac
  channels = numChannels
  encoder = Flac.create_libflac_encoder(sampleRate, channels, 16, 5, 0)
  if (encoder === 0) throw new Error('Failed to create FLAC encoder')
  const initStatus = Flac.init_encoder_stream(encoder, (buf: Uint8Array) => {
    flacChunks.push(buf.slice())
  })
  if (initStatus !== 0) throw new Error('init_encoder_stream failed: ' + initStatus)
}

function handleAudioBlock(block: Float32Array[]) {
  if (!recording.value) return
  const Flac = (window as any).Flac
  const frames = block[0].length

  let srcChannels: number[]
  if (captureMode === 'stereo') srcChannels = block.map((_, i) => i)
  else if (captureMode === 'mono2' && block.length > 1) srcChannels = [1]
  else srcChannels = [0]

  if (encoder === 0) {
    try { initEncoder(srcChannels.length) }
    catch (e: any) { status.value = e.message; stopRecording(); return }
  }

  const interleaved = new Int32Array(frames * channels)
  let peak = 0
  for (let f = 0; f < frames; f++) {
    for (let c = 0; c < channels; c++) {
      let s = block[srcChannels[c]][f]
      if (s > peak) peak = s; else if (-s > peak) peak = -s
      s = Math.max(-1, Math.min(1, s))
      interleaved[f * channels + c] = (s * 32767) | 0
    }
  }
  inputLevel.value = peak
  Flac.FLAC__stream_encoder_process_interleaved(encoder, interleaved, frames)
}

function stopRecording() {
  if (!recording.value) return
  recording.value = false

  distortionFx.disconnect()
  eqFx.disconnect()
  compressorFx.disconnect()
  tremoloFx.disconnect()
  chorusFx.disconnect()
  delayFx.disconnect()
  reverbFx.disconnect()
  wahFx.disconnect()
  try { sourceNode?.disconnect() } catch {}
  try { workletNode?.disconnect() } catch {}
  mediaStream?.getTracks().forEach(t => t.stop())
  audioCtx?.close()
  inputLevel.value = 0

  const Flac = (window as any).Flac
  if (encoder !== 0) {
    Flac.FLAC__stream_encoder_finish(encoder)
    Flac.FLAC__stream_encoder_delete(encoder)
    encoder = 0
    buildPending()
  } else {
    status.value = 'No audio captured.'
  }
}

function buildPending() {
  const blob = new Blob(flacChunks as BlobPart[], { type: 'audio/flac' })
  const url = URL.createObjectURL(blob)
  const stamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19)
  const name = `recording-${stamp}.flac`
  pending.value.unshift({ name, blob, url, uploading: false, uploaded: false, error: '' })
  status.value = `${channels} ch @ ${sampleRate} Hz, 16-bit — ${formatBytes(blob.size)}`
}

// ---------- Upload ----------

async function uploadPending(index: number) {
  const p = pending.value[index]
  p.uploading = true
  p.error = ''
  try {
    const res = await apiFetch('/api/recordings/upload', {
      method: 'POST',
      headers: { 'Content-Type': 'audio/flac', 'X-Filename': p.name },
      body: p.blob,
    })
    if (!res.ok) { p.error = `Upload failed (${res.status})`; return }
    p.uploaded = true
    showSnackBar(`Saved: ${p.name}`)
    await loadSaved()
  } catch (e: any) {
    p.error = 'Upload error: ' + e.message
  } finally {
    p.uploading = false
  }
}

// ---------- Delete ----------

async function deleteRecording(id: string) {
  try {
    const res = await apiFetch(`/api/recordings/${id}`, { method: 'DELETE' })
    if (!res.ok) { showSnackBar('Failed to delete recording.', 'error'); return }
    saved.value = saved.value.filter(r => r.id !== id)
    showSnackBar('Recording deleted.')
  } catch {
    showSnackBar('Network error.', 'error')
  }
}

// ---------- Download ----------

async function downloadRecording(id: string, filename: string) {
  const p = getPlayer(id)
  if (p.loaded && p.blobUrl) {
    triggerDownload(p.blobUrl, filename)
    return
  }
  try {
    const res = await apiFetch(`/api/recordings/${id}`)
    if (!res.ok) { showSnackBar('Failed to download recording.', 'error'); return }
    const blob = await res.blob()
    const url = URL.createObjectURL(blob)
    triggerDownload(url, filename)
    URL.revokeObjectURL(url)
  } catch {
    showSnackBar('Network error.', 'error')
  }
}

function triggerDownload(url: string, filename: string) {
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
}

// ---------- Load saved recordings ----------

async function loadSaved() {
  try {
    const res = await apiFetch('/api/recordings')
    if (!res.ok) { loadError.value = 'Failed to load recordings.'; return }
    saved.value = await res.json()
  } catch {
    loadError.value = 'Network error loading recordings.'
  }
}

// ---------- Custom audio player ----------

interface PlayerState {
  audio: HTMLAudioElement | null
  blobUrl: string | null
  loaded: boolean
  loading: boolean
  playing: boolean
  currentTime: number
  duration: number
  seeking: boolean
}

const players = ref<Record<string, PlayerState>>({})

function getPlayer(id: string): PlayerState {
  if (!players.value[id]) {
    players.value[id] = {
      audio: null,
      blobUrl: null,
      loaded: false,
      loading: false,
      playing: false,
      currentTime: 0,
      duration: 0,
      seeking: false,
    }
  }
  return players.value[id]
}

async function togglePlay(id: string) {
  const p = getPlayer(id)

  if (!p.loaded && !p.loading) {
    p.loading = true
    try {
      const res = await apiFetch(`/api/recordings/${id}`)
      if (!res.ok) { showSnackBar('Failed to load recording.', 'error'); return }
      const blob = await res.blob()
      const url = URL.createObjectURL(blob)
      const audio = new Audio(url)
      audio.preload = 'auto'
      audio.addEventListener('timeupdate', () => {
        if (!p.seeking) p.currentTime = audio.currentTime
      })
      audio.addEventListener('durationchange', () => {
        if (isFinite(audio.duration)) p.duration = audio.duration
      })
      audio.addEventListener('ended', () => { p.playing = false; p.currentTime = 0 })
      p.blobUrl = url
      p.audio = audio
      p.loaded = true
    } catch {
      showSnackBar('Network error loading recording.', 'error')
      return
    } finally {
      p.loading = false
    }
  }

  if (!p.audio) return

  if (p.playing) {
    p.audio.pause()
    p.playing = false
  } else {
    // Pause any other playing recordings
    for (const [otherId, other] of Object.entries(players.value)) {
      if (otherId !== id && other.playing && other.audio) {
        other.audio.pause()
        other.playing = false
      }
    }
    await p.audio.play()
    p.playing = true
  }
}

function handleSeekInput(id: string, value: number) {
  const p = getPlayer(id)
  p.seeking = true
  p.currentTime = value
}

function handleSeekChange(id: string, value: number) {
  const p = getPlayer(id)
  if (p.audio) p.audio.currentTime = value
  p.seeking = false
}

function formatTime(sec: number): string {
  if (!sec || isNaN(sec)) return '0:00'
  const m = Math.floor(sec / 60)
  const s = Math.floor(sec % 60).toString().padStart(2, '0')
  return `${m}:${s}`
}

// ---------- Helpers ----------

function formatBytes(n: number): string {
  if (n < 1024) return `${n} B`
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`
  return `${(n / 1024 / 1024).toFixed(2)} MB`
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleString()
}

// ---------- Lifecycle ----------

onMounted(async () => {
  // Set up locateFile BEFORE injecting the script so the WASM binary resolves.
  ;(window as any).Flac = {
    locateFile: (path: string) =>
      `https://cdn.jsdelivr.net/npm/libflacjs@5.4.0/dist/${path}`,
  }
  const script = document.createElement('script')
  script.src = 'https://cdn.jsdelivr.net/npm/libflacjs@5.4.0/dist/libflac.min.wasm.js'
  script.onload = () => {
    const Flac = (window as any).Flac
    if (Flac.isReady && Flac.isReady()) onFlacReady()
    else Flac.on('ready', onFlacReady)
  }
  document.head.appendChild(script)

  await loadSaved()
})

onUnmounted(() => {
  if (recording.value) stopRecording()
  URL.revokeObjectURL(workletUrl)
  for (const p of Object.values(players.value)) {
    p.audio?.pause()
    if (p.blobUrl) URL.revokeObjectURL(p.blobUrl)
  }
})
</script>

<style scoped>
.recordings-wrapper {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  background: var(--bg);
  padding: 2rem;
}

.recordings-card {
  width: 100%;
  max-width: 640px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 2.5rem;
  align-self: flex-start;
}

.label {
  font-family: var(--mono);
  color: var(--accent);
  font-size: 0.9rem;
  margin-bottom: 0.5rem;
}

h1 {
  font-size: 1.75rem;
  font-weight: 700;
  margin-bottom: 0.4rem;
}

.subtitle {
  font-size: 0.9rem;
  color: var(--text-muted);
  margin-bottom: 2rem;
}

h2 {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  font-family: var(--mono);
  margin-bottom: 1.25rem;
}

.section {
  display: flex;
  flex-direction: column;
}

.section-border {
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 1px solid var(--border);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
  margin-bottom: 1rem;
}

/* .field-label lives in main.css */

.field-select {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 0.6rem 0.875rem;
  color: var(--text);
  font-size: 0.95rem;
  outline: none;
  transition: border-color 0.15s;
  flex: 1;
}

.field-select:focus {
  border-color: var(--accent);
}

.field-select:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.row-group {
  display: flex;
  gap: 0.5rem;
}

.btn-row {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.btn-record {
  background: #c0392b;
  color: #fff;
  border: none;
  border-radius: 8px;
  padding: 0.55rem 1.1rem;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s;
  font-family: inherit;
}

.btn-record:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.btn-stop {
  background: var(--bg);
  color: var(--text);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 0.55rem 1.1rem;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s, border-color 0.15s;
  font-family: inherit;
}

.btn-stop:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.level-bar {
  width: 100%;
  height: 10px;
  border-radius: 5px;
  accent-color: var(--accent);
}

.status-msg {
  font-size: 0.85rem;
  color: var(--text-muted);
  font-family: var(--mono);
  min-height: 1.4rem;
}

.rec-row {
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.rec-row:last-child {
  border-bottom: none;
}

.rec-meta {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.rec-name {
  font-family: var(--mono);
  font-size: 0.9rem;
  font-weight: 500;
  word-break: break-all;
}

/* .muted lives in main.css */

.rec-acts {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.custom-player {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  width: 100%;
  margin-top: 0.25rem;
}

.play-btn {
  flex-shrink: 0;
  background: none;
  border: none;
  color: var(--text);
  cursor: pointer;
  padding: 0;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: color 0.15s;
}

.play-btn:hover:not(:disabled) { color: var(--accent); }
.play-btn:disabled { opacity: 0.4; cursor: not-allowed; }

.play-icon {
  font-size: 1rem;
  line-height: 1;
  pointer-events: none;
}

.time-label {
  flex-shrink: 0;
  font-family: var(--mono);
  font-size: 0.75rem;
  color: var(--text-muted);
  min-width: 2.8rem;
  text-align: center;
}

.seek-wrap {
  flex: 1;
  display: flex;
  align-items: center;
  height: 20px;
}

.seek-slider {
  --pct: 0%;
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 4px;
  border-radius: 4px;
  background: linear-gradient(
    to right,
    var(--accent) var(--pct),
    var(--border) var(--pct)
  );
  outline: none;
  cursor: pointer;
  transition: height 0.12s;
}

.seek-slider:hover { height: 6px; }

.seek-slider:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}

.seek-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 0;
  height: 0;
  background: #fff;
  border-radius: 50%;
  transition: width 0.12s, height 0.12s;
}

.seek-slider:hover::-webkit-slider-thumb {
  width: 14px;
  height: 14px;
}

.seek-slider::-moz-range-thumb {
  width: 0;
  height: 0;
  background: #fff;
  border: none;
  border-radius: 50%;
  transition: width 0.12s, height 0.12s;
}

.seek-slider:hover::-moz-range-thumb {
  width: 14px;
  height: 14px;
}

.btn-primary {
  background: var(--accent);
  color: #fff;
  border: none;
  border-radius: 8px;
  padding: 0.55rem 1.1rem;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s, opacity 0.15s;
  font-family: inherit;
  text-decoration: none;
  display: inline-block;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-primary:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.btn-ghost {
  background: none;
  border: 1px solid var(--border);
  color: var(--text-muted);
  border-radius: 8px;
  padding: 0.55rem 1.1rem;
  font-size: 0.9rem;
  cursor: pointer;
  transition: border-color 0.15s, color 0.15s;
  font-family: inherit;
  text-decoration: none;
  display: inline-block;
}

.btn-ghost:hover:not(:disabled) {
  border-color: var(--text-muted);
  color: var(--text);
}

.btn-ghost:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.btn-primary.small,
.btn-ghost.small {
  padding: 0.35rem 0.75rem;
  font-size: 0.8rem;
}

.btn-danger {
  background: none;
  border: 1px solid #f87171;
  color: #f87171;
  border-radius: 8px;
  padding: 0.55rem 1.1rem;
  font-size: 0.9rem;
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
  font-family: inherit;
}

.btn-danger:hover {
  background: #f87171;
  color: #fff;
}

.btn-danger.small {
  padding: 0.35rem 0.75rem;
  font-size: 0.8rem;
}

.field-error {
  font-size: 0.8rem;
  color: #f87171;
}

.state-msg {
  color: var(--text-muted);
  font-size: 0.9rem;
  padding: 1rem 0;
}

.state-msg.error {
  color: #f87171;
}

/* toggle-switch, fx-* live in EffectBlock.vue */
</style>
