// Artifact transport and hashes only. Credentials enter through non-echoing stdin.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';

const study='research/twist_routes_1_3_20261021';
const output=process.argv[2];
if(!output||fs.existsSync(output))throw Error('fresh publication receipt directory required');
fs.mkdirSync(output,{recursive:true});
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const expected=read(study+'/execution-v2/RAW_ARTIFACTS.json');
const config=read('store/config.json');
if(config.remote!=='s3://crypto-autoresearcher/store')throw Error('unexpected configured store');
if(sha(expected.archive.local_path)!==expected.archive.sha256||fs.statSync(expected.archive.local_path).size!==expected.archive.bytes)throw Error('prepared archive changed');
const native=path.resolve('target/release/ecbench');
const aws='/workspace/shared/twist-tools-20261023/awscli/aws/dist/aws';
const receipt={schema:'twist.store-publication/v1',started_utc:new Date().toISOString(),remote:config.remote,
  name:expected.expected_store_name,sha256:expected.archive.sha256,bytes:expected.archive.bytes,
  uploader_binary_sha256:sha(native),aws_binary_sha256:sha(aws),inherited_uploader_binary:true,
  original_source_revision:'07fbf5a8fe586302908abcfbb395786de2f11b1f',
  credentials_persisted:false,credential_values_in_receipts:false,scientific_results_changed:false,performance_claim:false,steps:[]};
let access='',secret='';
const redact=s=>[access,secret].filter(Boolean).reduce((v,key)=>v.split(key).join('[REDACTED]'),String(s)).replace(/AKIA[A-Z0-9]{16}/g,'[REDACTED_ACCESS_KEY]');
const save=()=>fs.writeFileSync(output+'/RECEIPT.json',JSON.stringify(receipt,null,2)+'\n');
async function command(label,args,env){
  const started=new Date().toISOString();
  const result=await new Promise((resolve,reject)=>{
    const child=spawn(native,args,{env,stdio:['ignore','pipe','pipe']});
    let stdout='',stderr='';
    child.stdout.on('data',x=>stdout+=x);child.stderr.on('data',x=>stderr+=x);
    const timer=setTimeout(()=>child.kill('SIGTERM'),180000);
    child.on('error',e=>{clearTimeout(timer);reject(e);});
    child.on('close',(code,signal)=>{clearTimeout(timer);resolve({code,signal,stdout:redact(stdout),stderr:redact(stderr)});});
  });
  fs.writeFileSync(output+'/'+label+'.stdout',result.stdout);
  fs.writeFileSync(output+'/'+label+'.stderr',result.stderr);
  receipt.steps.push({label,argv:[native,...args],started_utc:started,finished_utc:new Date().toISOString(),
    exit_code:result.code,signal:result.signal,stdout_sha256:sha(output+'/'+label+'.stdout'),stderr_sha256:sha(output+'/'+label+'.stderr'),outputs_redacted_before_persistence:true});
  save();
  if(result.code!==0)throw Error(label+' failed: '+result.stderr.trim());
  return result.stdout;
}
try{
  console.log('AWAITING_EPHEMERAL_CREDENTIALS');
  let line='';
  for await(const chunk of process.stdin){line+=chunk.toString();if(line.includes('\n'))break;}
  const parts=line.trim().split(/\s+/);line='';
  if(parts.length!==2)throw Error('expected access key and secret on one stdin line');
  [access,secret]=parts;parts.fill('');
  if(!/^AKIA[A-Z0-9]{16}$/.test(access)||secret.length!==40)throw Error('unexpected credential format');
  const base={...process.env,AWS_ACCESS_KEY_ID:access,AWS_SECRET_ACCESS_KEY:secret,AWS_SESSION_TOKEN:'',
    AWS_EC2_METADATA_DISABLED:'true',AWS_DEFAULT_REGION:'us-east-1',AWS_REGION:'us-east-1',AWS_MAX_ATTEMPTS:'2',
    AWS_CLI_AUTO_PROMPT:'off',AWS_PAGER:'',AWS_CA_BUNDLE:process.env.AWS_CA_BUNDLE||'/etc/ssl/certs/ca-certificates.crt',
    CRYPTO_STORE_AWS:aws,CRYPTO_STORE_REMOTE:config.remote,CRYPTO_STORE_INDEX:path.resolve('store/index.json'),
    CRYPTO_STORE_CACHE:path.resolve(output+'/upload-cache')};
  delete base.CRYPTO_STORE_ENDPOINT;
  const provenance='Exact 2026-10-23 native twist E18-E30 raw remainder; source/packing recipe in '+study+'/execution-v2/RAW_ARTIFACTS.json; no historical committed evidence repacked.';
  const push=JSON.parse(await command('push',['store','push',expected.archive.local_path,'--name',receipt.name,
    '--description','Frozen twist follow-up raw evidence, including failed and incomplete attempts',
    '--provenance',provenance,'--tag','twist-followup-20261023','--no-cache','--json'],base));
  if(push.sha256!==receipt.sha256||push.bytes!==receipt.bytes||push.name!==receipt.name)throw Error('native push receipt mismatch');
  receipt.newly_uploaded=push.uploaded;
  const fresh=path.resolve(output+'/fresh-download-cache');
  if(fs.existsSync(fresh))throw Error('download cache must be fresh');
  const destination=path.resolve(output+'/downloaded-runs.tar.xz');
  await command('fresh-pull',['store','pull','store:'+receipt.name,'--out',destination],{...base,CRYPTO_STORE_CACHE:fresh});
  if(sha(destination)!==receipt.sha256||fs.statSync(destination).size!==receipt.bytes)throw Error('fresh download mismatch');
  await command('verify-remote',['store','verify',receipt.name,'--remote'],{...base,CRYPTO_STORE_CACHE:fresh});
  receipt.downloaded_path=destination;
  receipt.download_sha256=sha(destination);
  receipt.fresh_download_verified=true;
  receipt.remote_verified=true;
  receipt.status='remote-object-and-fresh-download-verified';
  delete base.AWS_ACCESS_KEY_ID;delete base.AWS_SECRET_ACCESS_KEY;
  console.log(JSON.stringify({status:receipt.status,name:receipt.name,sha256:receipt.sha256,bytes:receipt.bytes}));
}catch(error){
  receipt.status='publication-incomplete';receipt.error=redact(error);
  console.error(receipt.error);process.exitCode=1;
}finally{
  receipt.finished_utc=new Date().toISOString();save();access='';secret='';
}
