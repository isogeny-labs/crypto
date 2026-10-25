// Artifact transport and hashes only. Credentials enter through non-echoing stdin.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';

const study='research/twist_routes_1_3_20261021';
const output=process.argv[2];
if(!output||fs.existsSync(output))throw Error('fresh publication receipt directory required');
fs.mkdirSync(output,{recursive:true});
fs.copyFileSync(new URL(import.meta.url),output+'/PUBLISHER.mjs');
fs.writeFileSync(output+'/.gitignore','/upload-cache/\n/fresh-download-cache/\n/downloaded-runs.tar.xz\n/extracted/\n');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const expected=read(study+'/execution-v2/RAW_ARTIFACTS.json');
// This study uses the user-selected bucket. The repository's shared default
// serves other artifacts and must not redirect this publication.
const config=read(study+'/store/config.json');
const indexPath=path.resolve(study+'/store/index.json');
if(config.remote!=='s3://setec-astronomy-artifacts/store')throw Error('unexpected study store');
const remote=new URL(config.remote);
const objectKey=remote.pathname.replace(/^\/+|\/+$/g,'')+'/sha256/'+expected.archive.sha256.slice(0,2)+'/'+expected.archive.sha256;
if(sha(expected.archive.local_path)!==expected.archive.sha256||fs.statSync(expected.archive.local_path).size!==expected.archive.bytes)throw Error('prepared archive changed');
const native=path.resolve('target/release/ecbench');
const aws='/workspace/shared/twist-tools-20261023/awscli/aws/dist/aws';
const receipt={schema:'twist.store-publication/v1',started_utc:new Date().toISOString(),remote:config.remote,
  bucket:remote.hostname,object_key:objectKey,index_path:indexPath,index_before_sha256:sha(indexPath),
  name:expected.expected_store_name,sha256:expected.archive.sha256,bytes:expected.archive.bytes,
  uploader_binary_sha256:sha(native),aws_binary_sha256:sha(aws),inherited_uploader_binary:true,
  script_sha256:sha(new URL(import.meta.url)),
  artifact_source_revision:'07fbf5a8fe586302908abcfbb395786de2f11b1f',uploader_source_revision:null,
  credentials_persisted:false,credential_values_in_receipts:false,scientific_results_changed:false,performance_claim:false,steps:[]};
let access='',secret='';
const redact=s=>[access,secret].filter(Boolean).reduce((v,key)=>v.split(key).join('[REDACTED]'),String(s)).replace(/AKIA[A-Z0-9]{16}/g,'[REDACTED_ACCESS_KEY]').replace(/arn:aws:[^\s"\\]+/g,'[REDACTED_AWS_ARN]');
const save=()=>fs.writeFileSync(output+'/RECEIPT.json',JSON.stringify(receipt,null,2)+'\n');
async function command(label,args,env,program=native,identityProbe=false){
  const started=new Date().toISOString();
  const result=await new Promise((resolve,reject)=>{
    const child=spawn(program,args,{env,stdio:['ignore','pipe','pipe']});
    let stdout='',stderr='';
    child.stdout.on('data',x=>stdout+=x);child.stderr.on('data',x=>stderr+=x);
    const timer=setTimeout(()=>child.kill('SIGTERM'),180000);
    child.on('error',e=>{clearTimeout(timer);reject(e);});
    child.on('close',(code,signal)=>{clearTimeout(timer);resolve({code,signal,stdout:redact(stdout),stderr:redact(stderr)});});
  });
  if(identityProbe){
    // Caller identity values are unnecessary for a public transport receipt.
    const errorCode=result.stderr.match(/An error occurred \(([^)]+)\)/)?.[1]??null;
    result.stdout=JSON.stringify({authentication_accepted:result.code===0})+'\n';
    result.stderr=result.code===0?'':JSON.stringify({error_code:errorCode,operation:label,exit_code:result.code})+'\n';
  }
  fs.writeFileSync(output+'/'+label+'.stdout',result.stdout);
  fs.writeFileSync(output+'/'+label+'.stderr',result.stderr);
  receipt.steps.push({label,argv:[program,...args],started_utc:started,finished_utc:new Date().toISOString(),
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
    CRYPTO_STORE_AWS:aws,CRYPTO_STORE_REMOTE:config.remote,CRYPTO_STORE_INDEX:indexPath,
    CRYPTO_STORE_CACHE:path.resolve(output+'/upload-cache')};
  delete base.CRYPTO_STORE_ENDPOINT;
  if(process.argv[3]==='--diagnose-auth'){
    receipt.authentication_diagnostics=[];
    const key=objectKey;
    for(const [label,args] of [
      ['sts-caller-identity',['sts','get-caller-identity','--output','json']],
      ['s3-list-exact-object',['s3api','list-objects-v2','--bucket',remote.hostname,'--prefix',key,'--max-keys','1','--output','json']]
    ]){
      try{await command(label,args,base,aws,true);receipt.authentication_diagnostics.push({label,accepted:true});}
      catch(error){receipt.authentication_diagnostics.push({label,accepted:false,error:redact(error)});}
    }
    receipt.remote_write_performed=false;
    receipt.status='read-only-authentication-diagnostics-complete';
    console.log(JSON.stringify({status:receipt.status,diagnostics:receipt.authentication_diagnostics}));
  }else{
  // Listing permission is unnecessary for an atomic immutable-object write.
  // The ordinary store client can register it after the known object exists.
  if(process.argv[3]==='--conditional-put-first'){
    const checksum=Buffer.from(receipt.sha256,'hex').toString('base64');
    const key=objectKey;
    try{
      const reply=JSON.parse(await command('conditional-put',['s3api','put-object','--bucket',remote.hostname,
        '--key',key,'--body',expected.archive.local_path,'--if-none-match','*','--checksum-algorithm','SHA256',
        '--checksum-sha256',checksum,'--output','json'],base,aws));
      if(reply.ChecksumSHA256!==checksum)throw Error('S3 conditional-write checksum mismatch');
      receipt.conditional_write_performed=true;
      receipt.conditional_write_checksum_verified=true;
    }catch(error){
      if(!String(error).includes('PreconditionFailed'))throw error;
      receipt.conditional_write_performed=false;
      receipt.existing_object_retained=true;
    }
  }
  const provenance='Exact 2026-10-23 native twist E18-E30 raw remainder; source/packing recipe in '+study+'/execution-v2/RAW_ARTIFACTS.json; remote '+config.remote+'; study-local store/index.json; no historical committed evidence repacked.';
  const push=JSON.parse(await command('push',['store','push',expected.archive.local_path,'--name',receipt.name,
    '--description','Frozen twist follow-up raw evidence, including failed and incomplete attempts',
    '--provenance',provenance,'--tag','twist-followup-20261023','--no-cache','--json'],base));
  if(push.sha256!==receipt.sha256||push.bytes!==receipt.bytes||push.name!==receipt.name)throw Error('native push receipt mismatch');
  receipt.newly_uploaded=push.uploaded||receipt.conditional_write_performed===true;
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
  }
  delete base.AWS_ACCESS_KEY_ID;delete base.AWS_SECRET_ACCESS_KEY;
  console.log(JSON.stringify({status:receipt.status,name:receipt.name,sha256:receipt.sha256,bytes:receipt.bytes}));
}catch(error){
  receipt.status='publication-incomplete';receipt.error=redact(error);
  console.error(receipt.error);process.exitCode=1;
}finally{
  receipt.index_after_sha256=sha(indexPath);
  receipt.finished_utc=new Date().toISOString();save();access='';secret='';
}
