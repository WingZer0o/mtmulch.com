<template>
  <section id="about">
    <div class="container">
      <h2 class="section-title">About Me</h2>
      <div class="content">
        <div class="bio">
          <p>
            I'm a software engineer with a passion for building robust, high-performance
            applications. I enjoy working close to the metal — whether that's systems
            programming in Rust or designing efficient backend APIs.
          </p>
          <p>
            When I'm not writing code I'm exploring new technologies, contributing to
            open-source projects, and constantly looking for ways to make software faster
            and more reliable.
          </p>
        </div>
        <div class="skills">
          <h3>Technologies</h3>
          <ul>
            <li v-for="skill in skills" :key="skill">
              <span class="bullet">▸</span> {{ skill }}
            </li>
          </ul>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
const skills = [
  'Rust', 'Actix-web', 'Diesel',
  'PostgreSQL', 'Docker', 'Vue.js',
  'GitHub Actions', 'Azure',
]
</script>

<style scoped>
.content {
  display: grid;
  grid-template-columns: 3fr 2fr;
  gap: 4rem;
}

.bio {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  color: var(--text-muted);
}

h3 {
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 1rem;
  color: var(--text);
}

ul {
  list-style: none;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
}

li {
  font-family: var(--mono);
  font-size: 0.85rem;
  color: var(--text-muted);
}

.bullet {
  color: var(--accent);
  margin-right: 0.4rem;
}

@media (max-width: 640px) {
  .content {
    grid-template-columns: 1fr;
    gap: 2rem;
  }
}
</style>
