<template>
  <div class="admin-shell">
    <!-- Sidebar -->
    <aside class="admin-sidebar">
      <div class="sidebar-top" style="margin-top: 75px">
        <span class="sidebar-title">Blog CMS</span>
        <v-btn class="btn-new" @click="newPost">+ New</v-btn>
      </div>

      <div class="sidebar-list">
        <div v-if="loadingPosts" class="sidebar-loading">
          <v-progress-circular indeterminate size="20" color="#7c6af7" />
        </div>

        <template v-else>
          <div v-if="posts.length === 0" class="sidebar-empty">No posts yet</div>
          <div
            v-for="p in posts"
            :key="p.id"
            class="sidebar-item"
            :class="{ active: selected?.id === p.id }"
            @click="selectPost(p)"
          >
            <span class="sidebar-item-title">{{ p.title }}</span>
            <span class="sidebar-item-badge" :class="p.published ? 'live' : 'draft'">
              {{ p.published ? 'Live' : 'Draft' }}
            </span>
          </div>
        </template>
      </div>
    </aside>

    <!-- Main editor area -->
    <main class="admin-main">
      <div v-if="!editing" class="editor-empty">
        <div class="editor-empty-inner">
          <span class="editor-empty-icon">✍️</span>
          <p>Select a post or click <strong>+ New</strong> to start writing.</p>
        </div>
      </div>

      <template v-else>
        <div class="editor-header">
          <input
            v-model="form.title"
            class="input-title"
            placeholder="Post title…"
          />
          <div class="editor-meta">
            <div class="meta-row">
              <label class="meta-label">Slug</label>
              <input
                v-model="form.slug"
                class="input-slug"
                placeholder="my-post-slug"
              />
            </div>
            <div class="meta-row">
              <label class="meta-label">Status</label>
              <label class="toggle-switch">
                <input type="checkbox" v-model="form.published" />
                <span class="toggle-track">
                  <span class="toggle-thumb"></span>
                </span>
                <span class="toggle-label">{{ form.published ? 'Published' : 'Draft' }}</span>
              </label>
            </div>
          </div>
        </div>

        <div class="editor-body">
          <div ref="editorEl" class="stacks-editor-mount"></div>
        </div>

        <div class="editor-footer">
          <button
            v-if="selected"
            class="btn-delete"
            :disabled="deleting"
            @click="confirmDelete"
          >
            {{ deleting ? 'Deleting…' : 'Delete' }}
          </button>
          <span v-if="saveError" class="save-error">{{ saveError }}</span>
          <span v-if="saveSuccess" class="save-success">Saved!</span>
          <button
            class="btn-save"
            :disabled="saving"
            @click="save"
          >
            {{ saving ? 'Saving…' : selected ? 'Save changes' : 'Create post' }}
          </button>
        </div>
      </template>
    </main>
  </div>
</template>

<script setup>
import { ref, nextTick, onBeforeUnmount } from 'vue'
import { StacksEditor } from '@stackoverflow/stacks-editor'
import '@stackoverflow/stacks-editor/dist/styles.css'
import { apiFetch } from '../../api'

const posts = ref([])
const loadingPosts = ref(true)
const selected = ref(null)
const editing = ref(false)
const saving = ref(false)
const deleting = ref(false)
const saveError = ref(null)
const saveSuccess = ref(false)

const editorEl = ref(null)
let editorInstance = null

const form = ref({ title: '', slug: '', published: false })

async function loadPosts() {
  loadingPosts.value = true
  try {
    const res = await apiFetch('/api/admin/posts')
    if (!res.ok) throw new Error(res.Error)
    posts.value = await res.json()
  } finally {
    loadingPosts.value = false
  }
}

function destroyEditor() {
  editorInstance?.destroy?.()
  editorInstance = null
  if (editorEl.value) editorEl.value.innerHTML = ''
}

async function mountEditor(initialContent = '') {
  await nextTick()
  if (!editorEl.value) return
  destroyEditor()
  editorInstance = new StacksEditor(editorEl.value, initialContent, {
    editorHelpLink: '',
  })
}

function newPost() {
  selected.value = null;
  form.value = { title: '', slug: '', published: false };
  editing.value = true;
  mountEditor('');
}

function selectPost(post) {
  selected.value = post;
  form.value = { title: post.title, slug: post.slug, published: post.published };
  editing.value = true;
  mountEditor(post.content);
}

