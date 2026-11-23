// Administrative source/binary manifest; no cryptographic algorithm or analysis.
const fs=require('node:fs'), path=require('node:path'), crypto=require('node:crypto');
const root=__dirname, build=path.join(root,'unmodified-build');
if(fs.readFileSync(path.join(build,'exit-code.txt'),'utf8').trim()!=='0') throw Error('unmodified build did not succeed');
const out=path.join(root,'qualification'); fs.mkdirSync(out);fs.mkdirSync(path.join(out,'bin'));
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const rows=fs.readFileSync(path.join(build,'cargo.jsonl'),'utf8').trim().split('\n').map(JSON.parse);
const artifacts=rows.filter(x=>x.reason==='compiler-artifact'&&x.executable);
const executables=[];
for(const a of artifacts) {
  const dest=path.join(out,'bin',path.basename(a.executable));
  fs.copyFileSync(a.executable,dest,fs.constants.COPYFILE_EXCL);fs.chmodSync(dest,0o555);fs.chmodSync(a.executable,0o555);
  executables.push({name:a.target.name,kind:a.target.kind,test:a.profile.test,original:a.executable,qualified_copy:dest,sha256:hash(dest),bytes:fs.statSync(dest).size});
}
const tests=executables.filter(x=>x.test).sort((a,b)=>(a.kind[0]==='lib'?-1:0)-(b.kind[0]==='lib'?-1:0)||a.name.localeCompare(b.name));
const manifest={schema:'native-source-qualification/v1',created_utc:new Date().toISOString(),scope:'Unchanged current standalone Rust crate regression qualification; no runtime or algorithmic gain inference',source_head:fs.readFileSync(path.join(build,'source-head.txt'),'utf8').trim(),source_manifest_sha256:hash(path.join(build,'source-sha256.txt')),build_log_sha256:hash(path.join(build,'cargo.jsonl')),rustc:fs.readFileSync(path.join(build,'rustc.txt'),'utf8'),executables,schedule:tests.map((x,i)=>({index:i,name:x.name,binary:x.qualified_copy,args:['--test-threads=1'],working_directory:'/tmp/ecdlp-native-crypto-20261123/crates/hashed-ecdl-tensor'})),limits:{global_wall_seconds:600,per_test_executable_wall_seconds:300,cgroup_memory_stop_bytes:15032385536,threads:1},S3_used:false,additional_external_spend_usd:0,retries:0,scientific_panel:false};
fs.writeFileSync(path.join(out,'pre-call.json'),JSON.stringify(manifest,null,2)+'\n',{flag:'wx',mode:0o444});
fs.writeFileSync(path.join(out,'schedule.tsv'),tests.map((x,i)=>`${i}\t${x.name}\t${x.qualified_copy}`).join('\n')+'\n',{flag:'wx',mode:0o444});
const verifyLines=executables.flatMap(x=>[`${x.sha256}  ${x.original}`,`${x.sha256}  ${x.qualified_copy}`]);
fs.writeFileSync(path.join(out,'executables.sha256'),verifyLines.join('\n')+'\n',{flag:'wx',mode:0o444});
console.log(JSON.stringify({pre_call:path.join(out,'pre-call.json'),test_executables:tests.length,all_executables:executables.length,manifest_sha256:hash(path.join(out,'pre-call.json'))}));
