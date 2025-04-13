import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['./tsconfig.json'],
  ignores: ['**/lib/api/bindings.ts'],
  features: {
    vue: true,
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
