import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, it } from "vitest";

const stylesheet = readFileSync(
  resolve(process.cwd(), "docs/.vitepress/theme/custom.css"),
  "utf8",
);

describe("RTL layout contract", () => {
  it("uses logical inline properties instead of physical horizontal spacing", () => {
    const forbiddenPhysicalProperties = [
      /\bpadding-(?:left|right)\s*:/g,
      /\bmargin-(?:left|right)\s*:/g,
      /\bborder-(?:left|right)(?:-[a-z-]+)?\s*:/g,
      /(?:^|[;{]\s*)(?:left|right)\s*:/gm,
    ];

    for (const pattern of forbiddenPhysicalProperties) {
      expect(stylesheet.match(pattern) ?? [], pattern.source).toEqual([]);
    }
  });

  it("defines RTL overrides with logical start/end primitives", () => {
    expect(stylesheet).toContain("padding-inline-start");
    expect(stylesheet).toContain("padding-inline-end");
    expect(stylesheet).toContain("margin-inline-start");
    expect(stylesheet).toContain("margin-inline-end");
    expect(stylesheet).toContain("inset-inline-start");
    expect(stylesheet).toContain("inset-inline-end");
    expect(stylesheet).toContain("border-inline-start");
  });
});
