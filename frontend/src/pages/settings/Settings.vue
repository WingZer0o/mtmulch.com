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

        <div class="section section-border">
          <h2>Change Password</h2>

          <form class="pw-form" @submit.prevent="handleChangePassword">
            <div class="field">
              <label class="field-label">Current password</label>
              <input
                v-model="pw.current"
                type="password"
                class="field-input"
                autocomplete="current-password"
                required
              />
            </div>
            <div class="field">
              <label class="field-label">New password</label>
              <input
                v-model="pw.next"
                type="password"
                class="field-input"
                autocomplete="new-password"
                required
              />
            </div>
            <div class="field">
              <label class="field-label">Confirm new password</label>
              <input
                v-model="pw.confirm"
                type="password"
                class="field-input"
                autocomplete="new-password"
                required
              />
              <p v-if="pw.next && pw.confirm && pw.next !== pw.confirm" class="field-error">
                Passwords do not match.
              </p>
            </div>
            <button
              type="submit"
              class="btn-primary"
              :disabled="pwSaving || pw.next !== pw.confirm"
            >
              {{ pwSaving ? 'Updating…' : 'Update password' }}
            </button>
          </form>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { apiFetch } from '../../api'
import {showSnackBar} from "../../components/subjects/snack-bar-subject";

const email = ref('')
const otpEnabled = ref(false)
const loading = ref(true)
const fetchError = ref('')
const saving = ref(false)

const pw = reactive({ current: '', next: '', confirm: '' })
const pwSaving = ref(false)


onMounted(async () => {
  try {
    const res = await apiFetch('/api/settings', { credentials: 'include' })
    if (res.status === 401) return
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
    const res = await apiFetch('/api/settings/2fa', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ enabled: newEnabled }),
    })
    if (!res.ok) { showSnackBar('Failed to update 2FA setting.', 'error'); return }
    otpEnabled.value = newEnabled
    showSnackBar(newEnabled ? '2FA enabled.' : '2FA disabled.')
  } catch {
    showSnackBar('Network error.', 'error')
  } finally {
    saving.value = false
  }
}

async function handleChangePassword() {
  if (pw.next !== pw.confirm) return
  pwSaving.value = true
  try {
    const res = await apiFetch('/api/settings/change-password', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        current_password: pw.current,
        new_password: pw.next,
        confirm_password: pw.confirm,
      }),
    })

    if (res.status === 401) { showSnackBar('Current password is incorrect.', 'error'); return }
    if (res.status === 400) {
      const text = await res.text()
      showSnackBar(text.includes('recently') ? 'That password was used recently.' : text, 'error')
      return
    }
    if (!res.ok) { showSnackBar('Failed to change password.', 'error'); return }

    pw.current = ''
    pw.next = ''
    pw.confirm = ''
    showSnackBar('Password updated successfully.')
  } catch {
    showSnackBar('Network error.', 'error')
  } finally {
    pwSaving.value = false
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

.section-border {
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 1px solid var(--border);
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

.pw-form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.field-label {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-muted);
}

.field-input {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 0.6rem 0.875rem;
  color: var(--text);
  font-size: 0.95rem;
  outline: none;
  transition: border-color 0.15s;
}

.field-input:focus {
  border-color: var(--accent);
}

.field-error {
  font-size: 0.8rem;
  color: #f87171;
  margin-top: 0.1rem;
}

.btn-primary {
  margin-top: 0.25rem;
  background: var(--accent);
  color: #fff;
  border: none;
  border-radius: 8px;
  padding: 0.65rem 1.25rem;
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s, opacity 0.15s;
  align-self: flex-start;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-primary:disabled {
  opacity: 0.45;
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
