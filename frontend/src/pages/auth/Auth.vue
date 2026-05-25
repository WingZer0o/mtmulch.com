<template>
  <div class="auth-wrapper">
    <div class="auth-card">

      <!-- Email verified banner -->
      <div v-if="verifiedBanner" class="verified-banner">
        Email verified — you can now sign in.
      </div>

      <!-- Account unlocked banner -->
      <div v-if="unlockedBanner" class="unlocked-banner">
        Account unlocked — you can now sign in.
      </div>

      <!-- Login -->
      <div v-if="view === 'login'">
        <p class="label">Welcome back</p>
        <h1>Sign in</h1>

        <v-form @submit.prevent="handleLogin" class="form">
          <div class="field">
            <label for="login-email">Email</label>
            <v-text-field
              id="login-email"
              v-model="login.email"
              type="email"
              variant="outlined"
              density="compact"
              placeholder="you@example.com"
              hide-details
              required
            />
          </div>
          <div class="field">
            <label for="login-password">Password</label>
            <v-text-field
              id="login-password"
              v-model="login.password"
              type="password"
              variant="outlined"
              density="compact"
              placeholder="••••••••"
              hide-details
              required
            />
          </div>
          <button type="submit" class="btn-primary">Sign in</button>
        </v-form>

        <p class="switch">
          Don't have an account?
          <button class="btn-link" @click="router.push('/register')">Register</button>
        </p>
      </div>

      <!-- Register -->
      <div v-else>
        <p class="label">Get started</p>
        <h1>Create account</h1>

        <v-form @submit.prevent="handleRegister" class="form">
          <div class="field">
            <label for="reg-email">Email</label>
            <v-text-field
              id="reg-email"
              v-model="register.email"
              type="email"
              variant="outlined"
              density="compact"
              placeholder="you@example.com"
              hide-details
              required
            />
          </div>
          <div class="field">
            <label for="reg-password">Password</label>
            <v-text-field
              id="reg-password"
              v-model="register.password"
              type="password"
              variant="outlined"
              density="compact"
              placeholder="••••••••"
              hide-details
              required
            />
          </div>
          <div class="field">
            <label for="reg-confirm">Confirm password</label>
            <v-text-field
              id="reg-confirm"
              v-model="register.confirm"
              type="password"
              variant="outlined"
              density="compact"
              placeholder="••••••••"
              hide-details
              required
            />
          </div>
          <button type="submit" class="btn-primary">Create account</button>
        </v-form>

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
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { LoginRequest } from './types/login-request';
import { RegisterRequest } from './types/register-request';
import { showSnackBar } from '../../components/subjects/snack-bar-subject';

const route = useRoute();
const router = useRouter();

const view = computed(() => route.path === '/register' ? 'register' : 'login');
const verifiedBanner = ref(false);
const unlockedBanner = ref(false);

onMounted(() => {
  if (route.query.verified === '1') verifiedBanner.value = true;
  if (route.query.unlocked === '1') unlockedBanner.value = true;
})

const login = reactive({ email: '', password: '' });
const register = reactive({ email: '', password: '', confirm: '' });

const showOtpDialog = ref(false);
const otpCode = ref('');
const otpError = ref(false);
const verifying = ref(false);

function cancelOtp() {
  showOtpDialog.value = false;
  otpCode.value = '';
  otpError.value = false;
}

async function handleLogin() {
  const body: LoginRequest = new LoginRequest(login.email, login.password);
  try {
    const res = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify(body),
    });

    if (res.status === 401) { 
      showSnackBar('Invalid email or password.', 'error'); 
      return; 
    }
    if (res.status === 403) {
      const msg = await res.text();
      showSnackBar(msg.toLowerCase().includes('locked')
        ? 'Account locked. Check your email for an unlock link.'
        : 'Please verify your email before signing in.',
        'error')
      return;
    }
    if (!res.ok) { 
      showSnackBar('Sign in failed.', 'error'); 
      return;
    }

    const data = await res.json();
    if (data.otp_required) {
      otpCode.value = '';
      otpError.value = false;
      showOtpDialog.value = true;
    } else {
      localStorage.setItem('logged_in', '1');
      showSnackBar('Signed in!');
      router.push('/');
    }
  } catch {
    showSnackBar('Network error', 'error');
  }
}

async function handleVerifyOtp() {
  if (otpCode.value.length < 6 || verifying.value) 
    return;
  verifying.value = true;
  otpError.value = false;

  try {
    const res = await fetch('/api/auth/verify-otp', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ email: login.email, code: otpCode.value }),
    });

    if (res.status === 401) {
      otpError.value = true;
      otpCode.value = '';
      showSnackBar('Invalid or expired code', 'error');
      return;
    }
    if (!res.ok) { showSnackBar('Verification failed', 'error'); return; }

    showOtpDialog.value = false;
    localStorage.setItem('logged_in', '1');
    showSnackBar('Signed in!');
    router.push('/');
  } catch {
    showSnackBar('Network error', 'error');
  } finally {
    verifying.value = false;
  }
}

function handleRegister() {
  if (register.password !== register.confirm) {
    showSnackBar('Passwords do not match', 'error');
    return;
  }
  const body: RegisterRequest = new RegisterRequest(register.email, register.password)
  fetch('/api/auth/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
    .then(response => {
      if (response.ok) {
        showSnackBar('Account created! Check your email to verify your address.')
        router.push('/login')
      } else if (response.status === 409) {
        showSnackBar('Email already registered', 'error')
      } else {
        showSnackBar('Registration failed', 'error')
      }
    })
    .catch(() => showSnackBar('Network error', 'error'))
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

.verified-banner {
  background: #14532d;
  border: 1px solid #166534;
  color: #bbf7d0;
  border-radius: 6px;
  padding: 0.65rem 1rem;
  font-size: 0.875rem;
  margin-bottom: 1.5rem;
}

.unlocked-banner {
  background: #1e3a5f;
  border: 1px solid #1d4ed8;
  color: #bfdbfe;
  border-radius: 6px;
  padding: 0.65rem 1rem;
  font-size: 0.875rem;
  margin-bottom: 1.5rem;
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

.btn-primary {
  border: none;
  cursor: pointer;
  font-family: inherit;
  font-size: 0.95rem;
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
