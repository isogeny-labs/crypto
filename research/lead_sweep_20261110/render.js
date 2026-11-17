// Render figures/sweep_outcomes.png from the SVG and SWEEP_REPORT.pdf from
// SWEEP_REPORT.html (pandoc output) with the pre-installed Chromium.
// Usage: pandoc SWEEP_REPORT.md -s --metadata title=" " -o SWEEP_REPORT.html && node render.js
const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
(async () => {
  const here = __dirname;
  const b = await chromium.launch();
  const p = await b.newPage({ viewport: { width: 1500, height: 600 }, deviceScaleFactor: 2 });
  const svg = fs.readFileSync(path.join(here, 'figures', 'sweep_outcomes.svg'), 'utf8');
  await p.setContent('<!doctype html><html><body style="margin:0">' + svg + '</body></html>');
  await p.screenshot({ path: path.join(here, 'figures', 'sweep_outcomes.png'), clip: { x: 0, y: 0, width: 1500, height: 600 }, timeout: 90000 });
  const q = await b.newPage();
  await q.goto('file://' + path.join(here, 'SWEEP_REPORT.html'));
  await q.pdf({ path: path.join(here, 'SWEEP_REPORT.pdf'), format: 'A4',
                margin: { top: '16mm', bottom: '16mm', left: '16mm', right: '16mm' }, printBackground: true });
  await b.close();
  console.log('wrote figures/sweep_outcomes.png and SWEEP_REPORT.pdf');
})();
