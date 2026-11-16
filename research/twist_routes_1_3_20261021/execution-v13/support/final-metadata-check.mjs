// Hash and byte-preservation audit only; no mathematical result generation.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const s='research/twist_routes_1_3_20261021',sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const checks=[];
for(const wave of [11,12,13]){
 const v=s+'/execution-v'+wave,f=JSON.parse(fs.readFileSync(v+'/FROZEN.json'));
 const files={...f.source_hashes,...f.input_hashes,[v+'/protocol.json']:f.protocol_file_sha256};
 for(const [p,h]of Object.entries(files)){if(sha(p)!==h)throw Error('frozen hash mismatch: '+p);checks.push({path:p,sha256:h,matches:true});}
 for(const b of [f.binary,f.transport,f.reused_native?.binary].filter(Boolean)){if(sha(b.path)!==b.sha256)throw Error('pinned binary mismatch');checks.push({path:b.path,sha256:b.sha256,matches:true});}
}
const c=spawn('git',['show','880fc7552:docs/curves/registry.json']);let old='',err='';c.stdout.on('data',x=>old+=x);c.stderr.on('data',x=>err+=x);const code=await new Promise((r,j)=>{c.on('error',j);c.on('close',r);});if(code)throw Error(err);
const current=fs.readFileSync('docs/curves/registry.json','utf8'),split=old.lastIndexOf('\n ]\n}');if(split<0||!current.startsWith(old.slice(0,split))||!current.endsWith(old.slice(split)))throw Error('existing registry bytes changed');
fs.writeFileSync(s+'/execution-v13/FINAL_METADATA_CHECK.json',JSON.stringify({schema:'twist.final-metadata-check/v1',created:new Date().toISOString(),all_new_frozen_pins_match:true,checks,old_registry_revision:'880fc7552',old_registry_prefix_and_suffix_byte_preserved:true,scope:'new frozen sources/inputs/binaries/protocols and append-only registry bytes; mathematical checks are separate native/PARI evidence'},null,2)+'\n',{flag:'wx'});
