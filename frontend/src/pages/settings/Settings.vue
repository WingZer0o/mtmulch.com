<template>
  <div class="settings-wrapper">
    <div class="settings-card">
      <p class="label">Account</p>
      <h1>Settings</h1>

      <div v-if="loading" class="state-msg">Loading…</div>
      <div v-else-if="fetchError" class="state-msg error">{{ fetchError }}</div>

      <template v-else>
        <p class="user-email">{{ email }}</p>

        <div class="section">
          <h2>Security</h2>

          <div class="toggle-row">
            <div>
              <p class="toggle-title">Two-factor authentication</p>
              <p class="toggle-desc">Require an email code when signing in.</p>
            </div>
            <label class="switch">
              <input
                type="checkbox"
                :checked="otpEnabled"
                :disabled="saving"
                @change="handleToggle"
              />
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </template>
    </div>
  </div>

  <v-snackbar v-model="snackbar.show" :color="snackbar.color" :timeout="3000" location="bottom">
    {{ snackbar.message }}
  </v-snackbar>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const email = ref('')
const otpEnabled = ref(false)
const loading = ref(true)
const fetchError = ref('')
const saving = ref(false)
const snackbar = ref({ show: false, message: '', color: 'success' as 'success' | 'error' })

function toast(message: string, color: 'success' | 'error' = 'success') {
  snackbar.value = { show: true, message, color }
}

onMounted(async () => {
  try {
    const res = await fetch('/api/settings', { credentials: 'include' })
    if (res.status === 401) { fetchError.value = 'You must be logged in to view settings.'; return }
    if (!res.ok) throw new Error()
    const data = await res.json()
    email.value = data.email
    otpEnabled.value = data.otp_enabled
  } catch {
    fetchError.value = 'Failed to load settings.'
  } finally {
    loading.value = false
  }
})

async function handleToggle() {
  const newEnabled = !otpEnabled.value
  saving.value = true
  try {
    const res = await fetch('/api/settings/2fa', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ enabled: newEnabled }),
    })
    if (!res.ok) { toast('Failed to update 2FA setting.', 'error'); return }
    otpEnabled.value = newEnabled
    toast(newEnabled ? '2FA enabled.' : '2FA disabled.')
  } catch {
    toast('Network error.', 'error')
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.settings-wrapper {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg);
  padding: 2rem;
}

.settings-card {
  width: 100%;
  max-width: 520px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 2.5rem;
}

.label {
  font-family: var(--mono);
  color: var(--accent);
  font-size: 0.9rem;
  margin-bottom: 0.5rem;
}

h1 {
  font-size: 1.75rem;
  font-weight: 700;
  margin-bottom: 0.75rem;
}

h2 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  font-family: var(--mono);
  margin-bottom: 1.25rem;
}

.user-email {
  font-family: var(--mono);
  font-size: 0.9rem;
  color: var(--text-muted);
  margin-bottom: 2rem;
  padding-bottom: 2rem;
  border-bottom: 1px solid var(--border);
}

.section {
  display: flex;
  flex-direction: column;
}

.toggle-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 2rem;
}

.toggle-title {
  font-weight: 500;
  margin-bottom: 0.2rem;
}

.toggle-desc {
  font-size: 0.85rem;
  color: var(--text-muted);
}

.switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  flex-shrink: 0;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  inset: 0;
  background: var(--border);
  border-radius: 24px;
  cursor: pointer;
  transition: background 0.2s;
}

.slider::before {
  content: '';
  position: absolute;
  width: 18px;
  height: 18px;
  left: 3px;
  bottom: 3px;
  background: #fff;
  border-radius: 50%;
  transition: transform 0.2s;
}

.switch input:checked + .slider {
  background: var(--accent);
}

.switch input:checked + .slider::before {
  transform: translateX(20px);
}

.switch input:disabled + .slider {
  opacity: 0.5;
  cursor: not-allowed;
}

.state-msg {
  color: var(--text-muted);
  font-size: 0.9rem;
  padding: 1rem 0;
}

.state-msg.error {
  color: #f87171;
}
</style>
