<template>
  <EffectBlock label="Auto-Wah" sub="Envelope filter" tint="#4ade80" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M1,16 L4,16 L5.5,14 L7,9 L9,4 L10.5,9 L12,14 L13.5,16 L17,16" />
        <line x1="17" y1="16" x2="20" y2="16" opacity=".4" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Rate</label>
        <span class="val">{{ state.rate.toFixed(1) }} Hz</span>
      </div>
      <input type="range" v-model.number="state.rate" min="0.1" max="10" step="0.1" />
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { wahFxKey } from '../composables/useWahFx'

const { state } = inject(wahFxKey)!
</script>
