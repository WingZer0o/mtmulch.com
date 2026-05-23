<template>
  <v-app>
    <NavBar />
    <main :class="isFullWidth ? 'full-width-page' : 'container'">
      <RouterView />
    </main>
  </v-app>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import NavBar from './components/NavBar.vue'

const route = useRoute()
const isFullWidth = computed(() => route.path === '/admin')
</script>

<style scoped>
.full-width-page {
  width: 100%;
  padding: 0;
}
</style>
