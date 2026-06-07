<template>
  <div class="recordings-wrapper">
    <div class="rec-layout">

      <!-- Header -->
      <header class="rec-header">
        <p class="rec-eyebrow"><span class="eyebrow-dot"></span>Audio</p>
        <h1>Recordings</h1>
        <p class="rec-lede">Record your guitar and save as FLAC, stored in your account.</p>
      </header>

      <!-- Two-column grid -->
      <div class="rec-grid">

        <!-- Left: Capture -->
        <section class="rec-panel capture-panel">
          <p class="section-label">Capture</p>

          <div class="rec-field">
            <label>Input device</label>
            <div class="row-g">
              <div class="select-wrap">
                <select v-model="selectedDevice" :disabled="recording">
                  <option v-for="d in devices" :key="d.deviceId" :value="d.deviceId">{{ d.label }}</option>
                </select>
              </div>
              <button class="rec-btn" :disabled="recording" @click="enumerate">Refresh</button>
            </div>
          </div>

          <div class="rec-field">
            <label>Channels</label>
            <div class="select-wrap">
              <select v-model="channelMode" :disabled="recording">
                <option value="mono1">Mono — Input 1 (instrument)</option>
                <option value="mono2">Mono — Input 2 (XLR mic)</option>
                <option value="stereo">Stereo — both inputs</option>
              </select>
            </div>
          </div>

          <div class="transport">
            <button class="btn-rec" :class="{ live: recording }" :disabled="!flacReady" @click="startRecording">
              <span class="recdot"></span>{{ recording ? 'Recording…' : 'Record' }}
            </button>
            <button class="rec-btn btn-stop" :disabled="!recording" @click="stopRecording">■ Stop</button>
          </div>

          <div class="meter-section">
            <div class="meter-head">
              <span>Input level</span>
              <span class="mono-txt">{{ recording ? Math.round(inputLevel * 100) + ' / 100' : '—' }}</span>
            </div>
            <div class="meter">
              <div v-for="(seg, i) in meterSegs" :key="i" class="meter-seg"
                   :class="{ 'meter-seg--on': seg.on }"
                   :style="{ '--segc': seg.color }"></div>
            </div>
            <div class="meter-scale">
              <span>-∞</span><span>-24</span><span>-12</span><span>-6</span><span>0 dB</span>
            </div>
          </div>

          <p class="rec-status mono-txt" :class="{ 'rec-status--live': recording }">{{ status }}</p>
        </section>

        <!-- Right: Effects -->
        <section class="effects-section">
          <div class="effects-head">
            <div class="effects-head-left">
              <p class="section-label">Effects</p>
              <span class="count-chip">{{ activeCount }} / 8 active</span>
            </div>
            <button class="link-btn" @click="toggleAll">{{ allOn ? 'Bypass all' : 'Enable all' }}</button>
          </div>
          <div class="fx-grid">
            <FxDistortion />
            <FxEq />
            <FxCompressor />
            <FxDelay />
            <FxReverb />
            <FxTremolo />
            <FxChorus />
            <FxWah />
          </div>
        </section>

      </div>

      <!-- FX Profiles -->
      <section class="rec-panel panel-below">
        <p class="section-label">FX Profiles</p>
        <div class="rec-field">
          <div class="row-g">
            <input
              v-model="profileName"
              class="rec-input"
              placeholder="Profile name…"
              maxlength="64"
              @keydown.enter="saveProfile"
            />
            <button class="rec-btn rec-btn-primary" :disabled="!profileName.trim() || savingProfile" @click="saveProfile">
              {{ savingProfile ? 'Saving…' : 'Save' }}
            </button>
          </div>
        </div>
        <div v-if="profiles.length === 0" class="empty-msg">No saved profiles yet.</div>
        <div v-for="profile in profiles" :key="profile.id" class="rec-row">
          <div class="rec-meta">
            <span class="rec-name">{{ profile.name }}</span>
            <span class="muted-txt">{{ formatDate(profile.created_at) }}</span>
          </div>
          <div class="rec-acts">
            <button class="rec-btn rec-btn-sm" @click="loadProfile(profile)">Load</button>
            <button class="rec-btn rec-btn-danger rec-btn-sm" @click="deleteProfile(profile.id)">Delete</button>
          </div>
        </div>
      </section>

      <!-- Pending (locally recorded, not yet uploaded) -->
      <section v-if="pending.length > 0" class="rec-panel panel-below">
        <p class="section-label">New Recordings</p>
        <div v-for="(p, i) in pending" :key="i" class="rec-row">
          <div class="rec-meta">
            <span class="rec-name">{{ p.name }}</span>
            <span class="muted-txt">{{ formatBytes(p.blob.size) }}</span>
          </div>
          <div class="rec-acts">
            <a :href="p.url" :download="p.name" class="rec-btn rec-btn-sm">⬇ Save</a>
            <button
              class="rec-btn rec-btn-primary rec-btn-sm"
              :disabled="p.uploading || p.uploaded"
              @click="uploadPending(i)"
            >{{ p.uploading ? 'Uploading…' : p.uploaded ? '✓ Saved' : 'Upload' }}</button>
          </div>
          <p v-if="p.error" class="field-error">{{ p.error }}</p>
        </div>
      </section>

      <!-- Saved recordings -->
      <section v-if="loadError" class="rec-panel panel-below">
        <p class="empty-msg error">{{ loadError }}</p>
      </section>
      <section v-else-if="saved.length > 0" class="rec-panel panel-below">
        <p class="section-label">Your Recordings</p>
        <div v-for="rec in saved" :key="rec.id" class="rec-row">
          <div class="rec-meta">
            <span class="rec-name">{{ rec.filename }}</span>
            <span class="muted-txt">{{ formatBytes(rec.size_bytes) }} · {{ formatDate(rec.created_at) }}</span>
          </div>
          <div class="custom-player">
            <button class="play-btn" :disabled="getPlayer(rec.id).loading" @click="togglePlay(rec.id)">
              <span v-if="getPlayer(rec.id).loading" class="play-icon">⋯</span>
              <span v-else-if="getPlayer(rec.id).playing" class="play-icon">⏸</span>
              <span v-else class="play-icon">▶</span>
            </button>
            <span class="time-label mono-txt">{{ formatTime(getPlayer(rec.id).currentTime) }}</span>
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
            <span class="time-label mono-txt">{{ formatTime(getPlayer(rec.id).duration) }}</span>
          </div>
          <div class="rec-acts">
            <button class="rec-btn rec-btn-sm" @click="downloadRecording(rec.id, rec.filename)">⬇ Download</button>
            <button class="rec-btn rec-btn-danger rec-btn-sm" @click="deleteRecording(rec.id)">Delete</button>
          </div>
        </div>
      </section>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, provide, computed, onMounted, onUnmounted } from 'vue'
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

