import router from './router'

export async function apiFetch(input: RequestInfo, init?: RequestInit): Promise<Response> {
  const res = await fetch(input, init)
  if (res.status === 401) {
    localStorage.removeItem('logged_in')
    router.push('/login')
  }
  return res
}
