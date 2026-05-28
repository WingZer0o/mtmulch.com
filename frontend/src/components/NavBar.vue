<template>
  <v-app-bar>
    <template v-slot:prepend>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>
    </template>
    <v-app-bar-title class="logo">mm</v-app-bar-title>
    <template v-slot:append>
      <v-btn
        :icon="isDark ? 'mdi-weather-sunny' : 'mdi-weather-night'"
        variant="text"
        @click="toggleTheme"
      />
    </template>
  </v-app-bar>

  <v-navigation-drawer v-model="drawer" temporary>
    <v-list nav>
      <v-list-item v-if="isLoggedIn" class="user-avatar-item">
        <div class="avatar-wrap">
          <v-avatar size="48" class="nav-avatar">
            <v-img v-if="profileImage" :src="profileImage" cover />
            <span v-else class="avatar-initial">{{ userInitial }}</span>
          </v-avatar>
        </div>
      </v-list-item>
      <v-list-item prepend-icon="mdi-home-outline" title="Home" @click="navigate('/')"></v-list-item>
      <v-list-item prepend-icon="mdi-post-outline" title="Blog" @click="navigate('/blog')"></v-list-item>
      <template v-if="isLoggedIn">
        <v-list-item prepend-icon="mdi-cog-outline" title="Settings" @click="navigate('/settings')"></v-list-item>
        <v-list-item v-if="isAdmin" prepend-icon="mdi-shield-account-outline" title="Admin" @click="navigate('/admin')"></v-list-item>
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
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useSubject } from '@vueuse/rxjs';
import { useRoute, useRouter } from 'vue-router'
import { useTheme } from 'vuetify'
import { logOut } from '../api';
import { profileImageSubject } from './subjects/profile-image-subject';

const theme = useTheme()
const isDark = computed(() => theme.global.name.value === 'dark')

function toggleTheme() {
  const next = isDark.value ? 'light' : 'dark'
  theme.change(next);
  localStorage.setItem('theme', next)
}

const route = useRoute()
const router = useRouter()
const drawer = ref(false)

const isLoggedIn = ref(localStorage.getItem('logged_in') === '1')
const isAdmin = ref(getIsAdminCookie())
const profileImage = useSubject(profileImageSubject);

function getIsAdminCookie() {
  const match = document.cookie.split('; ').find(row => row.startsWith('is_admin='))
  return match ? match.split('=')[1] === 'true' : false
}

const userInitial = computed(() => {
  return isLoggedIn.value ? 'U' : ''
})

let imageSub = null
onMounted(() => {
  imageSub = profileImageSubject.subscribe(url => {
    profileImage.value = url
  })
})
onUnmounted(() => {
  imageSub?.unsubscribe()
})

watch(route, () => {
  isLoggedIn.value = localStorage.getItem('logged_in') === '1'
  isAdmin.value = getIsAdminCookie()
})

function navigate(path) {
  drawer.value = false
  router.push(path)
}

async function handleLogout() {
  drawer.value = false
  await logOut();
  isLoggedIn.value = false
  isAdmin.value = false
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

.user-avatar-item {
  padding: 1rem 1rem 0.5rem;
}

.avatar-wrap {
  display: flex;
  justify-content: center;
}

.nav-avatar {
  background: var(--accent);
  color: #fff;
  font-family: var(--mono);
  font-weight: 600;
  font-size: 1.1rem;
}

.avatar-initial {
  font-family: var(--mono);
  font-weight: 600;
  font-size: 1.1rem;
  color: #fff;
}
</style>
