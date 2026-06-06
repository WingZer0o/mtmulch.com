<template>
  <EffectBlock label="EQ" v-model="state.enabled">
    <div class="fx-knob">
      <label class="field-label">Bass</label>
      <input type="range" class="fx-slider" v-model.number="state.bass" min="-12" max="12" step="1" />
      <span class="fx-val muted">{{ state.bass > 0 ? '+' : '' }}{{ state.bass }} dB</span>
    </div>
    <div class="fx-knob">
      <label class="field-label">Mid</label>
      <input type="range" class="fx-slider" v-model.number="state.mid" min="-12" max="12" step="1" />
      <span class="fx-val muted">{{ state.mid > 0 ? '+' : '' }}{{ state.mid }} dB</span>
    </div>
    <div class="fx-knob">
      <label class="field-label">Treble</label>
      <input type="range" class="fx-slider" v-model.number="state.treble" min="-12" max="12" step="1" />
      <span class="fx-val muted">{{ state.treble > 0 ? '+' : '' }}{{ state.treble }} dB</span>
    </div>
  </EffectBlock>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import EffectBlock from './EffectBlock.vue'
import { eqFxKey } from '../composables/useEqFx'

const { state } = inject(eqFxKey)!
</script>
