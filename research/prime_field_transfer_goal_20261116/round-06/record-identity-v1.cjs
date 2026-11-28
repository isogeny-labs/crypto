// Execution bookkeeping only. All identity arithmetic is in the native issuer.
const fs=require('fs'),cp=require('child_process'),crypto=require('crypto'),path=require('path');
const root='research/prime_field_transfer_goal_20261116/',receipts=root+'receipts/round-06/';
const qualifier='/tmp/prime-transfer-round06-idc-01';
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write=(p,v)=>fs.writeFileSync(p,JSON.stringify(v,null,2)+'\n',{flag:'wx'});
const inputs=['round-06/identity-issuer-v1.cpp','round-04/identity_support.hpp',
  'round-06/record-identity-v1.cjs','round-06/PACKAGING-01.md','lean/SplitPrime199V1.lean'];
fs.mkdirSync(qualifier);
const pins=inputs.map(relative=>{
  const original=root+relative,destination=qualifier+'/'+relative;
  fs.mkdirSync(path.dirname(destination),{recursive:true});
  fs.copyFileSync(original,destination,fs.constants.COPYFILE_EXCL);fs.chmodSync(destination,0o444);
  return {path:original,sha256:hash(original),qualifier_path:destination,qualifier_sha256:hash(destination)};
});
const executable=qualifier+'/issuer';
const compile=['-std=c++20','-O2','-Wall','-Wextra','-Werror',qualifier+'/round-06/identity-issuer-v1.cpp','-lcrypto','-o',executable];
write(receipts+'identity-build-pre-01.json',{timestamp:new Date().toISOString(),pins,command:['g++',...compile]});
const built=cp.spawnSync('g++',compile,{maxBuffer:4*1024*1024});
for(const [suffix,data] of [['stdout',built.stdout],['stderr',built.stderr],['exit',String(built.status)+'\n']])
  fs.writeFileSync(receipts+'identity-build-01.'+suffix,data||'',{flag:'wx'});
if(built.status!==0)process.exit(built.status||127);
fs.chmodSync(executable,0o555);
const identityDir=receipts+'identity-01',proof=root+'lean/SplitPrime199V1.lean';
const manifest={timestamp:new Date().toISOString(),pins,executable:{path:executable,sha256:hash(executable)},
  command:['timeout','60s',executable,identityDir,proof]};
write(receipts+'identity-pre-01.json',manifest);
const result=cp.spawnSync('timeout',['60s',executable,identityDir,proof],{maxBuffer:4*1024*1024});
for(const [suffix,data] of [['stdout',result.stdout],['stderr',result.stderr],['exit',String(result.status)+'\n']])
  fs.writeFileSync(receipts+'identity-run-01.'+suffix,data||'',{flag:'wx'});
for(const pin of pins)if(hash(pin.path)!==pin.sha256||hash(pin.qualifier_path)!==pin.qualifier_sha256)throw Error('changed input '+pin.path);
if(hash(executable)!==manifest.executable.sha256)throw Error('changed executable');
const outputs=fs.existsSync(identityDir)?fs.readdirSync(identityDir).sort().map(p=>({path:identityDir+'/'+p,sha256:hash(identityDir+'/'+p)})):[];
write(receipts+'identity-post-01.json',{timestamp:new Date().toISOString(),exit:result.status,signal:result.signal,
  pre_sha256:hash(receipts+'identity-pre-01.json'),outputs,unchanged_inputs:true});
process.stdout.write(result.stdout||'');process.stderr.write(result.stderr||'');process.exitCode=result.status||0;
