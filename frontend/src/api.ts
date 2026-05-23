import router from './router'

export async function apiFetch(input: RequestInfo, init?: RequestInit): Promise<Response> {
  const res = await fetch(input, { credentials: 'include', ...init });
  if (res.status === 401) {
    await unauthorizedHandler();
    throw new Error('Unauthorized');
  }
  return res;
}


const unauthorizedHandler = async () => {
  await fetch('/api/auth/logout', { method: 'POST', credentials: 'include' });
  localStorage.removeItem('logged_in');
  router.push('/login');
}