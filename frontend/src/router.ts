import { createRouter, createWebHistory } from 'vue-router'
import Home from './pages/home/Home.vue'
import Auth from './pages/auth/Auth.vue'
import Settings from './pages/settings/Settings.vue'
import Blog from './pages/blog/Blog.vue'
import BlogPost from './pages/blog/BlogPost.vue'
import Admin from './pages/admin/Admin.vue'

function getIsAdminCookie(): boolean {
  const match = document.cookie.split('; ').find(row => row.startsWith('is_admin='))
  return match ? match.split('=')[1] === 'true' : false
}

export default createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/login', component: Auth },
    { path: '/register', component: Auth },
    { path: '/blog', component: Blog },
    { path: '/blog/:slug', component: BlogPost },
    {
      path: '/settings',
      component: Settings,
      beforeEnter: () => {
        if (localStorage.getItem('logged_in') !== '1') return '/login'
      },
    },
    {
      path: '/admin',
      component: Admin,
      beforeEnter: () => {
        if (localStorage.getItem('logged_in') !== '1') return '/login'
        if (!getIsAdminCookie()) return '/'
      },
    },
  ]
})
