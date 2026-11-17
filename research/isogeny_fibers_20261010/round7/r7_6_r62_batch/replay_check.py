"""Replay certificate for the native example's records.

Tooling only (no result is computed here): re-runs the recorded command line
of each `runs/*_v1.json` with the native example binary into a scratch
directory, and compares the replay with the committed record on everything
except the contended wall-clock note and the `--out` argument. Writes
`replay_certificate.json` with the SHA-256 of every committed record and the
verdict per file.

    python3 -I replay_check.py BINARY SCRATCH_DIR [--skip NAME ...]
"""
import hashlib, json, pathlib, subprocess, sys

here = pathlib.Path(__file__).resolve().parent
binary, scratch = sys.argv[1], pathlib.Path(sys.argv[2])
skip = set(sys.argv[sys.argv.index("--skip") + 1:]) if "--skip" in sys.argv else set()
scratch.mkdir(parents=True, exist_ok=True)


def norm(d):
    d = dict(d)
    d.pop("wall_seconds_contended", None)
    d["args"] = [a for a in d["args"] if "/" not in a and not a.endswith(".json")]
    return d


rows = []
for rec in sorted((here / "runs").glob("*_v[0-9].json")):
    if rec.name in skip:
        rows.append({"file": rec.name, "replayed": False, "reason": "skipped (time)"})
        continue
    old = json.loads(rec.read_text())
    out = scratch / rec.name
    if old["schema"] == "r7_6.scaling_fit/v1":
        # A fit record names its inputs and bootstrap instead of an argv.
        args = ["--mode", "fit", "--inputs", ",".join(old["inputs"]),
                "--nboot", str(old["bootstrap"]["nboot"]), "--out", str(out)]
        old["args"] = []
    else:
        args = list(old["args"])
        args[args.index("--out") + 1] = str(out)
    subprocess.run([binary] + args, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    new = json.loads(out.read_text())
    new.setdefault("args", [])
    rows.append({
        "file": rec.name,
        "sha256": hashlib.sha256(rec.read_bytes()).hexdigest(),
        "replayed": True,
        "identical_except_wall": norm(old) == norm(new),
    })
    print(rows[-1], flush=True)
(here / "replay_certificate.json").write_text(json.dumps({
    "schema": "r7_6.replay/v1",
    "binary": binary.split("/")[-1],
    "binary_sha256": hashlib.sha256(pathlib.Path(binary).read_bytes()).hexdigest(),
    "records": rows,
    "all_replayed_identical": all(r.get("identical_except_wall") for r in rows if r["replayed"]),
}, indent=1) + "\n")
