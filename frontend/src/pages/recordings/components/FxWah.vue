<template>
  <EffectBlock label="Auto-Wah" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Rate</label>
      <input type="range" class="fx-slider" v-model.number="state.rate" min="0.1" max="10" step="0.1" />
      <span class="fx-val muted">{{ state.rate.toFixed(1) }} Hz</span>
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { wahFxKey } from '../composables/useWahFx'

const { state } = inject(wahFxKey)!
</script>
