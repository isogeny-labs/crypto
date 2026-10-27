"""Infrastructure transport: read in the foreground, then pipe through busy.

The busy wrapper creates a process group; a child must not read the terminal.
Credential input stays in memory and is never printed or written to disk.
"""
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parents[3]
attempt = sys.argv[1] if len(sys.argv) == 2 else 'v1'
assert attempt in ('v1', 'v2')
print('READY: foreground hidden input; publication child receives a pipe.', flush=True)
credential_line = sys.stdin.readline()
assert credential_line.endswith('\n')
result = subprocess.run(
    [sys.executable, str(root / 'tools/isolated_bench.py'), 'busy', '--',
     'timeout', '--kill-after=10s', '1200s', sys.executable,
     str(Path(__file__).with_name('publish_archive.py')), attempt],
    cwd=root, input=credential_line, text=True,
)
credential_line = None
raise SystemExit(result.returncode)