// ---------- Active effects + toggle all ----------

const allFx = [distortionFx, eqFx, compressorFx, delayFx, reverbFx, tremoloFx, chorusFx, wahFx]
const activeCount = computed(() => allFx.filter(fx => fx.state.enabled).length)
const allOn = computed(() => allFx.every(fx => fx.state.enabled))

function toggleAll() {
  const val = !allOn.value
  allFx.forEach(fx => { fx.state.enabled = val })
}

// ---------- Level meter segments ----------

const SEGS = 24
const meterSegs = computed(() => {
  const active = Math.round(inputLevel.value * SEGS)
  return Array.from({ length: SEGS }, (_, i) => ({
    on: i < active,
    color: i >= 21 ? '#f87171' : i >= 16 ? '#facc15' : '#4ade80',
  }))
})

// ---------- Non-reactive audio state ----------

let audioCtx: AudioContext | null = null
let mediaStream: MediaStream | null = null
let workletNode: AudioWorkletNode | null = null
let sourceNode: MediaStreamAudioSourceNode | null = null

let encoder = 0
let flacChunks: Uint8Array[] = []
let captureMode: 'mono1' | 'mono2' | 'stereo' = 'mono1'
let channels = 0
let sampleRate = 0

// ---------- AudioWorklet ----------

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

  audioCtx = new AudioContext({sampleRate: 48000, latencyHint: 0.005})
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

  recording.value = true
  const baseLat = (audioCtx.baseLatency * 1000).toFixed(1)
  const outLat  = (audioCtx.outputLatency * 1000).toFixed(1)
  status.value = `Recording… (${sampleRate} Hz, ${captureMode}) — base ${baseLat} ms / out ${outLat} ms`
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
  if (p.loaded && p.blobUrl) { triggerDownload(p.blobUrl, filename); return }
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

// ---------- FX Profiles ----------

interface FxProfileData {
  id: string
  name: string
  settings: string
  created_at: string
}

const profiles = ref<FxProfileData[]>([])
const profileName = ref('')
const savingProfile = ref(false)

function snapshotFx() {
  return JSON.stringify({
    distortion: { ...distortionFx.state },
    eq: { ...eqFx.state },
    compressor: { ...compressorFx.state },
    delay: { ...delayFx.state },
    reverb: { ...reverbFx.state },
    tremolo: { ...tremoloFx.state },
    chorus: { ...chorusFx.state },
    wah: { ...wahFx.state },
  })
}

