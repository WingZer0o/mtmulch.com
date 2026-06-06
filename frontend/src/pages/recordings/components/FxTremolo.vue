<template>
  <EffectBlock label="Tremolo" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Rate</label>
      <input type="range" class="fx-slider" v-model.number="state.rate" min="0.5" max="20" step="0.1" />
      <span class="fx-val muted">{{ state.rate.toFixed(1) }} Hz</span>
    </div>
    <div class="fx-knob">
      <label class="field-label">Depth</label>
      <input type="range" class="fx-slider" v-model.number="state.depth" min="0" max="100" step="1" />
      <span class="fx-val muted">{{ state.depth }}%</span>
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { tremoloFxKey } from '../composables/useTremoloFx'

const { state } = inject(tremoloFxKey)!
</script>
