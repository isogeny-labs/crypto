from pathlib import Path
import hashlib,json,tarfile,csv,re
B=Path('/home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets');P=B/'pivot-control-results-20261111';P.mkdir(exist_ok=True)
N=Path('/home/ec2-user/pencil-native-pivot-control-operational-v1-20261111');O=Path(str(N)+'.outer');C=Path('/var/tmp/pencil-pivot-control-caller-20261111');S=B/'pivot-control-admitted-20261111'
h=lambda b:hashlib.sha256(b).hexdigest();j=lambda p:json.loads(p.read_text());put=lambda n,x:(P/n).write_text(json.dumps(x,indent=2)+'\n')
cmd=j(N/'commands.json');comp=j(O/'completion.json');caller=j(C/'caller-result.json');pre=j(N/'pre-receipt.json');post=j(N/'post-receipt.json');compiler=j(N/'compiler-receipt.json');maps=j(N/'pre-hashes.json')
assert maps==j(N/'post-hashes.json') and len(maps)==16736 and sum(v['bytes'] for v in maps.values())==3111892318
assert j(N/'pre-runtime.json')==j(N/'post-runtime.json')
assert len(cmd)==5 and [x['exit'] for x in cmd]==[0,0,0,0,1] and all(x['reaped'] and not x['remaining_group'] for x in cmd)
assert all(x['terminal_owner_confirmed'] and x['passed'] for x in comp['phases'])
for x in cmd:assert (N/(x['module']+'.lean')).read_bytes()==(S/(x['module']+'.lean.txt')).read_bytes()
expected=sum(j(S/'names.json').values(),[]);readback={}
for line in (N/'Readback.stdout').read_text().splitlines():
 m=re.fullmatch("'([^']+)' depends on axioms: \\[(.*)\\]",line)
 if m:readback[m[1]]=[x.strip() for x in m[2].split(',') if x.strip()]
 else:
  m=re.fullmatch("'([^']+)' does not depend on any axioms",line);assert m,line;readback[m[1]]=[]
assert set(readback)==set(expected) and len(readback)==17
assert set(sum(readback.values(),[])) <= {'propext','Classical.choice','Quot.sound'}
assert 'unsolved goals' in (N/'ExpectedFailure.stdout').read_text() and '⊢ False' in (N/'ExpectedFailure.stdout').read_text()
raw={}
for label,root in [('native',N),('outer',O),('caller',C)]:
 for f in sorted(root.rglob('*')):
  if f.is_file():raw[label+'/'+str(f.relative_to(root))]=f
with tarfile.open(P/'raw-receipts.tar.gz','w:gz') as t:
 for name,f in raw.items():t.add(f,arcname=name,recursive=False)
put('raw-manifest.json',{'schema':'raw.evidence/v1','files':{n:{'source_path':str(f),'bytes':f.stat().st_size,'sha256':h(f.read_bytes())} for n,f in raw.items()}})
put('axiom-readback.json',{'expected_names':expected,'observed_axioms':readback,'allowed':['propext','Classical.choice','Quot.sound'],'all17_present':True,'independent_kernel_recheck':False})
times={'native_pre':pre['elapsed_seconds'],'native_compiler':compiler['elapsed_seconds'],'native_post':post['elapsed_seconds'],'outer_completion':comp['elapsed_seconds'],'caller':caller['elapsed_seconds']}
rows=[{'module':x['module'],'exit':x['exit'],'expected_exit':x['expected_exit'],'reaped':x['reaped'],'remaining_group':x['remaining_group'],'elapsed_seconds':x['end_seconds']-x['start_seconds']} for x in cmd]
result={'schema':'research.terminal-result/v1','attempt':'pivot-control-operational-v1-20261111','status':comp['status'],'caller_exit_code':caller['exit_code'],'native_pre_status':pre['status'],'native_post_status':post['status'],'dependency_files':len(maps),'dependency_bytes':3111892318,'pre_post_hash_maps_equal':True,'runtime_maps_equal':True,'compiler_invocations':len(cmd),'command_results':rows,'unattempted_modules':[],'all_phase_owners_terminal':True,'readback_names':17,'allowed_axioms_only':True,'false_control_rejected':True,'compiled_artifacts':{f.name:h(f.read_bytes()) for f in N.glob('*.olean')},'elapsed_seconds':times,'attempt_charge_seconds':2400,'campaign_charged_seconds':20580,'campaign_remaining_seconds':109020,'deadline_utc':'2026-11-11T12:00:00Z','raw_files':len(raw),'reporter_self_checks':True,'independent_result_review':False,'independent_kernel_recheck_complete':False,'custody_limit':'Pinned shared caches; pre/post checks do not exclude transient concurrent changes.','dollar_cost':None,'scientific_cost_ratio':None}
put('summary.json',result)
with (P/'metrics.csv').open('w') as f:
 w=csv.writer(f);w.writerow(['metric','seconds','exit','expected_exit']);w.writerows([(r['module'],r['elapsed_seconds'],r['exit'],r['expected_exit']) for r in rows]);w.writerows([(k,v,'','') for k,v in times.items()])
