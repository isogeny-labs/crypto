from pathlib import Path
import os,subprocess,json,hashlib,sys
D=Path(__file__).parent;n=sys.argv[1];tag=sys.argv[2];out=D/tag;out.mkdir();src=D/(n+'.lean');(out/src.name).write_bytes(src.read_bytes());lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean';e=os.environ.copy();e['LEAN_SYSROOT']=str(Path(lean).parent.parent);e['LEAN_PATH']=str(D)+':/var/tmp/pencil-degree22-guards-20261031-v1:/var/tmp/pencil-degree22-bzero-extraction-20261031-v1:/var/tmp/pencil-sameu-mixed-composition-20261031-v1:/var/tmp/pencil-mixed-positive-word-20261031-v1:/var/tmp/pencil-sameu-zero-slope-20261031-v1:/var/tmp/pencil-sameu-global-smooth-20261031-v1:/var/tmp/pencil-sameu-local-smooth-20261031-v1:/var/tmp/pencil-sameu-mixed-point-20261031-v1:/var/tmp/pencil-mixed-positive-distinct-20261031-v1:/var/tmp/pencil-mixed-generic-word-20261031-v1:/var/tmp/pencil-doubling-word-composition-20261031-v1:/var/tmp/pencil-weighted-doubling-invariant-20261031-v1:/var/tmp/pencil-mixed-generic-20261031-v1:/var/tmp/pencil-mixed-cancellation-20261031-v1:/var/tmp/pencil-add11-word-validity-20261031-v1:/var/tmp/pencil-add11-word-composition-20261031-v1:/var/tmp/pencil-add11-weighted-invariant-20261031-v1:/var/tmp/pencil-word-error-exclusion-review-20261031-v1/replay:/var/tmp/pencil-add11-distinct-independent-review-20261031/replay:/var/tmp/pencil-word-error-exclusion-review-20261031-v1/replay:/var/tmp/independent-same-support-replay-ow8hrtl9:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union';c=['timeout','-k','5s','120s',lean,'-j1','-M4096','-o',str(D/(n+'.olean')),str(src)]
def binding():
 paths={src,Path(__file__),Path(lean)}
 for root in e['LEAN_PATH'].split(':'):
  for ext in ('*.lean','*.olean'):
   paths.update(Path(root).glob(ext))
 paths.add(Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs'))
 paths.add(Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/PROOF_AUDIT.md'))
 return {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths) if p!=D/(n+'.olean')}
before=binding()
(out/'input-bindings-before.json').write_text(json.dumps(before,indent=2))
r=subprocess.run(c,cwd=D,env=e,capture_output=True,text=True)
after=binding()
(out/'input-bindings-after.json').write_text(json.dumps(after,indent=2))
(out/'binding-check.json').write_text(json.dumps({'unchanged':before==after,'files':len(before),'toolchain':subprocess.check_output([lean,'--version'],env=e,text=True).strip()},indent=2))
(out/'stdout').write_text(r.stdout);(out/'stderr').write_text(r.stderr);(out/'receipt.json').write_text(json.dumps({'command':c,'exit_code':r.returncode,'LEAN_PATH':e['LEAN_PATH'],'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest()},indent=2));print(r.returncode);print(r.stdout);print(r.stderr)

gate_exit = r.returncode if r.returncode else (0 if before == after else 2)
(out/'gate-exit.json').write_text(json.dumps({'native_exit_code':r.returncode,'input_bindings_unchanged':before==after,'gate_exit_code':gate_exit},indent=2)+'\n')
sys.exit(gate_exit)
