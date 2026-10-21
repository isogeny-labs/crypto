#!/usr/bin/env python3
"""Stand-in for research/ic_leads_rented_20261018/aws_spot/aws_spot.py with the
same verbs: `run JOB CELLS` prints {"id": ...}; `fetch ID DEST`; `terminate ID`.
The "host" is a detached shell writing `<UTC> <TOKEN>` lines to
.fakehost/<id>/STATUS, as boot.sh does."""
import json, shutil, subprocess, sys, time
from pathlib import Path

base = Path(".fakehost")
cmd = sys.argv[1]
if cmd == "run":
    job, cells = sys.argv[2], sys.argv[3].split(",")
    iid = f"i-{job}-{time.time_ns() % 10**8}"
    d = base / iid
    (d / "out").mkdir(parents=True)
    script = f'cd {d}; s(){{ echo "$(date -u +%FT%TZ) $1" >> STATUS; }}; s BOOT; s BUILT; '
    for c in cells:
        script += f'echo \'{{"cell":"{c}","refuted":true}}\' > out/{c}.json; s "MEASURED {c}"; '
    script += "s ALL_DONE"
    subprocess.run(["bash", "-c", script], check=True)
    print(json.dumps({"id": iid, "type": job}))
elif cmd == "fetch":
    d = base / sys.argv[2]
    dest = Path(sys.argv[3])
    if not d.exists():
        sys.exit(7)
    dest.mkdir(parents=True, exist_ok=True)
    shutil.copy(d / "STATUS", dest / "STATUS")
    shutil.copytree(d / "out", dest / "out", dirs_exist_ok=True)
elif cmd == "terminate":
    (base / sys.argv[2] / "TERMINATED").write_text("1")
    print("terminated", sys.argv[2])
