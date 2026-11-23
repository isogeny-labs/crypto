"""Version a package-root command repair; metadata only, no proof/scientific call."""
from pathlib import Path
from datetime import datetime,timezone
import ast,difflib,hashlib,json,subprocess,shutil
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_cubic_inverse_20261117'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
old=json.loads((s/'SOURCE_LOCK_15.json').read_text())
for row in old['files']:assert sha(s/row['path'])==row['sha256'],row['path']
negative=json.loads((s/'first-frozen-15A-Lean-root-negative-01/ACTUAL_FIRST15A_ROOT_NEGATIVE.json').read_text())
claim=Path('/workspace/scratch/ct780-first15A-negative.claim').read_text().strip()
with (r/'docs/session-logs/2026-11-17.md').open('a') as f:f.write('\n\n---\n\n'+datetime.now(timezone.utc).isoformat()+'\n# Preserve first15A root-path failure before a separately frozen caller repair\n\n## What was done\n'+claim+': frozen14e707ea produces25 phases,24 matching expectations and only Lean root-path exit1. Rust compile/strictClippy/fmt pass, proof not elaborated and arithmetic not called. Entire65-member1737984B archive4c36de55 includes all3 rlibs, compiled ELF, source/proof and complete logs; every full byte/mode compared. All after guards pass. [pcc:unverified]\n\n## Decisions made\nNew15B caller only will pass an explicit Lean package root covering the copied immutable proof, as documented by Lean4.35 --help. All Rust/Lean/protocolA bytes,34 paired calls, controls, strict general-zero acceptance,120s limits and8/3MiB floors remain. PreserveA source and complete first-negative data before B. [pcc:unverified]\n\n## Open threads\nFreeze/publicly read back B source then execute its first unit; inspectedA report, full4544CI custody, updated3904 and eventual regular15 publication. [pcc:unverified]\n\n## Errors\nThe immutable proof lives beside source/, while Lean defaults its root to source/; the explicit --root is a command-path repair, not a mathematical change or raised proof limit. No15 result or axiom success yet. [pcc:unverified]\n')
helptext=subprocess.run(['/home/agent/.elan/toolchains/leanprover--lean4---v4.35.0/bin/lean','--help'],stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True)
assert b'--root=dir' in helptext.stdout
(s/'LEAN435_COMPLETE_HELP_FOR_ROOT.txt').write_bytes(helptext.stdout);assert not helptext.stderr
protocol='''# Prospective caller revision15B: explicit Lean package root

The first frozen15A source14e707ea stopped at the Lean executable before proof
elaboration: its default root was custody/source/, while the immutable proof was
in custody/immutable-proof-source/. Complete first-negative packet and source A
remain unchanged. A new caller passes --root=<custody> to the same pinned Lean
binary, covering the separately copied read-only proof. The compiler's complete
--help documents this option. This is a package-path repair only. [pcc:unverified]

Every original PROTOCOL_15.md criterion remains literal:64/8 primitive inputs,
2 zero/8 sign/3 constructor controls, all17forward+17dual calls, exact routes,
count identities and aggregate candidate general-inverse=0. Rust and Lean proof
files remain byte-identical to14e707ea. No math formula, input, sign, code counter,
test selection, timeout, resource floor, proof axiom or proof limit is changed.
Freeze this new caller/lock publicly before its first scientific call. If proof
or arithmetic fails, preserve the full packet before any further revision.
This revision is not evidence that either will pass. [pcc:unverified]
'''
(s/'PROTOCOL_15B_CALLER_ROOT.md').write_text(protocol)
before=(s/'run_first_unit_15.py').read_text()
after=before.replace("lock=study/'SOURCE_LOCK_15.json'","lock=study/'SOURCE_LOCK_15B.json'")
needle="lean,'-DwarningAsError=true','-o'";assert after.count(needle)==1
after=after.replace(needle,"lean,'--root='+str(hold),'-DwarningAsError=true','-o'")
assert before!=after;ast.parse(after);(s/'run_first_unit_15B.py').write_text(after)
diff=''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='frozen-A/run_first_unit_15.py',tofile='new-B/run_first_unit_15B.py'))
metadata={'prepared':datetime.now(timezone.utc).isoformat(),'original_source_A':'14e707ea89a6f2b50f1331db50faff899255e678','first_negative_claim_path':claim,'changes_full_diff':diff,'all28_original_A_source_hashes_still_equal':True,'all_scientific_source_and_original_acceptance_unchanged':True,'metadata_AST_parse_only':True,'B_scientific_compiler_proof_native_calls':0,'whole_failed_A_packet':negative['whole_packet']}
(s/'ACTUAL_CALLER_15B_PREPARATION.json').write_text(json.dumps(metadata,indent=2)+'\n')
paths=[row['path'] for row in old['files'] if row['path']!='run_first_unit_15.py']+['run_first_unit_15B.py','PROTOCOL_15B_CALLER_ROOT.md','ACTUAL_CALLER_15B_PREPARATION.json','LEAN435_COMPLETE_HELP_FOR_ROOT.txt']
rows=[{'path':n,'bytes':(s/n).stat().st_size,'sha256':sha(s/n),'mode':(s/n).stat().st_mode&0o7777} for n in sorted(paths)]
(s/'SOURCE_LOCK_15B.json').write_text(json.dumps({'schema':'source-input-lock15B/v1','time':datetime.now(timezone.utc).isoformat(),'public_original_protocol':'8dfcbd23ec70b3aa8baa497fe35dd3dfa1e57e74','original_A_source':'14e707ea89a6f2b50f1331db50faff899255e678','files':rows,'unchanged_original_native_filter':'nonsplit::map::tests_14::frozen_cubic_inverse_15','unchanged_original_strict_general_zero_criterion':True,'new_B_scientific_execution_not_started':True},indent=2)+'\n')
shutil.copy2(Path(__file__),s/'prepare-caller-B-metadata-only.py')
print(json.dumps({'B_lock_files':len(rows),'full_diff':diff,'science_bytes_unchanged':True},indent=2))
