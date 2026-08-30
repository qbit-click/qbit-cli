import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';

import {describe, expect, it} from 'vitest';

const pkg = JSON.parse(readFileSync(resolve(process.cwd(), 'package.json'), 'utf8'));
const config = readFileSync(resolve(process.cwd(), 'docusaurus.config.ts'), 'utf8');
const homepage = readFileSync(resolve(process.cwd(), 'src/pages/index.tsx'), 'utf8');
const templateSource = JSON.parse(readFileSync(resolve(process.cwd(), 'template.source.json'), 'utf8'));
const sha256 = (path: string) =>
  createHash('sha256').update(readFileSync(resolve(process.cwd(), path))).digest('hex');

describe('Qbit AI Toolkit documentation template contract', () => {
  it('pins this site to the canonical AI Toolkit template revision', () => {
    expect(templateSource).toMatchObject({
      template: 'qbit-documentation',
      templateVersion: '1.0.1',
      source: {
        repository: 'https://github.com/qbit-click/qbit-ai-toolkit.git',
        path: 'website',
        revision: '4b033caa1dafbfd6b6e5104512e98d3f95b8c3d5',
      },
    });
  });

  it('keeps byte-identical shared presentation files from the template', () => {
    for (const [path, expected] of Object.entries(templateSource.sharedFileHashes) as Array<[string, string]>) {
      expect(sha256(path), path).toBe(expected);
    }
  });

  it('clones the canonical homepage shell instead of using a product-specific layout', () => {
    for (const token of ['styles.heroGrid', 'styles.heroPanel', 'styles.domainGrid', 'styles.assetSection', 'styles.principleGrid', 'styles.closing']) {
      expect(homepage, token).toContain(token);
    }
  });

  it('matches the canonical documentation toolchain and compatibility contract', () => {
    expect(pkg.packageManager).toBe('bun@1.4.0');
    expect(pkg.dependencies['@docusaurus/core']).toBe('3.10.2');
    expect(pkg.dependencies['@docusaurus/faster']).toBe('3.10.2');
    expect(pkg.dependencies['@docusaurus/preset-classic']).toBe('3.10.2');
    expect(pkg.dependencies['@easyops-cn/docusaurus-search-local']).toBe('0.55.3');
    expect(pkg.dependencies['@fontsource-variable/vazirmatn']).toBe('5.3.0');
    expect(pkg.devDependencies['@playwright/test']).toBe('1.62.1');
    expect(pkg.devDependencies.vitest).toBe('4.1.11');
    expect(pkg.engines.node).toBe('>=20.0');
    expect(config).toContain('future: {');
    expect(config).toContain('v4: true');
    expect(config).toContain("'@easyops-cn/docusaurus-search-local'");
    expect(config).toContain('highlightSearchTermsOnTargetPage: true');
    expect(config).toContain('explicitSearchResultPath: true');
  });
});
