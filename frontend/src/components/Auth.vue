<template>
  <div class="auth-wrapper">
    <div class="auth-card">

      <div v-if="mode === 'login'">
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
          <button class="btn-link" @click="mode = 'register'">Register</button>
        </p>
      </div>

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
          <button class="btn-link" @click="mode = 'login'">Sign in</button>
        </p>
      </div>

    </div>
  </div>
</template>

<script setup>
import { ref, reactive, watch } from 'vue'

const props = defineProps({
  initialMode: { type: String, default: 'login' }
})

const mode = ref(props.initialMode)

watch(() => props.initialMode, val => { mode.value = val })

const login = reactive({ email: '', password: '' })
const register = reactive({ email: '', password: '', confirm: '' })

function handleLogin() {
  // TODO: wire up login API call
}

function handleRegister() {
  // TODO: wire up register API call
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

.btn-primary:hover {
  background: var(--accent-hover);
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
</style>
