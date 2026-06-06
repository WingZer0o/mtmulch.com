import { type InjectionKey, reactive, watch } from 'vue'

function makeDistortionCurve(drive: number): Float32Array {
  const k = drive * 4
  const n = 512
  const curve = new Float32Array(n)
  for (let i = 0; i < n; i++) {
    const x = (i * 2) / n - 1
    curve[i] = k === 0 ? x : ((Math.PI + k) * x) / (Math.PI + k * Math.abs(x))
  }
  return curve
}

export function useDistortionFx() {
  const state = reactive({ enabled: false, drive: 30 })

  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let shaper: WaveShaperNode | null = null

  function apply() {
    if (!dryGain || !wetGain || !shaper) return
    shaper.curve = state.enabled
      ? makeDistortionCurve(state.drive) as Float32Array<ArrayBuffer>
      : null
    dryGain.gain.value = state.enabled ? 0 : 1
    wetGain.gain.value = state.enabled ? 1 / (1 + state.drive / 100) : 0
  }

  watch(() => [state.enabled, state.drive], apply)

  function connect(ctx: AudioContext, prev: AudioNode): AudioNode {
    const input = ctx.createGain()
    const output = ctx.createGain()
    dryGain = ctx.createGain()
    wetGain = ctx.createGain()
    shaper  = ctx.createWaveShaper()
    shaper.oversample = '4x'

    prev.connect(input)
    input.connect(dryGain);  dryGain.connect(output)
    input.connect(shaper);   shaper.connect(wetGain); wetGain.connect(output)

    apply()
    return output
  }

  function disconnect() {
    dryGain = null; wetGain = null; shaper = null
  }

  return { state, connect, disconnect }
}

export type DistortionFx = ReturnType<typeof useDistortionFx>
export const distortionFxKey: InjectionKey<DistortionFx> = Symbol('distortionFx')
