import { expect, test } from "@playwright/test";

test("Persian home is RTL and exposes direct downloads", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto("/");
  await expect(page).toHaveTitle(/مستندات Qbit CLI/);
  await expect(page.locator("html")).toHaveAttribute("lang", "fa-IR");
  await expect(page.locator("html")).toHaveAttribute("dir", "rtl");
  await expect(page.getByText("محیط توسعه و اتوماسیون، با یک دستور")).toBeVisible();

  const windowsLink = page.getByRole("link", { name: "qbit-windows-setup.zip" }).first();
  await expect(windowsLink).toHaveAttribute(
    "href",
    "https://github.com/qbit-click/qbit-cli/releases/latest/download/qbit-windows-setup.zip",
  );

  const logoSpacing = await page.locator(".VPNavBarTitle .logo").evaluate((node) => {
    const style = getComputedStyle(node);
    return { start: style.marginRight, end: style.marginLeft };
  });
  expect(logoSpacing).toEqual({ start: "0px", end: "8px" });

  const searchSpacing = await page.locator(".VPNavBarSearch").evaluate((node) => {
    const style = getComputedStyle(node);
    return { start: style.paddingRight, end: style.paddingLeft };
  });
  expect(searchSpacing).toEqual({ start: "32px", end: "0px" });
});

test("English documentation is available as LTR", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto("/en/");
  await expect(page).toHaveTitle(/Qbit CLI Docs/);
  await expect(page.locator("html")).toHaveAttribute("lang", "en-US");
  await expect(page.locator("html")).toHaveAttribute("dir", "ltr");
  await expect(page.getByText("One CLI for development setup and automation")).toBeVisible();

  const logoSpacing = await page.locator(".VPNavBarTitle .logo").evaluate((node) => {
    const style = getComputedStyle(node);
    return { start: style.marginLeft, end: style.marginRight };
  });
  expect(logoSpacing).toEqual({ start: "0px", end: "8px" });

  const searchSpacing = await page.locator(".VPNavBarSearch").evaluate((node) => {
    const style = getComputedStyle(node);
    return { start: style.paddingLeft, end: style.paddingRight };
  });
  expect(searchSpacing).toEqual({ start: "32px", end: "0px" });
});

test("command guidance routes render their documented runtime contract", async ({ page }) => {
  await page.goto("/guide/javascript");
  await expect(page).toHaveURL(/\/guide\/javascript$/);
  await expect(page.getByRole("heading", { name: "JavaScript", level: 1 })).toBeVisible();
  await expect(page.getByText("QBIT_JS_PM").first()).toBeVisible();
});

test("desktop navigation spacing mirrors correctly between RTL and LTR", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 900 });

  const readNavigationSpacing = () =>
    page.evaluate(() => {
      const logo = document.querySelector<HTMLElement>(".VPNavBarTitle .logo");
      const search = document.querySelector<HTMLElement>(".VPNavBarSearch");
      if (!logo || !search) throw new Error("navigation elements not found");

      const logoStyle = getComputedStyle(logo);
      const searchStyle = getComputedStyle(search);
      return {
        logoMarginLeft: logoStyle.marginLeft,
        logoMarginRight: logoStyle.marginRight,
        searchPaddingLeft: searchStyle.paddingLeft,
        searchPaddingRight: searchStyle.paddingRight,
      };
    });

  await page.goto("/");
  await expect.poll(readNavigationSpacing).toEqual({
    logoMarginLeft: "8px",
    logoMarginRight: "0px",
    searchPaddingLeft: "0px",
    searchPaddingRight: "32px",
  });

  await page.goto("/en/");
  await expect.poll(readNavigationSpacing).toEqual({
    logoMarginLeft: "0px",
    logoMarginRight: "8px",
    searchPaddingLeft: "32px",
    searchPaddingRight: "0px",
  });
});
