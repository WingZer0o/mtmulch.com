<template>
  <section id="hero">
    <div class="container">
      <p class="greeting">Hi, my name is</p>
      <h1 class="typewriter" style="--tw-chars: 15">Mike Mulchrone.</h1>
      <h2>I build fast, reliable software.</h2>
      <p class="bio">
        I'm a software engineer specialising in systems programming and web development,
        with a focus on performance and correctness.
      </p>
      <div class="actions">
        <a href="#projects" class="btn-primary">View my work</a>
        <a href="#contact" class="btn-ghost">Get in touch</a>
        <RouterLink to="/blog" class="btn-ghost">Read my blog</RouterLink>
      </div>
    </div>
  </section>
</template>

<script setup>
import { RouterLink } from 'vue-router'
</script>

<style scoped>
section {
  min-height: 100vh;
  display: flex;
  align-items: center;
  padding-top: 60px;
}

.greeting {
  font-family: var(--mono);
  color: var(--accent);
  font-size: 1rem;
  margin-bottom: 1rem;
}

h1 {
  font-size: clamp(2.5rem, 6vw, 4.5rem);
  font-weight: 700;
  line-height: 1.1;
  margin-bottom: 0.5rem;
}

h2 {
  font-size: clamp(1.5rem, 4vw, 3rem);
  font-weight: 700;
  color: var(--text-muted);
  line-height: 1.1;
  margin-bottom: 1.5rem;
}

.bio {
  max-width: 540px;
  color: var(--text-muted);
  margin-bottom: 2.5rem;
}

.actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.btn-primary {
  padding: 0.75rem 1.75rem;
  background: var(--accent);
  color: #fff;
  border-radius: 6px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: var(--accent-hover);
  color: #fff;
}

.btn-ghost {
  padding: 0.75rem 1.75rem;
  border: 1px solid var(--accent);
  color: var(--accent);
  border-radius: 6px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-ghost:hover {
  background: rgba(124, 106, 247, 0.1);
  color: var(--accent);
}

.ping-response {
  margin-top: 1rem;
  font-family: var(--mono);
  font-size: 0.875rem;
  color: var(--accent);
}
</style>