function applySnapshot(settings: string) {
  try {
    const s = JSON.parse(settings)
    if (s.distortion) Object.assign(distortionFx.state, s.distortion)
    if (s.eq)         Object.assign(eqFx.state, s.eq)
    if (s.compressor) Object.assign(compressorFx.state, s.compressor)
    if (s.delay)      Object.assign(delayFx.state, s.delay)
    if (s.reverb)     Object.assign(reverbFx.state, s.reverb)
    if (s.tremolo)    Object.assign(tremoloFx.state, s.tremolo)
    if (s.chorus)     Object.assign(chorusFx.state, s.chorus)
    if (s.wah)        Object.assign(wahFx.state, s.wah)
  } catch {
    showSnackBar('Invalid profile data.', 'error')
  }
}

async function loadProfiles() {
  try {
    const res = await apiFetch('/api/fx-profiles')
    if (!res.ok) return
    profiles.value = await res.json()
  } catch {}
}

async function saveProfile() {
  const name = profileName.value.trim()
  if (!name || savingProfile.value) return
  savingProfile.value = true
  try {
    const res = await apiFetch('/api/fx-profiles', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, settings: snapshotFx() }),
    })
    if (!res.ok) { showSnackBar('Failed to save profile.', 'error'); return }
    profileName.value = ''
    showSnackBar(`Profile "${name}" saved.`)
    await loadProfiles()
  } catch {
    showSnackBar('Network error.', 'error')
  } finally {
    savingProfile.value = false
  }
}

function loadProfile(profile: FxProfileData) {
  applySnapshot(profile.settings)
  showSnackBar(`Loaded "${profile.name}".`)
}

