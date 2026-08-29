import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, it } from "vitest";

const dist = resolve(process.cwd(), "docs/.vitepress/dist");
const read = (path: string) => readFileSync(resolve(dist, path), "utf8");

describe("Qbit CLI documentation build", () => {
  it("renders Persian RTL and English LTR output", () => {
    const fa = read("index.html");
    const en = read("en/index.html");
    expect(fa).toContain('lang="fa-IR"');
    expect(fa).toContain('dir="rtl"');
    expect(en).toContain('lang="en-US"');
    expect(en).toContain('dir="ltr"');
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

  it("ships representative command documentation and the Qbit icon", () => {
    expect(read("guide/javascript.html")).toContain("QBIT_JS_PM");
    expect(read("en/guide/configuration.html")).toContain("QBIT_PACKAGE_MANAGER");
    expect(read("icon.svg")).toContain("svg");
  });
});
