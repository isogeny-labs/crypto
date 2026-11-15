// Fixed aggregate metadata comparison; no finite-field or EC arithmetic.
import fs from 'node:fs';
const [actualPath, referencePath] = process.argv.slice(2);
if (!actualPath || !referencePath) throw Error('ACTUAL REFERENCE');
const parse = p => fs.readFileSync(p, 'utf8').trim().split('\n').filter(Boolean).map(JSON.parse);
const primes = [5, 7, 11, 13, 17, 19];
const actual = parse(actualPath), reference = parse(referencePath).filter(r => r.m === 3 && primes.includes(r.q));
const keys = ['q', 'm', 'curves', 'tuples', 'zeros', 'false_zeros', 'missing_zeros'];
if (actual.length !== 6 || reference.length !== 6) throw Error('Incomplete six-field population');
for (const q of primes) {
  const a = actual.filter(r => r.q === q), b = reference.filter(r => r.q === q);
  if (a.length !== 1 || b.length !== 1) throw Error('Duplicate/missing field: ' + q);
  for (const k of keys) if (!Number.isSafeInteger(a[0][k]) || a[0][k] < 0 || a[0][k] !== b[0][k])
    throw Error('Aggregate mismatch: ' + q + ':' + k);
  if (a[0].false_zeros !== 0 || a[0].missing_zeros !== 0) throw Error('Directional discrepancy: ' + q);
}
console.log(JSON.stringify({comparison: 'Fresh hosted C++ S3 versus frozen historical C++ known answer',
  fields: primes, matched: true, scope: 'Both directions and exact counts; no timing, binary, S4/S5 or signed execution claim'}));
