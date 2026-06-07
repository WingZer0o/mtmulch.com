<template>
  <article class="fx-card" :class="{ 'fx-off': !enabled }" :style="{ '--fxtint': tint ?? '#7c6af7' }">
    <div class="fx-top">
      <div class="fx-ico">
        <slot name="icon" />
      </div>
      <div class="fx-name">
        <h3>{{ label }}</h3>
        <p v-if="sub">{{ sub }}</p>
      </div>
      <button class="fx-toggle" :class="{ 'fx-toggle--on': enabled }"
              @click="enabled = !enabled"
              :aria-pressed="enabled" :aria-label="label">
        <span class="fx-toggle-knob"></span>
      </button>
    </div>
    <div class="fx-body">
      <slot />
    </div>
  </article>
</template>

<script setup lang="ts">
defineProps<{ label: string; tint?: string; sub?: string }>()
const enabled = defineModel<boolean>({ default: false })
</script>

<style>
.fx-card {
  position: relative;
  background: var(--card, #161d29);
  border: 1px solid var(--border-sft, #1d2533);
  border-radius: 14px;
  padding: 18px 18px 20px;
  overflow: hidden;
  transition: border-color .15s, transform .15s, box-shadow .15s;
}

.fx-card:hover {
  border-color: var(--border, #242d3c);
  transform: translateY(-2px);
  box-shadow: 0 14px 30px -18px rgba(0,0,0,.8);
}

.fx-card::before {
  content: '';
  position: absolute;
  left: 0; top: 0; bottom: 0;
  width: 3px;
  background: var(--fxtint);
  opacity: .9;
  transition: opacity .2s;
}

.fx-off::before { opacity: .2; }

.fx-top {
  display: flex;
  align-items: center;
  gap: 13px;
}

.fx-ico {
  width: 40px;
  height: 40px;
  border-radius: 11px;
  flex-shrink: 0;
  display: grid;
  place-items: center;
  color: var(--fxtint);
  background: color-mix(in srgb, var(--fxtint) 16%, transparent);
  border: 1px solid color-mix(in srgb, var(--fxtint) 26%, transparent);
}

.fx-ico svg {
  width: 21px;
  height: 21px;
}

.fx-name {
  flex: 1;
  min-width: 0;
}

.fx-name h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  letter-spacing: -.01em;
  color: var(--text, #eef2f8);
}

.fx-name p {
  margin: 2px 0 0;
  font-size: 12px;
  color: var(--dim, #626e80);
}

.fx-toggle {
  width: 46px;
  height: 27px;
  border-radius: 999px;
  background: var(--track, #2a3340);
  border: none;
  position: relative;
  cursor: pointer;
  flex-shrink: 0;
  transition: background .18s;
  padding: 0;
}

.fx-toggle--on { background: var(--accent, #4ade80); }

.fx-toggle-knob {
  position: absolute;
  top: 3px; left: 3px;
  width: 21px; height: 21px;
  border-radius: 50%;
  background: #fff;
  transition: transform .18s cubic-bezier(.4,1.3,.6,1);
  box-shadow: 0 2px 5px rgba(0,0,0,.35);
}

.fx-toggle--on .fx-toggle-knob { transform: translateX(19px); }

.fx-body {
  margin-top: 18px;
  display: grid;
  gap: 15px;
  transition: opacity .2s;
}

.fx-off .fx-body {
  opacity: .32;
  filter: saturate(.4);
  pointer-events: none;
}

/* Controls */
.ctrl-head {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 9px;
}

.ctrl-head label {
  font-size: 13px;
  color: var(--muted, #8a94a6);
  font-weight: 500;
  cursor: default;
}

.ctrl-head .val {
  font-family: var(--mono, monospace);
  font-size: 12px;
  color: var(--accent, #4ade80);
  background: color-mix(in srgb, var(--accent, #4ade80) 12%, transparent);
  padding: 2px 8px;
  border-radius: 6px;
}

.fx-off .ctrl-head .val {
  color: var(--muted, #8a94a6);
  background: transparent;
}

/* Range inputs inside fx-card */
.fx-card input[type=range] {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 6px;
  border-radius: 999px;
  background: var(--track, #2a3340);
  cursor: pointer;
  outline: none;
}

.fx-card input[type=range]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px; height: 18px;
  border-radius: 50%;
  background: var(--accent, #4ade80);
  border: 3px solid var(--card, #161d29);
  cursor: grab;
  box-shadow: 0 0 0 1px color-mix(in srgb, var(--accent, #4ade80) 60%, transparent), 0 2px 6px rgba(0,0,0,.4);
  transition: transform .12s;
}

.fx-card input[type=range]::-webkit-slider-thumb:active {
  cursor: grabbing;
  transform: scale(1.15);
}

.fx-card input[type=range]::-moz-range-thumb {
  width: 18px; height: 18px;
  border-radius: 50%;
  background: var(--accent, #4ade80);
  border: 3px solid var(--card, #161d29);
  cursor: grab;
  box-shadow: 0 0 0 1px color-mix(in srgb, var(--accent, #4ade80) 60%, transparent);
}
</style>
