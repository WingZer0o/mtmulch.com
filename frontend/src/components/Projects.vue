<template>
  <section id="projects">
    <div class="container">
      <h2 class="section-title">Projects</h2>
      <div class="grid">
        <div v-for="project in projects" :key="project.name" class="card">
          <div class="card-header">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3h18v18H3z"/><path d="M3 9h18M9 21V9"/></svg>
            <div class="links">
              <a v-if="project.github" :href="project.github" target="_blank" rel="noopener" title="GitHub">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/></svg>
              </a>
            </div>
          </div>
          <h3>{{ project.name }}</h3>
          <p>{{ project.description }}</p>
          <ul class="tags">
            <li v-for="tag in project.tags" :key="tag">{{ tag }}</li>
          </ul>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
const projects = [
  {
    name: 'Rust Playground API',
    description: 'A high-performance REST API built with Actix-web and Diesel ORM, backed by PostgreSQL and deployed to Azure Container Apps.',
    tags: ['Rust', 'Actix-web', 'PostgreSQL', 'Azure'],
    github: 'https://github.com/WingZer0o/rust-playground',
  },
  {
    name: 'Project Two',
    description: 'A short description of what this project does and the problem it solves.',
    tags: ['Vue.js', 'TypeScript'],
    github: null,
  },
  {
    name: 'Project Three',
    description: 'A short description of what this project does and the problem it solves.',
    tags: ['Rust', 'WebAssembly'],
    github: null,
  },
]
</script>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
}

.card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  transition: border-color 0.2s, transform 0.2s;
}

.card:hover {
  border-color: var(--accent);
  transform: translateY(-4px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: var(--accent);
}

.links {
  display: flex;
  gap: 0.75rem;
  color: var(--text-muted);
}

.links a {
  color: var(--text-muted);
  transition: color 0.2s;
}

.links a:hover {
  color: var(--text);
}

h3 {
  font-size: 1.05rem;
  font-weight: 600;
}

p {
  font-size: 0.9rem;
  color: var(--text-muted);
  flex: 1;
}

.tags {
  list-style: none;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

.tags li {
  font-family: var(--mono);
  font-size: 0.75rem;
  color: var(--accent);
  background: rgba(124, 106, 247, 0.1);
  padding: 0.2rem 0.6rem;
  border-radius: 4px;
}
</style>
