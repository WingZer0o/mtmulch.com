import { type InjectionKey, reactive, watch } from 'vue'

export function useCompressorFx() {
  const state = reactive({ enabled: false, threshold: -24, ratio: 4 })

  let dryGain: GainNode | null = null
  let wetGain: GainNode | null = null
  let compressor: DynamicsCompressorNode | null = null

  function apply() {
    if (!dryGain || !wetGain || !compressor) return
    compressor.threshold.value = state.threshold
    compressor.ratio.value     = state.ratio
    dryGain.gain.value = state.enabled ? 0 : 1
    wetGain.gain.value = state.enabled ? 1 : 0
  }

  watch(() => [state.enabled, state.threshold, state.ratio], apply)

  function connect(ctx: AudioContext, prev: AudioNode): AudioNode {
    const input = ctx.createGain()
    const output = ctx.createGain()
    dryGain    = ctx.createGain()
    wetGain    = ctx.createGain()
    compressor = ctx.createDynamicsCompressor()
    compressor.knee.value    = 10
    compressor.attack.value  = 0.003
    compressor.release.value = 0.25

    prev.connect(input)
    input.connect(dryGain);    dryGain.connect(output)
    input.connect(compressor); compressor.connect(wetGain); wetGain.connect(output)

    apply()
    return output
  }

  function disconnect() {
    dryGain = null; wetGain = null; compressor = null
  }

  return { state, connect, disconnect }
}

export type CompressorFx = ReturnType<typeof useCompressorFx>
export const compressorFxKey: InjectionKey<CompressorFx> = Symbol('compressorFx')
