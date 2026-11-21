// Operational AWS/Setec checkpoint transport only. No research calculations.
// Credential input is read once from a non-echoing terminal, retained in memory,
// inherited by SDK/CLI children and removed from all captured output.
import fs from 'node:fs';
import path from 'node:path';
import readline from 'node:readline';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
const aws='/workspace/shared/twist-tools-20261023/awscli/aws/dist/aws';
const dir='research/twist_routes_1_3_20261021/execution-v4/checks';
fs.mkdirSync(dir,{recursive:true});
let access,secret;
function scrub(s){for(const v of [access,secret])if(v)s=s.replaceAll(v,'[REDACTED]');return s.replace(/AKIA[A-Z0-9]{16}/g,'[REDACTED_ACCESS_KEY]');}
const lines=readline.createInterface({input:process.stdin,terminal:false});
console.log('Awaiting non-echoing credential input; values will not be stored.');
for await(const line of lines){
  if(!access){const values=line.trim().split(/\s+/);if(values.length!==2||!/^AKIA[A-Z0-9]{16}$/.test(values[0])||values[1].length!==40)throw Error('Credential input shape rejected');[access,secret]=values;console.log('Credential input accepted in memory. Send a JSON operation.');continue;}
  const request=JSON.parse(line);if(request.mode==='quit')break;
  let command,args;
  if(request.mode==='aws'){
    command=aws;args=request.args;
    if(!Array.isArray(args)||args.some(a=>typeof a!=='string')||!['sts','codebuild','ecr','eks','s3api','s3'].includes(args[0]))throw Error('Unsupported operational AWS command');
  }else if(request.mode==='artifact'){
    // Maintained operational storage transport; it computes no research result.
    command='python3';args=['tools/artifacts/artifacts.py',request.action,...request.paths];
    if(!['push','pull','verify','status','plan'].includes(request.action)||!Array.isArray(request.paths)||request.paths.some(p=>typeof p!=='string'||p.startsWith('-')||p.includes('..')||path.isAbsolute(p)))throw Error('Artifact operation rejected');
  }else if(request.mode==='checkpoint'){
    command='/bin/bash';args=['scripts/checkpoint.sh',request.action];
    if(!['push','gate','status'].includes(request.action))throw Error('Checkpoint operation rejected');
  }else throw Error('Unknown mode');
  if(!/^[a-z0-9][a-z0-9-]*$/.test(request.name))throw Error('Receipt name rejected');
  const started=new Date().toISOString();let stdout='',stderr='';
  const child=spawn(command,args,{env:{...process.env,PATH:path.dirname(aws)+':'+process.env.PATH,AWS_ACCESS_KEY_ID:access,AWS_SECRET_ACCESS_KEY:secret,AWS_REGION:'us-east-2',AWS_DEFAULT_REGION:'us-east-2',AWS_EC2_METADATA_DISABLED:'true',AWS_MAX_ATTEMPTS:'1',AWS_PAGER:'',CHECKPOINT_BUCKET:'setec-astronomy-artifacts',ARTIFACTS_CACHE:'/workspace/shared/twist-artifact-cache-20261027'}});
  child.stdout.on('data',x=>stdout+=x);child.stderr.on('data',x=>stderr+=x);
  let error;child.on('error',x=>error=x.message);
  const result=await new Promise(resolve=>child.on('close',(exit_code,signal)=>resolve({exit_code,signal})));
  stdout=scrub(stdout);stderr=scrub(stderr);
  const prefix=dir+'/'+request.name;
  if(fs.existsSync(prefix+'.json'))throw Error('Fresh receipt name required');
  fs.writeFileSync(prefix+'.stdout',stdout);fs.writeFileSync(prefix+'.stderr',stderr);
  const hash=s=>crypto.createHash('sha256').update(s).digest('hex');
  fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.operational-aws-receipt/v1',started,finished:new Date().toISOString(),argv:[command,...args],...result,error:error?scrub(error):null,stdout:prefix+'.stdout',stderr:prefix+'.stderr',stdout_sha256:hash(stdout),stderr_sha256:hash(stderr),credential_values_persisted:false},null,2)+'\n');
  console.log(JSON.stringify({name:request.name,...result,stdout:stdout.slice(0,10000),stderr:stderr.slice(0,6000)}));
}
access=undefined;secret=undefined;
