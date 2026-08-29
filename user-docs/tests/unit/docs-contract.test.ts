import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, it } from "vitest";

import { enNav, enSidebar, faNav, faSidebar, latestDownloadUrl, releaseAssets } from "../../docs/.vitepress/site";

const repoRoot = resolve(process.cwd(), "..");
const docsRoot = resolve(process.cwd(), "docs");
const readRepo = (path: string) => readFileSync(resolve(repoRoot, path), "utf8");
const readDocs = (path: string) => readFileSync(resolve(docsRoot, path), "utf8");

const markdownPathForLink = (link: string) => resolve(docsRoot, `${link.replace(/^\//, "")}.md`);

describe("Qbit CLI user documentation contracts", () => {
  it("keeps Persian and English navigation in parity and points at real pages", () => {
    expect(faSidebar).toHaveLength(enSidebar.length);
    expect(faSidebar).toHaveLength(10);
    expect(enSidebar.map(({ link }) => link.replace(/^\/en\//, "/"))).toEqual(faSidebar.map(({ link }) => link));

    for (const item of [...faSidebar, ...enSidebar]) {
      expect(existsSync(markdownPathForLink(item.link)), item.link).toBe(true);
    }

    const sidebarLinks = new Set([...faSidebar, ...enSidebar].map(({ link }) => link));
    for (const item of [...faNav, ...enNav]) expect(sidebarLinks.has(item.link), item.link).toBe(true);
  });

  it("keeps download links aligned with the release and upgrader asset contract", () => {
    const releaseWorkflow = readRepo(".github/workflows/release.yml");
    const upgrader = readRepo("src/os/upgrade.rs");
    const faDownload = readDocs("guide/download-install.md");
    const enDownload = readDocs("en/guide/download-install.md");

    expect(releaseWorkflow).toContain("qbit-${{ matrix.artifact }}-setup.zip");
    expect(releaseWorkflow).toContain("qbit-${{ matrix.artifact }}-setup.tar.gz");

    for (const asset of Object.values(releaseAssets)) {
      expect(upgrader).toContain(asset);
      expect(faDownload).toContain(latestDownloadUrl(asset));
      expect(enDownload).toContain(latestDownloadUrl(asset));
    }
  });

  it("documents every stable top-level CLI command", () => {
    const cli = readRepo("src/cli.rs");
    const gettingStarted = `${readDocs("guide/getting-started.md")}\n${readDocs("en/guide/getting-started.md")}`;

    for (const [variant, command] of [["Install", "qbit install"], ["Run", "qbit run"], ["Py", "qbit py"], ["Js", "qbit js"], ["Dart", "qbit dart"], ["Upgrade", "qbit upgrade"]] as const) {
      expect(cli).toMatch(new RegExp(`\\b${variant}\\b`));
      expect(gettingStarted).toContain(command);
    }
  });
});
