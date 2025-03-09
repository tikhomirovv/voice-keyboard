import { createRouter, createWebHistory } from "vue-router";
import MainPage from "@/pages/main/MainPage.vue";
import SettingsPage from "@/pages/settings/SettingsPage.vue";

const routes = [
  {
    path: "/",
    name: "main",
    component: MainPage,
  },
  {
    path: "/settings",
    name: "settings",
    component: SettingsPage,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
