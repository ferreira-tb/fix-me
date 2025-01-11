import { twMerge } from 'tailwind-merge';
import { type ClassValue, clsx } from 'clsx';
import { message } from '@tauri-apps/plugin-dialog';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function onError(err: unknown) {
  if (err instanceof Error) {
    void message(err.message, { title: 'Error', kind: 'error' });
  }
}
