import { createRouter, createWebHistory } from "vue-router";
import MainPage from "@/pages/main/MainPage.vue";
import SettingsPage from "@/pages/settings/SettingsPage.vue";
import ControlsPanel from "@/pages/controls/ControlsPanel.vue";
import MainLayout from "@/layouts/MainLayout.vue";
import PanelsLayout from "@/layouts/PanelsLayout.vue";
const routes = [
  {
    path: "/",
    redirect: "/main",
  },
  {
    path: "/main",
    name: "main",
    component: MainLayout,
    children: [
      {
        path: "",
        name: "main-page",
        component: MainPage,
      },
      {
        path: "settings",
        name: "settings",
        component: SettingsPage,
      },
    ],
  },
  {
    path: "/panels",
    name: "panels",
    component: PanelsLayout,
    children: [
      {
        path: "controls",
        name: "controls",
        component: ControlsPanel,
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
