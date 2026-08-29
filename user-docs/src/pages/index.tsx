import type {ReactNode} from 'react';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import styles from './index.module.css';

const downloadBase = 'https://github.com/qbit-click/qbit-cli/releases/latest/download';
const downloads = [
  ['Windows', 'qbit-windows-setup.zip'],
  ['macOS', 'qbit-macos-setup.tar.gz'],
  ['Linux', 'qbit-linux-setup.tar.gz'],
] as const;

const copy = {
  fa: {
    title: 'محیط توسعه و اتوماسیون، با یک دستور',
    tagline: 'ابزارهای سیستم را نصب کنید، پروژه‌های Python/JavaScript/Dart را مدیریت کنید و workflowهای تکراری را از یک CLI چندسکویی اجرا کنید.',
    primary: 'دانلود Qbit CLI',
    secondary: 'شروع کار',
    features: [
      ['Windows، macOS و Linux', 'هر release شامل setup archive مخصوص سیستم‌عامل و installer متناظر است.'],
      ['نصب ابزارهای سیستم', 'Qbit package manager سیستم را تشخیص می‌دهد و command مناسب را برای ابزار موردنظر می‌سازد.'],
      ['مدیریت پروژه', 'Python virtualenv، JavaScript package manager و Dart pub را از commandهای یکپارچه Qbit کنترل کنید.'],
      ['qbit.yml / qbit.toml', 'workflowهای پروژه و mapping نصب ابزارها را به صورت قابل تکرار در repository نگه دارید.'],
    ],
    downloadTitle: 'دانلود آخرین نسخه',
    platform: 'سیستم‌عامل',
    direct: 'دانلود مستقیم آخرین release',
    installHelp: 'برای دستور نصب هر سیستم‌عامل، راهنمای دانلود و نصب را ببینید.',
  },
  en: {
    title: 'One CLI for development setup and automation',
    tagline: 'Install system tools, manage Python/JavaScript/Dart projects, and run repeatable project workflows from a cross-platform command line.',
    primary: 'Download Qbit CLI',
    secondary: 'Get started',
    features: [
      ['Windows, macOS, and Linux', 'Every release ships a platform-specific setup archive and installer.'],
      ['System dependencies', 'Qbit detects the native package manager and builds the appropriate install command.'],
      ['Project management', 'Manage Python virtualenvs, JavaScript package managers, and Dart pub through consistent commands.'],
      ['qbit.yml / qbit.toml', 'Keep reusable project workflows and package mappings in version control.'],
    ],
    downloadTitle: 'Download the latest release',
    platform: 'Platform',
    direct: 'Direct latest download',
    installHelp: 'See Download and install for platform-specific steps.',
  },
} as const;

export default function Home(): ReactNode {
  const {i18n} = useDocusaurusContext();
  const locale = i18n.currentLocale === 'en' ? 'en' : 'fa';
  const text = copy[locale];

  return (
    <Layout title={locale === 'fa' ? 'مستندات Qbit CLI' : 'Qbit CLI Docs'} description={text.tagline}>
      <main className={styles.page}>
        <section className={styles.hero}>
          <div className={styles.heroInner}>
            <div className={styles.heroCopy}>
              <div className={styles.productName}>Qbit CLI</div>
              <h1>{text.title}</h1>
              <p>{text.tagline}</p>
              <div className={styles.actions}>
                <Link className="button button--primary button--lg" to="/guide/download-install">{text.primary}</Link>
                <Link className="button button--secondary button--lg" to="/guide/getting-started">{text.secondary}</Link>
              </div>
            </div>
          </div>
        </section>
        <section className={styles.features}>
          <div className={styles.featureGrid}>
            {text.features.map(([title, details]) => (
              <article className={styles.feature} key={title}>
                <h2>{title}</h2>
                <p>{details}</p>
              </article>
            ))}
          </div>
        </section>
        <section className={styles.contentSection}>
          <h2>{text.downloadTitle}</h2>
          <div className="table-responsive">
            <table>
              <thead><tr><th>{text.platform}</th><th>{text.direct}</th></tr></thead>
              <tbody>
                {downloads.map(([platform, asset]) => (
                  <tr key={platform}>
                    <td>{platform}</td>
                    <td><a href={`${downloadBase}/${asset}`}>{asset}</a></td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <p><Link to="/guide/download-install">{text.installHelp}</Link></p>
        </section>
      </main>
    </Layout>
  );
}
