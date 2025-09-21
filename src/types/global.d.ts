/* eslint-disable no-inner-declarations */
/* eslint-disable @typescript-eslint/naming-convention */

declare global {
  var __DEBUG_ASSERTIONS__: boolean;

  interface Promise<T> {
    err: () => void;
  }
}

export {};
