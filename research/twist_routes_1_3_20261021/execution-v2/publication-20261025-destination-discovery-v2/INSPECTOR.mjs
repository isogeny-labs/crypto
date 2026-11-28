// Read-only bucket-name discovery. Credentials and unrelated bucket names are never persisted.
import fs from 'node:fs';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';

const output=process.argv[2];
if(!output||fs.existsSync(output))throw Error('fresh discovery receipt directory required');
fs.mkdirSync(output,{recursive:true});
fs.copyFileSync(new URL(import.meta.url),output+'/INSPECTOR.mjs');
const aws='/workspace/shared/twist-tools-20261023/awscli/aws/dist/aws';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const receipt={schema:'twist.destination-discovery/v1',started_utc:new Date().toISOString(),
  requested_destination:'Ctech Astronomy artifacts',read_only:true,remote_write_performed:false,
  credentials_persisted:false,credential_values_in_receipts:false,
  script_sha256:sha(new URL(import.meta.url)),aws_binary_sha256:sha(aws),
  argv:[aws,'s3api','list-buckets','--output','json']};
let access='',secret='',credentialEnv;
try{
  console.log('AWAITING_EPHEMERAL_CREDENTIALS');
  let line='';for await(const chunk of process.stdin){line+=chunk.toString();if(line.includes('\n'))break;}
  const parts=line.trim().split(/\s+/);line='';
  if(parts.length!==2)throw Error('expected access key and secret on one stdin line');
  [access,secret]=parts;parts.fill('');
  if(!/^AKIA[A-Z0-9]{16}$/.test(access)||secret.length!==40)throw Error('unexpected credential format');
  credentialEnv={...process.env,AWS_ACCESS_KEY_ID:access,AWS_SECRET_ACCESS_KEY:secret,AWS_SESSION_TOKEN:'',
    AWS_EC2_METADATA_DISABLED:'true',AWS_DEFAULT_REGION:'us-east-1',AWS_REGION:'us-east-1',AWS_MAX_ATTEMPTS:'2',
    AWS_CLI_AUTO_PROMPT:'off',AWS_PAGER:'',AWS_CA_BUNDLE:process.env.AWS_CA_BUNDLE||'/etc/ssl/certs/ca-certificates.crt'};
  const result=await new Promise((resolve,reject)=>{
    const child=spawn(aws,receipt.argv.slice(1),{env:credentialEnv,stdio:['ignore','pipe','pipe']});
    let stdout='',stderr='';child.stdout.on('data',x=>stdout+=x);child.stderr.on('data',x=>stderr+=x);
    const timer=setTimeout(()=>child.kill('SIGTERM'),90000);
    child.on('error',error=>{clearTimeout(timer);reject(error);});
    child.on('close',(code,signal)=>{clearTimeout(timer);resolve({code,signal,stdout,stderr});});
  });
  receipt.exit_code=result.code;receipt.signal=result.signal;
  if(result.code!==0){
    receipt.error_code=result.stderr.match(/An error occurred \(([^)]+)\)/)?.[1]??'aws-command-failed';
    throw Error('bucket discovery failed');
  }
  receipt.matching_buckets=JSON.parse(result.stdout).Buckets.map(b=>b.Name).filter(name=>/ctech|astronomy/i.test(name));
  receipt.status='matching-destination-names-discovered';
}catch(error){receipt.status='discovery-incomplete';receipt.failure=error.message==='bucket discovery failed'?error.message:'input-or-command-error';process.exitCode=1;}
finally{
  if(credentialEnv){delete credentialEnv.AWS_ACCESS_KEY_ID;delete credentialEnv.AWS_SECRET_ACCESS_KEY;}
  access='';secret='';receipt.finished_utc=new Date().toISOString();
  fs.writeFileSync(output+'/RECEIPT.json',JSON.stringify(receipt,null,2)+'\n');
  console.log(JSON.stringify(receipt));
}
