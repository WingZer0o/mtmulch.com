<template>
  <EffectBlock label="Tremolo" sub="Amplitude modulation" tint="#eab308" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
        <path d="M1,13 C2,11 3,15 4.5,13 C6,11 7,15 8.5,13 C10,7 11,18 12.5,13 C14,7 15,18 16.5,13 C18,11 19,15 20,13" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Rate</label>
        <span class="val">{{ state.rate.toFixed(1) }} Hz</span>
      </div>
      <input type="range" v-model.number="state.rate" min="0.5" max="20" step="0.1" />
    </div>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Depth</label>
        <span class="val">{{ state.depth }}%</span>
      </div>
      <input type="range" v-model.number="state.depth" min="0" max="100" step="1" />
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { tremoloFxKey } from '../composables/useTremoloFx'

const { state } = inject(tremoloFxKey)!
</script>
