<template>
  <v-app-bar>
    <template v-slot:prepend>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>
    </template>
    <v-app-bar-title class="logo">mm</v-app-bar-title>
  </v-app-bar>

  <v-navigation-drawer v-model="drawer" temporary>
    <v-list nav>
      <v-list-item prepend-icon="mdi-home-outline" title="Home" @click="navigate('/')"></v-list-item>
      <template v-if="isLoggedIn">
        <v-list-item prepend-icon="mdi-logout" title="Logout" @click="handleLogout"></v-list-item>
      </template>
      <template v-else>
        <v-list-item prepend-icon="mdi-login" title="Login" @click="navigate('/login')"></v-list-item>
        <v-list-item prepend-icon="mdi-account-plus-outline" title="Register" @click="navigate('/register')"></v-list-item>
      </template>
    </v-list>
  </v-navigation-drawer>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const drawer = ref(false)

const isLoggedIn = ref(localStorage.getItem('logged_in') === '1')

watch(route, () => {
  isLoggedIn.value = localStorage.getItem('logged_in') === '1'
})

function navigate(path) {
  drawer.value = false
  router.push(path)
}

async function handleLogout() {
  drawer.value = false
  await fetch('/api/auth/logout', { method: 'POST', credentials: 'include' })
  localStorage.removeItem('logged_in')
  isLoggedIn.value = false
  router.push('/')
}
</script>

<style scoped>
.logo {
  font-family: var(--mono);
  font-weight: 500;
  font-size: 1.2rem;
  color: var(--accent);
}
</style>
