<template>
  <EffectBlock label="Delay" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Time</label>
      <input type="range" class="fx-slider" v-model.number="state.time" min="0.05" max="2" step="0.01" />
      <span class="fx-val muted">{{ state.time.toFixed(2) }}s</span>
    </div>
    <div class="fx-knob">
      <label class="field-label">Feedback</label>
      <input type="range" class="fx-slider" v-model.number="state.feedback" min="0" max="90" step="1" />
      <span class="fx-val muted">{{ state.feedback }}%</span>
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
import { delayFxKey } from '../composables/useDelayFx'

const { state } = inject(delayFxKey)!
</script>
