import { createRouter, createWebHashHistory } from 'vue-router'
import main from '@/pages/main.vue'
import setting from '@/pages/setting.vue'

const routes = [
    {
        path: "/",
        redirect: '/main'
    },
    {
        path: "/main",
        component: main,
        name: "mainPage",
        meta: {
            title: '焦距',
        }

    },
    {
        path: "/setting",
        component: setting,
        name: "setting",
        meta: {
            title: '设置',
        }

    }
]
const router = createRouter({
    model: 'hash',
    history: createWebHashHistory(),
    routes: routes
})
export default router