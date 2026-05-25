import { Subject } from 'rxjs'

interface SnackBarMessage {
  message: string
  color: 'success' | 'error'
}

export const useSnackBarSubject = new Subject<SnackBarMessage>();

export function showSnackBar(message: string, color: 'success' | 'error' = 'success') {
  useSnackBarSubject.next({ message, color });
}