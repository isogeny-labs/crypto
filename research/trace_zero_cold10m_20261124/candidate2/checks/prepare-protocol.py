#!/usr/bin/env python3
"""Protocol and source/binary custody metadata only; never launches experiments."""
from pathlib import Path
import json,hashlib,subprocess
p=Path(__file__).resolve().parent.parent;workspace=p.parent;root=Path.cwd()
def sha(f):return hashlib.sha256(Path(f).read_bytes()).hexdigest()
q=json.loads(Path('research/trace_zero_cold15m_20261124/candidate3/protocol.json').read_text())
def replace(v):
 if isinstance(v,str):return v.replace('15,000,000','10,000,000').replace('15M','10M').replace('cold15m','cold10m')
 if isinstance(v,list):return [replace(x) for x in v]
 if isinstance(v,dict):return {k:replace(x) for k,x in v.items()}
 return v
q=replace(q);q.update(protocol_id='TZ10M-20261124-lazy-wire',version=1,study_dir=str(p.relative_to(root)),question='Does the complete deferred-map/checked-wire bundle bring the unchanged n131 uniform cold query strictly below10M Ir, versus14.69M baseline and12.286M incumbent?')
q['claims'][0]['id']='cold10m';q['claims'][0]['text']='Finite complete cold-query instructions of the configured implementation; strictly below10M prospective target. No native-rate or isolated per-change attribution.'
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
for a in q['arms']:
 name='baseline' if a['id'].startswith('baseline') else a['id'];feature='legacy-halftrace' if name=='baseline' else ('incumbent' if name=='incumbent' else '')
 a['instrument'].update(commit=head,binary_sha256=sha(p/'bin'/name),build=[['cargo','build','--locked','--release','--bins','--manifest-path',str(workspace.relative_to(root)/'native/Cargo.toml')]+(['--features',feature] if feature else [])]);a['instrument']['flags']['features']=feature;a['instrument']['env']['CARGO_TARGET_DIR']='/workspace/scratch/cold10m-candidate2-target'
 a['description']=('Exact accepted14.69M snapshots and original borrowed decoder' if name=='baseline' else ('First qualified12.286M field/linearized/ledger snapshots and original borrowed decoder' if name=='incumbent' else 'Deferred complete coefficient tails, checked compact ASCII/raw-point ingestion, sparse trace setup and fixed map lookup'))+'; equal full LTO/raw_value dependency settings'
 a['instrument']['flags']['serde_json.features']='raw_value on every arm'
q['variable_ledger']['independent'][0]['levels']=['accepted14.69M','incumbent12.286M','lazy-wire-candidate']
for f in q['variable_ledger']['factors']:
 if f['name']=='instrument':f['how']='Same compiler/common control harness; both controls retain original borrowed decoder, candidate adds validated direct raw-point construction. Equal raw_value dependency feature and LTO flags. Entire implementation bundle differs; no isolated attribution.'
 if f['name']=='host':f['how']='Same Intel Xeon Platinum8573C, CPU0. Deterministic calibrated instruction counts only; no native-clock or uncontended-isolation claim.'
q['sample_size']['preflight_runtime']={'fixed_seconds':0,'per_sample_seconds':1.2,'threads':1,'estimated_total_seconds':180,'measured_on':'Prospective planning from retained completed120-row panel; fresh pilot replaces estimate before primary freeze.'}
q['analysis_plan']['script']={'path':'../native/src/main.rs','sha256':sha(workspace/'native/src/main.rs')}
q['literature']=[{'ref':'../checks/pilot-profile.txt; src/cryptanalysis/trace_zero_linearized.rs; serde_json RawValue contract; checked ASCII field construction; standard lazy linear maps and sparse matrix initialization','provenance':'internal','claim':'Retrospective lead selection only. Preserve original row order, first contradiction/rank index and exact remainder/root checks. No novelty or measured-gain claim.'},{'ref':'src/binary_ecc/operation_tally.rs; baseline_operation_tally_snapshot.rs; Rust atomic/TLS scope semantics','provenance':'internal','claim':'Inline only the inactive guard; retain every active event, scope transition, allocation and nested/drop behavior.'}]
q['amendments']=[];q['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':True}
old=q['extensions'];q['extensions']={k:old[k] for k in ['authorization','primary_cases','curve_slugs','phase_calibration','operation_ledgers','external_comparison_gap']}
q['extensions'].update(workload_path='../input-index.json',workload_sha256=sha(workspace/'input-index.json'),original_workload_sha256='2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5',planned_rows=120,payload_root='../../trace_zero_projected_20261115/inputs',goal_candidate_ceiling_ir_exclusive=10_000_000,scope='Whole second candidate versus exact14.69M baseline and first12.286M incumbent. Deferred coefficient tails keep complete fallback; direct raw-point construction retains full syntax/type/ASCII validation and extended decoder fallback. All cold setup/admission/equations/order/root/group/replay/output stay charged.',source_inventory='checks/source-inputs.json',binary_inventory='checks/binary-identities.json',audit_source_sha256=sha(workspace/'native/src/bin/audit_halftrace.rs'),equivalence_rows=195,units='Fewer than10 million total instructions per unchanged cold query; no instruction-rate criterion.',prior_reference_ir=14_688_697,prior_panel='../grid-audit.json; ../../trace_zero_cold15m_20261124/candidate3/grid-audit.json')
assert not (p/'protocol.json').exists();(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n')
oldinv=json.loads(Path('research/trace_zero_cold15m_20261124/candidate3/checks/source-inputs.json').read_text())
files={Path(x['path']) for x in oldinv['files'] if x['path'].startswith('src/')}
files.update(f for f in (workspace/'native').rglob('*') if f.is_file())
files.update(workspace.glob('*_snapshot.rs'));files.update((p/'checks').glob('*.sh'));files.add(workspace/'input-index.json')
entries=[{'path':str(f.relative_to(root)) if f.is_absolute() else str(f),'sha256':sha(f),'bytes':f.stat().st_size} for f in sorted(files)]
(p/'checks/source-inputs.json').write_text(json.dumps({'schema':'source-inventory/v1','files':entries},indent=2)+'\n')
bins={name:{'path':str((p/'bin'/f).relative_to(root)),'sha256':sha(p/'bin'/f),'bytes':(p/'bin'/f).stat().st_size} for name,f in [('baseline','baseline'),('incumbent','incumbent'),('elimination','elimination'),('audit','audit_halftrace'),('checker','check_polynomial'),('guard','preflight_guard')]}
(p/'checks/binary-identities.json').write_text(json.dumps(bins,indent=2)+'\n');(p/'checks/executables.sha256').write_text(''.join(f"{v['sha256']}  {v['path']}\n" for v in bins.values()))
