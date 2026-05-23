<template>
  <v-container class="post-container" max-width="800">
    <div v-if="loading" class="loading-wrap">
      <v-progress-circular indeterminate color="var(--accent)" />
    </div>

    <div v-else-if="error" class="error-msg">{{ error }}</div>

    <article v-else class="post-article">
      <p class="back-link" @click="router.push('/blog')">← Back to blog</p>
      <h1 class="post-title">{{ post.title }}</h1>
      <p class="post-date">{{ formatDate(post.created_at) }}</p>
      <div class="post-content" v-html="post.content"></div>
    </article>
  </v-container>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const post = ref(null)
const loading = ref(true)
const error = ref(null)

onMounted(async () => {
  try {
    const res = await fetch(`/api/blog/posts/${route.params.slug}`)
    if (res.status === 404) throw new Error('Post not found')
    if (!res.ok) throw new Error('Failed to load post')
    post.value = await res.json()
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
  }
})

function formatDate(iso) {
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}
</script>

<style scoped>
.post-container {
  padding-top: 2rem;
  padding-bottom: 4rem;
}

.loading-wrap {
  display: flex;
  justify-content: center;
  padding: 3rem 0;
}

.error-msg {
  color: var(--text-muted);
  text-align: center;
  padding: 3rem 0;
}

.back-link {
  color: var(--accent);
  cursor: pointer;
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
}

.back-link:hover {
  color: var(--accent-hover);
}

.post-title {
  font-size: 2rem;
  color: var(--text);
  margin: 0 0 0.5rem;
}

.post-date {
  color: var(--text-muted);
  font-size: 0.875rem;
  margin: 0 0 2rem;
}

.post-content :deep(h1),
.post-content :deep(h2),
.post-content :deep(h3) {
  color: var(--text);
  margin-top: 1.5rem;
}

.post-content :deep(p) {
  color: var(--text);
  line-height: 1.7;
  margin-bottom: 1rem;
}

.post-content :deep(a) {
  color: var(--accent);
}

.post-content :deep(code) {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 0.1em 0.4em;
  font-family: var(--mono);
  font-size: 0.9em;
}

.post-content :deep(pre) {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1rem;
  overflow-x: auto;
}

.post-content :deep(blockquote) {
  border-left: 3px solid var(--accent);
  padding-left: 1rem;
  color: var(--text-muted);
  margin: 1rem 0;
}
</style>
