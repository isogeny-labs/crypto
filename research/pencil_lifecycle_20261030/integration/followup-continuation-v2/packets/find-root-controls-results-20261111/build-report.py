from pathlib import Path
import hashlib,json,tarfile,csv,re
B=Path('/home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets');P=B/'find-root-controls-results-20261111';P.mkdir(exist_ok=True)
N=Path('/home/ec2-user/pencil-native-find-root-controls-successor-20261111');O=Path(str(N)+'.outer');C=Path('/var/tmp/pencil-findroot-controls-caller-20261111');S=B/'find-root-controls-admitted-20261111'
h=lambda b:hashlib.sha256(b).hexdigest();j=lambda p:json.loads(p.read_text());put=lambda n,x:(P/n).write_text(json.dumps(x,indent=2)+'\n')
cmd=j(N/'commands.json');comp=j(O/'completion.json');caller=j(C/'caller-result.json');pre=j(N/'pre-receipt.json');post=j(N/'post-receipt.json');compiler=j(N/'compiler-receipt.json');maps=j(N/'pre-hashes.json')
assert maps==j(N/'post-hashes.json') and len(maps)==16736 and sum(v['bytes'] for v in maps.values())==3111892318
assert j(N/'pre-runtime.json')==j(N/'post-runtime.json')
assert len(cmd)==11 and [x['exit'] for x in cmd]==[0]*10+[1] and all(x['reaped'] and not x['remaining_group'] for x in cmd)
assert all(x['terminal_owner_confirmed'] and x['passed'] for x in comp['phases'])
for x in cmd:assert (N/(x['module']+'.lean')).read_bytes()==(S/(x['module']+'.lean.txt')).read_bytes()
expected=sum(j(S/'names.json').values(),[]);readback={}
for line in (N/'Readback.stdout').read_text().splitlines():
 m=re.fullmatch("'([^']+)' depends on axioms: \\[(.*)\\]",line)
 if m:readback[m[1]]=[x.strip() for x in m[2].split(',') if x.strip()]
 else:
  m=re.fullmatch("'([^']+)' does not depend on any axioms",line);assert m,line;readback[m[1]]=[]
assert set(readback)==set(expected) and len(readback)==14
assert set(sum(readback.values(),[])) <= {'propext','Classical.choice','Quot.sound'}
assert 'unsolved goals' in (N/'ExpectedFailure.stdout').read_text() and '⊢ False' in (N/'ExpectedFailure.stdout').read_text()
raw={}
for label,root in [('native',N),('outer',O),('caller',C)]:
 for f in sorted(root.rglob('*')):
  if f.is_file():raw[label+'/'+str(f.relative_to(root))]=f
with tarfile.open(P/'raw-receipts.tar.gz','w:gz') as t:
 for name,f in raw.items():t.add(f,arcname=name,recursive=False)
put('raw-manifest.json',{'schema':'raw.evidence/v1','files':{n:{'source_path':str(f),'bytes':f.stat().st_size,'sha256':h(f.read_bytes())} for n,f in raw.items()}})
put('axiom-readback.json',{'expected_names':expected,'observed_axioms':readback,'allowed':['propext','Classical.choice','Quot.sound'],'all14_present':True,'independent_kernel_recheck':False})
times={'native_pre':pre['elapsed_seconds'],'native_compiler':compiler['elapsed_seconds'],'native_post':post['elapsed_seconds'],'outer_completion':comp['elapsed_seconds'],'caller':caller['elapsed_seconds']}
rows=[{'module':x['module'],'exit':x['exit'],'expected_exit':x['expected_exit'],'reaped':x['reaped'],'remaining_group':x['remaining_group'],'elapsed_seconds':x['end_seconds']-x['start_seconds']} for x in cmd]
result={'schema':'research.terminal-result/v1','attempt':'find-root-controls-successor-20261111','status':comp['status'],'caller_exit_code':caller['exit_code'],'native_pre_status':pre['status'],'native_post_status':post['status'],'dependency_files':len(maps),'dependency_bytes':3111892318,'pre_post_hash_maps_equal':True,'runtime_maps_equal':True,'compiler_invocations':len(cmd),'command_results':rows,'unattempted_modules':[],'all_phase_owners_terminal':True,'readback_names':17,'allowed_axioms_only':True,'false_control_rejected':True,'compiled_artifacts':{f.name:h(f.read_bytes()) for f in N.glob('*.olean')},'elapsed_seconds':times,'attempt_charge_seconds':2400,'campaign_charged_seconds':27780,'campaign_remaining_seconds':101820,'deadline_utc':'2026-11-11T12:00:00Z','raw_files':len(raw),'reporter_self_checks':True,'independent_result_review':False,'independent_kernel_recheck_complete':False,'custody_limit':'Pinned shared caches; pre/post checks do not exclude transient concurrent changes.','dollar_cost':None,'scientific_cost_ratio':None}
put('summary.json',result)
with (P/'metrics.csv').open('w') as f:
 w=csv.writer(f);w.writerow(['metric','seconds','exit','expected_exit']);w.writerows([(r['module'],r['elapsed_seconds'],r['exit'],r['expected_exit']) for r in rows]);w.writerows([(k,v,'','') for k,v in times.items()])
with (P/'graph-data.csv').open('w') as f:
 w=csv.writer(f);w.writerow(['module','seconds','exit','expected_exit']);w.writerows([(r['module'],r['elapsed_seconds'],r['exit'],r['expected_exit']) for r in rows])
