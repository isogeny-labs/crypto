const { chromium } = require('playwright');
const fs = require('fs'), path = require('path');
(async () => {
  const dir = process.argv[2], svg = path.join(dir, process.argv[3]), md = process.argv[4];
  const b = await chromium.launch(); const pg = await b.newPage({ viewport: { width: 640, height: 420 }, deviceScaleFactor: 2 });
  await pg.goto('file://' + svg); await pg.screenshot({ path: svg.replace(/\.svg$/, '.png') });
  if (md) {
    const html = fs.readFileSync(md, 'utf8');
    await pg.goto('file://' + md); await pg.pdf({ path: path.join(dir, 'REPORT.pdf'), format: 'A4', margin: { top: '18mm', bottom: '18mm', left: '16mm', right: '16mm' } });
  }
  await b.close();
})();
