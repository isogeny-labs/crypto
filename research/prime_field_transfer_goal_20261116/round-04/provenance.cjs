// Execution bookkeeping only. All mathematical work stays in the native program.
const fs=require('fs'),cp=require('child_process'),crypto=require('crypto');
const [manifestPath,id,...args]=process.argv.slice(2);
if(!manifestPath||!id||args.length!==3)throw Error('usage: provenance BUILD_JSON CALL_ID setup|qualify ARG ARG');
const root='research/prime_field_transfer_goal_20261116/receipts/round-04/';
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const put=(p,v)=>fs.writeFileSync(p,JSON.stringify(v,null,2)+'\n',{flag:'wx'});
const build=JSON.parse(fs.readFileSync(manifestPath)),pins=build.pins.slice();
for(const pin of pins)if(hash(pin.path)!==pin.sha256)throw Error('changed frozen input '+pin.path);
if(args[0]==='qualify')pins.push({path:args[1],sha256:hash(args[1])});
const pre=root+'pre-'+id+'.json',post=root+'post-'+id+'.json';
for(const name of [pre,post,...['jsonl','stderr','exit'].map(e=>root+id+'.'+e)])
  if(fs.existsSync(name))throw Error('refusing to replace '+name);
put(pre,{...build,phase:'pre-call',timestamp:new Date().toISOString(),pins,
  recorder_sha256:hash(__filename),command:['timeout','60s',build.qualifier+'/recognizer',...args],timeout_seconds:60});
const result=cp.spawnSync('timeout',['60s',build.qualifier+'/recognizer',...args],{maxBuffer:16*1024*1024});
fs.writeFileSync(root+id+'.jsonl',result.stdout||Buffer.alloc(0),{flag:'wx'});
fs.writeFileSync(root+id+'.stderr',result.stderr||Buffer.alloc(0),{flag:'wx'});
const exit=typeof result.status==='number'?result.status:127;
fs.writeFileSync(root+id+'.exit',String(exit)+'\n',{flag:'wx'});
const outputs=['jsonl','stderr','exit'].map(e=>id+'.'+e);
if(args[0]==='setup'&&fs.existsSync(args[2]))outputs.push(args[2]);
put(post,{schema:'development.provenance/v1',phase:'post-call',timestamp:new Date().toISOString(),exit,
  spawn_error:result.error?String(result.error):null,signal:result.signal||null,pre_call_sha256:hash(pre),
  outputs:outputs.map(p=>({path:p,sha256:hash(p.startsWith('research/')?p:root+p)}))});
console.log(JSON.stringify({id,exit,stderr:(result.stderr||Buffer.alloc(0)).toString().slice(-1000)}));
for(const line of (result.stdout||Buffer.alloc(0)).toString().split('\n')){
  try{const row=JSON.parse(line);if(['setup','precision_summary','controls'].includes(row.kind))
    console.log(JSON.stringify({...row,kernel:row.kernel?.length,x_numerator:undefined,x_quotient:undefined,y_numerator:undefined,y_quotient:undefined}));}
  catch{if(line)console.log('retained partial output: '+line.slice(0,150));}
}
process.exitCode=exit;
