from pathlib import Path
import json,hashlib,tarfile,io,csv
P=Path('/home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets');O=P/'combined-v7-results-20261111';N=Path('/home/ec2-user/pencil-validation-receipts/signed-kummer-combined-v7-20261111');C=Path('/var/tmp/pencil-combined-v7-caller-20261111');W=Path('/var/tmp/pencil-signed-kummer-combined-v7-20261111')
h=lambda b:hashlib.sha256(b).hexdigest(); read=lambda p:json.loads(p.read_text());write=lambda n,x:(O/n).write_text(json.dumps(x,indent=2)+'\n')
m=read(N/'terminal-output-manifest.json');c=read(N/'completion.json');w=read(N/'worker/worker-receipt.json');a=read(C/'caller-result.json');s=read(N/'supervisor/launch-receipt.json');z=read(W/'materialization.json')
assert c['terminal_manifest_sha256']==h((N/'terminal-output-manifest.json').read_bytes())
for name,v in m['files'].items():
 b=(N/name).read_bytes();assert len(b)==v['bytes'] and h(b)==v['sha256']
assert w['materialization_receipt_sha256']==h((W/'materialization.json').read_bytes())
raw={}
for label,root in [('native',N),('caller',C)]:
 for f in sorted(root.rglob('*')):
  if f.is_file():raw[label+'/'+str(f.relative_to(root))]=f
raw['workspace/materialization.json']=W/'materialization.json'
with tarfile.open(O/'raw-receipts.tar.gz','w:gz') as t:
 for name,f in raw.items():t.add(f,arcname=name,recursive=False)
write('raw-manifest.json',{'schema':'raw.evidence/v1','files':{n:{'source_path':str(f),'bytes':f.stat().st_size,'sha256':h(f.read_bytes())} for n,f in raw.items()}})
times={'materializer':w['commands'][0]['elapsed_seconds'],'compile_command':w['commands'][1]['elapsed_seconds'],'worker':w['elapsed_seconds'],'completion':c['elapsed_seconds'],'caller':a['elapsed_seconds']}
result={'schema':'research.terminal-result/v1','attempt':'signed-kummer-combined-v7-20261111','status':c['status'],'seal_status':c['seal_status'],'cleanup_complete':s['cleanup_complete'],'sealed_after_owned_cgroup_empty':m['sealed_after_owned_cgroup_empty'],'materialization':z,'commands_executed':[x['label'] for x in w['commands']],'command_reaped_exits':[x['reaped_exit_code'] for x in w['commands']],'all_commands_reaped':all(x['reaped'] for x in w['commands']),'tests_invoked':False,'unattempted':['root signed/Labels tests','full739 driver','forced probes','adapter negatives'],'failure':w['errors'][0],'elapsed_seconds':times,'cgroup_cpu_seconds':int(s['terminal']['CPUUsageNSec'])/1e9,'nested_intervals_not_additive':True,'attempt_charge_seconds':2400,'campaign_charged_seconds':18180,'campaign_remaining_seconds':111420,'deadline_utc':c['absolute_deadline'],'raw_files':len(raw),'raw_terminal_entries_rehashed':len(m['files']),'reporter_self_checks':True,'independent_result_review':False,'dollar_cost':None,'scientific_cost_ratio':None,'source_and_cache_modified_by_reporter':False}
write('summary.json',result)
with (O/'metrics.csv').open('w') as f:
 writer=csv.writer(f);writer.writerow(['metric','value','unit','kind']);writer.writerows([(k,v,'seconds','measured elapsed') for k,v in times.items()]);writer.writerow(['cgroup_cpu',result['cgroup_cpu_seconds'],'seconds','measured cgroup CPU']);writer.writerow(['attempt_charge',2400,'worker_seconds','conservative accounting'])
with (O/'graph-data.csv').open('w') as f:
 writer=csv.writer(f);writer.writerow(['interval','elapsed_seconds']);writer.writerows(times.items())
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="270" viewBox="0 0 760 270"><rect width="760" height="270" fill="white"/><text x="15" y="24" font-family="sans-serif" font-size="17">Combined v7 observed elapsed intervals (seconds)</text>']
for i,(name,value) in enumerate(times.items()):
 y=45+i*36;svg.append(f'<text x="15" y="{y+16}" font-family="sans-serif" font-size="13">{name}</text><rect x="145" y="{y}" width="{value/400*490}" height="23" fill="#426c99"/><text x="{152+value/400*490}" y="{y+16}" font-family="sans-serif" font-size="12">{value:.3f}</text>')
svg.append('<text x="15" y="250" font-family="sans-serif" font-size="12">One attempt. Nested intervals are not additive; no uncertainty estimate.</text></svg>');(O/'timing.svg').write_text(''.join(svg))
(O/'report.typ').write_text('''#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Combined signed/Kummer v7 result
Terminal receipt report · 2026-11-11

Materialization passed: 4,485 source files, 2,165 cache-file modes and 82 executable cache inputs were recorded as preserved. Compilation started, then the monitor raised FileNotFoundError while traversing target/release/deps/rmetaOgIf8S. The compiler process was killed and reaped with exit -9. No tests, full739 driver calls, forced probes or adapter negatives ran. Terminal sealing passed and cleanup completed.

#image("timing.svg",width:100%)

== Accounting and resource observations
The full 2,400-worker-second attempt reservation is charged: 18,180 charged and 111,420 remaining at this terminal boundary. Deadline: November 11, 12:00 UTC. The recorded cgroup CPU time is 318.248421 seconds. Runtime controls were 2 CPUs, 16 GiB memory and zero swap. Dollar cost remains unmeasured.

== Evidence and scope
The raw archive preserves exact native and caller receipts, logs, and materialization.json. All 11 native terminal-manifest entries and the completion-bound manifest digest were rehashed. The worker-bound materialization receipt matches. These are producer-report self-checks; independent result review remains separate. Prior v6 permission failure is retained in its original result packet. The v7 failure occurred in filesystem monitoring during compilation; it does not supply test outcomes.
''')
(O/'SESSION.md').write_text('## What was done\nArchived actual terminal native/caller receipts and materialization; recomputed intervals and hash bindings.\n## Decisions made\nReport actual monitor failure and killed compiler; retain unattempted panels and prior v6 failure. Self-checks disclosed.\n## Open threads\nIndependent result review and root publication.\n## Errors\nFileNotFoundError during recursive target traversal; compiler reaped -9. No payload launched or stopped by reporter.\n')
print('prepared',len(raw),'raw files')
