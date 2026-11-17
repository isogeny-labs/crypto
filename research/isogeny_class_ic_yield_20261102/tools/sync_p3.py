#!/usr/bin/env python3
"""Mirror main's p3_ecc2k130/full/ to exactly the outputs of COMPLETED P3 card receipts in the run worktree.
Copies each output (hash-checked against its receipt), drops files no completed receipt names, and returns the
list of files to push to the artifact store / git add. Orchestration only."""
import json, hashlib, shutil, subprocess, sys, pathlib
W = pathlib.Path('/home/user/crypto-p3run'); M = pathlib.Path('/home/user/crypto')
S = 'research/isogeny_class_ic_yield_20261102'; FULL = f'{S}/p3_ecc2k130/full'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
want = {}
for r in sorted((W/S/'cards').glob('CARD-isogeny-ic-yield-p3-*.receipt.json')):
    j = json.loads(r.read_text())
    if j.get('status') != 'completed': continue
    for c in j['commands']:
        for path, h in (c.get('outputs') or {}).items():
            if path.startswith(FULL + '/') and '/_failed/' not in path: want[path] = h
for path, h in want.items():
    src, dst = W/path, M/path
    if sha(src) != h: sys.exit(f'HASH MISMATCH in worktree: {path}')
    if not dst.exists() or sha(dst) != h: shutil.copy2(src, dst)
d = M/FULL
man = json.loads((d/'.artifacts.json').read_text()) if (d/'.artifacts.json').exists() else {}
files = man.get('files', man)
names = {pathlib.Path(p).name for p in want}
drop = [f.name for f in d.iterdir() if f.is_file() and not f.name.startswith('.') and f.name not in names]
for n in drop:
    (d/n).unlink(); files.pop(n, None)
    subprocess.run(['git','-C',str(M),'rm','-q','--cached','--ignore-unmatch',f'{FULL}/{n}'])
stale = [n for n in list(files) if n in names and files[n].get('sha256') != sha(d/n)]
if 'files' in man: man['files'] = files
(d/'.artifacts.json').write_text(json.dumps(man, indent=2, sort_keys=True) + '\n')
gi = (d/'.gitignore').read_text().splitlines() if (d/'.gitignore').exists() else []
gi = [l for l in gi if l.lstrip('/') not in drop]
(d/'.gitignore').write_text('\n'.join(gi) + '\n')
print('kept', len(names), 'dropped', drop, 'stale-manifest', stale)
big = [f'{FULL}/{n}' for n in sorted(names) if (d/n).stat().st_size > 65536 and (n.endswith(('.jsonl','.gz','.log','.json')) or (d/n).stat().st_size > 1048576)]
small = [f'{FULL}/{n}' for n in sorted(names) if f'{FULL}/{n}' not in big]
pathlib.Path('/tmp/claude-0/-home-user/fafe457a-a14a-53c5-a937-e53817fc07eb/scratchpad/sync_p3.big').write_text('\n'.join(big)+'\n')
pathlib.Path('/tmp/claude-0/-home-user/fafe457a-a14a-53c5-a937-e53817fc07eb/scratchpad/sync_p3.small').write_text('\n'.join(small)+'\n')
