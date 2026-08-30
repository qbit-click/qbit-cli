import {expect, test, type Page} from '@playwright/test';

async function expectTemplateChrome(page: Page, direction: 'ltr' | 'rtl') {
  await page.evaluate(() => document.fonts.ready);
  const font = await page.evaluate(() => ({
    ready: document.fonts.check('16px "Vazirmatn Variable"'),
    heading: getComputedStyle(document.querySelector('h1')!).fontFamily,
  }));
  expect(font.ready).toBe(true);
  expect(font.heading).toContain('Vazirmatn Variable');

  const hint = page.locator('.navbar__search [class*="searchHintContainer"]');
  await expect(hint).toBeVisible();
  const search = await page.locator('.navbar__search').evaluate((root) => {
    const input = root.querySelector<HTMLInputElement>('.navbar__search-input')!;
    const icon = root.querySelector<SVGElement>(':scope > svg')!;
    const shortcut = root.querySelector<HTMLElement>('[class*="searchHintContainer"]')!;
    const inputRect = input.getBoundingClientRect();
    const iconRect = icon.getBoundingClientRect();
    const shortcutRect = shortcut.getBoundingClientRect();
    const style = getComputedStyle(input);
    return {
      direction: style.direction,
      textAlign: style.textAlign,
      backgroundImage: style.backgroundImage,
      inputCenter: inputRect.left + inputRect.width / 2,
      iconCenter: iconRect.left + iconRect.width / 2,
      shortcutCenter: shortcutRect.left + shortcutRect.width / 2,
      separated: iconRect.right <= shortcutRect.left || shortcutRect.right <= iconRect.left,
    };
  });

  expect(search.direction).toBe(direction);
  expect(search.backgroundImage).toBe('none');
  expect(search.separated).toBe(true);
  if (direction === 'rtl') {
    expect(search.textAlign).toBe('right');
    expect(search.iconCenter).toBeGreaterThan(search.inputCenter);
    expect(search.shortcutCenter).toBeLessThan(search.inputCenter);
  } else {
    expect(search.iconCenter).toBeLessThan(search.inputCenter);
    expect(search.shortcutCenter).toBeGreaterThan(search.inputCenter);
  }
}

test('Persian home is RTL and exposes direct downloads', async ({page}) => {
  await page.setViewportSize({width: 1280, height: 720});
  await page.goto('/');
  await expect(page).toHaveTitle(/مستندات Qbit CLI/);
  await expect(page.locator('html')).toHaveAttribute('lang', 'fa-IR');
  await expect(page.locator('html')).toHaveAttribute('dir', 'rtl');
  await expect(page.getByText('محیط توسعه و اتوماسیون، با یک دستور')).toBeVisible();
  await expect(page.locator('.navbar__search-input')).toBeVisible();
  await expectTemplateChrome(page, 'rtl');

  const windowsLink = page.locator(
    'a[href="https://github.com/qbit-click/qbit-cli/releases/latest/download/qbit-windows-setup.zip"]',
  );
  await expect(windowsLink).toHaveCount(1);
  await expect(page.getByRole('complementary', {name: 'آخرین نسخه Qbit CLI'})).toBeVisible();

  const logoSpacing = await page.locator('.navbar__logo').evaluate((node) => {
    const style = getComputedStyle(node);
    return {left: style.marginLeft, right: style.marginRight};
  });
  expect(logoSpacing).toEqual({left: '8px', right: '0px'});
});

test('English documentation is available as LTR', async ({page}) => {
  await page.setViewportSize({width: 1280, height: 720});
  await page.goto('/en/');
  await expect(page).toHaveTitle(/Qbit CLI Docs/);
  await expect(page.locator('html')).toHaveAttribute('lang', 'en-US');
  await expect(page.locator('html')).toHaveAttribute('dir', 'ltr');
  await expect(page.getByText('One CLI for development setup and automation')).toBeVisible();
  await expect(page.locator('.navbar__search-input')).toBeVisible();
  await expectTemplateChrome(page, 'ltr');

  const logoSpacing = await page.locator('.navbar__logo').evaluate((node) => {
    const style = getComputedStyle(node);
    return {left: style.marginLeft, right: style.marginRight};
  });
  expect(logoSpacing).toEqual({left: '0px', right: '8px'});
});

test('command guidance routes render their documented runtime contract', async ({page}) => {
  await page.goto('/guide/javascript');
  await expect(page).toHaveURL(/\/guide\/javascript$/);
  await expect(page.getByRole('heading', {name: 'JavaScript', level: 1})).toBeVisible();
  await expect(page.getByText('QBIT_JS_PM').first()).toBeVisible();
});

test('local search returns CLI documentation in both locales', async ({page}) => {
  await page.goto('/search?q=JavaScript');
  await expect(page.getByRole('link', {name: /JavaScript/i}).first()).toBeVisible();

  await page.goto('/en/search?q=JavaScript');
  await expect(page.getByRole('link', {name: /JavaScript/i}).first()).toBeVisible();
});
