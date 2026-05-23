import { createRouter, createWebHistory } from 'vue-router'
import Home from './pages/home/Home.vue'
import Auth from './pages/auth/Auth.vue'
import Settings from './pages/settings/Settings.vue'

export default createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/login', component: Auth },
    { path: '/register', component: Auth },
    { path: '/settings', component: Settings },
  ]
})
