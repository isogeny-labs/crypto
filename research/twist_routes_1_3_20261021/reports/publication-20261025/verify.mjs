// Offline publication-receipt integrity checks; no AWS calls or scientific work.
import fs from 'node:fs';
import crypto from 'node:crypto';
const base='research/twist_routes_1_3_20261021',ex=base+'/execution-v2',report=base+'/reports/publication-20261025';
const output=process.argv[2];if(!output||fs.existsSync(output))throw Error('fresh check output required');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const assessment=read(report+'/ASSESSMENT.json'),meta=read(ex+'/RAW_ARTIFACTS.json');
if(assessment.prepared_sha256!==meta.archive.sha256||assessment.prepared_bytes!==meta.archive.bytes)throw Error('archive identity mismatch');
if(assessment.successful_remote_write||assessment.store_index_changed||assessment.policy_applied)throw Error('unexpected promotion of denied publication');
const policy=read(report+'/REQUIRED_OBJECT_POLICY.json');
if(policy.Statement.length!==1)throw Error('policy must target one object');
const statement=policy.Statement[0],resource='arn:aws:s3:::crypto-autoresearcher/store/sha256/'+meta.archive.sha256.slice(0,2)+'/'+meta.archive.sha256;
if(statement.Resource!==resource||statement.Effect!=='Allow'||JSON.stringify(statement.Action)!=='["s3:GetObject","s3:PutObject"]')throw Error('policy scope differs');
const checked=[];
for(const name of ['publication-20261025-v2','publication-20261025-auth-diagnostic','publication-20261025-v3']){
  const dir=ex+'/'+name,r=read(dir+'/RECEIPT.json');
  if(r.sha256!==assessment.prepared_sha256||r.bytes!==assessment.prepared_bytes)throw Error('receipt archive mismatch');
  if(r.credentials_persisted||r.credential_values_in_receipts)throw Error('credential persistence claimed');
  for(const s of r.steps){for(const suffix of ['stdout','stderr']){
    const p=dir+'/'+s.label+'.'+suffix;
    if(sha(p)!==s[suffix+'_sha256'])throw Error('transport hash mismatch');
    if(/AKIA[A-Z0-9]{16}/.test(fs.readFileSync(p,'utf8')))throw Error('access-key-shaped value in transport output');
  }}
  if(r.script_sha256&&sha(dir+'/PUBLISHER.mjs')!==r.script_sha256)throw Error('source snapshot mismatch');
  checked.push({receipt:dir+'/RECEIPT.json',sha256:sha(dir+'/RECEIPT.json'),source_snapshot_sha256:sha(dir+'/PUBLISHER.mjs')});
}
const files=fs.readdirSync(report).filter(n=>fs.statSync(report+'/'+n).isFile()).map(n=>({path:report+'/'+n,sha256:sha(report+'/'+n),bytes:fs.statSync(report+'/'+n).size}));
fs.writeFileSync(output,JSON.stringify({schema:'twist.publication-static-check/v1',policy_scope:'one-exact-object-GetObject-PutObject',policy_applied:false,receipt_integrity:'passed',access_key_pattern_scan:'no-match-in-transport-output',receipt_checks:checked,scientific_calculations_performed:false,files},null,2)+'\n');
