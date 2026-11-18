// Render the SVG graphs to PNG and report.html to report.pdf with headless Chromium (Playwright); layout only.
//   PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers node analysis/render.mjs   (cwd = study directory)
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const require = createRequire(import.meta.url);
const { chromium } = require(process.env.PLAYWRIGHT_MODULE ?? "/opt/node22/lib/node_modules/playwright");
const browser = await chromium.launch();
const page = await browser.newPage();
for (const f of ["ratios", "gap"]) {
  const svg = readFileSync(`${f}.svg`, "utf8");
  const [, w, h] = /width="(\d+)" height="(\d+)"/.exec(svg);
  await page.setViewportSize({ width: Number(w), height: Number(h) });
  await page.setContent(`<html><body style="margin:0">${svg}</body></html>`);
  await page.screenshot({ path: `${f}.png`, clip: { x: 0, y: 0, width: Number(w), height: Number(h) } });
}
await page.goto("file://" + resolve("report.html"));
await page.pdf({ path: "report.pdf", format: "A4", landscape: true, printBackground: true, margin: { top: "12mm", bottom: "12mm", left: "10mm", right: "10mm" } });
await browser.close();
console.log("ratios.png gap.png report.pdf");
