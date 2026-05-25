<template>
  <v-snackbar v-model="snackBarSubject" :color="snackBarSubject?.color" :timeout="3000" location="bottom">
    <div class="text-center w-100">{{ snackBarSubject?.message }}</div>
  </v-snackbar>
</template>

<script setup lang="ts">
import { useSubject } from '@vueuse/rxjs';
import { useSnackBarSubject } from './subjects/snack-bar-subject';
const snackBarSubject = useSubject(useSnackBarSubject);
</script>