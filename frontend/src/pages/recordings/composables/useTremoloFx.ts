import { type InjectionKey, reactive, watch } from 'vue'

export function useTremoloFx() {
  const state = reactive({ enabled: false, rate: 4, depth: 50 })

  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let tremGain: GainNode | null = null
  let lfo: OscillatorNode | null = null
  let lfoGain: GainNode | null = null

  function apply() {
    if (!dryGain || !wetGain || !tremGain || !lfo || !lfoGain) return
    lfo.frequency.value = state.rate
    const d = state.depth / 100
    tremGain.gain.value = 1 - d / 2
    lfoGain.gain.value  = d / 2
    dryGain.gain.value  = state.enabled ? 0 : 1
    wetGain.gain.value  = state.enabled ? 1 : 0
  }

  watch(() => [state.enabled, state.rate, state.depth], apply)

  function connect(ctx: AudioContext, prev: AudioNode): AudioNode {
    const input = ctx.createGain()
    const output = ctx.createGain()
    dryGain  = ctx.createGain()
    wetGain  = ctx.createGain()
    tremGain = ctx.createGain()
    lfo      = ctx.createOscillator(); lfo.type = 'sine'
    lfoGain  = ctx.createGain()

    prev.connect(input)
    input.connect(dryGain); dryGain.connect(output)
    input.connect(tremGain); tremGain.connect(wetGain); wetGain.connect(output)
    lfo.connect(lfoGain); lfoGain.connect(tremGain.gain)
    lfo.start()

    apply()
    return output
  }

  function disconnect() {
    try { lfo?.stop() } catch {}
    dryGain = null; wetGain = null; tremGain = null; lfo = null; lfoGain = null
  }

  return { state, connect, disconnect }
}

export type TremoloFx = ReturnType<typeof useTremoloFx>
export const tremoloFxKey: InjectionKey<TremoloFx> = Symbol('tremoloFx')
