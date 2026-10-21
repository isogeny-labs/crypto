#!/usr/bin/env python3
"""Replay the published N83 nearby-K controls and secondary probe screen."""

import gzip
import hashlib
import itertools
import json
import math
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_indexed5_20261013 import analyze as group
from research.n83_online_finalists_20261018.run import ic_environment
from research.n83_nearby_k_current_source_20261021 import derive_identities as identities

HERE = Path(__file__).resolve().parent
ARMS = (1104, 1107, 1120)
PRIOR = base.HERE / "k1107_current_source_base/base_dump.jsonl.gz"
PRIOR_SHA256 = "08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc"
PHASES = ("target_query_ms", "target_pdp_ms", "target_relation_check_ms",
          "target_descent_ms", "target_recovery_check_ms")


def check(condition, message):
    if not condition:
        raise ValueError(message)


def digest(blob):
    return hashlib.sha256(blob).hexdigest()


def read_json(path):
    return json.loads(path.read_text())


def read_rows(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def base_blob(k):
    path = PRIOR if k == 1107 else HERE / f"base_dump_k{k}.jsonl.gz"
    with gzip.open(path, "rb") as stream:
        return stream.read()


def replay_suffix(dump, prior, k):
    coords = [group.point_from_json(value) for value in
              dump["factor_base_point_coordinates"]]
    labels = dump["factor_base_point_labels"]
    reps = [group.point_from_json(value) for value in
            dump["factor_base_representatives"]]
    check(len(coords) == len(labels) == 166*k and len(reps) == k,
          "extended base dimensions differ")
    seen = {group.point_from_json(value) for value in
            prior["factor_base_point_coordinates"]}
    check(len(seen) == len(prior["factor_base_point_coordinates"]),
          "certified prefix contains duplicate points")
    for column in range(len(prior["factor_base_representatives"]), k):
        rep = reps[column]
        check(rep is not None and group.on_curve(rep) and
              group.multiply(rep, group.R) is None,
              f"representative {column} fails curve or subgroup check")
        current, coefficient = rep, 1
        for rotation in range(group.N):
            positive = 166*column + 2*rotation
            opposite = (current[0], current[0] ^ current[1])
            check(coords[positive] == current and
                  coords[positive+1] == opposite and
                  labels[positive] == [column, coefficient] and
                  labels[positive+1] ==
                  [column, (group.R-coefficient) % group.R],
                  f"orbit {column}/{rotation} differs")
            for point in (current, opposite):
                check(point not in seen, f"duplicate point in orbit {column}")
                seen.add(point)
            current = (group.product(current[0], current[0]),
                       group.product(current[1], current[1]))
            coefficient = coefficient*group.LAMBDA % group.R
        check(current == rep and coefficient == 1,
              f"orbit {column} does not close")
    check(len(seen) == len(coords), "extended base point count differs")


def verify_base(k, prior, prior_status):
    blob = base_blob(k)
    dump = json.loads(blob)
    check(prior_status == "PASS_INDEPENDENT_BASE_REPLAY",
          "prior K1107 base replay differs")
    check(digest(base_blob(1107)) == PRIOR_SHA256,
          "prior K1107 base digest differs")
    check(len(dump["factor_base_representatives"]) == k and
          len(dump["factor_base_point_coordinates"]) == 166*k and
          len(dump["factor_base_point_labels"]) == 166*k,
          f"K{k} base dimensions differ")
    for key in ("factor_base_representatives", "factor_base_point_coordinates",
                "factor_base_point_labels"):
        length = min(len(prior[key]), len(dump[key]))
        check(dump[key][:length] == prior[key][:length],
              f"K{k} certified prefix differs in {key}")
    if k > 1107:
        replay_suffix(dump, prior, k)
    return blob, dump


def verify_run(kind, k, blob, dump, answers):
    directory = HERE / kind / f"k{k}"
    launch, receipt = (read_json(directory / filename) for filename in
                       ("launch.json", "receipt.json"))
    summary = read_json(directory / "stdout.jsonl")
    rows = read_rows(directory / "targets_result.jsonl")
    input_name = "target.jsonl" if kind == "single" else "targets.jsonl"
    points = read_rows(directory / input_name)
    check(receipt["status"] == "PROCESS_EXITED" and receipt["exit_code"] == 0,
          f"{kind}/K{k} process did not complete")
    check(launch["source_sha256"] == base.SOURCE_SHA256 ==
          base.sha256(base.SOURCE), f"{kind}/K{k} source differs")
    custody = read_json(base.HERE / "source_binary_custody.json")
    check(launch["binary_sha256"] == custody["binary_sha256"],
          f"{kind}/K{k} binary custody differs")
    check(launch["k"] == k and launch["command"][1] ==
          f"construct:83:1:{k}", f"{kind}/K{k} command differs")
    expected_flags = {name: value for name, value in
                      ic_environment(Path(launch["flags"]["KIC_DUMP_BASE"]).parent,
                                     "root11_dynamic", "k1107_root11").items()
                      if name.startswith("KIC_")}
    check(launch["flags"] == expected_flags,
          f"{kind}/K{k} algorithm flags differ")
    check(launch["target_sha256"] == digest((directory / input_name).read_bytes()),
          f"{kind}/K{k} target input differs")
    for key, filename in (("stdout_sha256", "stdout.jsonl"),
                          ("stderr_sha256", "stderr.log"),
                          ("target_result_sha256", "targets_result.jsonl")):
        check(receipt[key] == digest((directory / filename).read_bytes()),
              f"{kind}/K{k} {filename} differs")
    check(receipt["base_dump_sha256"] == digest(blob) and
          summary["base_hash"] == dump["base_hash"],
          f"{kind}/K{k} base identity differs")
    check(summary["n"] == 83 and summary["a"] == 1 and
          summary["orbit_columns"] == summary["rank"] == k and
          summary["factor_base_points"] == 166*k and
          summary["targets"] == summary["targets_solved"] ==
          len(points) == len(rows) == len(answers) and
          summary["targets_failed"] == 0,
          f"{kind}/K{k} rank or target count differs")
    verified = []
    for index, (point_json, answer, row) in enumerate(zip(points, answers, rows)):
        q = group.point_from_json(point_json)
        scalar = int(row["recovered_scalar"])
        check(answer["fixture_index"] == row["fixture_index"] == index and
              row["target"] == point_json and
              scalar == int(answer["scalar"]) and
              group.multiply(group.G, scalar) == q and
              row["group_verified"] is True,
              f"{kind}/K{k} scalar replay differs at point {index}")
        indices = row["point_indices"]
        check(isinstance(indices, list) and len(indices) == 4 and
              all(isinstance(item, int) and not isinstance(item, bool) and
                  0 <= item < 166*k for item in indices),
              f"{kind}/K{k} relation indices differ at point {index}")
        relation = None
        for item in indices:
            relation = group.add(relation, group.point_from_json(
                dump["factor_base_point_coordinates"][item]))
        check(relation == q, f"{kind}/K{k} relation differs at point {index}")
        phases = [row[name] for name in PHASES]
        check(all(isinstance(value, (int, float)) and not isinstance(value, bool)
                  and math.isfinite(value) and value >= 0 for value in phases) and
              abs(math.fsum(phases) - row["target_ms"]) <= 1 and
              isinstance(row["probes"], int) and row["probes"] > 0,
              f"{kind}/K{k} phase or probe count differs at point {index}")
        verified.append({"fixture_index": index, "probes": row["probes"],
                         "online_ms": row["target_ms"]})
    timing = summary["timing_ms"]
    parts = [timing[name] for name in
             ("setup_total", "rank_stage", "targets_total")]
    check(math.isfinite(timing["process_total"]) and
          timing["process_total"] > 0 and
          all(math.isfinite(value) and value >= 0 for value in parts) and
          math.fsum(parts) <= timing["process_total"] + 1,
          f"{kind}/K{k} cold accounting differs")
    archived = read_json(directory / "independent_replay.json")
    check(archived["status"] in ("PASS_EXPLORATORY_ONE_TARGET_REPLAY",
                                 "PASS_SECONDARY_PROBE_REPLAY") and
          all(archived["checks"].values()) and
          archived["base_dump_sha256"] == digest(blob),
          f"{kind}/K{k} archived replay differs")
    return {"k": k, "kind": kind, "usable_points": 166*k,
            "folded_columns": k, "verified_targets": verified,
            "cold_process_ms": timing["process_total"],
            "base_dump_sha256": digest(blob)}


def sign_p(first_wins, second_wins):
    count = first_wins + second_wins
    if count == 0:
        return 1.0
    return min(1.0, 2*sum(math.comb(count, item) for item in
                          range(min(first_wins, second_wins)+1))/2**count)


def verify_selection(batch):
    selection = read_json(HERE / "batch_selection.json")
    geom = {k: math.exp(math.fsum(math.log(row["probes"]) for row in
                                    batch[k]["verified_targets"])/16)
            for k in ARMS}
    check(selection["target_count"] == 16 and
          selection["performance_selection_status"] == "BOUNDED_UNKNOWN",
          "batch selection scope differs")
    for k in ARMS:
        check(abs(selection["geometric_mean_probes"][str(k)]-geom[k]) < 1e-7,
              f"K{k} geometric probe mean differs")
    comparisons = []
    for first, second in itertools.combinations(ARMS, 2):
        a, b = (batch[k]["verified_targets"] for k in (first, second))
        wins_first = sum(x["probes"] < y["probes"] for x, y in zip(a, b))
        wins_second = sum(x["probes"] > y["probes"] for x, y in zip(a, b))
        match = next(row for row in selection["comparisons"] if
                     row["first_k"] == first and row["second_k"] == second)
        check(match["first_wins"] == wins_first and
              match["second_wins"] == wins_second and
              match["ties"] == 16-wins_first-wins_second and
              abs(match["two_sided_sign_p"] -
                  sign_p(wins_first, wins_second)) < 1e-14,
              f"K{first}/K{second} paired probe test differs")
        comparisons.append({"first_k": first, "second_k": second,
                            "first_wins": wins_first,
                            "second_wins": wins_second,
                            "two_sided_sign_p": sign_p(wins_first, wins_second)})
    check(selection["status"] == "BOUNDED_UNKNOWN" and
          selection["probe_leader_k"] is None and
          selection["descriptive_lowest_geometric_probes_k"] == 1104,
          "probe-screen status differs")
    return {"geometric_mean_probes": geom, "comparisons": comparisons}


def replay():
    prior_replay = read_json(base.HERE /
                             "k1107_current_source_base/independent_replay.json")
    check(prior_replay["base_dump_sha256"] == PRIOR_SHA256,
          "prior base receipt differs")
    prior = json.loads(base_blob(1107))
    answers_single = read_rows(base.ROOT /
        "research/n83_cold_factor_base_20261016/answers.jsonl")[:1]
    answers_batch = read_rows(base.ROOT /
        "research/n83_highk_beyond4405_20261021/screen_answers.jsonl")[:16]
    single, batch = {}, {}
    for k in ARMS:
        blob, dump = verify_base(k, prior, prior_replay["status"])
        single[k] = verify_run("single", k, blob, dump, answers_single)
        batch[k] = verify_run("batch", k, blob, dump, answers_batch)
    archived_single = read_json(HERE / "single_summary.json")
    check(archived_single["status"] == "PASS_THREE_REPLAYED_CONTROLS" and
          archived_single["performance_selection_status"] ==
          "BOUNDED_UNKNOWN" and
          archived_single["minimum_probes_on_this_point_k"] == 1120,
          "single-target summary differs")
    screen = verify_selection(batch)
    candidates, workloads, runs = identities.derive()
    identity_files = {
        **{f"candidate_{record['candidate_id']}.json": record
           for record in candidates.values()},
        **{f"workload_{record['workload_id']}.json": record
           for record in workloads.values()},
        "measurement_rows.json": runs,
    }
    for filename, record in identity_files.items():
        check(read_json(identities.IDENTITIES / filename) == record,
              f"{filename} identity or measurement differs")
    check(len(runs) == len({record["run_id"] for record in runs}) == 6,
          "run IDs collide")
    return {"schema": "n83.nearby-k-current-source-published-replay/v1",
            "status": "PASS_PUBLISHED_REPLAY",
            "performance_selection_status": "BOUNDED_UNKNOWN",
            "source_sha256": base.SOURCE_SHA256,
            "single": [single[k] for k in ARMS],
            "batch": [batch[k] for k in ARMS],
            "candidate_ids": {k: value["candidate_id"] for
                              k, value in candidates.items()},
            "workload_ids": {kind: value["workload_id"] for
                             kind, value in workloads.items()},
            "run_ids": [record["run_id"] for record in runs],
            "probe_screen": screen}


if __name__ == "__main__":
    try:
        result = replay()
        code = 0
    except (OSError, ValueError, KeyError, TypeError, IndexError) as error:
        result = {"schema": "n83.nearby-k-current-source-published-replay/v1",
                  "status": "VERIFICATION_FAILED", "error": str(error)}
        code = 2
    (HERE / "published_replay.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": result["status"],
                      "performance_selection_status":
                      result.get("performance_selection_status")}, sort_keys=True))
    raise SystemExit(code)
