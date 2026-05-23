import router from './router'

export async function apiFetch(input: RequestInfo, init?: RequestInit): Promise<Response> {
  const res = init ? await fetch(input, init) : await fetch(input);
  if (res.status === 401) {
    await unauthorizedHandler(res);
  }
  return res;
}


const unauthorizedHandler = async (res: Response) => {
  await fetch('/api/auth/logout', { method: 'POST', credentials: 'include' })
  localStorage.removeItem('logged_in');
  router.push('/login');
}