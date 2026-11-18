// Render speedup.svg to a 1x PNG and report.html to report.pdf (Node
// Playwright; called by analyze.py).
const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
(async () => {
  const d = process.argv[2];
  const b = await chromium.launch();
  const pg = await b.newPage({ deviceScaleFactor: 1 });
  for (const sub of ['', 'round2']) {
    await pg.setContent(fs.readFileSync(path.join(d, sub, 'speedup.svg'), 'utf8'));
    await pg.locator('svg').screenshot({ path: path.join(d, sub, 'speedup.png') });
  }
  await pg.goto('file://' + path.join(d, 'report.html'));
  await pg.pdf({ path: path.join(d, 'report.pdf'), format: 'A4', printBackground: true,
    margin: { top: '12mm', bottom: '12mm', left: '10mm', right: '10mm' } });
  await b.close();
})();
