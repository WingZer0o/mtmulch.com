import { type InjectionKey, reactive, watch } from 'vue'

export function useEqFx() {
  const state = reactive({ enabled: false, bass: 0, mid: 0, treble: 0 })

  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let bassF: BiquadFilterNode | null = null
  let midF: BiquadFilterNode | null = null
  let trebleF: BiquadFilterNode | null = null

  function apply() {
    if (!dryGain || !wetGain || !bassF || !midF || !trebleF) return
    bassF.gain.value   = state.bass
    midF.gain.value    = state.mid
    trebleF.gain.value = state.treble
    dryGain.gain.value = state.enabled ? 0 : 1
    wetGain.gain.value = state.enabled ? 1 : 0
  }

  watch(() => [state.enabled, state.bass, state.mid, state.treble], apply)

  function connect(ctx: AudioContext, prev: AudioNode): AudioNode {
    const input = ctx.createGain()
    const output = ctx.createGain()
    dryGain = ctx.createGain()
    wetGain = ctx.createGain()
    bassF   = ctx.createBiquadFilter(); bassF.type = 'lowshelf';  bassF.frequency.value = 200
    midF    = ctx.createBiquadFilter(); midF.type  = 'peaking';   midF.frequency.value  = 1000; midF.Q.value = 1
    trebleF = ctx.createBiquadFilter(); trebleF.type = 'highshelf'; trebleF.frequency.value = 3000

    prev.connect(input)
    input.connect(dryGain); dryGain.connect(output)
    input.connect(bassF); bassF.connect(midF); midF.connect(trebleF); trebleF.connect(wetGain); wetGain.connect(output)

    apply()
    return output
  }

  function disconnect() {
    dryGain = null; wetGain = null; bassF = null; midF = null; trebleF = null
  }

  return { state, connect, disconnect }
}

export type EqFx = ReturnType<typeof useEqFx>
export const eqFxKey: InjectionKey<EqFx> = Symbol('eqFx')
