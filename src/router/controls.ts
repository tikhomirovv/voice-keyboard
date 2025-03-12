import { createRouter, createWebHistory } from "vue-router";
import ControlsPanel from "@/pages/controls/ControlsPanel.vue";

const routes = [
  {
    path: "/controls",
    name: "controls",
    component: ControlsPanel,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
