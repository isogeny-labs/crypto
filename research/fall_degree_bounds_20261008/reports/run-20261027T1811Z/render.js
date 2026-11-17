// Render SVG to PNG and report.html to PDF with the preinstalled Chromium. Usage: node render.js
const { chromium } = require('playwright');
(async () => {
  const b = await chromium.launch();
  const p = await b.newPage({ viewport: { width: 720, height: 440 } });
  await p.goto('file://' + __dirname + '/e12b_r2_fall_degree.svg');
  await p.screenshot({ path: __dirname + '/e12b_r2_fall_degree.png' });
  const q = await b.newPage();
  await q.goto('file://' + __dirname + '/report.html');
  await q.pdf({ path: __dirname + '/report.pdf', format: 'A4', printBackground: true, margin: { top: '14mm', bottom: '14mm', left: '12mm', right: '12mm' } });
  await b.close();
})();
