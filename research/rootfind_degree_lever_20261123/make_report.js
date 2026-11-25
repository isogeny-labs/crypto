// Render figures/dF_vs_n.png (1x) and report.pdf from report.html + the SVG.
// NODE_PATH=$(npm root -g) node make_report.js
const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
(async () => {
  const dir = __dirname;
  const svg = fs.readFileSync(path.join(dir, 'figures', 'dF_vs_n.svg'), 'utf8');
  const b = await chromium.launch();
  const p = await b.newPage({ viewport: { width: 980, height: 430 }, deviceScaleFactor: 1 });
  await p.setContent(`<html><body style="margin:0">${svg}</body></html>`);
  await p.screenshot({ path: path.join(dir, 'figures', 'dF_vs_n.png'), clip: { x: 0, y: 0, width: 980, height: 430 } });
  const html = fs.readFileSync(path.join(dir, 'report.html'), 'utf8').replace('<!--FIGURE-->', svg);
  await p.setContent(html);
  await p.pdf({ path: path.join(dir, 'report.pdf'), format: 'A4', printBackground: true,
    margin: { top: '14mm', bottom: '14mm', left: '12mm', right: '12mm' } });
  await b.close();
  console.log('ok');
})().catch(e => { console.error(e); process.exit(1); });
