declare global {
  interface Promise<T> {
    err: () => void;
  }
}

export {};
