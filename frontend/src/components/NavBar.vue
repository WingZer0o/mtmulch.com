<template>
  <v-app-bar>
    <template v-slot:prepend>
      <v-app-bar-nav-icon></v-app-bar-nav-icon>
    </template>
    
    <v-app-bar-title class="logo">mm</v-app-bar-title>
  </v-app-bar>
</template>

<style scoped>
.logo {
  font-family: var(--mono);
  font-weight: 500;
  font-size: 1.2rem;
  color: var(--accent);
}
</style>
