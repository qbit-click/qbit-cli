import { expect, test } from "@playwright/test";

test("Persian home is RTL and exposes direct downloads", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto("/");
  await expect(page).toHaveTitle(/مستندات Qbit CLI/);
  await expect(page.locator("html")).toHaveAttribute("lang", "fa-IR");
  await expect(page.locator("html")).toHaveAttribute("dir", "rtl");
  await expect(page.getByText("محیط توسعه و اتوماسیون، با یک دستور")).toBeVisible();
  await expect(page.locator(".navbar__search-input")).toBeVisible();

  const searchDirection = await page.locator(".navbar__search-input").evaluate((node) => {
    const style = getComputedStyle(node);
    return { direction: style.direction, textAlign: style.textAlign };
  });
  expect(searchDirection).toEqual({ direction: "rtl", textAlign: "right" });

  const windowsLink = page.locator(
    'a[href="https://github.com/qbit-click/qbit-cli/releases/latest/download/qbit-windows-setup.zip"]',
  );
  await expect(windowsLink).toHaveCount(1);
  await expect(page.getByRole("complementary", { name: "آخرین نسخه Qbit CLI" })).toBeVisible();

  const logoSpacing = await page.locator(".navbar__logo").evaluate((node) => {
    const style = getComputedStyle(node);
    return { left: style.marginLeft, right: style.marginRight };
  });
  expect(logoSpacing).toEqual({ left: "8px", right: "0px" });
});

test("English documentation is available as LTR", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto("/en/");
  await expect(page).toHaveTitle(/Qbit CLI Docs/);
  await expect(page.locator("html")).toHaveAttribute("lang", "en-US");
  await expect(page.locator("html")).toHaveAttribute("dir", "ltr");
  await expect(page.getByText("One CLI for development setup and automation")).toBeVisible();
  await expect(page.locator(".navbar__search-input")).toBeVisible();

  const logoSpacing = await page.locator(".navbar__logo").evaluate((node) => {
    const style = getComputedStyle(node);
    return { left: style.marginLeft, right: style.marginRight };
  });
  expect(logoSpacing).toEqual({ left: "0px", right: "8px" });
});

test("command guidance routes render their documented runtime contract", async ({ page }) => {
  await page.goto("/guide/javascript");
  await expect(page).toHaveURL(/\/guide\/javascript$/);
  await expect(page.getByRole("heading", { name: "JavaScript", level: 1 })).toBeVisible();
  await expect(page.getByText("QBIT_JS_PM").first()).toBeVisible();
});

test("local search returns CLI documentation in both locales", async ({ page }) => {
  await page.goto("/search?q=JavaScript");
  await expect(page.getByRole("link", { name: /JavaScript/i }).first()).toBeVisible();

  await page.goto("/en/search?q=JavaScript");
  await expect(page.getByRole("link", { name: /JavaScript/i }).first()).toBeVisible();
});
