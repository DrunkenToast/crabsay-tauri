import { createRouter, createWebHistory } from '@ionic/vue-router';
import { createWebHashHistory, RouteRecordRaw } from 'vue-router';
import TabsPage from '../views/TabsPage.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/creator'
  },
  {
    path: '/',
    component: TabsPage,
    children: [
      {
        path: '',
        redirect: '/creator'
      },
      {
        path: 'creator',
        component: () => import('@/views/CreatorPage.vue')
      },
      {
        path: 'generator',
        component: () => import('@/views/GeneratorPage.vue')
      },
    ]
  }
]

const router = createRouter({
  history: createWebHashHistory(process.env.BASE_URL),
  routes
})

export default router
