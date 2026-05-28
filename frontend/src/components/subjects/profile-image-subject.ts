import { BehaviorSubject } from 'rxjs'

export const profileImageSubject = new BehaviorSubject<string | null>(null)

export function setProfileImage(dataUrl: string | null) {
  profileImageSubject.next(dataUrl)
}
