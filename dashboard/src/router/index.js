import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import NotFound from '../views/NotFound.vue'

Vue.use(VueRouter)

const routes = [{
        path: '*',
        component: () =>
            import ('../views/NotFound.vue')
    },
    {
        path: '/',
        name: 'Home',
        component: Home
    },
    {
        path: '/h/:uuid',
        name: 'Host details',
        // route level code-splitting
        component: () =>
            import ('../views/Details.vue')
    }
]

const router = new VueRouter({
    mode: 'history',
    base: process.env.BASE_URL,
    routes
})

export default router