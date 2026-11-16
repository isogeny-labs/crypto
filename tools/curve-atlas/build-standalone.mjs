import { readFile, writeFile } from 'node:fs/promises';

const root = new URL('./', import.meta.url);
const read = name => readFile(new URL(name, root), 'utf8');
const [html, css, math, exporter, app] = await Promise.all([
  read('dist/index.html'), read('dist/style.css'),
  read('dist/math.mjs'), read('dist/export.mjs'), read('dist/app.mjs'),
]);
const bundled = html
  .replace('<link rel="stylesheet" href="style.css">', `<style>${css}</style>`)
  .replace('<script type="module" src="app.mjs"></script>',
    `<script type="module">${math.replace(/^export /gm, '')}\n${exporter.replace(/^export /gm, '')}\n${app.replace("import {build} from './math.mjs';", '').replace("import { renderDiagram, serializeGraph, rasterizeSvg } from './export.mjs';", '')}</script>`);
await writeFile(new URL('curve-atlas.html', root), bundled);
console.log('Wrote tools/curve-atlas/curve-atlas.html');
