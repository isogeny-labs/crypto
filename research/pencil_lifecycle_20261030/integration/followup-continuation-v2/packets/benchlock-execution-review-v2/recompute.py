from pathlib import Path
import json,hashlib,re,html,subprocess
r=Path(__file__).parent;e=Path('/var/tmp/pencil-benchlock-controls-execution-20261031-v2')
h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
a=json.loads((e/'admission.json').read_text());b=json.loads((e/'binary-binding.json').read_text());c=json.loads((e/'compile.receipt.json').read_text());run=json.loads((e/'run.receipt.json').read_text())
checks={p:h(p)==x for p,x in a['bindings'].items()};checks['binary']=h(e/'helper')==b['sha256'];checks['admission']=h(e/'admission.json')==b['source_admission_sha256'];checks['review']=h('/var/tmp/pencil-benchlock-independent-review-20261031-v2/manifest.json')==a['review_manifest_sha256'];assert all(checks.values())
producer=Path('/var/tmp/pencil-benchlock-controls-20261031-v2');pm=json.loads((producer/'manifest.json').read_text());assert all(h(producer/p)==v for p,v in pm.items())
s=(e/'run.stdout').read_text().splitlines();counts={m:s.count(m) for m in a['required_markers']};assert all(x==1 for x in counts.values());assert s[-1]=='ALL_SIX_CONTROLS_PASS'
order=[x for x in s if re.fullmatch(r'C\d PASS',x)];assert order==['C1 PASS','C2 PASS','C3 PASS','C4 PASS','C6 PASS','C5 PASS']
probes=[(v=='true',int(n)) for v,n in re.findall(r'probe acquired=(true|false) errno=(\d+)', '\n'.join(s))];assert probes==[(False,11),(True,0),(False,11),(True,0),(True,0),(False,11),(True,0),(True,0),(True,0)]
ready=re.findall(r'child_ready pid=(\d+) destructor_body=(true|false)','\n'.join(s));assert len(ready)==4 and len({p for p,_ in ready})==4;assert [v for _,v in ready]==['false','false','true','false'];assert s.count('child_reaped status=0')==4;assert s.count('C2 fd_cloexec=true')==1
assert c['exit_code']==run['exit_code']==0;assert c['command'][:4]==['timeout','-k','5s','120s'];assert run['command'][:4]==['timeout','-k','5s','60s'];assert c['ended']<=run['started'];assert (e/'run.stderr').stat().st_size==0
raw={str(p):h(p) for p in e.iterdir() if p.is_file()};out={'schema':'software.fixed-controls-review/v1','status':'retained_execution_verified','native_rerun':False,'independent_of_patch_and_execution':True,'binding_checks':checks,'producer_manifest_entries':len(pm),'raw_receipt_hashes':raw,'marker_counts':counts,'ordered_controls':order,'probes':probes,'child_ready_count':4,'child_zero_reap_count':4,'compile_exit':0,'run_exit':0,'scientific_cost_ratio':None,'limits':['standalone extracted primitive','no wait=true','no injected non-WouldBlock runtime error','child release method plus close rather than full field destruction','original GPU failure cause not established']}
(r/'result.json').write_text(json.dumps(out,indent=2)+'\n')
lines=['BenchLock fixed controls: retained execution review','Compiler exit 0; native helper exit 0; no reviewer rerun','Six control markers once each; terminal marker once','Nine probes verified: three errno 11, six acquisitions','Four child-ready observations; four zero-status reaps','Compiler, libc, source, review and executable hashes match','Scope: extracted primitive; integrated tests are separate','No attribution of the original GPU test failures']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="550"><rect width="1000" height="550" fill="white"/>'
for i,l in enumerate(lines):svg+=f'<text x="30" y="{50+i*60}" font-family="sans-serif" font-size="{24 if i==0 else 18}" fill="#172b4d">{html.escape(l)}</text>'
svg+='</svg>';(r/'report.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(r/'report.pdf'),str(r/'report.svg')],check=True)
(r/'REPORT.md').write_text('''# Fixed BenchLock controls: independent retained-output review

The admitted compiler and native helper both exited zero. Independent raw-output recomputation found each of C1–C6 and the terminal marker exactly once, in prescribed order C1,C2,C3,C4,C6,C5. Nine probes match their source assertions: three contention failures with errno 11 and six successful acquisitions. Four distinct child-ready PIDs and four zero-status child reaps are present; C2 confirms CLOEXEC. No native rerun occurred.

The admission-bound compiler, libc rlib, helper source, extracted primitive and producer manifest match current bytes. The prior independent source-review manifest matches its admission binding. The executable matches binary-binding.json, which binds the exact admission digest. Compiler/run commands retain their 120-second/60-second outer caps and 5-second kill grace. Native run stderr is empty. All producer manifest entries were also rechecked.

C1 checks ordinary ownership/drop; C2 close-only inheritance; C3 parent release while child retains a descriptor; C4 child nonowner release-method call plus close while parent remains owner; C6 early-return Drop; C5 cross-thread owner Drop. These observations agree with the previously reviewed source assertions. Output markers alone are not substituted for their probe records.

Scope remains an extracted production primitive with an unexercised signal stub. wait=true, injected non-WouldBlock errors, and complete Rust child-field destruction are not tested. This packet does not establish which interleaving or error caused the historical GPU failures. Root owns subsequent integrated tests. Reviewer authored an earlier close-only mechanism diagnostic, not this patch/helper or execution.

Exact input/output hashes and counts are in result.json; recompute.py records this read-only analysis. Execution source: /var/tmp/pencil-benchlock-controls-execution-20261031-v2. Producer source: /var/tmp/pencil-benchlock-controls-20261031-v2. Prospective review: /var/tmp/pencil-benchlock-independent-review-20261031-v2.
''')
(r/'manifest.json').write_text(json.dumps({p.name:h(p) for p in sorted(r.iterdir()) if p.is_file() and p.name!='manifest.json'},indent=2)+'\n');print(h(r/'manifest.json'))
