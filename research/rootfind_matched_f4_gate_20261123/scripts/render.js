// Render figure.svg -> figure.png (1x) and report.html -> report.pdf.
// NODE_PATH=$(npm root -g) node scripts/render.js <dir>
const { chromium } = require('playwright');
const path = require('path');
(async () => {
  const dir = path.resolve(process.argv[2]);
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 980, height: 830 }, deviceScaleFactor: 1 });
  await page.goto('file://' + path.join(dir, 'figure.svg'));
  await page.screenshot({ path: path.join(dir, 'figure.png'), clip: { x: 0, y: 0, width: 980, height: 830 } });
  const rep = await browser.newPage();
  await rep.goto('file://' + path.join(dir, 'report.html'));
  await rep.pdf({ path: path.join(dir, 'report.pdf'), format: 'A4', printBackground: true,
                  margin: { top: '14mm', bottom: '14mm', left: '12mm', right: '12mm' } });
  await browser.close();
})();
