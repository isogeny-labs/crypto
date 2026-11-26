#!/usr/bin/env python3
"""Run one arm as a fresh process; keep stdout JSON, stderr, wall and peak RSS."""
import json, resource, subprocess, sys, time, hashlib, pathlib
out, binary, *args = sys.argv[1:]
out = pathlib.Path(out); out.parent.mkdir(parents=True, exist_ok=True)
t = time.perf_counter()
p = subprocess.run([binary, *args], capture_output=True)
wall = time.perf_counter() - t
ru = resource.getrusage(resource.RUSAGE_CHILDREN)
out.with_suffix('.json').write_bytes(p.stdout)
out.with_suffix('.stderr').write_bytes(p.stderr)
meta = {"argv": [binary, *args], "exit": p.returncode, "process_wall_s": wall,
        "max_rss_kib": ru.ru_maxrss, "binary_sha256": hashlib.sha256(open(binary,'rb').read()).hexdigest(),
        "stdout_sha256": hashlib.sha256(p.stdout).hexdigest()}
out.with_suffix('.meta.json').write_text(json.dumps(meta, indent=1))
print(json.dumps(meta))
sys.exit(p.returncode)
