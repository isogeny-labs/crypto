"""Seal complete recorded native/Lean observations; no mathematical computation."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_cubic_inverse_20261117';h=Path('/dev/shm/ct783-first-cubic15B');o=s/'first-frozen-15B-success-02';o.mkdir(exist_ok=False)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
j=json.loads((h/'all-phases.json').read_text());assert j['source_commit']=='fe5d0da839e0f9496f7b0326feebbedb9b80cc64' and j['first_failure'] is None
assert len(j['phases'])==30 and all(p['exit']==p['expected_exit'] for p in j['phases'])
stdout=(h/'23-first-frozen-arithmetic.stdout').read_text();stderr=(h/'23-first-frozen-arithmetic.stderr').read_text();axioms=(h/'19-first-frozen-Lean-proof.stdout').read_text()
accepted='CT15|accepted|cubic=64|noncubic=8|zero_rejections=2|wrong_sign=8|constructor_controls=3|forward=17|dual=17|closed_orbits=2|shortcuts=1054|general=0|no_timing_ratio|unsigned'
assert accepted in stdout and '1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out' in stdout
assert stderr.count('zero in compositum inverse')==2 and axioms.count('depends on axioms: [propext, Classical.choice, Quot.sound]')==3
assert sha(h/'compiler-output')==sha(h/'immutable-qualifier') and (h/'immutable-qualifier').stat().st_mode&0o7777==0o555
files={('custody/'+p.relative_to(h).as_posix()):p for p in sorted(h.rglob('*')) if p.is_file()}
for suffix in ['stdout','stderr','exit']:
 p=Path('/workspace/scratch/ct783-first-cubic15B-caller.'+suffix);files['outer-caller/'+p.name]=p
a=Path('/dev/net/chained-transfers-owned-storage-20261117/native15-complete-packets/entire-first15B-source-full-three-rlibs-both-ELFs-Lean-object-and-all-logs.tar.xz');assert not a.exists()
with tarfile.open(a,'w:xz',preset=6) as t:
 for n,p in files.items():t.add(p,arcname=n,recursive=False)
inventory=[]
with tarfile.open(a,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for n,p in files.items():
  member=t.getmember(n);b=t.extractfile(member).read();assert b==p.read_bytes() and member.mode==p.stat().st_mode&0o7777
  inventory.append({'member':n,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':member.mode})
for p in sorted(h.iterdir()):
 if p.is_file() and p.suffix in ['.stdout','.stderr','.json','.txt']:shutil.copy2(p,o/p.name)
for suffix in ['stdout','stderr','exit']:shutil.copy2(Path('/workspace/scratch/ct783-first-cubic15B-caller.'+suffix),o/('outer-caller.'+suffix))
data={'recorded':datetime.now(timezone.utc).isoformat(),'protocol_commit':'8dfcbd23ec70b3aa8baa497fe35dd3dfa1e57e74','first_source_A':'14e707ea89a6f2b50f1331db50faff899255e678','successful_source_B':j['source_commit'],'only_B_command_repair':'explicit Lean --root=custody; all original mathematical criteria and source unchanged','all30_phase_expectations_match':True,'native_summary':accepted,'native_test_scope':'1 new passed;14 inherited compiled/filtered','complete_stdout':{'bytes':(h/'23-first-frozen-arithmetic.stdout').stat().st_size,'sha256':sha(h/'23-first-frozen-arithmetic.stdout')},'complete_two_expected_panic_diagnostics':{'bytes':(h/'23-first-frozen-arithmetic.stderr').stat().st_size,'sha256':sha(h/'23-first-frozen-arithmetic.stderr')},'complete_Lean_axioms_stdout':axioms,'Lean_scope':'three conditional standard Lean.Grind.Field identities; no Rust cubic Field instance or general Picard formalization','compiler_ELF_and_copied_qualifier_sha256':sha(h/'compiler-output'),'whole_packet':{'path':str(a),'bytes':a.stat().st_size,'sha256':sha(a),'members':len(files),'all_full_bytes_and_modes_compared':True},'artifact_status':'whole raw packet retained privately while S3 migration unresolved; all complete textual outputs, source and custody records published under unchanged artifact policy','cost_scope':'exact CubicField multiplication wrapper calls only, not complete Fp-operation cost or runtime; inner inverse input changes from a^2 to a','not_independent_arithmetic_or_signed_verification':True,'prospective_general_zero_criterion_passed_without_revision':True,'unchanged_open_obligations':['original full EKS populations','paid STOPe42949','locked vault8edec422','signed independent replay','generic Rust Field/Picard bridge','admitted matched complete costs']}
(o/'ACTUAL_FIRST15B_SUCCESS.json').write_text(json.dumps(data,indent=2)+'\n');(o/'WHOLE_PACKET_INVENTORY.json').write_text(json.dumps(inventory,indent=2)+'\n');shutil.copy2(Path(__file__),o/'seal-complete-metadata-only.py')
print(json.dumps(data,indent=2))
