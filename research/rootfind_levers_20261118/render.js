const { chromium } = require('playwright');
const fs = require('fs'); const path = require('path');
(async () => {
  const d = process.argv[2];
  const b = await chromium.launch();
  const pg = await b.newPage({ deviceScaleFactor: 1 });
  for (const n of ['stack', 'stack-256', 'lanes']) {
    await pg.setContent(fs.readFileSync(path.join(d, n + '.svg'), 'utf8'));
    await pg.locator('svg').screenshot({ path: path.join(d, n + '.png') });
  }
  await pg.goto('file://' + path.join(d, 'report.html'));
  await pg.pdf({ path: path.join(d, 'report.pdf'), format: 'A4', printBackground: true,
    margin: { top: '12mm', bottom: '12mm', left: '10mm', right: '10mm' } });
  await b.close();
})();
