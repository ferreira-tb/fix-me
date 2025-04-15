import { type MessageDialogOptions, message as showMessage } from '@tauri-apps/plugin-dialog';

export function onError(err: unknown) {
  const options: MessageDialogOptions = { title: 'Error', kind: 'error' };
  const message = err instanceof Error ? err.message : String(err);
  showMessage(message, options).catch(console.error);
}
