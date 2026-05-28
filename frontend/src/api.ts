import router from './router'
import { SESSION_COOKIE_NAME, ADMIN_COOKIE_NAME } from './helpers/auth/cookies';
import { setProfileImage } from './components/subjects/profile-image-subject';

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
  setProfileImage(null);
  try {
    await fetch('/api/auth/logout', { method: 'POST', credentials: 'include' });
  } catch { /* best-effort */ }
}

function clearAuthCookies() {
  const root = window.location.hostname.replace(/^www\./, '');
  const cookies = [
    SESSION_COOKIE_NAME,
    ADMIN_COOKIE_NAME,
  ];
  for (const name of cookies) {
    document.cookie = `${name}=; Max-Age=0; Path=/; Secure; SameSite=None`;
    document.cookie = `${name}=; Max-Age=0; Path=/; Domain=${root}; Secure; SameSite=None`;
    document.cookie = `${name}=; Max-Age=0; Path=/; Domain=.${root}; Secure; SameSite=None`;
  }
}