import router from './router'

export const apiFetch = async (input: RequestInfo, init?: RequestInit): Promise<Response> => {
  const res = await fetch(input, { ...init, credentials: 'include' });
  if (res.status === 401) {
    await unauthorizedHandler();
    throw new Error('Unauthorized');
  }
  return res;
}

const unauthorizedHandler = async () => {
  await logOut();
  router.push('/login');
}

export const logOut = async () => {
  await fetch('/api/auth/logout', { method: 'POST', credentials: 'include' });
  const root = window.location.hostname.replace(/^www\./, '');
  for (const name of ['session', 'is_admin']) {
    document.cookie = `${name}=; Max-Age=0; Path=/; Secure; SameSite=Lax`;
    document.cookie = `${name}=; Max-Age=0; Path=/; Domain=${root}; Secure; SameSite=Lax`;
    document.cookie = `${name}=; Max-Age=0; Path=/; Domain=.${root}; Secure; SameSite=Lax`;
  }
  localStorage.removeItem('logged_in');
}