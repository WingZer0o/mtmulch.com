<template>
  <EffectBlock label="EQ" sub="3-band equalizer" tint="#3b82f6" v-model="state.enabled">
    <template #icon>
      <svg viewBox="0 0 21 21" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
        <line x1="5" y1="4" x2="5" y2="17" />
        <circle cx="5" cy="13" r="2.5" fill="currentColor" stroke="none" />
        <line x1="10.5" y1="4" x2="10.5" y2="17" />
        <circle cx="10.5" cy="8" r="2.5" fill="currentColor" stroke="none" />
        <line x1="16" y1="4" x2="16" y2="17" />
        <circle cx="16" cy="11" r="2.5" fill="currentColor" stroke="none" />
      </svg>
    </template>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Bass</label>
        <span class="val">{{ state.bass > 0 ? '+' : '' }}{{ state.bass }} dB</span>
      </div>
      <input type="range" v-model.number="state.bass" min="-12" max="12" step="1" />
    </div>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Mid</label>
        <span class="val">{{ state.mid > 0 ? '+' : '' }}{{ state.mid }} dB</span>
      </div>
      <input type="range" v-model.number="state.mid" min="-12" max="12" step="1" />
    </div>
    <div class="ctrl">
      <div class="ctrl-head">
        <label>Treble</label>
        <span class="val">{{ state.treble > 0 ? '+' : '' }}{{ state.treble }} dB</span>
      </div>
      <input type="range" v-model.number="state.treble" min="-12" max="12" step="1" />
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { eqFxKey } from '../composables/useEqFx'

const { state } = inject(eqFxKey)!
</script>
