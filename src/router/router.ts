import HomePage from '../views/HomePage.vue'
import Introduction from '../components/Introduction.vue'
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        name: 'home',
        component: HomePage
    },
    {
        path: '/introduction',
        name: 'introduction',
        component: Introduction
    }
]

export const router = createRouter({
    history: createWebHistory(),
    routes
})