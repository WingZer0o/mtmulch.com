<template>
  <v-app>
    <NavBar />
    <main :class="isFullWidth ? 'full-width-page' : 'container'">
      <RouterView />
    </main>
    <SnackBar />
  </v-app>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useTheme } from 'vuetify'
import NavBar from './components/NavBar.vue'
import SnackBar from './components/SnackBar.vue'

const route = useRoute();
const isFullWidth = computed(() => ['/', '/admin'].includes(route.path));

const theme = useTheme();
onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  theme.change(savedTheme ?? 'dark');
})
</script>

<style scoped>
.full-width-page {
  width: 100%;
  max-width: 100%;
  padding: 0;
}
</style>
