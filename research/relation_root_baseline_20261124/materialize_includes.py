"""Source-file metadata hydration only; no numerical/research execution.

Restore literal include and #[path] inputs only when they are tracked at the
unchanged current source commit. This is not a complete Rust parser. Dynamic
macros and absent historical inputs remain explicit compiler obligations.
"""
from pathlib import Path
import hashlib
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
STUDY = Path(__file__).resolve().parent
GIT = ['git', '-c', 'gc.auto=0', '-c', 'maintenance.auto=false']
ROOTS = ['src', 'crates', 'examples', 'tools/factor-base', 'tools/volcano-cli',
         'isogeny_algos', 'research/pencil_large_prime_20261029']
seen = set()
restored = {}
absent = set()
outside = set()
literal = re.compile(r'(?:include(?:_str|_bytes)?!\s*\(\s*|#\[path\s*=\s*)"([^"\n]+)"')
pending = [p for d in ROOTS for p in (ROOT / d).rglob('*.rs')]
while pending:
    source = pending.pop()
    if source in seen:
        continue
    seen.add(source)
    for name in literal.findall(source.read_text(errors='replace')):
        if '\\' in name:
            continue
        target = (source.parent / name).resolve()
        if not target.is_relative_to(ROOT):
            outside.add(str(target))
            continue
        if target.is_file():
            continue
        rel = target.relative_to(ROOT).as_posix()
        entry = subprocess.check_output(GIT + ['ls-tree', 'HEAD', '--', rel], cwd=ROOT, text=True)
        if not entry or entry.split()[0] != '100644':
            absent.add(rel)
            continue
        subprocess.run(GIT + ['restore', '--source=HEAD', '--worktree',
                             '--ignore-skip-worktree-bits', '--', rel], cwd=ROOT, check=True)
        restored[rel] = {'git_blob': entry.split()[2],
                         'sha256': hashlib.sha256(target.read_bytes()).hexdigest()}
        if target.suffix == '.rs':
            pending.append(target)
receipt = {'schema': 'literal-source-hydration/v1',
           'canonical': subprocess.check_output(GIT + ['rev-parse', 'origin/main'], cwd=ROOT, text=True).strip(),
           'restored': restored, 'absent_tracked_inputs': sorted(absent),
           'outside_root_not_restored': sorted(outside),
           'dynamic_macros': 'not resolved; compiler remains authoritative',
           'no_generated_or_historical_substitutes': True,
           'numerical_execution': False}
with (STUDY / 'INCLUDE_HYDRATION_436.json').open('x') as stream:
    json.dump(receipt, stream, indent=2)
    stream.write('\n')
print(json.dumps({'restored_files': len(restored), 'absent_inputs': len(absent),
                  'outside_root': len(outside), 'complete_parser': False}))