with (P/'graph-data.csv').open('w') as f:
 w=csv.writer(f);w.writerow(['module','seconds','exit','expected_exit']);w.writerows([(r['module'],r['elapsed_seconds'],r['exit'],r['expected_exit']) for r in rows])
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="270"><rect width="760" height="270" fill="white"/><text x="15" y="24" font-family="sans-serif" font-size="17">Pivot control: five compiler invocations (seconds)</text>']
for i,r in enumerate(rows):
 y=45+i*36;v=r['elapsed_seconds'];svg.append(f'<text x="15" y="{y+16}" font-family="sans-serif" font-size="13">{r["module"]}</text><rect x="150" y="{y}" width="{v/25*450}" height="23" fill="#426c99"/><text x="{160+v/25*450}" y="{y+16}" font-family="sans-serif" font-size="12">{v:.3f}; exit {r["exit"]}</text>')
svg.append('<text x="15" y="250" font-family="sans-serif" font-size="12">One invocation per module. ExpectedFailure exit 1 is the specified rejection.</text></svg>');(P/'timing.svg').write_text(''.join(svg))
(P/'report.typ').write_text('''#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Pivot control native replay
Terminal producer report · 2026-11-11

EliminationField, PivotSwap, Controls and Readback compiled with exit 0. ExpectedFailure exited 1 with an unsolved False goal, as specified. All five processes were reaped with empty remaining groups. Readback printed all 17 requested declarations; their reported axioms were confined to propext, Classical.choice and Quot.sound. canonical_preserved reported no axioms. Four compiled artifacts are hash-bound in summary.json.

#image("timing.svg",width:100%)

== Custody and timings
Pre and post checks passed on identical maps of 16,736 dependency files totaling 3,111,892,318 bytes; runtime maps also matched. Native pre: 43.208809 s; compiler phase: 83.963936 s; post: 34.731056 s. Outer completion: 162.773863 s; caller: 162.874211 s, exit 0. Outer/caller intervals contain phase work and are not additive. All three phase owners were terminal. Shared-cache pre/post checks do not exclude transient concurrent changes.

== Accounting and evidence
The 2,400-second conservative charge brings this terminal boundary to 20,580 charged and 109,020 remaining, deadline November 11 at 12:00 UTC. Native phases use 1 CPU and 8 GiB memory, zero swap. Exact commands, receipts, logs and compiled artifacts are archived with hashes. This report's parsing and custody checks are producer self-checks; independent result review and separate binary kernel rechecking are not claimed. Prior failed pivot attempts remain preserved.
''')
(P/'SESSION.md').write_text('## What was done\nCollected terminal native/outer/caller files; matched all five Lean input bytes to admitted source; recomputed command intervals and17-name axiom readback; verified pre/post dependency and runtime maps.\n## Decisions made\nReport four compiled modules and specified False rejection, preserving prior failures and shared-cache scope. No new native invocation.\n## Open threads\nRoot publication and independent result review.\n## Errors\nExpectedFailure exited1 as required with unsolved False goal.\n')
print(len(raw),'raw files',len(readback),'names')
