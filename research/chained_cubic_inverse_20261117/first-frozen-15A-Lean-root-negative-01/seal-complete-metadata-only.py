"""Retain an entire failed build/proof packet; metadata and byte custody only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_cubic_inverse_20261117';hold=Path('/dev/shm/ct778-first-cubic15');o=s/'first-frozen-15A-Lean-root-negative-01';o.mkdir(exist_ok=False)
private=Path('/dev/net/chained-transfers-owned-storage-20261117/native15-complete-packets');private.mkdir(exist_ok=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
j=json.loads((hold/'all-phases.json').read_text());assert j['source_commit']=='14e707ea89a6f2b50f1331db50faff899255e678' and j['first_failure']=='first-frozen-Lean-proof'
assert len(j['phases'])==25 and [p['name'] for p in j['phases'] if p['exit']!=p['expected_exit']]==['first-frozen-Lean-proof']
error=(hold/'19-first-frozen-Lean-proof.stderr').read_text();assert 'must be contained in root directory' in error
assert not (hold/'immutable-qualifier').exists() and not list(hold.glob('*first-frozen-arithmetic*'))
files={('custody/'+p.relative_to(hold).as_posix()):p for p in sorted(hold.rglob('*')) if p.is_file()}
for suffix in ['stdout','stderr','exit']:
 p=Path('/workspace/scratch/ct778-first-cubic15-caller.'+suffix);files['outer-caller/'+p.name]=p
archive=private/'entire-first15A-source-full-three-rlibs-compiled-ELF-proof-and-all-logs.tar.xz'
assert not archive.exists()
with tarfile.open(archive,'w:xz',preset=6) as t:
 for name,p in files.items():t.add(p,arcname=name,recursive=False)
inventory=[]
with tarfile.open(archive,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for name,p in files.items():
  member=t.getmember(name);b=t.extractfile(member).read();assert b==p.read_bytes() and member.mode==p.stat().st_mode&0o7777
  inventory.append({'member':name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':member.mode})
for p in sorted(hold.iterdir()):
 if p.is_file() and (p.suffix in ['.stdout','.stderr','.json','.txt']):shutil.copy2(p,o/p.name)
for suffix in ['stdout','stderr','exit']:shutil.copy2(Path('/workspace/scratch/ct778-first-cubic15-caller.'+suffix),o/('outer-caller.'+suffix))
data={'recorded':datetime.now(timezone.utc).isoformat(),'source_commit':j['source_commit'],'first_failure':j['first_failure'],'phases':25,'all_other24_expectations_match':True,'complete_Lean_root_diagnostic':error,'compiled_Rust_test_ELF_retained':True,'strict_Clippy_and_format_pass':True,'Lean_has_not_elaborated_the_proof':True,'new_native_arithmetic_not_called':True,'qualifier_copy_not_created_because_proof_prerequisite_failed':True,'all_post_guards_pass':True,'whole_packet':{'path':str(archive),'bytes':archive.stat().st_size,'sha256':sha(archive),'members':len(files),'all_full_bytes_and_modes_compared':True},'public_artifact_status':'whole raw packet retained privately; current artifact policy unchanged, S3 migration unresolved; source and all complete textual streams published separately','next_prospective_repair':'new caller revision only: explicit Lean --root covering the copied immutable proof; Rust, Lean proof, populations, criteria and limits unchanged','no_signature_or_independent_replay':True}
(o/'WHOLE_PACKET_INVENTORY.json').write_text(json.dumps(inventory,indent=2)+'\n');(o/'ACTUAL_FIRST15A_ROOT_NEGATIVE.json').write_text(json.dumps(data,indent=2)+'\n')
shutil.copy2(Path(__file__),o/'seal-complete-metadata-only.py')
print(json.dumps(data,indent=2))
