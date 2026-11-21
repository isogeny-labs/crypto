import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { readFile, mkdir } from 'node:fs/promises';
import { createServer } from 'node:http';
import { resolve } from 'node:path';

// Optional developer check: provide Playwright and a Chromium installation.
const { chromium } = createRequire(import.meta.url)('playwright');
const root = new URL('./', import.meta.url);
const output = process.env.CURVE_ATLAS_EXPORT_DIR;
if (output) await mkdir(output, { recursive: true });
const files = new Set(['curve-atlas.html', 'index.html', 'style.css', 'math.mjs', 'app.mjs', 'export.mjs']);
const server = createServer(async (request, response) => {
  const name = request.url.slice(1);
  if (!files.has(name)) { response.writeHead(404).end(); return; }
  try {
    const type = name.endsWith('.mjs') ? 'text/javascript' : name.endsWith('.css') ? 'text/css' : 'text/html';
    response.writeHead(200, { 'Content-Type': type });
    response.end(await readFile(new URL(name === 'curve-atlas.html' ? name : 'dist/' + name, root)));
  } catch { response.writeHead(500).end(); }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
let browser;
try {
  browser = await chromium.launch({
    executablePath: process.env.CHROMIUM_PATH || undefined,
    headless: true, args: ['--no-sandbox'],
  });
  const context = await browser.newContext({ acceptDownloads: true });
  // The exported diagram must work without the optional Google Fonts network calls.
  await context.route(/https:\/\/fonts\.(googleapis|gstatic)\.com\//, route => route.abort());
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  async function download(id, path) {
    const event = page.waitForEvent('download');
    await page.locator(id).click();
    const result = await event;
    if (path) await result.saveAs(path);
    const bytes = await readFile(await result.path());
    return bytes;
  }
  const variants = [
    ['standalone', `http://127.0.0.1:${server.address().port}/curve-atlas.html`],
    ['modules', `http://127.0.0.1:${server.address().port}/index.html`],
  ];
  for (const [name, url] of variants) {
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.goto(url);
    await page.locator('#graph .node').first().waitFor();
    await page.locator('#graph [data-node="C2_2"]').click();
    assert((await page.locator('#details').textContent()).includes('Hyperelliptic'));
    await page.locator('#graph [data-edge="cC2_2"]').click();
    assert((await page.locator('#details').textContent()).includes('Finite separable cover'));
    const model = JSON.parse((await download('#export')).toString());
    const displayed = await page.locator('#graph [data-edge]').evaluateAll(nodes => nodes.map(n => n.dataset.edge).sort());
    assert.deepEqual(displayed, model.edges.map(e => e.id).sort());
    await page.locator('#plus').click();
    const svg = await download('#svgExport', output && resolve(output, name + '.svg'));
    const text = svg.toString();
    assert(text.includes('Elliptic transfers'));
    assert(text.includes('same source in every panel'));
    assert(text.includes('c = 0'));
    assert(!text.includes('<parsererror'));
    assert(text.includes('Hyperelliptic'));
    assert(text.includes('width="1000"'));
    assert(!text.includes('scale(1.15)'));
    const png = await download('#pngExport', output && resolve(output, name + '.png'));
    assert.equal(png.subarray(0, 8).toString('hex'), '89504e470d0a1a0a');
    assert.equal(png.readUInt32BE(16), 2000);
    const initialHeight = png.readUInt32BE(20);
    assert(initialHeight > 2000);
    const pixels = await page.evaluate(async base64 => {
      const bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
      const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
      const canvas = document.createElement('canvas');
      canvas.width = bitmap.width; canvas.height = bitmap.height;
      const ctx = canvas.getContext('2d');
      ctx.drawImage(bitmap, 0, 0);
      const image = ctx.getImageData(0, 0, canvas.width, canvas.height).data;
      const counts = [0, 0, 0, 0];
      const colors = [[49,95,168], [22,123,121], [135,83,156], [113,96,168]];
      for (let i = 0; i < image.length; i += 4) {
        colors.forEach((c, j) => { if (c.every((v, k) => image[i + k] === v)) counts[j]++; });
      }
      return { corner: Array.from(image.slice(0, 4)), counts };
    }, png.toString('base64'));
    assert.deepEqual(pixels.corner, [248, 250, 252, 255]);
    assert(pixels.counts.every(count => count > 40), 'all four object types appear in the PNG');
    await page.locator('#n').fill('6');
    await page.locator('#form button[type=submit]').click();
    const tall = await download('#pngExport');
    assert(tall.readUInt32BE(20) > initialHeight, 'the PNG expands for larger graphs');
    await page.setViewportSize({ width: 390, height: 844 });
    await page.locator('#pngExport').scrollIntoViewIfNeeded();
    assert(await page.locator('#pngExport').isVisible());
    await download('#pngExport');
    if (output) await page.screenshot({ path: resolve(output, name + '-mobile.png'), fullPage: true });
    // The error path must give feedback and re-enable the control.
    await page.evaluate(() => { HTMLCanvasElement.prototype.toBlob = callback => callback(null); });
    await page.locator('#pngExport').click();
    await page.locator('#exportStatus').filter({ hasText: 'Export failed:' }).waitFor();
    assert(await page.locator('#pngExport').isEnabled());
    await download('#svgExport');
    console.log(`${name}: PNG/SVG downloads, image dimensions/colors, full-graph zoom, tall graph, mobile and encoder failure checks passed.`);
  }
  assert.deepEqual(errors, []);
} finally {
  await browser?.close();
  await new Promise(resolve => server.close(resolve));
}
