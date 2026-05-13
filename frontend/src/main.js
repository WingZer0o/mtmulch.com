import { createApp } from 'vue'
import router from './router'
import App from './App.vue'

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import { VApp, VAppBar, VAppBarNavIcon, VAppBarTitle, VList, VListItem, VNavigationDrawer, VSnackbar } from 'vuetify/components'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

// Styles
import './assets/main.css'
import '@mdi/font/css/materialdesignicons.css'
import '@fontsource/roboto/300.css'
import '@fontsource/roboto/400.css'
import '@fontsource/roboto/500.css'
import '@fontsource/roboto/700.css'

const vuetify = createVuetify({
  components: { VApp, VAppBar, VAppBarNavIcon, VAppBarTitle, VList, VListItem, VNavigationDrawer, VSnackbar },
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
})

createApp(App).use(vuetify).use(router).mount('#app')
