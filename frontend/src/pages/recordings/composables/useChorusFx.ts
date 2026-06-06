import { type InjectionKey, reactive, watch } from 'vue'

export function useChorusFx() {
  const state = reactive({ enabled: false, rate: 1.5, mix: 30 })

  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let delayNode: DelayNode | null = null
  let lfo: OscillatorNode | null = null
  let lfoGain: GainNode | null = null

  function apply() {
    if (!dryGain || !wetGain || !delayNode || !lfo || !lfoGain) return
    delayNode.delayTime.value = 0.025  // 25 ms base
    lfo.frequency.value       = state.rate
    lfoGain.gain.value        = 0.003  // ±3 ms
    dryGain.gain.value        = 1
    wetGain.gain.value        = state.enabled ? state.mix / 100 : 0
  }

  watch(() => [state.enabled, state.rate, state.mix], apply)

  function connect(ctx: AudioContext, prev: AudioNode): AudioNode {
    const input = ctx.createGain()
    const output = ctx.createGain()
    dryGain   = ctx.createGain()
    wetGain   = ctx.createGain()
    delayNode = ctx.createDelay(0.1)
    lfo       = ctx.createOscillator(); lfo.type = 'sine'
    lfoGain   = ctx.createGain()

    prev.connect(input)
    input.connect(dryGain); dryGain.connect(output)
    input.connect(delayNode); delayNode.connect(wetGain); wetGain.connect(output)
    lfo.connect(lfoGain); lfoGain.connect(delayNode.delayTime)
    lfo.start()

    apply()
    return output
  }

  function disconnect() {
    try { lfo?.stop() } catch {}
    dryGain = null; wetGain = null; delayNode = null; lfo = null; lfoGain = null
  }

  return { state, connect, disconnect }
}

export type ChorusFx = ReturnType<typeof useChorusFx>
export const chorusFxKey: InjectionKey<ChorusFx> = Symbol('chorusFx')
