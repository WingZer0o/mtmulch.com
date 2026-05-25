import { createApp } from 'vue'
import router from './router'
import App from './App.vue'

// Vuetify
import { createVuetify } from 'vuetify'
import { VApp, VRow, VCol, VTextarea, VAppBar, VForm, VAppBarNavIcon, VAppBarTitle, VList, VListItem, VNavigationDrawer, VSnackbar, VDialog, VOtpInput, VBtn, VProgressCircular, VContainer, VTextField, VSwitch, VSpacer } from 'vuetify/components'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

// Styles
import 'vuetify/styles'
import './assets/main.css'
import '@mdi/font/css/materialdesignicons.css'
import '@fontsource/roboto/300.css'
import '@fontsource/roboto/400.css'
import '@fontsource/roboto/500.css'
import '@fontsource/roboto/700.css'

const vuetify = createVuetify({
  components: { VApp, VRow, VCol, VTextarea, VForm, VAppBar, VAppBarNavIcon, VAppBarTitle, VList, VListItem, VNavigationDrawer, VSnackbar, VDialog, VOtpInput, VBtn, VProgressCircular, VContainer, VTextField, VSwitch, VSpacer },
  defaults: {
    VBtn: {
      color: 'primary',
      variant: 'flat',
    },
  },
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
  theme: {
    defaultTheme: 'light',
    themes: {
      light: {
        dark: false,
        colors: {
          background: '#ffffff',
          surface: '#f5f6fa',
          primary: '#7c6af7',
          'primary-darken-1': '#5a4fd4',
          secondary: '#64748b',
          error: '#ef4444',
          info: '#3b82f6',
          success: '#22c55e',
          warning: '#f59e0b',
        },
      },
      dark: {
        dark: true,
        colors: {
          background: '#0f1117',
          surface: '#1a1d27',
          primary: '#7c6af7',
          'primary-darken-1': '#9585f8',
          secondary: '#8892a4',
          error: '#ef4444',
          info: '#3b82f6',
          success: '#22c55e',
          warning: '#f59e0b',
        },
      },
    },
  },
})

createApp(App).use(vuetify).use(router).mount('#app');