async function deleteProfile(id: string) {
  try {
    const res = await apiFetch(`/api/fx-profiles/${id}`, { method: 'DELETE' })
    if (!res.ok) { showSnackBar('Failed to delete profile.', 'error'); return }
    profiles.value = profiles.value.filter(p => p.id !== id)
    showSnackBar('Profile deleted.')
  } catch {
    showSnackBar('Network error.', 'error')
  }
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
      audio: null, blobUrl: null, loaded: false, loading: false,
      playing: false, currentTime: 0, duration: 0, seeking: false,
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
      audio.addEventListener('timeupdate', () => { if (!p.seeking) p.currentTime = audio.currentTime })
      audio.addEventListener('durationchange', () => { if (isFinite(audio.duration)) p.duration = audio.duration })
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

  await Promise.all([loadSaved(), loadProfiles()])
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
@import url('https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&display=swap');

/* ── Page-level design tokens (cascade into child components via CSS vars) ── */
.recordings-wrapper {
  --accent:      #4ade80;
  --accent-hover:#6ee7a0;
  --bg:          #0c0f16;
  --panel:       #141a24;
  --panel-2:     #11161f;
  --card:        #161d29;
  --card-hi:     #1a2230;
  --border:      #242d3c;
  --border-sft:  #1d2533;
  --text:        #eef2f8;
  --text-muted:  #8a94a6;
  --dim:         #626e80;
  --track:       #2a3340;
  --record:      #d85c40;
  --record-hi:   #e26a4e;

  min-height: 100vh;
  background:
    radial-gradient(1200px 560px at 78% -8%, color-mix(in srgb, #4ade80 8%, transparent), transparent 60%),
    radial-gradient(900px 500px at 0% 0%, #182233, transparent 55%),
    #0c0f16;
  color: var(--text);
  font-family: "Space Grotesk", system-ui, sans-serif;
}

.rec-layout {
  max-width: 1140px;
  margin: 0 auto;
  padding: 56px 28px 80px;
}

/* ── Header ── */
.rec-header { margin-bottom: 40px; }

.rec-eyebrow {
  font-family: var(--mono);
  font-size: 13px;
  letter-spacing: .28em;
  text-transform: uppercase;
  color: var(--accent);
  margin: 0 0 14px;
  display: flex;
  align-items: center;
  gap: 9px;
}

.eyebrow-dot {
  width: 7px; height: 7px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 10px var(--accent);
  flex-shrink: 0;
}

.rec-layout h1 {
  font-size: 46px;
  font-weight: 700;
  letter-spacing: -.02em;
  margin: 0 0 10px;
  line-height: 1;
  color: var(--text);
}

.rec-lede {
  color: var(--text-muted);
  font-size: 17px;
  margin: 0;
  max-width: 60ch;
}

/* ── Two-column grid ── */
.rec-grid {
  display: grid;
  grid-template-columns: 392px 1fr;
  gap: 24px;
  align-items: start;
}

@media (max-width: 900px) {
  .rec-grid { grid-template-columns: 1fr; }
  .capture-panel { position: static !important; }
}

/* ── Panels ── */
.rec-panel {
  background: linear-gradient(180deg, var(--panel) 0%, var(--panel-2) 100%);
  border: 1px solid var(--border-sft);
  border-radius: 16px;
  padding: 26px;
}

.capture-panel {
  position: sticky;
  top: 28px;
}

.panel-below { margin-top: 24px; }

.section-label {
  font-family: var(--mono);
  font-size: 12px;
  letter-spacing: .26em;
  text-transform: uppercase;
  color: var(--dim);
  margin: 0 0 18px;
}

/* ── Form fields ── */
.rec-field { margin-bottom: 18px; }

.rec-field > label {
  display: block;
  font-size: 14px;
  color: var(--text-muted);
  margin-bottom: 9px;
  font-weight: 500;
}

.row-g { display: flex; gap: 10px; }
.row-g .select-wrap { flex: 1; }

.select-wrap { position: relative; }

.select-wrap::after {
  content: '';
  position: absolute;
  right: 16px; top: 50%;
  width: 9px; height: 9px;
  border-right: 2px solid var(--text-muted);
  border-bottom: 2px solid var(--text-muted);
  transform: translateY(-65%) rotate(45deg);
  pointer-events: none;
}

.select-wrap select {
  width: 100%;
  appearance: none;
  -webkit-appearance: none;
  background: var(--card);
  color: var(--text);
  border: 1px solid var(--border);
  border-radius: 11px;
  padding: 14px 40px 14px 15px;
  font-size: 15px;
  font-family: inherit;
  cursor: pointer;
  transition: border-color .15s, box-shadow .15s;
  outline: none;
}
.select-wrap select:hover { border-color: #303b4d; }
.select-wrap select:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent) 20%, transparent);
}
.select-wrap select:disabled { opacity: .5; cursor: not-allowed; }

/* ── Buttons ── */
.rec-btn {
  border: 1px solid var(--border);
  background: var(--card);
  color: var(--text);
  border-radius: 11px;
  padding: 0 18px;
  font-family: inherit;
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  transition: border-color .15s, background .15s;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  height: 49px;
  white-space: nowrap;
  text-decoration: none;
  flex-shrink: 0;
}
.rec-btn:hover:not(:disabled) { border-color: #3a4658; background: var(--card-hi); }
.rec-btn:disabled { opacity: .45; cursor: not-allowed; }

.rec-btn-sm { height: 36px; padding: 0 14px; font-size: 13px; border-radius: 9px; }

.rec-btn-primary {
  background: var(--accent);
  color: #0c0f16;
  border-color: var(--accent);
  font-weight: 600;
}
.rec-btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
  border-color: var(--accent-hover);
}

.rec-btn-danger { border-color: #f87171; color: #f87171; background: none; }
.rec-btn-danger:hover:not(:disabled) { background: #f87171; color: #fff; border-color: #f87171; }

/* ── Transport ── */
.transport { display: flex; gap: 12px; margin: 22px 0 6px; }

.btn-rec {
  flex: 1;
  height: 54px;
  border: none;
  border-radius: 12px;
  cursor: pointer;
  font-family: inherit;
  font-size: 16px;
  font-weight: 600;
  color: #fff;
  background: linear-gradient(180deg, var(--record-hi), var(--record));
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  box-shadow: 0 8px 22px -10px var(--record), inset 0 1px 0 rgba(255,255,255,.18);
  transition: transform .12s, filter .15s;
}
.btn-rec:hover:not(:disabled) { filter: brightness(1.08); }
.btn-rec:active:not(:disabled) { transform: translateY(1px); }
.btn-rec:disabled:not(.live) { opacity: .45; cursor: not-allowed; }
.btn-rec.live {
  background: linear-gradient(180deg, #2a3240, #20262f);
  color: var(--text);
  box-shadow: inset 0 0 0 1px var(--border);
}

.recdot {
  width: 11px; height: 11px;
  border-radius: 50%;
  background: #fff;
  flex-shrink: 0;
}
.btn-rec.live .recdot {
  background: var(--record);
  animation: rec-pulse 1.1s ease-in-out infinite;
}
@keyframes rec-pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50%       { opacity: .35; transform: scale(.8); }
}

.btn-stop { width: 118px; }

/* ── Level meter ── */
.meter-section { margin-top: 20px; }

.meter-head {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 10px;
  font-size: 14px;
  color: var(--text-muted);
  font-weight: 500;
}

.mono-txt {
  font-family: var(--mono);
  font-size: 12px;
  color: var(--dim);
  letter-spacing: .04em;
}

.meter {
  display: flex;
  gap: 3px;
  height: 26px;
  padding: 6px;
  border-radius: 10px;
  background: #0c1119;
  border: 1px solid var(--border-sft);
}

.meter-seg {
  flex: 1;
  border-radius: 3px;
  background: #1b222e;
  transition: background .06s, box-shadow .06s;
}

.meter-seg--on {
  background: var(--segc);
  box-shadow: 0 0 9px -1px var(--segc);
}

.meter-scale {
  display: flex;
  justify-content: space-between;
  margin-top: 7px;
  font-family: var(--mono);
  font-size: 10px;
  color: var(--dim);
  letter-spacing: .05em;
}

.rec-status {
  margin-top: 16px;
  font-size: 13px;
  color: var(--text-muted);
  min-height: 1.4rem;
}
.rec-status--live { color: var(--record-hi); }

/* ── Effects section ── */
.effects-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 18px;
}

.effects-head-left {
  display: flex;
  align-items: baseline;
  gap: 12px;
}

.effects-head-left .section-label { margin-bottom: 0; }

.count-chip {
  font-family: var(--mono);
  font-size: 11px;
  letter-spacing: .08em;
  color: var(--accent);
  background: color-mix(in srgb, var(--accent) 14%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent) 30%, transparent);
  padding: 3px 9px;
  border-radius: 999px;
}

.link-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  font-family: var(--mono);
  font-size: 12px;
  cursor: pointer;
  letter-spacing: .04em;
  padding: 6px 4px;
  transition: color .15s;
}
.link-btn:hover { color: var(--text); }

.fx-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

/* ── Profile name input ── */
.rec-input {
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: 11px;
  padding: 14px 15px;
  color: var(--text);
  font-size: 15px;
  font-family: inherit;
  outline: none;
  transition: border-color .15s;
  flex: 1;
  min-width: 0;
  height: 49px;
}
.rec-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent) 20%, transparent);
}
.rec-input::placeholder { color: var(--text-muted); }

