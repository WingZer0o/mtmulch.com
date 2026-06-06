import { type InjectionKey, reactive, watch } from 'vue'

function createIR(ctx: AudioContext, decaySec: number): AudioBuffer {
  const sr  = ctx.sampleRate
  const len = Math.ceil(sr * decaySec)
  const buf = ctx.createBuffer(2, len, sr)
  for (let c = 0; c < 2; c++) {
    const d = buf.getChannelData(c)
    for (let i = 0; i < len; i++)
      d[i] = (Math.random() * 2 - 1) * Math.pow(1 - i / len, 3)
  }
  return buf
}

export function useReverbFx() {
  const state = reactive({ enabled: false, decay: 2.5, mix: 25 })

  let ctx: AudioContext | null = null
  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let convolver: ConvolverNode | null = null

  function apply() {
    if (!dryGain || !wetGain) return
    dryGain.gain.value = 1
    wetGain.gain.value = state.enabled ? state.mix / 100 : 0
  }

  watch(() => [state.enabled, state.mix], apply)
  watch(() => state.decay, () => {
    if (convolver && ctx) convolver.buffer = createIR(ctx, state.decay)
    apply()
  })

  function connect(_ctx: AudioContext, prev: AudioNode): AudioNode {
    ctx = _ctx
    const input = _ctx.createGain()
    const output = _ctx.createGain()
    dryGain  = _ctx.createGain()
    wetGain  = _ctx.createGain()
    convolver = _ctx.createConvolver()
    convolver.buffer = createIR(_ctx, state.decay)

    prev.connect(input)
    input.connect(dryGain);  dryGain.connect(output)
    input.connect(convolver); convolver.connect(wetGain); wetGain.connect(output)

    apply()
    return output
  }

  function disconnect() {
    ctx = null; dryGain = null; wetGain = null; convolver = null
  }

  return { state, connect, disconnect }
}

export type ReverbFx = ReturnType<typeof useReverbFx>
export const reverbFxKey: InjectionKey<ReverbFx> = Symbol('reverbFx')