assert comp['status']=='passed' and caller['exit_code']==0
previous=B/'find-root-results-20261111/summary.json';old=j(previous);assert old['status']=='failed' and old['planned_readback_names']==14
result['previous_failed_attempt']={'path':str(previous),'sha256':h(previous.read_bytes()),'failure':old['failure'],'preserved':True};put('summary.json',result)
put('source-bindings.json',{'admitted_source':str(S),'freeze_sha256':h((S/'freeze.json').read_bytes()),'names_sha256':h((S/'names.json').read_bytes()),'modules':{x['module']:h((N/(x['module']+'.lean')).read_bytes()) for x in cmd}})
# Verify retained archive regular bytes against the just-collected manifest.
with tarfile.open(P/'raw-receipts.tar.gz') as t:
 seen=set()
 for m in t:
  assert m.isfile() and m.name in raw and m.name not in seen;assert h(t.extractfile(m).read())==h(raw[m.name].read_bytes());seen.add(m.name)
 assert seen==set(raw)
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="840" height="560"><rect width="840" height="560" fill="white"/><g font-family="sans-serif" fill="#16364b"><text x="20" y="28" font-size="20">Find-root successor: recorded compiler-call seconds</text>']
maxsec=max(r['elapsed_seconds'] for r in rows)
for i,r in enumerate(rows):
 y=55+i*40;v=r['elapsed_seconds'];svg.append(f'<text x="20" y="{y+17}" font-size="13">{r["module"]}</text><rect x="200" y="{y}" width="{v/maxsec*490:.2f}" height="24" fill="#527896"/><text x="{210+v/maxsec*490:.2f}" y="{y+17}" font-size="12">{v:.3f}; exit{r["exit"]}</text>')
svg.append('<text x="20" y="525" font-size="13">One recorded invocation per module. ExpectedFailure exit1 is the required rejection.</text><text x="20" y="548" font-size="13">These are worker wall intervals, not CPU time, scientific timings or cost ratios.</text></g></svg>');(P/'timing.svg').write_text(''.join(svg))
text='''#set page(paper:"a4",margin:18mm)
#set text(size:10pt,font:"DejaVu Sans")
= Find-root successor: terminal result
Producer record • 11 November 2026 • no new invocation by report builder

Ten positive module invocations compiled with exit 0; ExpectedFailure exited 1 with the specified unsolved False goal. All eleven child processes were reaped with empty remaining groups. Separate Readback output contains exactly fourteen frozen declaration names, with only permitted axioms. Compiled artifacts and native source bytes are bound in the machine-readable records.

#image("timing.svg",width:100%)

Prior failed Controls evidence remains preserved. The earlier Decidable synthesis failure occurred after eight substantive modules compiled; Readback and ExpectedFailure were not reached in that attempt. This successor changes the previously reviewed four control proof bodies; its result does not rewrite the earlier failure.

#pagebreak()
= Custody, accounting and scope
Pre and post maps match on 16,736 dependency files totaling 3,111,892,318 bytes. Runtime maps also match. These checks bind the saved shared-cache observations; they do not rule out transient intervening changes.

'''
for key,value in times.items():text+=f'- {key.replace("_"," ")}: {value:.6f} seconds.\n'
text+='''
Outer and caller intervals include phase work and are not additive. All three recorded phase owners were terminal. The full conservative attempt charge is 2,400 seconds. At this reported terminal boundary, cumulative charged time is 27,780 seconds and remaining time is 101,820 seconds, with the recorded November 11 noon UTC stop deadline. Root owns the authoritative campaign ledger update.

The raw archive includes native, outer and caller receipts, exact commands, logs and compiled artifacts. Its members were hash-checked after creation. Source bindings match every compiled Lean input against the admitted packet. The fourteen named readbacks were independently parsed by this report builder as producer self-checks.

This is a terminal producer report, not independent result review, independent mathematical source-fidelity review or fresh binary kernel rechecking. No universal mathematical claim, cohort observation or cost ratio is inferred. The report builder ran no compiler or experiment.

== Evidence locations
The raw archive and raw-manifest.json preserve original absolute paths and hashes. source-bindings.json identifies find-root-controls-admitted-20261111. summary.json links the earlier find-root-results-20261111 failure. axiom-readback.json lists every expected declaration and its reported axioms; metrics.csv and graph-data.csv retain recorded intervals. [pcc:unverified]
'''
(P/'report.typ').write_text(text)
(P/'README.md').write_text('# Find-root controls terminal result\n\nEleven recorded compiler calls, fourteen named readbacks, expected False rejection and matching custody; prior failed Controls attempt retained. PDF/SVG/CSV/JSON and raw archive. No new native execution by report builder. Independent result review and kernel/source-fidelity checks remain separate.\n')
(P/'SESSION.md').write_text('## What was done\nCollected actual terminal native, outer and caller receipts; matched eleven Lean source inputs, fourteen named readbacks and pre/post custody; archived all raw bytes and produced neutral report.\n## Decisions made\nFull2400-second charge retained; root updates campaign ledger. Earlier Controls failure preserved.\n## Open threads\nRoot publication and independent result review.\n## Errors\nExpectedFailure exited1 as specified; no unexpected failure in this terminal attempt.\n')
print(json.dumps({'raw_files':len(raw),'names':len(readback),'commands':len(cmd),'caller_exit':caller['exit_code']}))
ledger=B/'campaign-renewal-20261110/ledger-findroot-controls-terminal.json'
assert h(ledger.read_bytes())=='3a17a448a98aa83b7735af33f0035ef98d4908afea456acab613f4d3fa78d5c4'
(P/'campaign-ledger.json').write_bytes(ledger.read_bytes())
put('campaign-ledger-binding.json',{'path':str(ledger),'sha256':h(ledger.read_bytes()),'charged_seconds':27780,'remaining_seconds':101820,'updated_by_reporter':False})
