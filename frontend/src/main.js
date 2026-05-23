import { createApp } from 'vue'
import router from './router'
import App from './App.vue'

// Vuetify
import { createVuetify } from 'vuetify'
import { VApp, VAppBar, VAppBarNavIcon, VAppBarTitle, VList, VListItem, VNavigationDrawer, VSnackbar, VDialog, VOtpInput, VBtn, VProgressCircular, VContainer, VTextField, VSwitch, VSpacer } from 'vuetify/components'
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
  components: { VApp, VAppBar, VAppBarNavIcon, VAppBarTitle, VList, VListItem, VNavigationDrawer, VSnackbar, VDialog, VOtpInput, VBtn, VProgressCircular, VContainer, VTextField, VSwitch, VSpacer },
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
})

createApp(App).use(vuetify).use(router).mount('#app');