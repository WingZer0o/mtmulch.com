<template>
  <EffectBlock label="Distortion" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Drive</label>
      <input type="range" class="fx-slider" v-model.number="state.drive" min="0" max="100" step="1" />
      <span class="fx-val muted">{{ state.drive }}%</span>
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { distortionFxKey } from '../composables/useDistortionFx'

const { state } = inject(distortionFxKey)!
</script>
