import { expect, test } from "@playwright/test";

test("Persian home is RTL and exposes direct downloads", async ({ page }) => {
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
});

test("English documentation is available as LTR", async ({ page }) => {
  await page.goto("/en/");
  await expect(page).toHaveTitle(/Qbit CLI Docs/);
  await expect(page.locator("html")).toHaveAttribute("lang", "en-US");
  await expect(page.locator("html")).toHaveAttribute("dir", "ltr");
  await expect(page.getByText("One CLI for development setup and automation")).toBeVisible();
});

test("command guidance routes render their documented runtime contract", async ({ page }) => {
  await page.goto("/guide/javascript");
  await expect(page).toHaveURL(/\/guide\/javascript$/);
  await expect(page.getByRole("heading", { name: "JavaScript", level: 1 })).toBeVisible();
  await expect(page.getByText("QBIT_JS_PM").first()).toBeVisible();
});
