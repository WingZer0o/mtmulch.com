import { type InjectionKey, reactive, watch } from 'vue'

export function useWahFx() {
  const state = reactive({ enabled: false, rate: 2 })

  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let filter: BiquadFilterNode | null = null
  let lfo: OscillatorNode | null = null
  let lfoGain: GainNode | null = null

  function apply() {
    if (!dryGain || !wetGain || !filter || !lfo || !lfoGain) return
    filter.frequency.value = 1500   // centre (DC offset for LFO)
    filter.Q.value         = 5
    lfo.frequency.value    = state.rate
    lfoGain.gain.value     = 1200   // ±1200 Hz sweep
    dryGain.gain.value     = state.enabled ? 0 : 1
    wetGain.gain.value     = state.enabled ? 1 : 0
  }

  watch(() => [state.enabled, state.rate], apply)

  function connect(ctx: AudioContext, prev: AudioNode): AudioNode {
    const input = ctx.createGain()
    const output = ctx.createGain()
    dryGain = ctx.createGain()
    wetGain = ctx.createGain()
    filter  = ctx.createBiquadFilter(); filter.type = 'bandpass'
    lfo     = ctx.createOscillator();   lfo.type    = 'sine'
    lfoGain = ctx.createGain()

    prev.connect(input)
    input.connect(dryGain); dryGain.connect(output)
    input.connect(filter);  filter.connect(wetGain); wetGain.connect(output)
    lfo.connect(lfoGain);   lfoGain.connect(filter.frequency)
    lfo.start()

    apply()
    return output
  }

  function disconnect() {
    try { lfo?.stop() } catch {}
    dryGain = null; wetGain = null; filter = null; lfo = null; lfoGain = null
  }

  return { state, connect, disconnect }
}

export type WahFx = ReturnType<typeof useWahFx>
export const wahFxKey: InjectionKey<WahFx> = Symbol('wahFx')
