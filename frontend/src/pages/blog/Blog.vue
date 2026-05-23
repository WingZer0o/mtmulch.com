<template>
  <v-container class="blog-container" max-width="900">
    <h1 class="blog-title">Blog</h1>

    <div v-if="loading" class="loading-wrap">
      <v-progress-circular indeterminate color="var(--accent)" />
    </div>

    <div v-else-if="error" class="error-msg">{{ error }}</div>

    <div v-else-if="posts.length === 0" class="empty-msg">No posts yet.</div>

    <div v-else class="posts-list">
      <div
        v-for="post in posts"
        :key="post.id"
        class="post-card"
        @click="router.push(`/blog/${post.slug}`)"
      >
        <h2 class="post-title">{{ post.title }}</h2>
        <p class="post-date">{{ formatDate(post.created_at) }}</p>
      </div>
    </div>
  </v-container>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const posts = ref([])
const loading = ref(true)
const error = ref(null)

onMounted(async () => {
  try {
    const res = await fetch('/api/blog/posts')
    if (!res.ok) throw new Error('Failed to load posts')
    posts.value = await res.json()
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
.blog-container {
  padding-top: 5rem;
  padding-bottom: 4rem;
}

.blog-title {
  font-family: var(--mono);
  font-size: 2rem;
  color: var(--accent);
  margin-bottom: 2rem;
}

.loading-wrap {
  display: flex;
  justify-content: center;
  padding: 3rem 0;
}

.empty-msg,
.error-msg {
  color: var(--text-muted);
  text-align: center;
  padding: 3rem 0;
}

.posts-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.post-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1.5rem;
  cursor: pointer;
  transition: border-color 0.2s;
}

.post-card:hover {
  border-color: var(--accent);
}

.post-title {
  font-size: 1.25rem;
  color: var(--text);
  margin: 0 0 0.5rem;
}

.post-date {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}
</style>
