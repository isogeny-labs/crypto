// node render.js : report.html -> REPORT.pdf, figure -> large-prime-s.png (playwright, local files only)
const { chromium } = require('playwright');
const fs = require('fs');
(async () => {
  const b = await chromium.launch();
  const p = await b.newPage({ viewport: { width: 1240, height: 900 } });
  await p.setContent(fs.readFileSync('report.html', 'utf8'));
  await p.pdf({ path: 'REPORT.pdf', format: 'A4', landscape: true, printBackground: true, margin: { top: '12mm', bottom: '12mm', left: '10mm', right: '10mm' } });
  const q = await b.newPage({ viewport: { width: 1180, height: 430 } });
  await q.setContent('<body style="margin:0">' + fs.readFileSync('large-prime-s.svg', 'utf8') + '</body>');
  await q.screenshot({ path: 'large-prime-s.png' });
  await b.close();
})();
