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
import { setProfileImage } from './components/subjects/profile-image-subject'

const route = useRoute();
const isFullWidth = computed(() => ['/', '/admin'].includes(route.path));

const theme = useTheme();
onMounted(async () => {
  const savedTheme = localStorage.getItem('theme');
  theme.change(savedTheme ?? 'dark');

  if (localStorage.getItem('logged_in') === '1') {
    try {
      const res = await fetch('/api/settings', { credentials: 'include' });
      if (res.ok) {
        const data = await res.json();
        setProfileImage(data.profile_image ?? null);
      }
    } catch { /* non-critical */ }
  }
})
</script>

<style scoped>
.full-width-page {
  width: 100%;
  max-width: 100%;
  padding: 0;
}
</style>
