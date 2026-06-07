<template>
  <EffectBlock label="Reverb" sub="Space & ambience" tint="#10b981" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
        <circle cx="10.5" cy="10.5" r="2" />
        <path d="M6.5,6.5 a5.66,5.66 0 0,0 0,8" />
        <path d="M14.5,6.5 a5.66,5.66 0 0,1 0,8" />
        <path d="M3,3.5 a10.5,10.5 0 0,0 0,14" />
        <path d="M18,3.5 a10.5,10.5 0 0,1 0,14" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Decay</label>
        <span class="val">{{ state.decay.toFixed(1) }}s</span>
      </div>
      <input type="range" v-model.number="state.decay" min="0.5" max="8" step="0.1" />
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
import { reverbFxKey } from '../composables/useReverbFx'

const { state } = inject(reverbFxKey)!
</script>
