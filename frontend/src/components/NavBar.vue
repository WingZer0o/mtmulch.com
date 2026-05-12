<template>
  <nav>
    <div class="container nav-inner">
      <span class="logo">mm</span>
      <ul>
        <li><a href="#about">About</a></li>
        <li><a href="#projects">Projects</a></li>
        <li><a href="#contact">Contact</a></li>
      </ul>
    </div>
  </nav>
</template>

<style scoped>
nav {
  position: fixed;
  top: 0;
  width: 100%;
  z-index: 100;
  backdrop-filter: blur(12px);
  background: rgba(15, 17, 23, 0.8);
  border-bottom: 1px solid var(--border);
}

.nav-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 60px;
}

.logo {
  font-family: var(--mono);
  font-weight: 500;
  font-size: 1.1rem;
  color: var(--accent);
}

ul {
  list-style: none;
  display: flex;
  gap: 2rem;
}

a {
  font-size: 0.9rem;
  color: var(--text-muted);
  transition: color 0.2s;
}

a:hover {
  color: var(--text);
}
</style>
