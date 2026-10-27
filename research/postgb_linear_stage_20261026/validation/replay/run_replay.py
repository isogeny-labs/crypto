#!/usr/bin/env python3
"""Run the replay invocations with the pinned binaries; record what ran.

argv[1]: sample.json (from sample.py).  Binaries are the pinned copies in
PINNED; each one's sha256 is taken immediately before its launch.  Every
invocation runs with cwd /home/user/crypto and writes its JSON and stderr
under validation/replay/out/.  The journal (validation/replay/journal.json)
lists argv, env overrides, binary sha256, exit code, wall seconds and the
sha256 of every output.  Nothing here reads or writes the producer's records.
"""
import hashlib
import json
import os
import subprocess
import sys
import time
from pathlib import Path

REPO = Path("/home/user/crypto")
PINNED = Path("/tmp/claude-0/-home-user/a7c5a28f-8713-55c7-9cd1-51f299a2e440/scratchpad/pinned")
HERE = REPO / "research/postgb_linear_stage_20261026/validation/replay"
OUT = HERE / "out"
PIN = {"postgb_la_bench": "c5612e2eb33393ca4d446b482685179a8418ff9d52682dd70c53d62f0e5ba1d3",
       "jv_cover": "afa830fc99087f12978f1e072c16b3f6e164c99b2b15e17fd6a3a7cc92bb050f"}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(name: str, binary: str, args: list[str], env_extra: dict[str, str], journal: list) -> None:
    exe = PINNED / binary
    digest = sha(exe)
    if digest != PIN[binary]:
        raise SystemExit(f"{exe} sha256 {digest} is not the pin {PIN[binary]}")
    out_json = OUT / f"{name}.json"
    err_path, std_path = OUT / f"{name}.stderr", OUT / f"{name}.stdout"
    if out_json.exists():
        raise SystemExit(f"{out_json} exists; outputs are write-once")
    argv = [str(exe)] + args + ["--json", str(out_json.relative_to(REPO))]
    env = dict(os.environ, **env_extra)
    for var in ("ZERO_DIM_LA_SIMD", "ZERO_DIM_LA_THREADS", "JV_LIN"):
        if var not in env_extra:
            env.pop(var, None)
    started = time.time()
    proc = subprocess.run(argv, cwd=REPO, env=env, capture_output=True)
    secs = round(time.time() - started, 3)
    std_path.write_bytes(proc.stdout)
    err_path.write_bytes(proc.stderr)
    entry = {"name": name, "argv": argv, "cwd": str(REPO), "env": env_extra,
             "binary": str(exe), "binary_sha256": digest, "exit_code": proc.returncode,
             "seconds": secs, "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(started)),
             "outputs": {str(p.relative_to(REPO)): (sha(p) if p.exists() else None)
                         for p in (out_json, std_path, err_path)}}
    journal.append(entry)
    (HERE / "journal.json").write_text(json.dumps(journal, indent=1) + "\n")
    print(f"{name}: exit {proc.returncode} in {secs} s", flush=True)


def main() -> None:
    sample = json.loads(Path(sys.argv[1]).read_text())
    OUT.mkdir(parents=True, exist_ok=True)
    journal: list = []
    for gid, group in sample["groups"].items():
        run(f"bench_{gid}", "postgb_la_bench", group["argv"], {}, journal)
    dlp = ["--exp", "dlp", "--sizes", "53", "--seeds", "1", "--rho-runs", "4", "--check-every", "64"]
    run("dlp_fast_p53_s1", "jv_cover", dlp, {"JV_LIN": "fast"}, journal)
    run("dlp_sparse_p53_s1", "jv_cover", dlp, {"JV_LIN": "sparse"}, journal)


if __name__ == "__main__":
    main()
