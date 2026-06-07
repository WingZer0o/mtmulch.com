<template>
  <EffectBlock label="Chorus" sub="Pitch modulation" tint="#ec4899" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
        <path d="M1,8.5 C3.5,5 6,12 8.5,8.5 C11,5 13.5,12 16,8.5 C17.5,6.5 19,10.5 20,8.5" />
        <path d="M1,13.5 C3.5,10 6,17 8.5,13.5 C11,10 13.5,17 16,13.5 C17.5,11.5 19,15.5 20,13.5" opacity=".5" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Rate</label>
        <span class="val">{{ state.rate.toFixed(1) }} Hz</span>
      </div>
      <input type="range" v-model.number="state.rate" min="0.1" max="10" step="0.1" />
    </div>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Mix</label>
        <span class="val">{{ state.mix }}%</span>
      </div>
      <input type="range" v-model.number="state.mix" min="0" max="100" step="1" />
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { chorusFxKey } from '../composables/useChorusFx'

const { state } = inject(chorusFxKey)!
</script>
