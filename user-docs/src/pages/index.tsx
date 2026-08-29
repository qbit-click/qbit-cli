import type {ReactNode} from 'react';
import Link from '@docusaurus/Link';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import styles from './index.module.css';

type Domain = {
  code: string;
  title: string;
  description: string;
  path: string;
};

type Copy = {
  badge: string;
  title: string;
  subtitle: string;
  primaryCta: string;
  githubCta: string;
  scopeEyebrow: string;
  scopeTitle: string;
  scopeDescription: string;
  domains: Domain[];
  currentEyebrow: string;
  currentTitle: string;
  currentDescription: string;
  principlesEyebrow: string;
  principlesTitle: string;
  principles: Array<{title: string; description: string}>;
  closingTitle: string;
  closingDescription: string;
  closingCta: string;
};

const downloadBase = 'https://github.com/qbit-click/qbit-cli/releases/latest/download';
const downloads = [
  ['Windows', 'qbit-windows-setup.zip'],
  ['macOS', 'qbit-macos-setup.tar.gz'],
  ['Linux', 'qbit-linux-setup.tar.gz'],
] as const;

const copy: Record<'fa' | 'en', Copy> = {
  fa: {
    badge: 'CLI چندسکویی برای setup و automation توسعه',
    title: 'محیط توسعه و اتوماسیون، با یک دستور',
    subtitle:
      'Qbit CLI ابزارهای سیستم را نصب می‌کند، workflowهای repository را اجرا می‌کند و مدیریت Python، JavaScript و Dart را روی Windows، macOS و Linux یکپارچه می‌سازد.',
    primaryCta: 'شروع مطالعه مستندات',
    githubCta: 'مشاهده در GitHub',
    scopeEyebrow: 'یک CLI، قراردادهای قابل تکرار',
    scopeTitle: 'از نصب dependency سیستم تا اجرای script و مدیریت runtimeهای پروژه.',
    scopeDescription:
      'مستندات CLI بر commandهای stable، package-manager detection، فایل‌های qbit.yml/qbit.toml، workflowهای زبان و lifecycle ارتقا تمرکز دارد.',
    domains: [
      {
        code: 'DL',
        title: 'دانلود و نصب',
        description: 'Setup archive مناسب Windows، macOS یا Linux را دریافت و Qbit را روی PATH نصب کنید.',
        path: '/guide/download-install',
      },
      {
        code: 'PKG',
        title: 'پکیج‌های سیستم',
        description: 'ابزارها را از طریق package manager بومی سیستم‌عامل نصب کنید و در صورت نیاز manager را override کنید.',
        path: '/guide/system-packages',
      },
      {
        code: 'RUN',
        title: 'اسکریپت‌های پروژه',
        description: 'Commandهای تکرارشونده repository را در qbit.yml یا qbit.toml تعریف و با qbit run اجرا کنید.',
        path: '/guide/project-scripts',
      },
      {
        code: 'DEV',
        title: 'Python، JavaScript و Dart',
        description: 'Virtualenv، package managerهای JavaScript و dart pub را از commandهای یکپارچه Qbit مدیریت کنید.',
        path: '/guide/python',
      },
      {
        code: 'CFG',
        title: 'پیکربندی و ارتقا',
        description: 'Install mappingها، environment overrideها و رفتار qbit upgrade را با قراردادهای صریح کنترل کنید.',
        path: '/guide/configuration',
      },
    ],
    currentEyebrow: 'Release عملیاتی فعلی',
    currentTitle: 'آخرین نسخه Qbit CLI',
    currentDescription:
      'Release پایدار برای هر سه سیستم‌عامل archive ثابت دارد و همین نام assetها توسط installer، مستندات و upgrader به‌عنوان یک قرارداد مشترک استفاده می‌شود.',
    principlesEyebrow: 'اصول CLI',
    principlesTitle: 'برای setup قابل تکرار و automation بدون رفتار پنهان طراحی شده است.',
    principles: [
      {
        title: 'چندسکویی با قرارداد یکسان',
        description: 'Command surface روی Windows، macOS و Linux ثابت می‌ماند و فقط package manager یا pathهای platform-specific تغییر می‌کنند.',
      },
      {
        title: 'پیکربندی داخل repository',
        description: 'Scriptها و install mappingها در qbit.yml، qbit.yaml یا qbit.toml نگهداری می‌شوند تا setup پروژه قابل بازتولید باشد.',
      },
      {
        title: 'Fallback پنهان نداریم',
        description: 'وقتی lockfile یا override یک package manager را تعیین می‌کند ولی executable موجود نیست، Qbit خطای actionable می‌دهد و silently manager دیگری انتخاب نمی‌کند.',
      },
    ],
    closingTitle: 'از command یا runtime متناسب با کاری که می‌خواهید انجام دهید شروع کنید.',
    closingDescription:
      'راهنمای شروع، نصب ابزارهای سیستم، scriptهای پروژه، Python، JavaScript، Dart، پیکربندی، upgrade و troubleshooting در navigation در دسترس هستند.',
    closingCta: 'باز کردن راهنمای شروع',
  },
  en: {
    badge: 'Cross-platform CLI for development setup and automation',
    title: 'One CLI for development setup and automation',
    subtitle:
      'Qbit CLI installs system tools, runs repository workflows, and unifies Python, JavaScript, and Dart project management across Windows, macOS, and Linux.',
    primaryCta: 'Start with the documentation',
    githubCta: 'View on GitHub',
    scopeEyebrow: 'One CLI, repeatable contracts',
    scopeTitle: 'From system dependency installation to project scripts and language runtimes.',
    scopeDescription:
      'The CLI documentation focuses on stable commands, package-manager detection, qbit.yml/qbit.toml configuration, language workflows, and the upgrade lifecycle.',
    domains: [
      {
        code: 'DL',
        title: 'Download & install',
        description: 'Get the Windows, macOS, or Linux setup archive and install Qbit on your PATH.',
        path: '/guide/download-install',
      },
      {
        code: 'PKG',
        title: 'System packages',
        description: 'Install tools through the native OS package manager and override the manager when required.',
        path: '/guide/system-packages',
      },
      {
        code: 'RUN',
        title: 'Project scripts',
        description: 'Define repeatable repository commands in qbit.yml or qbit.toml and execute them with qbit run.',
        path: '/guide/project-scripts',
      },
      {
        code: 'DEV',
        title: 'Python, JavaScript & Dart',
        description: 'Manage virtualenvs, JavaScript package managers, and dart pub through a consistent command surface.',
        path: '/guide/python',
      },
      {
        code: 'CFG',
        title: 'Configuration & upgrade',
        description: 'Control install mappings, environment overrides, and qbit upgrade behavior through explicit contracts.',
        path: '/guide/configuration',
      },
    ],
    currentEyebrow: 'Current operational release',
    currentTitle: 'Latest Qbit CLI release',
    currentDescription:
      'The stable release provides fixed archive names for all three operating systems. Installers, documentation, and the upgrader share those asset names as one release contract.',
    principlesEyebrow: 'CLI principles',
    principlesTitle: 'Designed for repeatable setup and automation without hidden behavior.',
    principles: [
      {
        title: 'One contract across platforms',
        description: 'The command surface remains consistent on Windows, macOS, and Linux while package managers and platform paths vary where necessary.',
      },
      {
        title: 'Configuration lives with the repository',
        description: 'Scripts and install mappings live in qbit.yml, qbit.yaml, or qbit.toml so project setup can be reproduced.',
      },
      {
        title: 'No silent fallback',
        description: 'When a lockfile or override selects a package manager that is unavailable, Qbit returns an actionable error instead of silently switching managers.',
      },
    ],
    closingTitle: 'Start from the command or runtime that matches the task you need to complete.',
    closingDescription:
      'Getting started, system packages, project scripts, Python, JavaScript, Dart, configuration, upgrade, and troubleshooting are available from the navigation.',
    closingCta: 'Open getting started',
  },
};

