#!/usr/bin/env python3
"""N83 K1120/root11 padd rank-stage panel (protocol: PROTOCOL.md).

Twenty points; per point three fresh IC processes of ONE binary that differ
only in KIC_RANK_SOLVE (unset = current producer twice, the A/A pair; `padd`
once), in a rotating Latin order, and three fresh same-point rho processes.
"""
import argparse, importlib.util, json, os, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
spec = importlib.util.spec_from_file_location(
    "n83_e2e_holdout", ROOT / "research/n83_e2e_holdout_20261016/run.py")
holdout = importlib.util.module_from_spec(spec)
spec.loader.exec_module(holdout)
parent, cold, dynamic = holdout.parent, holdout.cold, holdout.dynamic

TARGETS = HERE / "targets.jsonl"
K = 1120
RANK_SEED = 29
IC_CAP_SECONDS = 480
RHO_CAP_SECONDS = 120
RHO_DRAWS = 3
RHO_SEED_BASE = 2026101900
ARMS = ("cur_a", "cur_b", "padd")
ORDERS = [("cur_a", "cur_b", "padd"), ("cur_b", "padd", "cur_a"), ("padd", "cur_a", "cur_b"),
          ("padd", "cur_b", "cur_a"), ("cur_a", "padd", "cur_b"), ("cur_b", "cur_a", "padd")]


def schedule(points):
    return [{"point_index": i, "ic_order": list(ORDERS[(i - 1) % 6]), "rho_first": i % 2 == 0}
            for i in range(1, points + 1)]


def run_ic(output, binary, i, point_file, arm, k, group):
    name = f"p{i}_{arm}"
    folder = output / "runs" / name
    env = dynamic.environment(folder, "root11_dynamic")
    if arm == "padd":
        env["KIC_RANK_SOLVE"] = "padd"
    command = [str(binary), f"construct:83:1:{k}", str(point_file), str(RANK_SEED),
               str(folder / "targets_result.jsonl")]
    print(f"starting {name} at {parent.now()}", flush=True)
    usage = holdout.run_with_telemetry(folder, command, env, IC_CAP_SECONDS, group)
    print(f"{name}: {usage['status']} after {usage['wall_seconds']:.3f}s", flush=True)


def run_rho(output, binary, i, point_file, draw, group):
    name = f"p{i}_rho{draw}"
    folder = output / "runs" / name
    seed = RHO_SEED_BASE + 10 * i + draw
    command = [str(binary), str(point_file), str(seed), str(cold.RHO_CONFIG),
               str(folder / "receipt.json")]
    usage = holdout.run_with_telemetry(folder, command,
                                       {"PATH": os.environ.get("PATH", "/usr/bin:/bin")},
                                       RHO_CAP_SECONDS, group)
    print(f"{name}: {usage['status']} after {usage['wall_seconds']:.3f}s", flush=True)


def run(binary, rho_binary, output, points, k):
    if output.exists():
        raise ValueError("output directory already exists")
    output.mkdir(parents=True)
    group = parent.cgroup_root()
    host = parent.host_receipt(group)
    host["schema"] = "n83.padd-rank-host/v1"
    host["topology"] = holdout.topology_receipt(host["visible_affinity"] or [])
    host["cpu_telemetry_before"] = holdout.cpu_telemetry()
    parent.write_json(output / "host.json", host)
    if (host["mem_total_kib"] is None
            or host["mem_total_kib"] * 1024 < cold.RSS_CAP_BYTES + 256 * 1024**2
            or len(host["visible_affinity"] or []) < 4):
        parent.write_json(output / "preflight.json", {"status": "REJECTED_HOST_CAPACITY"})
        return 2
    parent.write_json(output / "preflight.json", {"status": "PASS_CAPACITY_PREFLIGHT"})
    lines = TARGETS.read_text().splitlines()[:points]
    folder = output / "inputs"
    folder.mkdir()
    paths = {}
    for i, line in enumerate(lines, start=1):
        paths[i] = folder / f"point_{i}.jsonl"
        paths[i].write_text(line + "\n")
    sched = schedule(points)
    parent.write_json(output / "source.json", {
        "schema": "n83.padd-rank-source/v1", "k": k, "rank_seed": RANK_SEED,
        "binary_sha256": parent.digest(binary), "rho_binary_sha256": parent.digest(rho_binary),
        "runner_sha256": parent.digest(Path(__file__)),
        "targets_sha256": parent.digest(TARGETS), "schedule": sched,
        "rho_draws": RHO_DRAWS, "rho_seed_base": RHO_SEED_BASE,
        "point_input_sha256": {str(i): parent.digest(p) for i, p in paths.items()}})
    for row in sched:
        i = row["point_index"]
        if row["rho_first"]:
            for d in range(RHO_DRAWS):
                run_rho(output, rho_binary, i, paths[i], d, group)
        for arm in row["ic_order"]:
            run_ic(output, binary, i, paths[i], arm, k, group)
        if not row["rho_first"]:
            for d in range(RHO_DRAWS):
                run_rho(output, rho_binary, i, paths[i], d, group)
    return 0


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", type=Path, required=True)
    ap.add_argument("--rho-binary", type=Path, required=True)
    ap.add_argument("--output", type=Path, required=True)
    ap.add_argument("--points", type=int, default=20)
    ap.add_argument("--k", type=int, default=K)
    a = ap.parse_args()
    sys.exit(run(a.binary, a.rho_binary, a.output, a.points, a.k))
