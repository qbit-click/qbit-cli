import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, it } from "vitest";

const stylesheets = [
  resolve(process.cwd(), "src/css/custom.css"),
  resolve(process.cwd(), "src/pages/index.module.css"),
].map((path) => readFileSync(path, "utf8")).join("\n");

describe("RTL layout contract", () => {
  it("uses logical inline properties instead of physical horizontal spacing", () => {
    const forbiddenPhysicalProperties = [
      /\bpadding-(?:left|right)\s*:/g,
      /\bmargin-(?:left|right)\s*:/g,
      /\bborder-(?:left|right)(?:-[a-z-]+)?\s*:/g,
      /(?:^|[;{]\s*)(?:left|right)\s*:/gm,
    ];

    for (const pattern of forbiddenPhysicalProperties) {
      expect(stylesheets.match(pattern) ?? [], pattern.source).toEqual([]);
    }
  });

  it("keeps horizontal spacing direction-aware", () => {
    expect(stylesheets).toContain("padding-inline");
    expect(stylesheets).toContain("margin-inline");
    expect(stylesheets).toContain("border-inline-start");
    expect(stylesheets).toContain("border-inline-end");
    expect(stylesheets).toContain("text-align: start");
  });
});
