<template>
  <EffectBlock label="Reverb" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Decay</label>
      <input type="range" class="fx-slider" v-model.number="state.decay" min="0.5" max="8" step="0.1" />
      <span class="fx-val muted">{{ state.decay.toFixed(1) }}s</span>
    </div>
    <div class="fx-knob">
      <label class="field-label">Mix</label>
      <input type="range" class="fx-slider" v-model.number="state.mix" min="0" max="100" step="1" />
      <span class="fx-val muted">{{ state.mix }}%</span>
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { reverbFxKey } from '../composables/useReverbFx'

const { state } = inject(reverbFxKey)!
</script>
