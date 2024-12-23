import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['./tsconfig.json'],
  ignores: ['src/lib/components/ui/*', '**/lib/api/bindings.ts'],
  features: {
    svelte: true,
    vue: false,
  },
  overrides: {
    javascript: {
      'no-undefined': 'off',
    },
    typescript: {
      '@typescript-eslint/consistent-type-definitions': 'off',
    },
  },
});
