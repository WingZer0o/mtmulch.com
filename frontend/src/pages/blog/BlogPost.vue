<template>
  <v-container class="post-container" max-width="800">
    <div v-if="loading" class="loading-wrap">
      <v-progress-circular indeterminate color="var(--accent)" />
    </div>

    <div v-else-if="error" class="error-msg">{{ error }}</div>

    <template v-else>
      <article class="post-article">
        <p class="back-link" @click="router.push('/blog')">← Back to blog</p>
        <h1 class="post-title">{{ post.title }}</h1>
        <p class="post-date">{{ formatUtcDateForDisplay(post.created_at) }}</p>
        <div class="post-content" v-html="renderedContent"></div>
      </article>

      <section class="comments-section">
        <h2 class="comments-heading">Comments</h2>

        <div v-if="commentsLoading" class="loading-wrap">
          <v-progress-circular indeterminate color="var(--accent)" size="24" />
        </div>

        <div v-else-if="comments.length === 0 && !commentsLoading" class="no-comments">
          No comments yet. Be the first!
        </div>

        <div v-else class="comment-list">
          <div v-for="c in comments" :key="c.id" class="comment-card">
            <div class="comment-header">
              <div class="comment-avatar">
                <img v-if="commentImageMap[c.user_id]" :src="commentImageMap[c.user_id]" alt="" class="comment-avatar-img" />
                <div v-else class="comment-avatar-initial">{{ c.author_email.charAt(0).toUpperCase() }}</div>
              </div>
              <div class="comment-meta">
                <span class="comment-author">{{ maskEmail(c.author_email) }}</span>
                <span class="comment-date">{{ formatUtcDateForDisplay(c.created_at) }}</span>
              </div>
            </div>
            <p class="comment-body">{{ c.content }}</p>
          </div>
        </div>

        <div v-if="isLoggedIn" class="comment-form">
          <h3 class="form-heading">Leave a comment</h3>
          <div class="comment-form-author">
            <div class="comment-avatar">
              <img v-if="currentUserImage" :src="currentUserImage" alt="" class="comment-avatar-img" />
              <div v-else class="comment-avatar-initial">U</div>
            </div>
          </div>
          <v-textarea
            v-model="newComment"
            rows="5"
            placeholder="Write a comment..."
            :disabled="submitting"
            hide-details
            clearable
            class="comment-textarea"
          />
          <div class="form-actions">
            <v-btn
              variant="outlined"
              @click="submitComment"
              :loading="submitting"
              :disabled="!newComment.trim()"
            >
              Post Comment
            </v-btn>
          </div>
          <p v-if="submitError" class="submit-error">{{ submitError }}</p>
        </div>

        <div v-else class="login-prompt">
          <span>
            <a class="accent-link" @click="router.push('/login')">Log in</a>
            to leave a comment.
          </span>
        </div>
      </section>
    </template>
  </v-container>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, nextTick } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { marked } from 'marked'
import { useSubject } from '@vueuse/rxjs';
import hljs from 'highlight.js/lib/common'
import 'highlight.js/styles/github-dark-dimmed.css'
import { formatUtcDateForDisplay } from '../../helpers/utilities/date-time'
import { profileImageSubject } from '../../components/subjects/profile-image-subject'

marked.use({
  gfm: true,
  breaks: true
})

const route = useRoute();
const router = useRouter();

const post = ref(null);
const loading = ref(true);
const error = ref(null);

const comments = ref([]);
const commentsLoading = ref(true);
const commentImageMap = ref({});

const newComment = ref('');
const submitting = ref(false);
const submitError = ref(null);

const isLoggedIn = computed(() => localStorage.getItem('logged_in') === '1');
const renderedContent = computed(() => marked.parse(post.value?.content ?? ''));
const currentUserImage = useSubject(profileImageSubject);

onMounted(async () => {
  await loadPost();
});

async function fetchCommentImages(commentList) {
  const userIds = [...new Set(
    commentList.filter(c => c.has_profile_image).map(c => c.user_id)
  )];
  const results = await Promise.all(
    userIds.map(async id => {
      try {
        const res = await fetch(`/api/users/${id}/profile-image`);
        if (!res.ok) return null;
        const data = await res.json();
        return [id, data.profile_image];
      } catch {
        return null;
      }
    })
  );
  const map = {};
  for (const entry of results) {
    if (entry) map[entry[0]] = entry[1];
  }
  commentImageMap.value = map;
  commentsLoading.value = false;
}

