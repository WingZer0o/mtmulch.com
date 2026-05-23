<template>
  <div class="auth-wrapper">
    <div class="auth-card">

      <!-- Login -->
      <div v-if="view === 'login'">
        <p class="label">Welcome back</p>
        <h1>Sign in</h1>

        <form @submit.prevent="handleLogin" class="form">
          <div class="field">
            <label for="login-email">Email</label>
            <input id="login-email" v-model="login.email" type="email" placeholder="you@example.com" required />
          </div>
          <div class="field">
            <label for="login-password">Password</label>
            <input id="login-password" v-model="login.password" type="password" placeholder="••••••••" required />
          </div>
          <button type="submit" class="btn-primary">Sign in</button>
        </form>

        <p class="switch">
          Don't have an account?
          <button class="btn-link" @click="router.push('/register')">Register</button>
        </p>
      </div>

      <!-- Register -->
      <div v-else>
        <p class="label">Get started</p>
        <h1>Create account</h1>

        <form @submit.prevent="handleRegister" class="form">
          <div class="field">
            <label for="reg-email">Email</label>
            <input id="reg-email" v-model="register.email" type="email" placeholder="you@example.com" required />
          </div>
          <div class="field">
            <label for="reg-password">Password</label>
            <input id="reg-password" v-model="register.password" type="password" placeholder="••••••••" required />
          </div>
          <div class="field">
            <label for="reg-confirm">Confirm password</label>
            <input id="reg-confirm" v-model="register.confirm" type="password" placeholder="••••••••" required />
          </div>
          <button type="submit" class="btn-primary">Create account</button>
        </form>

        <p class="switch">
          Already have an account?
          <button class="btn-link" @click="router.push('/login')">Sign in</button>
        </p>
      </div>

    </div>
  </div>

  <!-- 2FA OTP modal -->
  <v-dialog v-model="showOtpDialog" max-width="420" persistent>
    <div class="otp-modal">
      <p class="label">Two-factor authentication</p>
      <h2>Check your email</h2>
      <p class="otp-hint">
        Enter the 6-digit code sent to <strong>{{ login.email }}</strong>.
        It expires in 10 minutes.
      </p>

      <v-otp-input
        v-model="otpCode"
        length="6"
        variant="outlined"
        :error="otpError"
        :loading="verifying"
        autofocus
        @finish="handleVerifyOtp"
      />

      <div class="modal-actions">
        <button class="btn-ghost" :disabled="verifying" @click="cancelOtp">Cancel</button>
        <button class="btn-primary" :disabled="verifying || otpCode.length < 6" @click="handleVerifyOtp">
          {{ verifying ? 'Verifying…' : 'Verify' }}
        </button>
      </div>
    </div>
  </v-dialog>

  <v-snackbar v-model="snackbar.show" :color="snackbar.color" :timeout="3000" location="bottom">
    {{ snackbar.message }}
  </v-snackbar>
</template>

<script setup lang="ts">
import { reactive, ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { LoginRequest } from './types/login-request'
import { RegisterRequest } from './types/register-request'

const route = useRoute()
const router = useRouter()

const view = computed(() => route.path === '/register' ? 'register' : 'login')

const login = reactive({ email: '', password: '' })
const register = reactive({ email: '', password: '', confirm: '' })

const showOtpDialog = ref(false)
const otpCode = ref('')
const otpError = ref(false)
const verifying = ref(false)

const snackbar = reactive({ show: false, message: '', color: 'success' as 'success' | 'error' })

function toast(message: string, color: 'success' | 'error' = 'success') {
  snackbar.message = message
  snackbar.color = color
  snackbar.show = true
}

function cancelOtp() {
  showOtpDialog.value = false
  otpCode.value = ''
  otpError.value = false
}

async function handleLogin() {
  const body: LoginRequest = new LoginRequest(login.email, login.password)
  try {
    const res = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify(body),
    })

    if (res.status === 401) { toast('Invalid email or password', 'error'); return }
    if (!res.ok) { toast('Sign in failed', 'error'); return }

    const data = await res.json()
    if (data.otp_required) {
      otpCode.value = ''
      otpError.value = false
      showOtpDialog.value = true
    } else {
      localStorage.setItem('logged_in', '1')
      toast('Signed in!')
      router.push('/')
    }
  } catch {
    toast('Network error', 'error')
  }
}

async function handleVerifyOtp() {
  if (otpCode.value.length < 6 || verifying.value) return
  verifying.value = true
  otpError.value = false

  try {
    const res = await fetch('/api/auth/verify-otp', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ email: login.email, code: otpCode.value }),
    })

    if (res.status === 401) {
      otpError.value = true
      otpCode.value = ''
      toast('Invalid or expired code', 'error')
      return
    }
    if (!res.ok) { toast('Verification failed', 'error'); return }

    showOtpDialog.value = false
    localStorage.setItem('logged_in', '1')
    toast('Signed in!')
    router.push('/')
  } catch {
    toast('Network error', 'error')
  } finally {
    verifying.value = false
  }
}

function handleRegister() {
  if (register.password !== register.confirm) {
    toast('Passwords do not match', 'error')
    return
  }
  const body: RegisterRequest = new RegisterRequest(register.email, register.password)
  fetch('/api/auth/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
    .then(response => {
      if (response.ok) {
        toast('Account created! You can now sign in.')
        router.push('/login')
      } else if (response.status === 409) {
        toast('Email already registered', 'error')
      } else {
        toast('Registration failed', 'error')
      }
    })
    .catch(() => toast('Network error', 'error'))
}
</script>

<style scoped>
.auth-wrapper {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg);
  padding: 2rem;
}

.auth-card {
  width: 100%;
  max-width: 420px;
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
  margin-bottom: 2rem;
}

.form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  margin-bottom: 1.5rem;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

label {
  font-size: 0.85rem;
  color: var(--text-muted);
  font-family: var(--mono);
}

input {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 0.65rem 0.9rem;
  color: var(--text);
  font-size: 0.95rem;
  font-family: inherit;
  transition: border-color 0.2s;
  outline: none;
}

input:focus {
  border-color: var(--accent);
}

input::placeholder {
  color: var(--text-muted);
  opacity: 0.6;
}

.btn-primary {
  padding: 0.75rem 1.75rem;
  background: var(--accent);
  color: #fff;
  border: none;
  border-radius: 6px;
  font-size: 0.95rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
  font-family: inherit;
  margin-top: 0.25rem;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.switch {
  text-align: center;
  font-size: 0.875rem;
  color: var(--text-muted);
}

.btn-link {
  background: none;
  border: none;
  color: var(--accent);
  font-size: 0.875rem;
  cursor: pointer;
  padding: 0;
  font-family: inherit;
  transition: color 0.2s;
}

.btn-link:hover {
  color: var(--accent-hover);
}

/* OTP modal */
.otp-modal {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 2rem;
}

.otp-modal h2 {
  font-size: 1.4rem;
  font-weight: 700;
  margin-bottom: 0.5rem;
  color: var(--text);
}

.otp-hint {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin-bottom: 1.5rem;
  line-height: 1.5;
}

.modal-actions {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
}

.btn-ghost {
  padding: 0.6rem 1.25rem;
  background: none;
  border: 1px solid var(--border);
  color: var(--text-muted);
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: border-color 0.2s, color 0.2s;
  font-family: inherit;
}

.btn-ghost:hover:not(:disabled) {
  border-color: var(--text-muted);
  color: var(--text);
}

.btn-ghost:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
