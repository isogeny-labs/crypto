#!/usr/bin/env python3
"""The validator's own runs of the dump-enabled jv_cover (243a718e41).

Not the producer's scalar-dump runs: this re-runs part of them (three of the
eight instances) so that the dumped instances can be tied to a run the
validator made.  Writes validation/replay/out/dump_*.{json,jsonl,stdout,stderr}
and validation/replay/journal-dump.json (argv, env, binary sha256, exit code,
seconds, output hashes).
"""
import hashlib
import json
import os
import subprocess
import time
from pathlib import Path

REPO = Path("/home/user/crypto")
EXE = Path("/tmp/claude-0/-home-user/a7c5a28f-8713-55c7-9cd1-51f299a2e440/scratchpad/pinned/jv_cover_dump")
PIN = "b2b8ab340fb6ab5dfca09552226b2005a1495e3ed1045d4bbe3250f15a8b3300"
HERE = REPO / "research/postgb_linear_stage_20261026/validation/replay"
OUT = HERE / "out"
BASE = ["--exp", "dlp", "--rho-runs", "4", "--check-every", "64"]
RUNS = [("dump_fast_p53-101_s1", "fast", ["--sizes", "53,101", "--seeds", "1"]),
        ("dump_sparse_p101_s2", "sparse", ["--sizes", "101", "--seeds", "1", "--first-seed", "2"])]


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    journal = []
    for name, mode, extra in RUNS:
        digest = sha(EXE)
        if digest != PIN:
            raise SystemExit(f"{EXE} sha256 {digest} is not the pin")
        js, dump = OUT / f"{name}.json", OUT / f"{name}.instances.jsonl"
        so, se = OUT / f"{name}.stdout", OUT / f"{name}.stderr"
        for p in (js, dump):
            if p.exists():
                raise SystemExit(f"{p} exists")
        argv = [str(EXE)] + BASE + extra + ["--json", str(js.relative_to(REPO))]
        env_extra = {"JV_LIN": mode, "JV_DUMP_INSTANCE": str(dump.relative_to(REPO))}
        env = dict(os.environ, **env_extra)
        for var in ("ZERO_DIM_LA_SIMD", "ZERO_DIM_LA_THREADS"):
            env.pop(var, None)
        t0 = time.time()
        proc = subprocess.run(argv, cwd=REPO, env=env, capture_output=True)
        so.write_bytes(proc.stdout)
        se.write_bytes(proc.stderr)
        journal.append({"name": name, "argv": argv, "cwd": str(REPO), "env": env_extra, "binary": str(EXE),
                        "binary_sha256": digest, "exit_code": proc.returncode,
                        "seconds": round(time.time() - t0, 3),
                        "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(t0)),
                        "outputs": {str(p.relative_to(REPO)): (sha(p) if p.exists() else None)
                                    for p in (js, dump, so, se)}})
        (HERE / "journal-dump.json").write_text(json.dumps(journal, indent=1) + "\n")
        print(name, "exit", proc.returncode, flush=True)


if __name__ == "__main__":
    main()
