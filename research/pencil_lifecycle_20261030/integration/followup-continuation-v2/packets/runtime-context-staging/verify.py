"""Read back a staged context without running any payload."""
from pathlib import Path
import hashlib,json,stat
p=Path(__file__).parent
r=json.loads((p/'staging-receipt.json').read_text()); root=Path(r['context_path'])
assert root.is_dir() and not root.is_symlink()
assert oct(stat.S_IMODE(root.stat().st_mode))==r['root_mode']
assert oct(stat.S_IMODE(root.parent.stat().st_mode))==r['private_parent_mode']
assert {str(f.relative_to(root)) for f in root.rglob('*')}=={m['context_path'] for m in r['members']}
for m in r['members']:
 f=root/m['context_path']; s=f.lstat()
 assert oct(stat.S_IMODE(s.st_mode))==m['mode']
 if m['kind']=='file':
  assert stat.S_ISREG(s.st_mode) and s.st_size==m['bytes']
  assert hashlib.sha256(f.read_bytes()).hexdigest()==m['sha256']
 else: assert m['kind']=='directory' and stat.S_ISDIR(s.st_mode)
print(json.dumps({'members_checked':len(r['members']),'exact_inventory_match':True}))
