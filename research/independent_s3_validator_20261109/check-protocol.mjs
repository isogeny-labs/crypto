// Metadata only: frozen canonical identity and file/binary binding, no EC arithmetic.
import fs from 'node:fs';import crypto from 'node:crypto';
const p=JSON.parse(fs.readFileSync('protocol.json','utf8'));
function sorted(value){
  if(Array.isArray(value))return value.map(sorted);
  if(value!==null&&typeof value==='object')return Object.fromEntries(Object.keys(value).sort().map(k=>[k,sorted(value[k])]));
  return value;
}
const body={...p};delete body.freeze;
const actual=crypto.createHash('sha256').update(JSON.stringify(sorted(body))).digest('hex');
if(p.freeze.status!=='frozen'||actual!==p.freeze.protocol_sha256)throw Error('frozen protocol identity mismatch');
if(p.execution.backend!=='mixed'||p.extensions.paid_spend!==0)throw Error('unexpected execution/budget contract');
console.log(JSON.stringify({protocol_id:p.protocol_id,protocol_sha256:actual,source_sha256:p.extensions.source_sha256}));
