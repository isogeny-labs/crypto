// Corrupt one saved-output field; never compute a curve result.
import fs from 'node:fs';
import path from 'node:path';
const [directory, kind] = process.argv.slice(2);
const filename = kind === 'character' ? 'characters.tsv' : kind === 'calibration' ? 'calibration.tsv' : 'summary.json';
const file = path.join(directory, filename);
const original = fs.readFileSync(file, 'utf8');
let changed;
if (kind === 'character') {
  const lines = original.trimEnd().split('\n');
  const fields = lines[2].split('\t');
  fields[2] = '9';
  lines[2] = fields.join('\t');
  changed = lines.join('\n') + '\n';
} else if (kind === 'calibration') {
  const lines = original.trimEnd().split('\n');
  const fields = lines[1].split('\t');
  fields[1] = fields[1] === '0' ? '1' : '0';
  lines[1] = fields.join('\t');
  changed = lines.join('\n') + '\n';
} else if (kind === 'summary') {
  changed = original.replace('"curve_choices":21', '"curve_choices":20');
} else throw new Error('unknown corruption');
if (changed === original) throw new Error('control did not change bytes');
fs.writeFileSync(file, changed);
fs.writeFileSync(path.join(directory, 'corruption.json'), JSON.stringify({kind, filename, field_corrupted: true}) + '\n');
