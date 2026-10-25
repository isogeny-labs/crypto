"""Frozen execution rules for two independent eight-point N83 strata."""

import hashlib
import json
from pathlib import Path

from research.n83_indexed5_20261013 import analyze as group
from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_nearby_grid_confirm16_20261024 import generate_fixture

HERE = Path(__file__).resolve().parent
POINTS_PER_STRATUM = 8
STRATA = 2
IC_CAP_SECONDS = screen.IC_CAP_SECONDS
RHO_CAP_SECONDS = screen.RHO_CAP_SECONDS
GLOBAL_CAP_SECONDS = 240 * 60
RSS_CAP_BYTES = screen.RSS_CAP_BYTES
MIN_MEMORY_BYTES = screen.MIN_MEMORY_BYTES
MIN_DISK_BYTES = screen.MIN_DISK_BYTES
AA_SPREAD_LIMIT = 0.20
STEAL_FRACTION_CAP = 0.005
RHO_SEED_BASE = 2026122500
PAIRWISE_ALPHA = "0.05/all_unordered_selected_pairs"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fixture():
    manifest = json.loads((HERE / "fixture_manifest.json").read_text())
    if (manifest.get("schema") != "n83.nearby-grid-confirm16-fixture/v1" or
            manifest.get("status") != "SEALED_UNMEASURED" or
            manifest.get("point_count") != STRATA * POINTS_PER_STRATUM or
            manifest.get("curve_id") != generate_fixture.CURVE_ID or
            manifest.get("subgroup_order") != group.R or
            manifest.get("domain") != generate_fixture.DOMAIN.decode() or
            manifest.get("generator_sha256") != digest(HERE / "generate_fixture.py") or
            manifest.get("screen_workload_id") != screen.WORKLOAD_ID or
            manifest.get("screen_catalog_sha256") != screen.CATALOG_SHA256 or
            manifest.get("screen_protocol_sha256") != digest(Path(screen.__file__))):
        raise ValueError("sealed confirmation manifest differs")
    if set(manifest.get("artifacts_sha256", {})) != {
            "targets.jsonl", "answers.jsonl", "workloads.jsonl",
            "exclusion_audit.json"}:
        raise ValueError("sealed confirmation artifact list differs")
    for name, expected in manifest["artifacts_sha256"].items():
        if digest(HERE / name) != expected:
            raise ValueError(f"sealed confirmation {name} differs")
    targets = (HERE / "targets.jsonl").read_text().splitlines(keepends=True)
    answers = [json.loads(line) for line in
               (HERE / "answers.jsonl").read_text().splitlines()]
    workloads = [json.loads(line) for line in
                 (HERE / "workloads.jsonl").read_text().splitlines()]
    if (len(targets) != 16 or len(answers) != 16 or len(workloads) != 16 or
            manifest["workload_ids"] != [w["workload_id"] for w in workloads]):
        raise ValueError("confirmation point or workload count differs")
    points = []
    for index, (line, answer, workload) in enumerate(zip(targets, answers, workloads)):
        if not line.endswith("\n"):
            raise ValueError("noncanonical target line")
        encoded = json.loads(line)
        point = group.point_from_json(encoded)
        seed = hashlib.sha256(generate_fixture.DOMAIN +
                              index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        canonical = {key: value for key, value in workload.items()
                     if key != "workload_id"}
        workload_id = hashlib.sha256(json.dumps(
            canonical, sort_keys=True, separators=(",", ":"),
            ensure_ascii=False).encode()).hexdigest()[:12]
        if (answer != {"fixture_index": index, "scalar": str(scalar),
                       "seed_sha256": seed.hex()} or
                workload.get("workload_id") != workload_id or
                workload.get("point_index") != index or
                workload.get("target_count") != 1 or
                workload.get("kind") != "primary_single_target" or
                workload.get("target_point") != encoded or
                workload.get("target_stream_sha256") !=
                hashlib.sha256(line.encode()).hexdigest() or
                workload.get("target_generation_seed_sha256") != seed.hex() or
                workload.get("target_domain") != generate_fixture.DOMAIN.decode() or
                workload.get("curve_id") != generate_fixture.CURVE_ID or
                group.multiply(group.G, scalar) != point):
            raise ValueError(f"confirmation point {index} fails scalar replay")
        points.append((line, point, scalar, workload_id))
    if len({point for _, point, _, _ in points}) != 16:
        raise ValueError("confirmation points repeat")
    audit = json.loads((HERE / "exclusion_audit.json").read_text())
    if (audit.get("status") != "PASS_ZERO_OVERLAP" or
            audit.get("overlap_count") != 0 or
            audit.get("duplicate_count") != 0 or
            audit.get("new_point_count") != 16):
        raise ValueError("confirmation exclusion audit differs")
    return manifest, points


def events(stratum, selected_ks):
    if (stratum not in range(STRATA) or
            not isinstance(selected_ks, list) or
            selected_ks != sorted(set(selected_ks)) or
            not 3 <= len(selected_ks) <= 7 or
            screen.BASELINE_K not in selected_ks or
            any(type(k) is not int or k not in screen.CANDIDATES
                for k in selected_ks)):
        raise ValueError("confirmation stratum or selected candidates differ")
    rows = []
    for index in range(stratum * POINTS_PER_STRATUM,
                       (stratum + 1) * POINTS_PER_STRATUM):
        ordered = selected_ks[index % len(selected_ks):] + \
                  selected_ks[:index % len(selected_ks)]
        rows.append({"point_index": index, "name": f"p{index:02d}_aa_before",
                     "kind": "aa", "k": screen.BASELINE_K,
                     "candidate_id": screen.CANDIDATES[screen.BASELINE_K][0]})
        for k in ordered:
            fixed_index = selected_ks.index(k)
            seed = RHO_SEED_BASE + index * len(selected_ks) + fixed_index
            order = ("ic", "rho") if (index + fixed_index) % 2 == 0 else ("rho", "ic")
            for kind in order:
                rows.append({"point_index": index, "name":
                             f"p{index:02d}_k{k}_{kind}", "kind": kind,
                             "k": k, "candidate_id": screen.CANDIDATES[k][0],
                             "seed": seed})
        rows.append({"point_index": index,
                     "name": f"p{index:02d}_aa_after", "kind": "aa",
                     "k": screen.BASELINE_K,
                     "candidate_id": screen.CANDIDATES[screen.BASELINE_K][0]})
    return rows
