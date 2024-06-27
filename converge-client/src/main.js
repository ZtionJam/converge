import { createApp } from "vue";
import App from "./App.vue";
import router from './router'

router.beforeEach((to, from, next) => {
    if (to.meta.title) {
        document.title = to.meta.title
    }
    next()
})

createApp(App)
    .use(router)
    .mount("#app");
