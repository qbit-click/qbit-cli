import { readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, it } from "vitest";

const dist = resolve(process.cwd(), "build");
const read = (path: string) => readFileSync(resolve(dist, path), "utf8");
const findSearchIndex = (localeDir = "") => {
  const dir = resolve(dist, localeDir);
  const file = readdirSync(dir).find((entry) => /^search-index-[a-f0-9]+\.json$/.test(entry));
  if (!file) throw new Error(`search index missing for ${localeDir || "fa"}`);
  return readFileSync(resolve(dir, file), "utf8");
};

describe("Qbit CLI Docusaurus build", () => {
  it("renders Persian RTL and English LTR output", () => {
    const fa = read("index.html");
    const en = read("en/index.html");
    expect(fa).toContain('lang=fa-IR');
    expect(fa).toContain('dir=rtl');
    expect(en).toContain('lang=en-US');
    expect(en).toContain('dir=ltr');
  });

  it("renders direct latest-release download links", () => {
    const fa = read("guide/download-install.html");
    const en = read("en/guide/download-install.html");
    for (const asset of ["qbit-windows-setup.zip", "qbit-macos-setup.tar.gz", "qbit-linux-setup.tar.gz"]) {
      const url = `https://github.com/qbit-click/qbit-cli/releases/latest/download/${asset}`;
      expect(fa).toContain(url);
      expect(en).toContain(url);
    }
  });

  it("ships command docs, local search indexes, and the Qbit icon", () => {
    expect(read("guide/javascript.html")).toContain("QBIT_JS_PM");
    expect(read("en/guide/configuration.html")).toContain("QBIT_PACKAGE_MANAGER");
    expect(findSearchIndex()).toContain("JavaScript");
    expect(findSearchIndex("en")).toContain("JavaScript");
    expect(read("icon.svg")).toContain("svg");
  });
});
