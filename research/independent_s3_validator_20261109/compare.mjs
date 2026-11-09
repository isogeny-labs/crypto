// Compare already-recorded aggregate metadata; does not implement field arithmetic.
import fs from 'node:fs';
const [actualPath,referencePath]=process.argv.slice(2);
if(!actualPath||!referencePath) throw Error('compare.mjs ACTUAL REFERENCE');
const parse=path=>fs.readFileSync(path,'utf8').trim().split('\n').filter(Boolean).map(s=>JSON.parse(s));
const primes=[5,7,11,13,17,19];
const actual=parse(actualPath);
const reference=parse(referencePath).filter(r=>r.m===3&&primes.includes(r.q));
const keys=['q','m','curves','tuples','zeros','false_zeros','missing_zeros'];
if(actual.length!==primes.length||reference.length!==primes.length) throw Error('incomplete population');
for(const q of primes){
  const aa=actual.filter(r=>r.q===q),rr=reference.filter(r=>r.q===q);
  if(aa.length!==1||rr.length!==1)throw Error('duplicate/missing field '+q);
  for(const key of keys){
    if(!Number.isSafeInteger(aa[0][key])||aa[0][key]<0||aa[0][key]!==rr[0][key])
      throw Error('aggregate mismatch '+q+':'+key);
  }
  if(aa[0].false_zeros||aa[0].missing_zeros)throw Error('nonzero discrepancy count '+q);
}
console.log(JSON.stringify({comparison:'Rust versus existing independent C++ S3 prime-field aggregates',fields:primes,matched:true,scope:'Both directions/counts only; not binary, S4/S5, symbolic degrees, signed or execution-proof standing.'}));
