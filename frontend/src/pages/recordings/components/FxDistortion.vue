<template>
  <EffectBlock label="Distortion" sub="Harmonic overdrive" tint="#f97316" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="1,13 5,13 7,5 9,5 11,16 13,16 15,8 20,8" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Drive</label>
        <span class="val">{{ state.drive }}%</span>
      </div>
      <input type="range" v-model.number="state.drive" min="0" max="100" step="1" />
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { distortionFxKey } from '../composables/useDistortionFx'

const { state } = inject(distortionFxKey)!
</script>
