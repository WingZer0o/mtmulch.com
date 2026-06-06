<template>
  <EffectBlock label="Chorus" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Rate</label>
      <input type="range" class="fx-slider" v-model.number="state.rate" min="0.1" max="10" step="0.1" />
      <span class="fx-val muted">{{ state.rate.toFixed(1) }} Hz</span>
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
import { chorusFxKey } from '../composables/useChorusFx'

const { state } = inject(chorusFxKey)!
</script>
