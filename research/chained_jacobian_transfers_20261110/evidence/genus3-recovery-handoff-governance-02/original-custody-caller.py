"""Metadata/raw custody checkpoint; live native source deliberately unstaged."""
from pathlib import Path
from datetime import datetime,timezone
import subprocess,json,hashlib,lzma
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='de0b472a189e38a9b861484b00423497327ecb90';assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
scope=[str(s.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-13.md','.resume/RESUME.md'];excluded=str((s/'genus3-prime-descent').relative_to(r))+'/'
d=s/'evidence/genus3-recovery-handoff-governance-02';d.mkdir()
def paths():
    a=(set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''}
    return sorted(p for p in a if not p.decode().startswith(excluded))
def run(label,cmd,allowed=(0,)):
    v=subprocess.run(cmd,cwd=r,capture_output=True);raw=v.stdout+v.stderr
    if len(raw)>60000:
        packed=lzma.compress(raw,format=lzma.FORMAT_XZ,preset=6);assert lzma.decompress(packed)==raw;(d/(label+'.txt.xz')).write_bytes(packed)
    else:(d/(label+'.txt')).write_bytes(raw)
    (d/(label+'.exit')).write_text(str(v.returncode)+'\n');print(label,v.returncode,flush=True);assert v.returncode in allowed,raw.decode(errors='replace')[:1500]
    return v,raw
before=json.loads(Path('/workspace/scratch/ct55-g3-unit-01/before.json').read_text());assert len(before)==807;assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==v for p,v in before.items())
subjects=[]
for p in paths():
    if p.startswith(b'pcc/claims/'):
        for proof in json.loads((r/p.decode()).read_text()).get('proofs',[]):
            if proof.get('kind')=='attestation':
                item=proof['subject'];assert hashlib.sha256((r/item['path']).read_bytes()).hexdigest()==item['sha256'];subjects.append(item['path'])
(d/'inputs.json').write_text(json.dumps(dict(base=base,current807_native_inputs_byte_match_original_unit01=True,all794_prior_inputs_included=True,source_scope='live Rust/Lean sources UNSTAGED;whole original snapshots in rawarchives',subjects=len(subjects),new_benchmark_verifier_policy_workflow_edits=False),indent=2)+'\n')
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode() for p in paths() if p.endswith(b'.md')]]);run('working-diff',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check']);bench,_=run('benchrec-global',['python3','tools/benchrec.py','gate'],(0,1))
def stage():
    ps=paths();assert all(any(p.decode()==x or p.decode().startswith(x+'/') for x in scope) and not p.decode().startswith(excluded) for p in ps)
    f=Path('/workspace/scratch/ct55-recovery-handoff.paths0');f.write_bytes(b'\0'.join(p for p in ps if p!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
    selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
    for p in subjects:assert subprocess.run(['git','cat-file','-e',base+':'+p],cwd=r,capture_output=True).returncode==0 or p.encode() in selected
    assert not subprocess.check_output(['git','diff','--cached','--name-only','--','src','isogeny_algos','.github/workflows','tools','benchmarks','.artifacts/config.json',excluded],cwd=r)
    return selected
stage();run('cached-diff',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();v,raw=run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree],(1,))
expected={str((s/'evidence/genus3-static-archive-handoff-01'/('static-'+i+'.tar.xz')).relative_to(r)) for i in ['01','02','03']};expected.add(str((s/'evidence/genus3-unit-archive-handoff-01/unit-01.tar.xz').relative_to(r)))
assert {x.split()[1] for x in v.stdout.decode().splitlines() if x.strip()}==expected
(d/'custody.json').write_text(json.dumps(dict(base=base,ledger_prose_workingcachedwhitespace_wiring=0,artifact_exit=1,exact_four_pending_whole_archives=sorted(expected),native_sources_unstaged=True,current807_including_prior794_unchanged=True,unit='7f48f6 COMPLETE FAILED4beforepointtests/807guards0',Lean='d9f9f6 COMPLETE local9named+3generated12/0trusted/796guards0',corrective_contract='6a3cfe frozen before correctedcode/unit02',static='604a22/65bae1fail then8c3b6astrictalltargets0',benchrec_exit=bench,standing='unsigned metadata recovery/handoff,not remoteavailability/nativeacceptance/scientificsignature/mainmerge'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Unsigned G3 recovery handoff governance:own de0b472 ledgerprefix/prose/workingcachedwhitespace/wiring0,current807 native inputs including prior794 unchanged,newsubjecthash/stagingcoverage. Only metadata/raw recovery handoffs staged;live native Rust/Lean sources intentionally unstaged. Artifact ACTUAL1 selects EXACTFOURwhole static01/02/03+failedunit01 archives PENDING existing trusted-base whole remote byte replay/realmanifests/normalbot,NOTgreen/availability. Unit7f48f6FAILED4beforepointtesting;Lean d9f9f6 local9named+3generated12/0trusted independent partialgeometry;corrective6a3cfe frozen BEFORE repairedcode/unit02. All original failures remain. Broadbenchrec actualexit'+str(bench)+' historicalscope/no benchmarkchange. No scientificsignature/newunitpass/solver/performance/mainmerge assertion.']
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.run(cmd,cwd=r,check=True,capture_output=True,text=True).stdout.strip();Path('/workspace/scratch/ct55-recovery-handoff-governance.claim').write_text(cl+'\n')
with (r/'docs/session-logs/2026-11-13.md').open('a') as f:f.write(f'''\n## {datetime.now(timezone.utc).isoformat()} — G3 recovery metadata governance terminal (Codex)\n\n### What was done (commands, files, features)\n+- {cl}:own de0b472 ledger/prose/workingcachedwhitespace/wiring0;current807 including prior794 unchanged;only metadata/raw handoffs staged. Artifact ACTUAL1 selects EXACTFOURwhole archives,explicitpending genuine trusted-base publication. Unit7f48f6failedbeforepoints;Lean d9f9f6local12/0trusted;corrective6a3cfe frozen before code/unit02. [pcc:unverified]\n\n### Decisions made (and why)\n+- Normal expected-parent/nonforce Git publication of whole recovery state,then implement explicit model transport with unchanged full fixture/test criteria.\n\n### Open threads (unfinished, pick up next time)\n+- Actual whole remote replay/manifest bot/exactdisjointFF;correctednativeunit02/exactLeanbindings/modellemma/reports/CLI/source commit/consumer replay.\n\n### Errors (what went wrong, how it was handled)\n+- Pending artifact gate is not green or waived;source snapshots retained whole. Actual scalar-only historical-model assumption failed and is corrected additively with explicit checked transport.\n''')
selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','commit','--quiet','-m','research: retain genus-three failures and freeze exact model transport'],cwd=r,check=True);print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
