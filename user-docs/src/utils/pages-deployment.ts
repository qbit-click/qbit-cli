export const DEFAULT_SITE_URL = 'https://qbit-hub.github.io';
export const DEFAULT_BASE_PATH = '/';

export type PagesDeploymentMetadata = {
  origin?: string;
  baseUrl?: string;
  basePath?: string;
};

export type PagesDeployment = {
  siteUrl: string;
  basePath: string;
};

const normalizeUrl = (value: string) => new URL(value).toString().replace(/\/$/, '');

const normalizeBasePath = (value: string) => {
  const path = value.trim().replace(/^\/+|\/+$/g, '');
  return path ? `/${path}/` : DEFAULT_BASE_PATH;
};

/**
 * Resolves the Docusaurus origin and base path from configure-pages metadata.
 */
export const resolvePagesDeployment = ({
  origin,
  baseUrl,
  basePath,
}: PagesDeploymentMetadata = {}): PagesDeployment => {
  const siteUrl = origin ? normalizeUrl(origin) : DEFAULT_SITE_URL;

  if (baseUrl && normalizeUrl(baseUrl) === siteUrl) {
    return {siteUrl, basePath: DEFAULT_BASE_PATH};
  }

  return {siteUrl, basePath: basePath ? normalizeBasePath(basePath) : DEFAULT_BASE_PATH};
};
