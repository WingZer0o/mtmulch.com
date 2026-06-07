<template>
  <EffectBlock label="Delay" sub="Echo & repeat" tint="#06b6d4" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
        <circle cx="10.5" cy="10.5" r="8" />
        <polyline points="10.5,5.5 10.5,10.5 14,13" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Time</label>
        <span class="val">{{ state.time.toFixed(2) }}s</span>
      </div>
      <input type="range" v-model.number="state.time" min="0.05" max="2" step="0.01" />
    </div>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Feedback</label>
        <span class="val">{{ state.feedback }}%</span>
      </div>
      <input type="range" v-model.number="state.feedback" min="0" max="90" step="1" />
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
import { delayFxKey } from '../composables/useDelayFx'

const { state } = inject(delayFxKey)!
</script>
