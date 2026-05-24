import router from './router'

export const apiFetch = async (input: RequestInfo, init?: RequestInit): Promise<Response> => {
  const res = await fetch(input, { ...init, credentials: 'include' });
  if (res.status === 401) {
    await unauthorizedHandler();
    throw new Error('Unauthorized');
  }
  return res;
}

let loggingOut = false;

const unauthorizedHandler = async () => {
  if (loggingOut) return;
  loggingOut = true;
  await logOut();
  router.push('/login');
  loggingOut = false;
}

export const logOut = async () => {
  clearAuthCookies();
  localStorage.removeItem('logged_in');
  try {
    await fetch('/api/auth/logout', { method: 'POST', credentials: 'include' });
  } catch { /* best-effort */ }
}

function clearAuthCookies() {
  const root = window.location.hostname.replace(/^www\./, '');
  for (const name of ['session', 'is_admin']) {
    document.cookie = `${name}=; Max-Age=0; Path=/; Secure; SameSite=None`;
    document.cookie = `${name}=; Max-Age=0; Path=/; Domain=${root}; Secure; SameSite=None`;
    document.cookie = `${name}=; Max-Age=0; Path=/; Domain=.${root}; Secure; SameSite=None`;
  }
}