import { type InjectionKey, reactive, watch } from 'vue'

export function useDelayFx() {
  const state = reactive({ enabled: false, time: 0.35, feedback: 40, mix: 25 })

  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let delayNode: DelayNode | null = null
  let fbGain: GainNode | null = null

  function apply() {
    if (!dryGain || !wetGain || !delayNode || !fbGain) return
    delayNode.delayTime.value = state.time
    fbGain.gain.value         = state.feedback / 100
    dryGain.gain.value        = 1
    wetGain.gain.value        = state.enabled ? state.mix / 100 : 0
  }

  watch(() => [state.enabled, state.time, state.feedback, state.mix], apply)

  function connect(ctx: AudioContext, prev: AudioNode): AudioNode {
    const input = ctx.createGain()
    const output = ctx.createGain()
    dryGain   = ctx.createGain()
    wetGain   = ctx.createGain()
    delayNode = ctx.createDelay(2.0)
    fbGain    = ctx.createGain()

    prev.connect(input)
    input.connect(dryGain); dryGain.connect(output)
    input.connect(delayNode)
    delayNode.connect(fbGain); fbGain.connect(delayNode) // feedback loop
    delayNode.connect(wetGain); wetGain.connect(output)

    apply()
    return output
  }

  function disconnect() {
    dryGain = null; wetGain = null; delayNode = null; fbGain = null
  }

  return { state, connect, disconnect }
}

export type DelayFx = ReturnType<typeof useDelayFx>
export const delayFxKey: InjectionKey<DelayFx> = Symbol('delayFx')
