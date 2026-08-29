export type SidebarItem = { text: string; link: string };

export const faSidebar: SidebarItem[] = [
  { text: "شروع کار", link: "/guide/getting-started" },
  { text: "دانلود و نصب", link: "/guide/download-install" },
  { text: "نصب ابزارهای سیستم", link: "/guide/system-packages" },
  { text: "اسکریپت‌های پروژه", link: "/guide/project-scripts" },
  { text: "Python", link: "/guide/python" },
  { text: "JavaScript", link: "/guide/javascript" },
  { text: "Dart", link: "/guide/dart" },
  { text: "ارتقا Qbit", link: "/guide/upgrade" },
  { text: "پیکربندی qbit", link: "/guide/configuration" },
  { text: "رفع اشکال", link: "/guide/troubleshooting" },
];

export const enSidebar: SidebarItem[] = [
  { text: "Getting started", link: "/en/guide/getting-started" },
  { text: "Download and install", link: "/en/guide/download-install" },
  { text: "System packages", link: "/en/guide/system-packages" },
  { text: "Project scripts", link: "/en/guide/project-scripts" },
  { text: "Python", link: "/en/guide/python" },
  { text: "JavaScript", link: "/en/guide/javascript" },
  { text: "Dart", link: "/en/guide/dart" },
  { text: "Upgrade Qbit", link: "/en/guide/upgrade" },
  { text: "Qbit configuration", link: "/en/guide/configuration" },
  { text: "Troubleshooting", link: "/en/guide/troubleshooting" },
];

export const faNav = [
  { text: "دانلود", link: "/guide/download-install" },
  { text: "دستورها", link: "/guide/getting-started" },
  { text: "پیکربندی", link: "/guide/configuration" },
];

export const enNav = [
  { text: "Download", link: "/en/guide/download-install" },
  { text: "Commands", link: "/en/guide/getting-started" },
  { text: "Configuration", link: "/en/guide/configuration" },
];

export const releaseAssets = {
  windows: "qbit-windows-setup.zip",
  macos: "qbit-macos-setup.tar.gz",
  linux: "qbit-linux-setup.tar.gz",
} as const;

export const latestDownloadUrl = (asset: string) =>
  `https://github.com/qbit-click/qbit-cli/releases/latest/download/${asset}`;
