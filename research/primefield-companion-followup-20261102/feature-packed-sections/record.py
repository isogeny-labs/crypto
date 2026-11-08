from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;root=p.parents[2];sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
rows=json.loads((p/'copied-sources.json').read_text())['files']
for row in rows:assert sha(p/row['file'])==row['sha256']==sha(p.parent/'feature-oriented-radius'/row['file'])
for n in [1,2]:
 v=p/f'validation{n:02}';binary=Path(f'/tmp/t1930-v{n}-check');deps=[(root/x).resolve() for x in Path(str(binary)+'.d').read_text().splitlines()[0].split(': ',1)[1].split()]
 hashes={str(f.relative_to(root)):sha(f) for f in deps}
 if n==1:hashes[str((p/'driver.rs').relative_to(root))]=sha(v/'driver.rs.snapshot')
 assert not (v/'check.stderr').read_bytes()
 receipt={'scope':'deterministic software qualification, not timing','revision':n,'build_exit':0,'check_exit':0,'limits_seconds':{'compile':90,'focused':60},'binary_sha256':sha(binary),'dependencies':hashes,'copied_sources':len(rows),'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},'commands':['rustc --edition=2021 -O '+('-C overflow-checks=yes ' if n==2 else '')+'--emit=link,dep-info '+str(p.relative_to(root)/'driver.rs')+' -o '+str(binary),str(binary)]}
 if n==2:
  ref=p.parent/'feature-native-planned-width/validation01/check.stdout';assert (v/'inherited.stdout').read_bytes()==ref.read_bytes();assert not(v/'inherited.stderr').read_bytes()
  receipt.update(inherited_exit=0,inherited_byte_equal=True,reference_sha256=sha(ref));receipt['limits_seconds']['inherited']=60;receipt['commands'].append(str(binary)+' --inherited')
 (v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
 print(n,'dependencies',len(deps),'copied',len(rows))