/* ── List rows ── */
.rec-row {
  padding: .75rem 0;
  border-bottom: 1px solid var(--border-sft);
  display: flex;
  flex-direction: column;
  gap: .5rem;
}
.rec-row:last-child { border-bottom: none; }

.rec-meta { display: flex; flex-direction: column; gap: .2rem; }

.rec-name {
  font-family: var(--mono);
  font-size: .9rem;
  font-weight: 500;
  word-break: break-all;
  color: var(--text);
}

.muted-txt {
  font-size: .8rem;
  color: var(--text-muted);
  font-family: var(--mono);
}

.rec-acts { display: flex; align-items: center; gap: .5rem; flex-wrap: wrap; }

.empty-msg { color: var(--text-muted); font-size: .9rem; padding: 1rem 0; }
.empty-msg.error { color: #f87171; }
.field-error { font-size: .8rem; color: #f87171; }

/* ── Audio player ── */
.custom-player {
  display: flex;
  align-items: center;
  gap: .6rem;
  width: 100%;
}

.play-btn {
  flex-shrink: 0;
  background: none;
  border: none;
  color: var(--text);
  cursor: pointer;
  padding: 0;
  width: 28px; height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: color .15s;
}
.play-btn:hover:not(:disabled) { color: var(--accent); }
.play-btn:disabled { opacity: .4; cursor: not-allowed; }
.play-icon { font-size: 1rem; line-height: 1; pointer-events: none; }

.time-label { flex-shrink: 0; min-width: 2.8rem; text-align: center; }

.seek-wrap { flex: 1; display: flex; align-items: center; height: 20px; }

.seek-slider {
  --pct: 0%;
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 4px;
  border-radius: 4px;
  background: linear-gradient(to right, var(--accent) var(--pct), var(--border) var(--pct));
  outline: none;
  cursor: pointer;
  transition: height .12s;
}
.seek-slider:hover { height: 6px; }
.seek-slider:disabled { opacity: .35; cursor: not-allowed; }
.seek-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 0; height: 0;
  background: #fff;
  border-radius: 50%;
  transition: width .12s, height .12s;
}
.seek-slider:hover::-webkit-slider-thumb { width: 14px; height: 14px; }
.seek-slider::-moz-range-thumb {
  width: 0; height: 0;
  background: #fff;
  border: none;
  border-radius: 50%;
}
.seek-slider:hover::-moz-range-thumb { width: 14px; height: 14px; }
</style>
