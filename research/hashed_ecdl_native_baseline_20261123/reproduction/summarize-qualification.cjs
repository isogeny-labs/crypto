// Administrative parsing of test-runner status, not scientific cost analysis.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const root=__dirname,out=path.join(root,'qualification');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const pre=JSON.parse(fs.readFileSync(path.join(out,'pre-call.json')));
const rows=fs.readFileSync(path.join(out,'rows.tsv'),'utf8').trim().split('\n').slice(1).map(line=>{const [index,name,status,exit_code]=line.split('\t');return {index:Number(index),name,status,exit_code:exit_code===''?null:Number(exit_code)};});
if(rows.length!==pre.schedule.length) throw Error('scheduled executable count mismatch');
const totals={passed:0,failed:0,ignored:0,measured:0,filtered_out:0};
for(const row of rows) {
 const file=path.join(out,`${row.index}-${row.name}.stdout`);
 if(!fs.existsSync(file)) continue;
 const text=fs.readFileSync(file,'utf8');
 const m=text.match(/test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;/);
 row.test_summary_present=Boolean(m);
 if(m) {Object.keys(totals).forEach((key,i)=>{totals[key]+=Number(m[i+1]);});row.tests_passed=Number(m[1]);row.tests_failed=Number(m[2]);}
 row.stdout_sha256=sha(file);
 row.stderr_sha256=sha(path.join(out,`${row.index}-${row.name}.stderr`));
}
for(const e of pre.executables) for(const p of [e.original,e.qualified_copy]) if(sha(p)!==e.sha256) throw Error('binary changed: '+p);
const summary={schema:'native-source-qualification-result/v1',scope:pre.scope,pcc_standing:'unverified',pre_call_sha256:sha(path.join(out,'pre-call.json')),source_head:pre.source_head,started_utc:fs.readFileSync(path.join(out,'started-utc.txt'),'utf8').trim(),finished_utc:fs.readFileSync(path.join(out,'finished-utc.txt'),'utf8').trim(),scheduled_executables:pre.schedule.length,completed_executables:rows.filter(x=>x.exit_code!==null).length,all_scheduled_executables_passed:rows.every(x=>x.status==='passed'&&x.exit_code===0&&x.test_summary_present),test_totals:totals,rows,all_42_original_and_qualified_binary_hashes_unchanged:true,peak_whole_cgroup_bytes:Number(fs.readFileSync(path.join(out,'peak-cgroup-bytes.txt'),'utf8')),S3_used:false,runtime_gain:null,scaling_exponent:null,complete_general_counter_proof:false};
fs.writeFileSync(path.join(out,'post-call.json'),JSON.stringify(summary,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({all_passed:summary.all_scheduled_executables_passed,total:totals,post_call_sha256:sha(path.join(out,'post-call.json'))}));
