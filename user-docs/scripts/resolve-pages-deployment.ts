import {appendFileSync} from 'node:fs';

import {resolvePagesDeployment} from '../src/utils/pages-deployment';

const deployment = resolvePagesDeployment({
  origin: process.env.PAGES_ORIGIN,
  baseUrl: process.env.PAGES_BASE_URL,
  basePath: process.env.PAGES_BASE_PATH,
});

const output = `site_url=${deployment.siteUrl}\nbase_path=${deployment.basePath}\n`;

if (process.env.GITHUB_OUTPUT) {
  appendFileSync(process.env.GITHUB_OUTPUT, output);
} else {
  process.stdout.write(output);
}
