<template>
  <section id="contact">
    <div class="container">
      <h2 class="section-title">Contact</h2>
      <div class="content">
        <p>
          Have a project in mind or just want to say hello? My inbox is always open.
        </p>
        <a href="mailto:mikemulchrone987@gmail.com" class="btn-primary">Say Hello</a>
        <div class="socials">
          <a href="https://github.com/WingZer0o" target="_blank" rel="noopener">GitHub</a>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
section {
  padding-bottom: 8rem;
}

.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 2rem;
  max-width: 500px;
  margin: 0 auto;
}

p {
  color: var(--text-muted);
}

.btn-primary {
  padding: 1rem 2rem;
  border: 1px solid var(--accent);
  color: var(--accent);
  border-radius: 6px;
  font-size: 0.95rem;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: rgba(124, 106, 247, 0.1);
  color: var(--accent);
}

.socials {
  display: flex;
  gap: 2rem;
}

.socials a {
  font-family: var(--mono);
  font-size: 0.85rem;
  color: var(--text-muted);
  transition: color 0.2s;
}

.socials a:hover {
  color: var(--text);
}
</style>
