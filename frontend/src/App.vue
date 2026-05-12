<template>
  <v-app>
    <main>
      <Hero />
      <About />
      <Projects />
      <Contact />
    </main>
    <NavBar />
  </v-app>
</template>

<script setup>
import { ref } from 'vue'
import NavBar from './components/NavBar.vue'
import Hero from './components/Hero.vue'
import About from './components/About.vue'
import Projects from './components/Projects.vue'
import Contact from './components/Contact.vue'

const response = ref('')
const isError = ref(false)

async function ping() {
  try {
    const res = await fetch('/api/hello')
    response.value = await res.text()
    isError.value = !res.ok
  } catch (e) {
    response.value = e.message
    isError.value = true
  }
}
</script>

<style>
.api-test {
  position: fixed;
  bottom: 1.5rem;
  right: 1.5rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  background: var(--bg-card);
  border: 1px solid var(--border);
  padding: 0.75rem 1rem;
  border-radius: 8px;
  font-family: var(--mono);
  font-size: 0.8rem;
  z-index: 200;
}

.api-test button {
  background: var(--accent);
  color: #fff;
  border: none;
  padding: 0.4rem 0.9rem;
  border-radius: 4px;
  cursor: pointer;
  font-family: var(--mono);
  font-size: 0.8rem;
}

.api-test button:hover {
  background: var(--accent-hover);
}

.api-test span {
  color: var(--text-muted);
}

.api-test span.error {
  color: #f87171;
}
</style>
