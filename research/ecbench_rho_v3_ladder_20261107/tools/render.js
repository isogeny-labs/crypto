// Render a report directory: REPORT.md (via pandoc to HTML, SVGs inline as
// images) to REPORT.pdf, and every *.svg to a .png beside it. Layout only.
// Usage: node render.js <report-dir>
const { chromium } = require('playwright');
const { execFileSync } = require('child_process');
const fs = require('fs');
const path = require('path');
(async () => {
  const dir = path.resolve(process.argv[2]);
  const html = path.join(dir, 'REPORT.html');
  execFileSync('pandoc', [path.join(dir, 'REPORT.md'), '-s', '-o', html,
    '--metadata', 'pagetitle=report',
    '-c', 'data:text/css,body{font-family:sans-serif;max-width:46em;margin:auto;font-size:11pt}table{border-collapse:collapse;font-size:9pt}td,th{border:1px solid %23bbb;padding:2px 5px}img{max-width:100%25}code{font-size:9pt}']);
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto('file://' + html);
  await page.pdf({ path: path.join(dir, 'REPORT.pdf'), format: 'A4', margin: { top: '15mm', bottom: '15mm', left: '12mm', right: '12mm' } });
  for (const f of fs.readdirSync(dir).filter((f) => f.endsWith('.svg'))) {
    const p = await browser.newPage({ deviceScaleFactor: 2 });
    await p.goto('file://' + path.join(dir, f));
    const el = await p.$('svg');
    await el.screenshot({ path: path.join(dir, f.replace(/\.svg$/, '.png')) });
    await p.close();
  }
  await browser.close();
  fs.unlinkSync(html);
})();
