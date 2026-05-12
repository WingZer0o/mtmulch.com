import { createRouter, createWebHistory } from 'vue-router'
import Home from './components/Home.vue'
import Auth from './components/Auth.vue'

export default createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/login', component: Auth },
    { path: '/register', component: Auth },
  ]
})