async function loadPost() {
  try {
    loading.value = true;
    commentsLoading.value = true;
    const postRespose = fetch(`/api/blog/posts/${route.params.slug}`);
    const commentsResponse = fetch(`/api/blog/posts/${route.params.slug}/comments`);
    await Promise.all([postRespose, commentsResponse]).then(async ([postRespose, commentsResponse]) => {
      if (postRespose.status === 404) throw new Error('Post not found');
      if (!postRespose.ok) throw new Error('Failed to load post');
      post.value = await postRespose.json();
      if (commentsResponse.ok) {
        const commentList = await commentsResponse.json();
        comments.value = commentList;
        fetchCommentImages(commentList);
      }
    });
  } catch (e) {
    error.value = e.message;
  } finally {
    loading.value = false;
  }
  await nextTick();
  hljs.highlightAll();
}

async function loadComments() {
  try {
    commentsLoading.value = true;
    const res = await fetch(`/api/blog/posts/${route.params.slug}/comments`);
    if (res.ok) {
      const commentList = await res.json();
      comments.value = commentList;
      await fetchCommentImages(commentList);
    }
  } finally {
    commentsLoading.value = false;
  }
}

async function submitComment() {
  submitError.value = null;
  submitting.value = true;
  try {
    const res = await fetch(`/api/blog/posts/${route.params.slug}/comments`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ content: newComment.value.trim() }),
    });
    if (res.status === 401) {
      submitError.value = 'Session expired. Please log in again.';
      return;
    }
    if (!res.ok) {
      submitError.value = 'Failed to post comment. Please try again.';
      return;
    }
    newComment.value = '';
    await loadComments();
  } catch {
    submitError.value = 'Network error. Please try again.';
  } finally {
    submitting.value = false;
  }
}

function maskEmail(email) {
  const [local, domain] = email.split('@')
  if (local.length <= 2) return `${local}@${domain}`
  return `${local[0]}${'*'.repeat(local.length - 2)}${local[local.length - 1]}@${domain}`
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
  border-radius: 8px;
  overflow-x: auto;
  margin: 1rem 0;
}

/* Let hljs own the code block — reset the inline-code styles that would override it */
.post-content :deep(pre code) {
  background: transparent;
  border: none;
  padding: 0;
  border-radius: 0;
  font-size: inherit;
}

.post-content :deep(blockquote) {
  border-left: 3px solid var(--accent);
  padding-left: 1rem;
  color: var(--text-muted);
  margin: 1rem 0;
}

/* Comments */
.comments-section {
  margin-top: 3rem;
  border-top: 1px solid var(--border);
  padding-top: 2rem;
}

.comments-heading {
  font-size: 1.25rem;
  color: var(--text);
  margin: 0 0 1.5rem;
}

.no-comments {
  color: var(--text-muted);
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
}

.comment-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-bottom: 2rem;
}

.comment-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1rem 1.25rem;
}

.comment-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.5rem;
}

.comment-avatar {
  flex-shrink: 0;
}

.comment-avatar-img {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
  border: 1px solid var(--border);
}

.comment-avatar-initial {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--accent);
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--mono);
  font-size: 0.85rem;
  font-weight: 700;
  color: #fff;
}

.comment-form-author {
  margin-bottom: 0.75rem;
}

.comment-meta {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.comment-author {
  font-size: 0.85rem;
  color: var(--accent);
  font-family: var(--mono);
}

.comment-date {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.comment-body {
  color: var(--text);
  font-size: 0.95rem;
  line-height: 1.6;
  margin: 0;
  white-space: pre-wrap;
}

.comment-form {
  margin-top: 1.5rem;
}

.form-heading {
  font-size: 1rem;
  color: var(--text);
  margin: 0 0 0.75rem;
  font-weight: 500;
}

.comment-textarea {
  padding-bottom: 0.5rem;
}

.comment-textarea :deep(.v-field__input) {
  mask-image: none;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
}

.submit-btn {
  font-weight: 600;
  letter-spacing: 0;
  text-transform: none;
  background-color: var(--accent) !important;
  color: #fff !important;
}

.submit-btn:hover {
  background-color: var(--accent-hover) !important;
}

.submit-error {
  color: #f87171;
  font-size: 0.85rem;
  margin-top: 0.5rem;
  text-align: right;
}

.login-prompt {
  margin-top: 1.5rem;
  color: var(--text-muted);
  font-size: 0.9rem;
}

.accent-link {
  color: var(--accent);
  cursor: pointer;
  text-decoration: none;
}

.accent-link:hover {
  color: var(--accent-hover);
  text-decoration: underline;
}
</style>
