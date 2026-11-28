// Administrative toolchain manifest reader, not cryptographic research code.
const fs=require('node:fs');
const crypto=require('node:crypto');
const root=__dirname;
const text=fs.readFileSync(root+'/rust-1.99.1-channel.toml','utf8');
const expected=fs.readFileSync(root+'/rust-1.99.1-channel.toml.sha256','utf8').trim().split(/\s+/)[0];
const actual=crypto.createHash('sha256').update(text).digest('hex');
if(actual!==expected)throw Error('Official channel manifest digest mismatch');
const sections={};let current='';
for(const line of text.split('\n')){
  const header=line.match(/^\[([^\]]+)\]$/);
  if(header){current=header[1];sections[current]={};continue;}
  const pair=line.match(/^([a-z_-]+)\s*=\s*(.+)$/);
  if(pair&&current){try{sections[current][pair[1]]=JSON.parse(pair[2]);}catch{}}
}
const components=['rustc','cargo','rust-std','rustfmt-preview','clippy-preview'];
const packages=components.map(component=>{
  const p=sections['pkg.'+component+'.target.x86_64-unknown-linux-gnu'];
  if(!p||p.available!==true||!p.xz_url||!p.xz_hash)throw Error('Required component unavailable: '+component);
  if(!p.xz_url.startsWith('https://static.rust-lang.org/dist/'))throw Error('Unexpected package host');
  return {component,url:p.xz_url,sha256:p.xz_hash,filename:p.xz_url.split('/').pop()};
});
const record={scope:'Pinned user-level toolchain setup for native source qualification',version:'1.99.1',channel_manifest_sha256:actual,packages,
  requested_cpus:2,additional_external_spend_usd_ceiling:0,new_capacity:false,S3_used:false,
  install_prefix:'/tmp/ecdlp-native-tools-20261123/rust-1.99.1',
  limits:{wall_seconds:900,cgroup_memory_stop_bytes:15032385536},
  provenance:'Compiler version reported in the existing native validation receipts; official release manifest and component digests checked before installation'};
fs.writeFileSync(root+'/rust-toolchain-install-plan.json',JSON.stringify(record,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({version:record.version,manifest_sha256:actual,packages}));
