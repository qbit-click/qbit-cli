import type {Config} from '@docusaurus/types';
import type {Options, ThemeConfig} from '@docusaurus/preset-classic';
import {themes as prismThemes} from 'prism-react-renderer';

const baseUrl = process.env.DOCS_BASE_PATH || '/';

const config: Config = {
  title: 'Qbit CLI',
  tagline: 'محیط توسعه و اتوماسیون، با یک دستور',
  favicon: 'icon.svg',

  future: {
    v4: true,
  },

  url: 'https://qbit-click.github.io',
  baseUrl,
  organizationName: 'qbit-click',
  projectName: 'qbit-cli',
  trailingSlash: false,
  onBrokenLinks: 'throw',
  i18n: {
    defaultLocale: 'fa',
    locales: ['fa', 'en'],
    localeConfigs: {
      fa: {label: 'فارسی', direction: 'rtl', htmlLang: 'fa-IR', calendar: 'persian'},
      en: {label: 'English', direction: 'ltr', htmlLang: 'en-US'},
    },
  },
  presets: [
    [
      'classic',
      {
        docs: {
          routeBasePath: 'guide',
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/qbit-click/qbit-cli/edit/main/user-docs/docs/',
        },
        blog: false,
        theme: {customCss: './src/css/custom.css'},
      } satisfies Options,
    ],
  ],
  themes: [
    [
      '@easyops-cn/docusaurus-search-local',
      {
        hashed: 'filename',
        indexDocs: true,
        indexBlog: false,
        indexPages: true,
        docsRouteBasePath: 'guide',
        docsDir: 'docs',
        highlightSearchTermsOnTargetPage: true,
        explicitSearchResultPath: true,
      },
    ],
  ],
  themeConfig: {
    navbar: {
      title: 'Qbit CLI',
      logo: {alt: 'Qbit CLI', src: 'icon.svg'},
      items: [
        {to: '/guide/download-install', label: 'دانلود', position: 'left'},
        {to: '/guide/getting-started', label: 'دستورها', position: 'left'},
        {to: '/guide/configuration', label: 'پیکربندی', position: 'left'},
        {type: 'localeDropdown', position: 'right'},
        {href: 'https://github.com/qbit-click/qbit-cli', label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'light',
      links: [
        {
          title: 'Qbit CLI',
          items: [
            {label: 'دانلود و نصب', to: '/guide/download-install'},
            {label: 'شروع کار', to: '/guide/getting-started'},
            {label: 'پیکربندی', to: '/guide/configuration'},
          ],
        },
        {
          title: 'Qbit',
          items: [
            {label: 'GitHub', href: 'https://github.com/qbit-click/qbit-cli'},
            {label: 'Qbit Console Docs', href: 'https://qbit-click.github.io/qbit-console-user-docs/'},
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Qbit.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['bash', 'powershell', 'json', 'toml'],
    },
  } satisfies ThemeConfig,
};

export default config;
