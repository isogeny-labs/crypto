#!/usr/bin/env python3
"""Frozen fixed-target process panel; Python only orchestrates and verifies."""
import gzip
import hashlib
import json
import os
import platform
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
BIN = Path(os.environ.get("M31_PANEL_BIN", "/workspace/scratch/target/release"))
OUT = HERE / "results"
P = 2_147_483_647
A = 847_514_846
N = 2_147_446_831
G = (625_037_715, 2_100_627_251)
Q = (976_979_610, 1_228_979_761)
SEEDS = [
    0x0EC0550C9344EB9E, 0xF10C1A4A1CD16E4E,
    0xB63D134C29E5A4E3, 0xC3D3B32963D19ADF,
    0xAF062FA466F7D0D4, 0x8F46B508F3A94547,
    0xF18D577C0116286F, 0xDDFB8A4A903F3935,
]


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % P == 0:
        return None
    if x1 == x2:
        slope = ((3 * x1 * x1 + A) * pow(2 * y1, -1, P)) % P
    else:
        slope = ((y2 - y1) * pow(x2 - x1, -1, P)) % P
    x3 = (slope * slope - x1 - x2) % P
    return x3, (slope * (x1 - x3) - y1) % P


def mul(k, point):
    out = None
    while k:
        if k & 1:
            out = add(out, point)
        point = add(point, point)
        k >>= 1
    return out


def sha(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def run(arm, round_no, seed=None, aa=False):
    name = f"{round_no:02d}-{arm}" + ("-aa" if aa else "")
    cmd = [str(BIN / ("m31-ic-fast-panel" if arm == "ic" else "m31-rho-seeded-panel"))]
    if arm == "ic":
        cmd += ["40", "12345678", "4"]
    else:
        cmd += [f"0x{seed:016x}"]
    stdout_path = OUT / f"{name}.stdout"
    stderr_path = OUT / f"{name}.stderr"
    with stdout_path.open("wb") as stdout, stderr_path.open("wb") as stderr:
        start = time.perf_counter_ns()
        proc = subprocess.Popen(cmd, stdout=stdout, stderr=stderr)
        _, status, usage = os.wait4(proc.pid, 0)
        wall_ns = time.perf_counter_ns() - start
    raw = stdout_path.read_bytes()
    result = None
    error = None
    if status == 0:
        try:
            result = json.loads(raw)
        except (ValueError, UnicodeError) as exc:
            error = f"invalid JSON: {exc}"
    else:
        error = f"process status {status}"
    if result is not None:
        scalar = result.get("recovered_scalar")
        if result.get("public_target") != list(Q):
            error = "target mismatch"
        elif scalar is not None and mul(scalar, G) != Q:
            error = "independent scalar replay failed"
        elif scalar is None:
            error = "censored without scalar"
        elif arm == "ic" and result.get("recovered_scalar_replay") is not True:
            error = "IC internal replay failed"
        elif arm == "rho" and result.get("verified") is not True:
            error = "rho internal replay failed"
    with gzip.GzipFile(filename=str(stdout_path) + ".gz", mode="wb", mtime=0) as compressed:
        compressed.write(raw)
    stdout_path.unlink()
    record = {
        "arm": arm, "round": round_no, "seed": seed, "aa": aa,
        "command": cmd, "wait_status": status, "error": error,
        "wall_ns": wall_ns, "cpu_user_s": usage.ru_utime,
        "cpu_sys_s": usage.ru_stime, "max_rss_kb": usage.ru_maxrss,
        "stdout_sha256": hashlib.sha256(raw).hexdigest(),
        "stdout_gzip": str(stdout_path.name) + ".gz",
        "stderr": stderr_path.name,
        "recovered_scalar": result.get("recovered_scalar") if result else None,
        "internal_wall_ns": result.get("cold_wall_ns") if result else None,
        "ic_probes": result.get("probe_additions") if arm == "ic" and result else None,
        "ic_rows": len(result.get("relation_rows", [])) if arm == "ic" and result else None,
        "ic_queries": len(result.get("queries", [])) if arm == "ic" and result else None,
        "rho_walks": result.get("walks") if arm == "rho" and result else None,
        "rho_steps": result.get("steps") if arm == "rho" and result else None,
        "rho_adds": result.get("group_adds") if arm == "rho" and result else None,
        "rho_cycles": result.get("cycles") if arm == "rho" and result else None,
        "rho_distinguished_points": result.get("distinguished_points") if arm == "rho" and result else None,
    }
    print(json.dumps({k: record[k] for k in ("arm", "round", "seed", "error", "wall_ns", "recovered_scalar")}), flush=True)
    return record


def main():
    assert mul(N, G) is None
    assert mul(987_654_321, G) == Q
    for i, seed in enumerate(SEEDS):
        expected = int.from_bytes(hashlib.sha256(f"m31-rho-seed-panel-20261128:{i}".encode()).digest()[:8], "big")
        assert seed == expected
    OUT.mkdir(exist_ok=False)
    source = ROOT / "research/prime31_graph_ecdlp_20261123/native/mitm_four_fast.rs"
    assert sha(source) == "098c6d3c67440677f0b53cf81ede2ddd6850c4ec600345635f526f6de4519c60"
    source_paths = [
        source, HERE / "rho_neg_dp_seeded.rs", HERE / "Cargo.toml",
        HERE / "fixture/Cargo.toml", HERE / "fixture/src/lib.rs", HERE / "run_panel.py",
        ROOT / "research/prime31_four_summand_20261122/native/b677_fixture.rs",
        ROOT / "research/prime31_four_summand_20261122/native/original-native/Cargo.toml",
        ROOT / "research/prime31_ic_20261115/native/lib.rs",
        ROOT / "research/prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv",
        ROOT / "src/cryptanalysis/f4_fp_tower.rs", ROOT / "src/cryptanalysis/sig_fp_tower.rs",
    ]
    manifest = {
        "git_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "python": sys.version, "platform": platform.platform(),
        "rustc": subprocess.check_output(["/workspace/scratch/cargo/bin/rustc", "--version"], text=True).strip(),
        "sources": {str(path.relative_to(ROOT)): sha(path) for path in source_paths},
        "binaries": {arm: sha(BIN / arm) for arm in ["m31-ic-fast-panel", "m31-rho-seeded-panel"]},
    }
    (OUT / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    records = []
    for i, seed in enumerate(SEEDS):
        order = ("ic", "rho") if i % 2 == 0 else ("rho", "ic")
        for arm in order:
            records.append(run(arm, i, seed if arm == "rho" else None))
    records.append(run("rho", 8, SEEDS[0], aa=True))
    with (OUT / "records.jsonl").open("w") as stream:
        for record in records:
            stream.write(json.dumps(record, sort_keys=True) + "\n")
    for record in records:
        assert record["error"] is None, record
    assert records[-1]["rho_steps"] == records[1]["rho_steps"]
    assert records[-1]["rho_adds"] == records[1]["rho_adds"]


if __name__ == "__main__":
    main()
