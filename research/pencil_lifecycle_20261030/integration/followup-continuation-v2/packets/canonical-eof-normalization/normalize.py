from pathlib import Path
import hashlib,json,subprocess,os
p=Path(__file__).parent
sha=lambda b:hashlib.sha256(b).hexdigest()
rows=[]
for name,old in [('Constructor','/var/tmp/pencil-weighted-constructor-invariant-20261031-v1'),('Add11','/var/tmp/pencil-add11-distinct-invariant-20261031-v1')]:
 o=Path(old);b=(o/(name+'.lean')).read_bytes();n=b.rstrip(b'\n')+b'\n'
 assert b.startswith(n) and set(b[len(n):])<={10} and len(b)>len(n)
 (p/(name+'.original.lean')).write_bytes(b);(p/(name+'.lean')).write_bytes(n)
 deps=json.loads((o/'dependency-hashes.json').read_text())
 for d in deps:assert sha(Path(d['path']).read_bytes())==d['sha256'],d['path']
 (p/(name+'.dependency-hashes.json')).write_text(json.dumps(deps,indent=2)+'\n')
 rows.append({'module':name,'original_path':str(o/(name+'.lean')),'original_sha256':sha(b),'normalized_sha256':sha(n),'removed_terminal_lf_bytes':len(b)-len(n),'exactly_equal_modulo_terminal_lf_normalization':True,'dependency_snapshot_hashes_checked':True})
(p/'normalization.json').write_text(json.dumps(rows,indent=2)+'\n')
lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean'
env=os.environ.copy();env['LEAN_SYSROOT']=str(Path(lean).parent.parent);env['LEAN_PATH']=str(p)+':/var/tmp/pencil-word-error-exclusion-review-20261031-v1/replay:/var/tmp/independent-same-support-replay-ow8hrtl9:/var/tmp/pencil-word-classifier-v2-20261031/dependency-union'
for row in rows:
 name=row['module'];cmd=['timeout','-k','5s','120s',lean,'-j1','-M4096','-o',str(p/(name+'.olean')),str(p/(name+'.lean'))]
 r=subprocess.run(cmd,cwd=p,env=env,capture_output=True);(p/(name+'.stdout')).write_bytes(r.stdout);(p/(name+'.stderr')).write_bytes(r.stderr)
 (p/(name+'.receipt.json')).write_text(json.dumps({'command':cmd,'exit_code':r.returncode,'source_sha256':row['normalized_sha256'],'LEAN_PATH':env['LEAN_PATH'],'lean_binary_sha256':sha(Path(lean).read_bytes())},indent=2)+'\n');row['fresh_compile_exit_code']=r.returncode
(p/'status.json').write_text(json.dumps({'scope':'terminal newline normalization only; no new mathematics','modules':rows,'all_fresh_compiles_passed':all(r['fresh_compile_exit_code']==0 for r in rows),'independent_review':None,'root_files_edited':False,'scientific_execution':False},indent=2)+'\n')
(p/'README.md').write_text('''# Canonical terminal-newline normalization

The two normalized modules remove only surplus LF bytes at EOF, leaving exactly one final newline. Original exact reviewed sources are retained alongside the normalized files, and original packets are unchanged. `normalization.json` binds both versions and checks byte equality modulo this precise transform. Every dependency listed in each original snapshot was hash-checked before compilation.

Each normalized module was freshly compiled against the same retained dependency search paths with the pinned Lean binary, one thread, 4096 MiB and a 120-second timeout. Native stdout, stderr, commands, exit codes and tool hashes are retained. This is formatting normalization, not a new theorem or independent semantic review. Root integration is separate.
''')
(p/'manifest.json').write_text(json.dumps({'files':{q.name:{'sha256':sha(q.read_bytes()),'bytes':q.stat().st_size} for q in sorted(p.iterdir()) if q.is_file() and q.name!='manifest.json'}},indent=2)+'\n')
print((p/'status.json').read_text());print('manifest_sha256='+sha((p/'manifest.json').read_bytes()))
