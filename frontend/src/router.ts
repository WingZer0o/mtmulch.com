import { createRouter, createWebHistory } from 'vue-router'
import { logOut } from './api'

const Home = () => import('./pages/home/Home.vue')
const Auth = () => import('./pages/auth/Auth.vue')
const Settings = () => import('./pages/settings/Settings.vue')
const Blog = () => import('./pages/blog/Blog.vue')
const BlogPost = () => import('./pages/blog/BlogPost.vue')
const Admin = () => import('./pages/admin/Admin.vue')
const Recordings = () => import('./pages/recordings/Recordings.vue')
const Tuner = () => import('./pages/tuner/Tuner.vue')

function getIsAdminCookie(): boolean {
  const match = document.cookie.split('; ').find(row => row.startsWith('is_admin='))
  return match ? match.split('=')[1] === 'true' : false
}

export default createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/tuner', component: Tuner },
    { path: '/login', component: Auth },
    { path: '/register', component: Auth },
    { path: '/blog', component: Blog },
    { path: '/blog/:slug', component: BlogPost },
    {
      path: '/settings',
      component: Settings,
      beforeEnter: async () => {
        if (localStorage.getItem('logged_in') !== '1') {
          await logOut();
          return '/login'
        }
      },
    },
    {
      path: '/recordings',
      component: Recordings,
      beforeEnter: async () => {
        if (localStorage.getItem('logged_in') !== '1') {
          await logOut();
          return '/login'
        }
      },
    },
    {
      path: '/admin',
      component: Admin,
      beforeEnter: async () => {
        if (localStorage.getItem('logged_in') !== '1') {
          await logOut();
          return '/login'
        } 
        if (!getIsAdminCookie()) {
          await logOut();
          return '/'
        }
      },
    },
  ]
})
