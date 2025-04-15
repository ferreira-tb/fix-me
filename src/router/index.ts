import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      component: () => import('@/views/home/index.vue'),
      name: 'home' satisfies Route,
      path: '/',
    },
    {
      component: () => import('@/views/history/index.vue'),
      name: 'history' satisfies Route,
      path: '/history',
    },
    {
      component: () => import('@/views/settings/index.vue'),
      name: 'settings' satisfies Route,
      path: '/settings',
    },
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
