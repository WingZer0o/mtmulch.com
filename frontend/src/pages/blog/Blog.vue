<template>
  <v-container class="blog-container" max-width="900">
    <div>
      <h1 class="text-center blog-title">Blog</h1>
      <p class="blog-subscribe-text">
        Like my posts? You can subscribe to get updates when I publish new content.
        Also, you can unsubscribe at any time if you change your mind.
        You must provide proof of ownership of the email address to subscribe or unsubscribe,
        so no one can abuse the system to subscribe/unsubscribe other people's emails.
      </p>
      <BlogSubscriberForm />
    </div>

    <div v-if="loading" class="loading-wrap">
      <v-progress-circular indeterminate color="var(--accent)" />
    </div>

    <div v-else-if="error" class="error-msg">{{ error }}</div>

    <div v-else-if="posts.length === 0" class="empty-msg">No posts yet.</div>

    <div v-else class="posts-list">
      <v-row v-for="post in posts" :key="post.id" class="post-card" @click="router.push(`/blog/${post.slug}`)">
        <v-col col="4">
          <h2 class="post-title">{{ post.title }}</h2>
          <p><b>Views:</b> {{ post.views }}</p>
          <p class="post-date"><b>Created:</b> {{ formatUtcDateForDisplay(post.created_at) }}</p>
          <p class="post-date"><b>Updated:</b> {{ formatUtcDateForDisplay(post.updated_at) }}</p>
        </v-col>
      </v-row>
    </div>
  </v-container>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { PostSummary } from './interfaces/blog';
import { formatUtcDateForDisplay } from './../../helpers/utilities/date-time';
import BlogSubscriberForm from './components/BlogSubscriberForm.vue';
import { showSnackBar } from '../../components/subjects/snack-bar-subject';

const route = useRoute();
const router = useRouter();
const posts = ref<PostSummary[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
  if (route.query.subscribed === '1') {
    showSnackBar('Your subscription has been confirmed!', 'success');
    router.replace({ path: '/blog' });
  } else if (route.query.unsubscribed === '1') {
    showSnackBar('You have been unsubscribed.', 'success');
    router.replace({ path: '/blog' });
  }

  try {
    const res = await fetch('/api/blog/posts');
    if (!res.ok) throw new Error('Failed to load posts');
    posts.value = await res.json();
  } catch (e: any) {
    error.value = e.message;
  } finally {
    loading.value = false;
  }
})
</script>

<style scoped>
.blog-subscriber-form {
  padding-bottom: 2rem;
}

.blog-subscribe-text {
  padding-bottom: 2rem;
}

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
  padding-top: 1rem;
}

.post-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1.5rem;
  cursor: pointer;
  width: 100%;
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
