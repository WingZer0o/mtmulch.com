<template>
  <EffectBlock label="Compressor" sub="Dynamic range control" tint="#a855f7" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M2,14 L6,6 L10.5,14 L15,6 L19,14" />
        <line x1="1" y1="14" x2="20" y2="14" stroke-dasharray="3 2" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Threshold</label>
        <span class="val">{{ state.threshold }} dB</span>
      </div>
      <input type="range" v-model.number="state.threshold" min="-60" max="0" step="1" />
    </div>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Ratio</label>
        <span class="val">{{ state.ratio }}:1</span>
      </div>
      <input type="range" v-model.number="state.ratio" min="1" max="20" step="0.5" />
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { compressorFxKey } from '../composables/useCompressorFx'

const { state } = inject(compressorFxKey)!
</script>
