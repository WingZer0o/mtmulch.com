<template>
  <v-app>
    <NavBar />
    <main class="container">
      <RouterView />
    </main>
  </v-app>
</template>

<script setup>
import NavBar from './components/NavBar.vue'
</script>