function Arrow(): ReactNode {
  return <span aria-hidden="true">→</span>;
}

export default function Home(): ReactNode {
  const {i18n} = useDocusaurusContext();
  const locale = i18n.currentLocale === 'en' ? 'en' : 'fa';
  const text = copy[locale];
  const localized = (path: string) => path;
  const logo = useBaseUrl('/icon.svg');

  return (
    <Layout title={locale === 'fa' ? 'مستندات Qbit CLI' : 'Qbit CLI Docs'} description={text.subtitle}>
      <main className={styles.page}>
        <section className={styles.hero}>
          <div className={styles.heroGlow} aria-hidden="true" />
          <div className={styles.shell}>
            <div className={styles.heroGrid}>
              <div className={styles.heroCopy}>
                <img className={styles.heroLogo} src={logo} alt="Qbit CLI" width="112" height="112" />
                <div className={styles.badge}>{text.badge}</div>
                <h1>{text.title}</h1>
                <p>{text.subtitle}</p>
                <div className={styles.actions}>
                  <Link className="button button--primary button--lg" to={localized('/guide/getting-started')}>
                    {text.primaryCta}
                  </Link>
                  <Link className="button button--secondary button--lg" href="https://github.com/qbit-click/qbit-cli">
                    {text.githubCta}
                  </Link>
                </div>
              </div>

              <aside className={styles.heroPanel} aria-label={text.currentTitle}>
                <div className={styles.terminalHeader}>
                  <span />
                  <span />
                  <span />
                  <strong>qbit</strong>
                </div>
                <div className={styles.terminalBody}>
                  <div><span>$</span> qbit commands</div>
                  <div className={styles.terminalOutput}>install</div>
                  <div className={styles.terminalOutput}>run</div>
                  <div className={styles.terminalOutput}>py / js / dart</div>
                  <div className={styles.terminalOutput}>upgrade</div>
                  <div className={styles.terminalOutput}>qbit.yml / qbit.toml</div>
                  <div className={styles.terminalStatus}>✓ Windows / macOS / Linux</div>
                </div>
              </aside>
            </div>
          </div>
        </section>

        <section className={styles.section}>
          <div className={styles.shell}>
            <div className={styles.sectionHeading}>
              <span>{text.scopeEyebrow}</span>
              <h2>{text.scopeTitle}</h2>
              <p>{text.scopeDescription}</p>
            </div>

            <div className={styles.domainGrid}>
              {text.domains.map((domain) => (
                <Link key={domain.path} className={styles.domainCard} to={localized(domain.path)}>
                  <div className={styles.domainCode}>{domain.code}</div>
                  <h3>{domain.title}</h3>
                  <p>{domain.description}</p>
                  <div className={styles.cardLink}>
                    <span>{locale === 'fa' ? 'مشاهده بخش' : 'Explore section'}</span>
                    <Arrow />
                  </div>
                </Link>
              ))}
            </div>
          </div>
        </section>

        <section className={styles.assetSection}>
          <div className={styles.shell}>
            <div className={styles.assetCard}>
              <div>
                <span className={styles.eyebrow}>{text.currentEyebrow}</span>
                <h2>{text.currentTitle}</h2>
                <p>{text.currentDescription}</p>
                <div className={styles.actions}>
                  {downloads.map(([platform, asset]) => (
                    <a key={asset} className="button button--secondary" href={`${downloadBase}/${asset}`}>
                      {platform}
                    </a>
                  ))}
                </div>
              </div>
              <Link className={styles.assetLink} to={localized('/guide/download-install')}>
                {locale === 'fa' ? 'راهنمای دانلود و نصب' : 'Download and install guide'} <Arrow />
              </Link>
            </div>
          </div>
        </section>

        <section className={styles.section}>
          <div className={styles.shell}>
            <div className={styles.sectionHeading}>
              <span>{text.principlesEyebrow}</span>
              <h2>{text.principlesTitle}</h2>
            </div>
            <div className={styles.principleGrid}>
              {text.principles.map((principle, index) => (
                <article key={principle.title} className={styles.principleCard}>
                  <div className={styles.principleNumber}>0{index + 1}</div>
                  <h3>{principle.title}</h3>
                  <p>{principle.description}</p>
                </article>
              ))}
            </div>
          </div>
        </section>

        <section className={styles.closing}>
          <div className={styles.shell}>
            <div className={styles.closingInner}>
              <div>
                <h2>{text.closingTitle}</h2>
                <p>{text.closingDescription}</p>
              </div>
              <Link className="button button--primary button--lg" to={localized('/guide/getting-started')}>
                {text.closingCta}
              </Link>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
