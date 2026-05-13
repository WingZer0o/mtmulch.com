<template>
  <Hero />
  <About />
  <Projects />
  <Contact />
</template>

<script setup>
import Hero from './Hero.vue'
import About from './About.vue'
import Projects from './Projects.vue'
import Contact from './Contact.vue'

fetch('/api/hello')
  .then((response) => response.json())
  .then((data) => {
    console.log(data)
  })
  .catch((error) => {
    console.error('Error fetching data:', error)
  })
</script>
