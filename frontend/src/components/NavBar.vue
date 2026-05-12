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
      <v-list-item prepend-icon="mdi-login" title="Login" @click="navigate('/login')"></v-list-item>
      <v-list-item prepend-icon="mdi-account-plus-outline" title="Register" @click="navigate('/register')"></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const drawer = ref(false)

function navigate(path) {
  drawer.value = false
  router.push(path)
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
