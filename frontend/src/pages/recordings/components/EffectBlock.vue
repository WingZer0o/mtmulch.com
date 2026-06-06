<template>
  <div class="fx-block">
    <div class="fx-header">
      <span class="fx-name">{{ label }}</span>
      <label class="toggle-switch">
        <input type="checkbox" v-model="enabled">
        <span class="toggle-track"></span>
      </label>
    </div>
    <div v-if="enabled" class="fx-controls">
      <div class="fx-row">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ label: string }>()
const enabled = defineModel<boolean>({ default: false })
</script>

<style>
.fx-block {
  padding: 0.65rem 0;
  border-bottom: 1px solid var(--border);
}

.fx-block:last-child {
  border-bottom: none;
}

.fx-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.fx-name {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text);
}

.fx-controls {
  margin-top: 0.75rem;
}

.fx-row {
  display: flex;
  gap: 1.25rem;
  flex-wrap: wrap;
}

.fx-knob {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  flex: 1;
  min-width: 80px;
}

.fx-slider {
  width: 100%;
  height: 4px;
  accent-color: var(--accent);
  cursor: pointer;
}

.fx-val {
  font-family: var(--mono);
  font-size: 0.75rem;
  text-align: center;
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
  cursor: pointer;
  flex-shrink: 0;
}

.toggle-switch input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-track {
  position: absolute;
  inset: 0;
  background: var(--border);
  border-radius: 22px;
  transition: background 0.2s;
}

.toggle-track::after {
  content: '';
  position: absolute;
  top: 3px;
  left: 3px;
  width: 16px;
  height: 16px;
  background: var(--text-muted);
  border-radius: 50%;
  transition: transform 0.2s, background 0.2s;
}

.toggle-switch input:checked + .toggle-track {
  background: var(--accent);
}

.toggle-switch input:checked + .toggle-track::after {
  transform: translateX(18px);
  background: #fff;
}
</style>
