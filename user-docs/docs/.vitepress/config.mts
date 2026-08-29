import { defineConfig } from "vitepress";
import { enNav, enSidebar, faNav, faSidebar } from "./site";

const base = process.env.DOCS_BASE_PATH || "/";

export default defineConfig({
  base,
  cleanUrls: true,
  lastUpdated: true,
  locales: {
    root: { label: "فارسی", lang: "fa-IR", dir: "rtl", title: "مستندات Qbit CLI", description: "راهنمای دانلود، نصب و استفاده از Qbit CLI" },
    en: { label: "English", lang: "en-US", dir: "ltr", link: "/en/", title: "Qbit CLI Docs", description: "Download, installation, and usage documentation for Qbit CLI" },
  },
  head: [["meta", { name: "theme-color", content: "#357da1" }], ["link", { rel: "icon", href: `${base}icon.svg`, type: "image/svg+xml" }]],
  themeConfig: {
    logo: "/icon.svg",
    search: { provider: "local" },
    socialLinks: [{ icon: "github", link: "https://github.com/qbit-click/qbit-cli" }],
    locales: {
      root: { nav: faNav, sidebar: [{ text: "راهنمای Qbit CLI", items: faSidebar }], outline: { label: "در این صفحه", level: [2, 3] }, docFooter: { prev: "صفحه قبل", next: "صفحه بعد" }, lastUpdated: { text: "آخرین به‌روزرسانی" }, returnToTopLabel: "بازگشت به بالا", sidebarMenuLabel: "منو", darkModeSwitchLabel: "حالت نمایش" },
      en: { nav: enNav, sidebar: [{ text: "Qbit CLI guide", items: enSidebar }], outline: { label: "On this page", level: [2, 3] } },
    },
  },
});
