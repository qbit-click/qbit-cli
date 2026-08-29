import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, it } from "vitest";

const pkg = JSON.parse(readFileSync(resolve(process.cwd(), "package.json"), "utf8"));
const config = readFileSync(resolve(process.cwd(), "docusaurus.config.ts"), "utf8");

describe("AI Toolkit documentation technical reference", () => {
  it("matches the canonical documentation platform versions", () => {
    expect(pkg.packageManager).toBe("bun@1.4.0");
    expect(pkg.dependencies["@docusaurus/core"]).toBe("3.10.2");
    expect(pkg.dependencies["@docusaurus/faster"]).toBe("3.10.2");
    expect(pkg.dependencies["@docusaurus/preset-classic"]).toBe("3.10.2");
    expect(pkg.dependencies["@easyops-cn/docusaurus-search-local"]).toBe("0.55.3");
    expect(pkg.dependencies["@fontsource-variable/vazirmatn"]).toBe("5.3.0");
    expect(pkg.devDependencies["@playwright/test"]).toBe("1.62.1");
    expect(pkg.devDependencies.vitest).toBe("4.1.11");
    expect(pkg.engines.node).toBe(">=20.0");
  });

  it("keeps the canonical Docusaurus compatibility contract", () => {
    expect(config).toContain("future: {");
    expect(config).toContain("v4: true");
    expect(config).toContain("'@easyops-cn/docusaurus-search-local'");
    expect(config).toContain("highlightSearchTermsOnTargetPage: true");
    expect(config).toContain("explicitSearchResultPath: true");
  });
});
