// Native certificate execution provenance; no arithmetic results computed here.
const fs=require('fs'),path=require('path'),crypto=require('crypto'),cp=require('child_process');
const [version,binary,source,moduleName]=process.argv.slice(2);
if(!/^\d+$/.test(version)||!binary||!source||!moduleName)throw Error('usage: issue VERSION BINARY SOURCE MODULE');
const base='research/prime_field_transfer_goal_20261116/',r=base+'receipts/round-04/',s=base+'round-04/';
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const put=(p,x)=>fs.writeFileSync(p,JSON.stringify(x,null,2)+'\n',{flag:'wx'});
const qualifier='/tmp/prime-transfer-round04-root-qualifier-'+version;fs.mkdirSync(qualifier);
const files=[s+source,s+'identity_support.hpp',s+'v2/exact.hpp',s+'PROOF-MAP.md',path.relative(process.cwd(),__filename)];
const pins=files.map(p=>{const out=path.join(qualifier,path.relative(s,p));
  fs.mkdirSync(path.dirname(out),{recursive:true});fs.copyFileSync(p,out,fs.constants.COPYFILE_EXCL);fs.chmodSync(out,0o444);
  return{path:p,sha256:hash(p),qualifier_path:out,qualifier_sha256:hash(out)};});
const exe=qualifier+'/issuer';fs.copyFileSync(binary,exe,fs.constants.COPYFILE_EXCL);fs.chmodSync(exe,0o555);
pins.push({path:exe,sha256:hash(exe)});
for(const p of [r+'models/u3.model',r+'root-input-01.txt'])pins.push({path:p,sha256:hash(p)});
const output=base+'lean/'+moduleName,directory=r+'root-certificates-'+version,id='root-'+version;
const cmd=['timeout','60s',exe,r+'models/u3.model',r+'root-input-01.txt',directory,output];
put(r+'pre-'+id+'.json',{phase:'pre-call',timestamp:new Date().toISOString(),pins,command:cmd});
const result=cp.spawnSync(cmd[0],cmd.slice(1),{maxBuffer:8*1024*1024});
for(const[e,b]of[['stdout',result.stdout],['stderr',result.stderr]])fs.writeFileSync(r+id+'.'+e,b||Buffer.alloc(0),{flag:'wx'});
fs.writeFileSync(r+id+'.exit',String(result.status)+'\n',{flag:'wx'});
for(const p of pins){if(hash(p.path)!==p.sha256)throw Error('input changed');
  if(p.qualifier_path&&hash(p.qualifier_path)!==p.qualifier_sha256)throw Error('qualifier changed');}
const outputs=[r+id+'.stdout',r+id+'.stderr',r+id+'.exit',output];
if(fs.existsSync(directory))outputs.push(...fs.readdirSync(directory).map(n=>directory+'/'+n));
put(r+'post-'+id+'.json',{phase:'post-call',timestamp:new Date().toISOString(),exit:result.status,
  pre_call_sha256:hash(r+'pre-'+id+'.json'),outputs:outputs.filter(p=>fs.existsSync(p)).map(p=>({path:p,sha256:hash(p)}))});
console.log(result.stdout?.toString());console.log(result.stderr?.toString());process.exitCode=result.status??127;
