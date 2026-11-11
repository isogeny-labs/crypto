// Render figures/route_closure_map.png from the SVG and REPORT.pdf from
// REPORT.html (pandoc output of REPORT.md) with the pre-installed Chromium.
// Usage: pandoc REPORT.md -s --metadata title="" -o REPORT.html && node render.js
const { chromium } = require('playwright');
const path = require('path');
(async () => {
  const here = __dirname;
  const b = await chromium.launch();
  const p = await b.newPage({ viewport: { width: 1500, height: 560 }, deviceScaleFactor: 2 });
  const svg = require('fs').readFileSync(path.join(here, 'figures', 'route_closure_map.svg'), 'utf8');
  await p.setContent('<!doctype html><html><body style="margin:0">' + svg + '</body></html>');
  await p.screenshot({ path: path.join(here, 'figures', 'route_closure_map.png'), clip: { x: 0, y: 0, width: 1500, height: 560 }, timeout: 90000 });
  const q = await b.newPage();
  await q.goto('file://' + path.join(here, 'REPORT.html'));
  await q.pdf({ path: path.join(here, 'REPORT.pdf'), format: 'A4',
                margin: { top: '16mm', bottom: '16mm', left: '16mm', right: '16mm' }, printBackground: true });
  await b.close();
  console.log('wrote figures/route_closure_map.png and REPORT.pdf');
})();
