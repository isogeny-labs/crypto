from pathlib import Path
import hashlib,json,tarfile,csv,re
B=Path('/home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets');P=B/'find-root-results-20261111';P.mkdir(exist_ok=True)
N=Path('/home/ec2-user/pencil-native-find-root-cardinality-renewal-20261111');O=Path(str(N)+'.outer');C=Path('/var/tmp/pencil-findroot-caller-20261111');S=B/'find-root-cardinality-admitted-20261111'
h=lambda b:hashlib.sha256(b).hexdigest();j=lambda p:json.loads(p.read_text());put=lambda n,x:(P/n).write_text(json.dumps(x,indent=2)+'\n')
cmd=j(N/'commands.json');comp=j(O/'completion.json');caller=j(C/'caller-result.json');pre=j(N/'pre-receipt.json');post=j(N/'post-receipt.json');compiler=j(N/'compiler-receipt.json');maps=j(N/'pre-hashes.json')
assert maps==j(N/'post-hashes.json') and len(maps)==16736 and sum(v['bytes'] for v in maps.values())==3111892318
assert j(N/'pre-runtime.json')==j(N/'post-runtime.json')
assert len(cmd)==9 and [x['exit'] for x in cmd]==[0]*8+[1] and all(x['reaped'] and not x['remaining_group'] for x in cmd)
assert all(x['terminal_owner_confirmed'] for x in comp['phases'])
for x in cmd:assert (N/(x['module']+'.lean')).read_bytes()==(S/(x['module']+'.lean.txt')).read_bytes()
expected=sum(j(S/'names.json').values(),[]);readback={}
assert len(expected)==14 and not (N/'Readback.stdout').exists()
raw={}
for label,root in [('native',N),('outer',O),('caller',C)]:
 for f in sorted(root.rglob('*')):
  if f.is_file():raw[label+'/'+str(f.relative_to(root))]=f
with tarfile.open(P/'raw-receipts.tar.gz','w:gz') as t:
 for name,f in raw.items():t.add(f,arcname=name,recursive=False)
put('raw-manifest.json',{'schema':'raw.evidence/v1','files':{n:{'source_path':str(f),'bytes':f.stat().st_size,'sha256':h(f.read_bytes())} for n,f in raw.items()}})
put('axiom-readback.json',{'expected_names':expected,'observed_axioms':readback,'allowed':['propext','Classical.choice','Quot.sound'],'all14_present':False,'independent_kernel_recheck':False})
times={'native_pre':pre['elapsed_seconds'],'native_compiler':compiler['elapsed_seconds'],'native_post':post['elapsed_seconds'],'outer_completion':comp['elapsed_seconds'],'caller':caller['elapsed_seconds']}
rows=[{'module':x['module'],'exit':x['exit'],'expected_exit':x['expected_exit'],'reaped':x['reaped'],'remaining_group':x['remaining_group'],'elapsed_seconds':x['end_seconds']-x['start_seconds']} for x in cmd]
result={'schema':'research.terminal-result/v1','attempt':'find-root-cardinality-renewal-20261111','status':comp['status'],'caller_exit_code':caller['exit_code'],'native_pre_status':pre['status'],'native_post_status':post['status'],'dependency_files':len(maps),'dependency_bytes':3111892318,'pre_post_hash_maps_equal':True,'runtime_maps_equal':True,'compiler_invocations':len(cmd),'command_results':rows,'unattempted_modules':['Readback','ExpectedFailure'],'all_phase_owners_terminal':True,'readback_names':0,'planned_readback_names':14,'allowed_axioms_only':None,'false_control_rejected':None,'failure':'Controls failed Decidable synthesis for RootMap chain rho, RootMap compressed rho, negated RootMap and Forest.','compiled_artifacts':{f.name:h(f.read_bytes()) for f in N.glob('*.olean')},'elapsed_seconds':times,'attempt_charge_seconds':2400,'campaign_charged_seconds':22980,'campaign_remaining_seconds':106620,'deadline_utc':'2026-11-11T12:00:00Z','raw_files':len(raw),'reporter_self_checks':True,'independent_result_review':False,'independent_kernel_recheck_complete':False,'custody_limit':'Pinned shared caches; pre/post checks do not exclude transient concurrent changes.','dollar_cost':None,'scientific_cost_ratio':None}
put('summary.json',result)
with (P/'metrics.csv').open('w') as f:
 w=csv.writer(f);w.writerow(['metric','seconds','exit','expected_exit']);w.writerows([(r['module'],r['elapsed_seconds'],r['exit'],r['expected_exit']) for r in rows]);w.writerows([(k,v,'','') for k,v in times.items()])
with (P/'graph-data.csv').open('w') as f:
 w=csv.writer(f);w.writerow(['module','seconds','exit','expected_exit']);w.writerows([(r['module'],r['elapsed_seconds'],r['exit'],r['expected_exit']) for r in rows])
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="420"><rect width="760" height="420" fill="white"/><text x="15" y="24" font-family="sans-serif" font-size="17">Find-root: nine compiler invocations (seconds)</text>']
for i,r in enumerate(rows):
 y=45+i*36;v=r['elapsed_seconds'];svg.append(f'<text x="15" y="{y+16}" font-family="sans-serif" font-size="13">{r["module"]}</text><rect x="150" y="{y}" width="{v/25*450}" height="23" fill="#426c99"/><text x="{160+v/25*450}" y="{y+16}" font-family="sans-serif" font-size="12">{v:.3f}; exit {r["exit"]}</text>')
svg.append('<text x="15" y="400" font-family="sans-serif" font-size="12">Eight exit 0; Controls exit 1. Readback and ExpectedFailure unattempted.</text></svg>');(P/'timing.svg').write_text(''.join(svg))
(P/'report.typ').write_text('#set page(paper:"a4",margin:16mm)\n#set text(size:9pt)\n= Find-root cardinality native replay\nTerminal producer report · 2026-11-11\n\nEight modules compiled with exit 0: GraphAlgebra, Find, Cycles, Union, Forest, Composition, UnionSize and FindRootCardinality. Controls exited 1 on four Decidable synthesis errors involving RootMap and Forest predicates. Readback and ExpectedFailure were unattempted; no 14-name axiom readback or False-control result is recorded. All nine invoked processes were reaped with empty remaining groups.\n\n#image("timing.svg",width:100%)\n\n== Custody, time and accounting\nPre and post checks passed on identical maps of 16,736 dependency files totaling 3,111,892,318 bytes; runtime maps also matched. Native pre: 41.079102 s; compiler phase: 127.747860 s; post: 33.885975 s. Caller: 203.739195 s, exit 1. Nested intervals are not additive. All phase owners were terminal. Shared-cache pre/post checks do not exclude transient concurrent changes.\n\nThe 2,400-second conservative charge brings this terminal boundary to 22,980 charged and 106,620 remaining; deadline November 11 at 12:00 UTC. Native phases used 1 CPU and 8 GiB memory, zero swap. Exact logs, receipts, source inputs and eight compiled artifacts are archived with hashes. These are producer self-checks; no independent result review or separate binary kernel recheck is claimed. Prior failures remain preserved.\n')
(P/'SESSION.md').write_text('## What was done\nArchived actual native, outer and caller records. Verified nine command exits, input source correspondence and pre/post dependency maps.\n## Decisions made\nPreserve Controls failure and unattempted Readback/ExpectedFailure; report eight compiled artifacts without claiming axiom readback.\n## Open threads\nIndependent review and source successor.\n## Errors\nFour Decidable synthesis errors in Controls. No new native invocation.\n')
print(len(raw), 'raw files')
