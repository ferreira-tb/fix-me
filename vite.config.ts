import tailwind from 'tailwindcss';
import { defineConfig } from 'vite';
import autoprefixer from 'autoprefixer';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  clearScreen: false,
  plugins: [sveltekit()],
  server: {
    port: 1422,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  css: {
    postcss: {
      plugins: [tailwind(), autoprefixer()],
    },
  },
  build: {
    emptyOutDir: true,
    minify: false,
    target: 'esnext',
  },
});
