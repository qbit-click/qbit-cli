import {describe, expect, it} from 'vitest';

import {resolvePagesDeployment} from '../../src/utils/pages-deployment';

describe('GitHub Pages deployment resolution', () => {
  it('keeps local documentation defaults at the site root', () => {
    expect(resolvePagesDeployment()).toEqual({
      siteUrl: 'https://qbit-hub.github.io',
      basePath: '/',
    });
  });

  it('uses the configured custom-domain origin at the site root', () => {
    expect(
      resolvePagesDeployment({
        origin: 'https://docs.example.com/',
        baseUrl: 'https://docs.example.com/',
        basePath: '',
      }),
    ).toEqual({siteUrl: 'https://docs.example.com', basePath: '/'});
  });

  it('keeps the project Pages fallback path', () => {
    expect(
      resolvePagesDeployment({
        origin: 'https://qbit-hub.github.io',
        baseUrl: 'https://qbit-hub.github.io/qbit-cli',
        basePath: 'qbit-cli',
      }),
    ).toEqual({siteUrl: 'https://qbit-hub.github.io', basePath: '/qbit-cli/'});
  });
});
