import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, it } from "vitest";

const repoRoot = resolve(process.cwd(), "..");
const docsRoot = resolve(process.cwd(), "docs");
const enDocsRoot = resolve(process.cwd(), "i18n/en/docusaurus-plugin-content-docs/current");
const readRepo = (path: string) => readFileSync(resolve(repoRoot, path), "utf8");
const readDocs = (path: string) => readFileSync(resolve(docsRoot, path), "utf8");
const readEnDocs = (path: string) => readFileSync(resolve(enDocsRoot, path), "utf8");

const docIds = [
  "getting-started",
  "download-install",
  "system-packages",
  "project-scripts",
  "python",
  "javascript",
  "dart",
  "upgrade",
  "configuration",
  "troubleshooting",
] as const;

const releaseAssets = {
  windows: "qbit-windows-setup.zip",
  macos: "qbit-macos-setup.tar.gz",
  linux: "qbit-linux-setup.tar.gz",
} as const;

const latestDownloadUrl = (asset: string) =>
  `https://github.com/qbit-click/qbit-cli/releases/latest/download/${asset}`;

describe("Qbit CLI user documentation contracts", () => {
  it("keeps Persian and English guide coverage in parity and preserves routes", () => {
    expect(docIds).toHaveLength(10);
    for (const id of docIds) {
      expect(existsSync(resolve(docsRoot, `${id}.md`)), `fa:${id}`).toBe(true);
      expect(existsSync(resolve(enDocsRoot, `${id}.md`)), `en:${id}`).toBe(true);
    }

    expect(docIds.map((id) => `/guide/${id}`)[0]).toBe("/guide/getting-started");
    expect(docIds.map((id) => `/en/guide/${id}`)[0]).toBe("/en/guide/getting-started");
  });

  it("keeps download links aligned with the release and upgrader asset contract", () => {
    const releaseWorkflow = readRepo(".github/workflows/release.yml");
    const upgrader = readRepo("src/os/upgrade.rs");
    const faDownload = readDocs("download-install.md");
    const enDownload = readEnDocs("download-install.md");

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
    const gettingStarted = `${readDocs("getting-started.md")}\n${readEnDocs("getting-started.md")}`;

    for (const [variant, command] of [["Install", "qbit install"], ["Run", "qbit run"], ["Py", "qbit py"], ["Js", "qbit js"], ["Dart", "qbit dart"], ["Upgrade", "qbit upgrade"]] as const) {
      expect(cli).toMatch(new RegExp(`\\b${variant}\\b`));
      expect(gettingStarted).toContain(command);
    }
  });
});
