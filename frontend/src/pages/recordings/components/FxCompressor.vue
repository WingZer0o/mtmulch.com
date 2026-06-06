<template>
  <EffectBlock label="Compressor" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Threshold</label>
      <input type="range" class="fx-slider" v-model.number="state.threshold" min="-60" max="0" step="1" />
      <span class="fx-val muted">{{ state.threshold }} dB</span>
    </div>
    <div class="fx-knob">
      <label class="field-label">Ratio</label>
      <input type="range" class="fx-slider" v-model.number="state.ratio" min="1" max="20" step="0.5" />
      <span class="fx-val muted">{{ state.ratio }}:1</span>
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { compressorFxKey } from '../composables/useCompressorFx'

const { state } = inject(compressorFxKey)!
</script>
