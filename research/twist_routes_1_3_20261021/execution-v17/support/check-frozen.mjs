// Byte identity and receipt checks only. No mathematical aggregation.
import fs from 'node:fs';import crypto from 'node:crypto';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
for(const d of process.argv.slice(2)){
 const f=JSON.parse(fs.readFileSync(d+'/FROZEN.json')),checked=[];
 for(const[p,h]of Object.entries(f.files)){if(sha(p)!==h)throw Error('frozen pin changed '+p);checked.push({path:p,sha256:h});}
 for(const b of[f.harness,f.summary]){if(sha(b.path)!==b.sha256)throw Error('frozen binary changed');checked.push(b);}
 const complete=JSON.parse(fs.readFileSync(d+'/runs/campaign-v1/COMPLETE.json')),r=JSON.parse(fs.readFileSync(d+'/runs/TRANSPORT.json'));
 for(const [p,h]of[[d+'/PLAN.json',complete.plan_sha256],[d+'/runs/campaign-v1/ROWS.jsonl',complete.rows_sha256],[d+'/runs/TRANSPORT.stdout',r.stdout_sha256],[d+'/runs/TRANSPORT.stderr',r.stderr_sha256]])if(sha(p)!==h)throw Error('campaign bytes changed '+p);
 if(r.exit_code!==0||r.signal||fs.readFileSync(d+'/runs/TRANSPORT.stderr','utf8').trim())throw Error('campaign transport failed');
 fs.writeFileSync(d+'/FROZEN_CHECK.json',JSON.stringify({schema:'twist.frozen-campaign-check/v1',created:new Date().toISOString(),all_frozen_pins_match:true,checked,complete_receipt_sha256:sha(d+'/runs/campaign-v1/COMPLETE.json'),transport_receipt_sha256:sha(d+'/runs/TRANSPORT.json'),rows_sha256:complete.rows_sha256},null,2)+'\n',{flag:'wx'});
 console.log(JSON.stringify({wave:d,status:'all frozen pins and complete receipt hashes match'}));
}
