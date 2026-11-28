// NODE_PATH=$(npm root -g) node render_pw.js  -> fig_sibling.png, report.pdf
const { chromium } = require('playwright');
const fs = require('fs'); const path = require('path');
(async () => {
  const dir = __dirname;
  const b = await chromium.launch();
  const pg = await b.newPage({ viewport: { width: 980, height: 480 }, deviceScaleFactor: 1 });
  await pg.goto('file://' + path.join(dir, 'fig_sibling.svg'));
  await pg.screenshot({ path: path.join(dir, 'fig_sibling.png') });
  if (fs.existsSync(path.join(dir, 'report.html'))) {
    const p2 = await b.newPage();
    await p2.goto('file://' + path.join(dir, 'report.html'));
    await p2.pdf({ path: path.join(dir, 'report.pdf'), format: 'A4', printBackground: true,
                   margin: { top: '14mm', bottom: '14mm', left: '12mm', right: '12mm' } });
  }
  await b.close();
})();