async function save() {
  saveError.value = null;
  saveSuccess.value = false;
  saving.value = true;
  const content = editorInstance?.content ?? '';

  try {
    const body = { ...form.value, content }
    const url = selected.value ? `/api/admin/posts/${selected.value.id}` : '/api/admin/posts'
    const method = selected.value ? 'PUT' : 'POST'

    const res = await apiFetch(url, {
      method,
      credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    })

    if (!res.ok) {
      const text = await res.text()
      throw new Error(text || 'Save failed')
    }

    const saved = await res.json()
    if (selected.value) {
      const idx = posts.value.findIndex(p => p.id === saved.id)
      if (idx !== -1) posts.value[idx] = saved
    } else {
      posts.value.unshift(saved)
    }
    selected.value = saved
    saveSuccess.value = true
    setTimeout(() => (saveSuccess.value = false), 2500)
  } catch (e) {
    saveError.value = e.message
  } finally {
    saving.value = false
  }
}

async function confirmDelete() {
  if (!selected.value) return
  if (!confirm(`Delete "${selected.value.title}"? This cannot be undone.`)) return
  deleting.value = true
  try {
    await apiFetch(`/api/admin/posts/${selected.value.id}`, {
      method: 'DELETE',
      credentials: 'include',
    })
    posts.value = posts.value.filter(p => p.id !== selected.value.id)
    selected.value = null
    editing.value = false
    destroyEditor()
  } finally {
    deleting.value = false
  }
}

onBeforeUnmount(destroyEditor)
loadPosts()
</script>

<!-- Global overrides: Stacks Editor dark theme -->
<style>
/* ── Popover fix ─────────────────────────────────────────────── */
.s-popover {
  display: none !important;
  position: absolute;
  z-index: 1050;
}
.s-popover.is-visible {
  display: block !important;
}

/* ── Design tokens ───────────────────────────────────────────── */
.stacks-editor-mount {
  --black-500: #8892a4;
  --black-600: #c8d3e8;
  --black-150: #2a2d3e;
  --white: #1a1d27;
  --br-sm: 5px;
  --su2: 2px;
  --su12: 12px;
  --su4: 4px;
  --theme-secondary-200: rgba(124,106,247,0.18);
  --theme-secondary-400: #7c6af7;
  --theme-secondary-600: #9585f8;
  --focus-neutral: rgba(0,0,0,.2);
  --focus-theme: rgba(124,106,247,.4);
  --fc-light: #5a6478;
  --s-prose-spacing: 1.1rem;
  --s-prose-spacing-condensed: 0.6rem;
  --s-step: 40px;
}

/* ── Container ───────────────────────────────────────────────── */
.stacks-editor-mount .s-editor-container {
  border: none;
  background: transparent;
}

/* ── Toolbar ─────────────────────────────────────────────────── */
.stacks-editor-mount .s-editor-toolbar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 1px;
  padding: 6px 12px;
  background: #13151f;
  border-bottom: 1px solid #23263a;
  position: sticky;
  top: 0;
  z-index: 10;
}

/* Toolbar buttons */
.stacks-editor-mount .s-btn.s-editor-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 30px;
  height: 30px;
  padding: 0 6px;
  color: #5a6880;
  background: transparent;
  border: none;
  border-radius: 5px;
  transition: color 0.12s, background 0.12s;
}

.stacks-editor-mount .s-btn.s-editor-btn:hover {
  color: #c8d3e8 !important;
  background: #1e2234 !important;
}

.stacks-editor-mount .s-btn.s-editor-btn.is-selected,
.stacks-editor-mount .s-btn.s-editor-btn:active {
  color: #9585f8 !important;
  background: rgba(124, 106, 247, 0.15) !important;
}

/* SVG icons */
.stacks-editor-mount .svg-icon-bg {
  width: 16px;
  height: 16px;
  opacity: 0.75;
  transition: opacity 0.12s;
}
.stacks-editor-mount .s-btn.s-editor-btn:hover .svg-icon-bg {
  opacity: 1;
}

/* Dropdown/btn-group caret */
.stacks-editor-mount .s-btn.s-editor-btn.s-btn__dropdown::after {
  border-top-color: currentColor;
}

/* Toolbar button groups — subtle separator */
.stacks-editor-mount .s-btn-group {
  display: inline-flex;
  gap: 1px;
  padding: 0 4px;
  border-right: 1px solid #23263a;
  margin-right: 2px;
}
.stacks-editor-mount .s-btn-group:last-child {
  border-right: none;
}

/* ── Mode toggle (Rich text / Markdown) ──────────────────────── */
.stacks-editor-mount .s-editor--mode-toggle-container,
.stacks-editor-mount [class*="mode-toggle"] {
  margin-left: auto;
}

/* Radio-style mode buttons */
.stacks-editor-mount .s-btn-group .s-btn--radio:checked + .s-editor-btn {
  background: rgba(124, 106, 247, 0.2) !important;
  color: #9585f8 !important;
}

/* ── Writing area ────────────────────────────────────────────── */
.stacks-editor-mount .ProseMirror {
  max-width: 760px;
  margin: 0 auto;
  padding: 32px 40px;
  min-height: 100%;
  color: #c8d3e8;
  font-size: 15.5px;
  line-height: 1.85;
  outline: none;
  caret-color: #7c6af7;
}

/* Placeholder */
.stacks-editor-mount .ProseMirror [data-placeholder]::before {
  color: #2e3448;
  font-style: italic;
}

/* Headings */
.stacks-editor-mount .ProseMirror h1,
.stacks-editor-mount .ProseMirror h2,
.stacks-editor-mount .ProseMirror h3 {
  color: #e2e8f0;
  margin-top: 1.6em;
  margin-bottom: 0.4em;
  line-height: 1.3;
}
.stacks-editor-mount .ProseMirror h1 { font-size: 1.75em; }
.stacks-editor-mount .ProseMirror h2 { font-size: 1.35em; }
.stacks-editor-mount .ProseMirror h3 { font-size: 1.1em;  }

/* Paragraphs */
.stacks-editor-mount .ProseMirror p {
  margin-bottom: 1em;
}

/* Inline code */
.stacks-editor-mount .ProseMirror code {
  background: #1a1d2e;
  border: 1px solid #2a2d3e;
  border-radius: 4px;
  padding: 0.1em 0.45em;
  font-size: 0.875em;
  color: #9585f8;
  font-family: var(--mono);
}

/* Code blocks */
.stacks-editor-mount .ProseMirror pre {
  background: #111320;
  border: 1px solid #23263a;
  border-radius: 8px;
  padding: 16px 20px;
  margin: 1.2em 0;
  overflow-x: auto;
}
.stacks-editor-mount .ProseMirror pre code {
  background: none;
  border: none;
  padding: 0;
  color: #a8b3cf;
  font-size: 0.9em;
}

/* Blockquote */
.stacks-editor-mount .ProseMirror blockquote {
  border-left: 3px solid #7c6af7;
  margin: 1.2em 0;
  padding: 4px 0 4px 16px;
  color: #6a7590;
  font-style: italic;
}

/* Lists */
.stacks-editor-mount .ProseMirror ul,
.stacks-editor-mount .ProseMirror ol {
  padding-left: 1.5em;
  margin-bottom: 1em;
}
.stacks-editor-mount .ProseMirror li {
  margin-bottom: 0.3em;
}

/* Links */
.stacks-editor-mount .ProseMirror a {
  color: #7c6af7;
  text-decoration: underline;
  text-underline-offset: 2px;
}

/* Horizontal rule */
.stacks-editor-mount .ProseMirror hr {
  border: none;
  border-top: 1px solid #23263a;
  margin: 2em 0;
}

/* Selection */
.stacks-editor-mount .ProseMirror ::selection {
  background: rgba(124, 106, 247, 0.25);
}

/* ── Dropdown menus ──────────────────────────────────────────── */
.stacks-editor-mount .s-editor--dropdown-item {
  color: #c8d3e8;
  padding: 7px 16px;
  font-size: 0.875rem;
  cursor: pointer;
}
.stacks-editor-mount .s-editor--dropdown-item:hover {
  background: #1e2234;
  color: #e2e8f0;
}
.stacks-editor-mount .s-editor--dropdown-item.is-selected {
  color: #9585f8;
  background: rgba(124, 106, 247, 0.12);
}
</style>

<style scoped>
/* Full-viewport admin shell */
.admin-shell {
  display: flex;
  height: calc(100vh - 64px);
  overflow: hidden;
  background: #0f1117;
}

/* ── Sidebar ─────────────────────────────────────────── */
.admin-sidebar {
  width: 260px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: #13151f;
  border-right: 1px solid #2a2d3e;
  overflow: hidden;
}

.sidebar-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid #2a2d3e;
  flex-shrink: 0;
}

.sidebar-title {
  font-family: var(--mono);
  font-size: 0.85rem;
  font-weight: 600;
  color: #7c6af7;
  text-transform: uppercase;
  letter-spacing: 0.08em;
}


.sidebar-list {
  overflow-y: auto;
  flex: 1;
}

.sidebar-loading {
  display: flex;
  justify-content: center;
  padding: 24px;
}

.sidebar-empty {
  padding: 20px 16px;
  font-size: 0.85rem;
  color: #4a5264;
  text-align: center;
}

.sidebar-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px;
  cursor: pointer;
  border-bottom: 1px solid #1e2130;
  transition: background 0.12s;
  gap: 8px;
}

.sidebar-item:hover {
  background: #1a1d2a;
}

.sidebar-item.active {
  background: rgba(124, 106, 247, 0.1);
  border-left: 2px solid #7c6af7;
  padding-left: 14px;
}

.sidebar-item-title {
  font-size: 0.875rem;
  color: #c8cfe0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.sidebar-item-badge {
  font-size: 0.68rem;
  font-weight: 600;
  border-radius: 4px;
  padding: 2px 6px;
  flex-shrink: 0;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.sidebar-item-badge.live {
  background: rgba(72, 199, 142, 0.15);
  color: #48c78e;
}

.sidebar-item-badge.draft {
  background: rgba(136, 146, 164, 0.12);
  color: #606878;
}

/* ── Main editor panel ───────────────────────────────── */
.admin-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.editor-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.editor-empty-inner {
  text-align: center;
  color: #4a5264;
}

.editor-empty-icon {
  font-size: 2.5rem;
  display: block;
  margin-bottom: 12px;
}

.editor-empty-inner p {
  font-size: 0.9rem;
}

.editor-empty-inner strong {
  color: #7c6af7;
}

/* Editor header (title + meta) */
.editor-header {
  padding: 20px 24px 0;
  border-bottom: 1px solid #2a2d3e;
  flex-shrink: 0;
}

.input-title {
  width: 100%;
  background: transparent;
  border: none;
  outline: none;
  font-size: 1.6rem;
  font-weight: 600;
  color: #e2e8f0;
  margin-bottom: 14px;
  font-family: inherit;
  caret-color: #7c6af7;
}

.input-title::placeholder {
  color: #3a4058;
}

.editor-meta {
  display: flex;
  gap: 24px;
  padding-bottom: 14px;
  flex-wrap: wrap;
}

.meta-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.meta-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: #4a5264;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  white-space: nowrap;
}

.input-slug {
  background: #1a1d27;
  border: 1px solid #2a2d3e;
  border-radius: 6px;
  padding: 4px 10px;
  color: #e2e8f0;
  font-family: var(--mono);
  font-size: 0.82rem;
  outline: none;
  transition: border-color 0.15s;
  min-width: 200px;
}

.input-slug:focus {
  border-color: #7c6af7;
}

/* Toggle switch */
.toggle-switch {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.toggle-switch input {
  display: none;
}

.toggle-track {
  width: 36px;
  height: 20px;
  background: #2a2d3e;
  border-radius: 10px;
  position: relative;
  transition: background 0.2s;
}

.toggle-switch input:checked + .toggle-track {
  background: #7c6af7;
}

.toggle-thumb {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 14px;
  height: 14px;
  background: #fff;
  border-radius: 50%;
  transition: transform 0.2s;
}

.toggle-switch input:checked + .toggle-track .toggle-thumb {
  transform: translateX(16px);
}

.toggle-label {
  font-size: 0.82rem;
  color: #8892a4;
  min-width: 60px;
}

/* Editor body */
.editor-body {
  flex: 1;
  overflow-y: auto;
  background: #0f1117;
}

.stacks-editor-mount {
  height: 100%;
}

/* Editor footer */
.editor-footer {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 24px;
  border-top: 1px solid #2a2d3e;
  background: #13151f;
  flex-shrink: 0;
}

.btn-save {
  margin-left: auto;
  background: #7c6af7;
  color: #fff;
  border: none;
  border-radius: 8px;
  padding: 8px 20px;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s, opacity 0.15s;
}

.btn-save:hover:not(:disabled) {
  background: #9585f8;
}

.btn-save:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-delete {
  background: transparent;
  color: #e05252;
  border: 1px solid rgba(224, 82, 82, 0.35);
  border-radius: 8px;
  padding: 7px 16px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s;
}

.btn-delete:hover:not(:disabled) {
  background: rgba(224, 82, 82, 0.1);
  border-color: rgba(224, 82, 82, 0.6);
}

.btn-delete:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.save-error {
  font-size: 0.82rem;
  color: #e05252;
}

.save-success {
  font-size: 0.82rem;
  color: #48c78e;
}
</style>
