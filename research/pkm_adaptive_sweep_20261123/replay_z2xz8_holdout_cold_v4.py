#!/usr/bin/env python3
"""Independently audit a source-pinned X1(2,8) cold-panel receipt prefix."""

import argparse
import hashlib
import json
from pathlib import Path

SOURCE = "62908d5299c04c0b9bda686eba7f19e271b09786"
BINARY_SHA = "de461514bcb5f6538781ee25785f11ac33940711a93c0b6cb6a5dfbe0a587002"
BUILD_SHA = "21b70ef0dc1799f545efd257b5d858ff8bda82cc65d6a9f4b87f378280cf82cf"
MANIFEST_SHA = "99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8"
PREFLIGHT_SHA = "4e849aebeb3da8047b7d69fa4c953483dd7d3bb6a7d45659f2971abd0a195cea"
RUNNER_SHA = "7471953f5a9c24fcbc79b052c4642661448cc68047ff1ea0acf762d180d68684"
PROTOCOL_SHA = "723ac30ff07386b4ad1769ce89d353229d8ba9636b5c0582813a3c7986378a3d"
AMENDMENTS = {
    "amendment_sha256": ("AMENDMENT_Z2XZ8_F4_PARITY_20261128.md",
                         "ebca9cd60c58f8e7b2fe0f6f43f941fe5c9acdcade4a9f81e4c26e8c47cdaa7a"),
    "interval_amendment_sha256": ("AMENDMENT_Z2XZ8_INTERVAL_ZERO_20261128.md",
                                  "efdca4d5ea496adf815943fcd7e5f0aa8f9b63463bd774724e3913cb1bdb98d4"),
    "rebuild_amendment_sha256": ("AMENDMENT_Z2XZ8_ORACLE_REBUILD_LEDGER_20261128.md",
                                 "ed5f4cb1cc45a2326ac48840e05c63ba2dbe72ada73d364a95f4fbd66ad6768d"),
    "resume_amendment_sha256": ("AMENDMENT_Z2XZ8_COLD_RESUME_HEADER_20261128.md",
                                "3f76cffcd7f4c146f7ea3db782facce835824eecaeb6afde362ce4268ace885a"),
}
ARMS = ("interval_a", "candidate_a", "candidate_b", "interval_b", "mitm", "kummer")
SELECTOR = "prime-isogeny:t=5,seed=0,aux=ordinary,family=z2xz8,selection=equal-size"
F4_M2 = "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"
F4_INTERVAL = "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"
F4_M3 = "pkm-semaev:m=3,mode=chain,engine=f4,descent=1,grid_crossover=0,trace=0,guess=0,degree=12,max_nnz=1000000"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_bytes())


def require(test, reason):
    if not test:
        raise ValueError(reason)


def point(curve, value):
    if value is None:
        return None
    return int(value["x"], 16), int(value["y"], 16)


def ec_add(left, right, a, p):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if left == right:
        slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    return x3, (slope * (x1 - x3) - y1) % p


def ec_mul(n, base, a, p):
    require(isinstance(n, int) and n >= 0, "invalid scalar")
    total = None
    while n:
        if n & 1:
            total = ec_add(total, base, a, p)
        base = ec_add(base, base, a, p)
        n >>= 1
    return total


def on_curve(q, a, b, p):
    return q is None or (0 <= q[0] < p and 0 <= q[1] < p
                         and (q[1] * q[1] - q[0] ** 3 - a * q[0] - b) % p == 0)


def field_roots(p):
    roots = {}
    for y in range(p):
        roots.setdefault(y * y % p, y)
    return roots


def prime(n):
    if n < 2:
        return False
    for divisor in range(2, int(n ** 0.5) + 1):
        if n % divisor == 0:
            return False
    return True


def group_census(a, b, p, roots):
    total = 1
    for x in range(p):
        rhs = (x ** 3 + a * x + b) % p
        if rhs in roots:
            total += 1 if rhs == 0 else 2
    return total


def insert_rank(row, basis, modulus):
    work = [v % modulus for v in row]
    for pivot in sorted(basis):
        scale = work[pivot]
        if scale:
            work = [(u - scale * v) % modulus for u, v in zip(work, basis[pivot])]
    pivot = next((j for j, v in enumerate(work) if v), None)
    if pivot is not None:
        inv = pow(work[pivot], -1, modulus)
        basis[pivot] = [(v * inv) % modulus for v in work]
    return len(basis)


def audit_rank(rank, curve, target, roots, case):
    p = int(curve["field"]["modulus"], 16)
    a, b = int(curve["a"], 16), int(curve["b"], 16)
    r = int(curve["subgroup_order"], 16)
    generator = point(curve, curve["generator"])
    keys = rank["column_keys"]
    require(rank["modulus"] == r and rank["transcript_complete"] is True,
            f"rank modulus/transcript: {case}")
    require(rank["generator_key"] == 2 * generator[0] + 2
            and rank["target_key"] == 2 * target[0] + 2,
            f"rank point keys: {case}")
    columns = []
    for key in keys:
        require(isinstance(key, int) and key >= 2 and key % 2 == 0,
                f"invalid column key: {case}")
        x = key // 2 - 1
        rhs = (x ** 3 + a * x + b) % p
        require(0 <= x < p and rhs in roots, f"invalid column abscissa: {case}")
        y = roots[rhs]
        columns.append((x, min(y, (-y) % p)))
    basis = {}
    base_basis = {}
    target_index = keys.index(rank["target_key"])
    for j, receipt in enumerate(rank["receipts"]):
        row = receipt["row"]
        require(len(row) == len(keys) and all(isinstance(v, int) for v in row)
                and isinstance(receipt["rhs"], int), f"rank row shape: {case}:{j}")
        before = len(basis)
        after = insert_rank(row, basis, r)
        base_after = insert_rank(row[:target_index] + row[target_index + 1:],
                                 base_basis, r)
        require((before, after, base_after) ==
                (receipt["rank_before"], receipt["rank_after"],
                 receipt["base_rank_after"]), f"rank step: {case}:{j}")
        lhs = None
        for coefficient, q in zip(row, columns):
            lhs = ec_add(lhs, ec_mul(coefficient % r, q, a, p), a, p)
        require(lhs == ec_mul(receipt["rhs"] % r, generator, a, p),
                f"group equation: {case}:{j}")
    require(len(basis) == rank["rank"] and len(base_basis) == rank["base_rank"],
            f"rank totals: {case}")
    return len(rank["receipts"])


def quiet(observation):
    return (isinstance(observation, dict)
            and observation.get("competing_processes") == []
            and len(observation.get("loadavg", [])) == 3
            and observation["loadavg"][0] <= observation.get("logical_cpus", 0))


def expected_command(binary, input_path, arm, seed):
    factor_base = ("prime-interval:t=5,lo=0" if arm.startswith("interval") else
                   "prime-kummer:t=5,g=1" if arm == "kummer" else SELECTOR)
    oracle = (F4_INTERVAL if arm.startswith("interval") else
              F4_M3 if arm == "kummer" else "mitm:m=2" if arm == "mitm" else F4_M2)
    return [str(binary), "bench", "--prime-workload", str(input_path),
            "--factor-base", factor_base, "--oracle", oracle, "--targets", "walk",
            "--linalg", "incremental-gauss-full-rank-checked", "--audit-rank",
            "--max-trials", "2000000" if arm == "kummer" else "20000",
            "--solver-budget-seconds", "5", "--rho-runs", "1",
            "--seed", str(seed), "--json"]


def raw_bytes(record, expected_path):
    path = Path(record["path"])
    require(path == expected_path and path.is_file(), f"raw path: {path}")
    data = path.read_bytes()
    require(len(data) == record["bytes"] and digest(data) == record["sha256"],
            f"raw hash: {path}")
    return data


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--store-dir", type=Path, required=True)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--build", type=Path, required=True)
    parser.add_argument("--preflight", type=Path, required=True)
    parser.add_argument("--runner", type=Path, required=True)
    parser.add_argument("--helper", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    here = Path(__file__).resolve().parent
    store = args.store_dir / "receipts.jsonl"
    require(digest(args.manifest.read_bytes()) == MANIFEST_SHA, "manifest hash")
    require(digest(args.bin.read_bytes()) == BINARY_SHA, "binary hash")
    require(digest(args.build.read_bytes()) == BUILD_SHA, "build hash")
    require(digest(args.preflight.read_bytes()) == PREFLIGHT_SHA, "preflight hash")
    require(digest(args.runner.read_bytes()) == RUNNER_SHA, "runner hash")
    require(digest((here / "PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md").read_bytes())
            == PROTOCOL_SHA, "protocol hash")
    for name, (filename, expected) in AMENDMENTS.items():
        require(digest((here / filename).read_bytes()) == expected,
                f"amendment hash: {name}")
    raw_store = store.read_bytes()
    records = [json.loads(line) for line in raw_store.splitlines()]
    require(records and records[0].get("id") == "header", "missing header")
    ids = [record.get("id") for record in records]
    require(all(isinstance(x, str) for x in ids) and len(set(ids)) == len(ids),
            "duplicate or invalid receipt ID")
    by_id = dict(zip(ids, records))
    header = records[0]
    expected_header = {
        "schema": "pkm-z2xz8-heldout-cold/v4", "source_commit": SOURCE,
        "binary_sha256": BINARY_SHA, "build_receipt_sha256": BUILD_SHA,
        "manifest_sha256": MANIFEST_SHA, "protocol_sha256": PROTOCOL_SHA,
        "preflight_sha256": PREFLIGHT_SHA, "runner_sha256": RUNNER_SHA,
        "helper_sha256": digest(args.helper.read_bytes()),
        "arms": list(ARMS), "cases": 576, "wall_cap_s": 120,
        "rss_cap_kib": 2 * 1024 * 1024,
        "interval_factor_base": "prime-interval:t=5,lo=0",
        **{key: value[1] for key, value in AMENDMENTS.items()},
    }
    for key, expected in expected_header.items():
        require(header.get(key) == expected, f"header field: {key}")
    manifest = read(args.manifest)
    roster_path = args.manifest.parent / "roster.json"
    require(digest(roster_path.read_bytes()) == manifest["roster_sha256"],
            "roster hash")
    roster = read(roster_path)
    require(len(manifest["entries"]) == 6, "curve roster")
    expected_cases = []
    cases = {}
    roots_by_curve = {}
    for entry in manifest["entries"]:
        name = f"b{entry['bits']}_c{entry['curve_index']}"
        workload_path = args.manifest.parent / entry["path"]
        require(digest(workload_path.read_bytes()) == entry["sha256"],
                f"workload hash: {name}")
        workload = read(workload_path)
        curve = workload["curve"]
        p = int(curve["field"]["modulus"], 16)
        a, b = int(curve["a"], 16), int(curve["b"], 16)
        r = int(curve["subgroup_order"], 16)
        generator = point(curve, curve["generator"])
        roots = roots_by_curve.setdefault(name, field_roots(p))
        group_order = int(curve["group_order"], 16)
        cofactor = int(curve["cofactor"], 16)
        require(p == entry["p"] and r == entry["r"] and prime(p) and prime(r)
                and group_order == cofactor * r
                and group_census(a, b, p, roots) == group_order
                and on_curve(generator, a, b, p)
                and generator is not None
                and ec_mul(r, generator, a, p) is None,
                f"curve identity: {name}")
        require(len(workload["targets"]) == 16, f"targets: {name}")
        roster_curve = next(c for c in roster["curves"]
                            if (c["bits"], c["curve_index"]) ==
                            (entry["bits"], entry["curve_index"]))
        for index, target_spec in enumerate(workload["targets"]):
            target = point(curve, target_spec["point"])
            scalar = target_spec["scalar"]
            require(on_curve(target, a, b, p)
                    and ec_mul(scalar % r, generator, a, p) == target,
                    f"target identity: {name}:q{index}")
            single = dict(workload)
            single["targets"] = [target_spec]
            single["sampling"] += f"; cold target_index={index}"
            input_path = args.store_dir / "inputs" / f"{name}_q{index}.json"
            expected_input = (json.dumps(single, sort_keys=True, indent=2) + "\n").encode()
            roster_target = roster_curve["targets"][index]
            require(roster_target["scalar"] == scalar
                    and roster_target["point"] == target_spec["point"],
                    f"roster target: {name}:q{index}")
            seed = int.from_bytes(
                bytes.fromhex(roster_target["relation_seed_sha256"])[:8], "little")
            for arm in ARMS:
                case = f"{name}_q{index}:{arm}"
                expected_cases.append(case)
                cases[case] = (entry, curve, target, scalar, roots,
                               input_path, expected_input, seed)
    active = 0
    attempts = {}
    launched = False
    for record in records[1:]:
        receipt_id = record["id"]
        if receipt_id == "verdict":
            require(active == len(expected_cases) and record is records[-1],
                    "premature or nonterminal verdict")
            continue
        require(active < len(expected_cases), "receipt after final case")
        case = expected_cases[active]
        if receipt_id.startswith("load:"):
            base, number = receipt_id.rsplit(":", 1)
            require(base == f"load:{case}" and number.isdecimal()
                    and int(number) == attempts.get(case, 0) + 1
                    and not launched, f"load order: {receipt_id}")
            attempts[case] = int(number)
            require(record.get("quiet") is quiet(record),
                    f"load classification: {receipt_id}")
        elif receipt_id == f"launch:{case}":
            require(attempts.get(case, 0) > 0 and not launched,
                    f"launch order: {receipt_id}")
            launched = True
        elif receipt_id == f"result:{case}":
            require(launched, f"result order: {receipt_id}")
            active += 1
            launched = False
        else:
            raise ValueError(f"unknown or out-of-order receipt: {receipt_id}")
    outcomes = {"parsed": 0, "censored": 0, "failed": 0}
    rank_receipts = 0
    scalar_successes = 0
    scalar_mismatches = 0
    rho_successes = 0
    rank_transcripts = 0
    completed_cases = []
    for case in expected_cases:
        loads = [record for record in records
                 if record["id"].startswith(f"load:{case}:")]
        launch = by_id.get(f"launch:{case}")
        result = by_id.get(f"result:{case}")
        if result is None:
            require(launch is None, f"unresolved launch: {case}")
            continue
        require(launch is not None and loads, f"missing launch/load: {case}")
        completed_cases.append(case)
        entry, curve, target, scalar, roots, input_path, expected_input, seed = cases[case]
        arm = case.split(":")[-1]
        require(input_path.read_bytes() == expected_input,
                f"one-target input: {case}")
        require(launch["id"] == f"launch:{case}" and launch["arm"] == arm
                and launch["target_scalar"] == scalar
                and launch["relation_seed"] == seed
                and launch["workload_sha256"] == digest(expected_input)
                and launch["command"] == expected_command(args.bin, input_path, arm, seed),
                f"launch identity: {case}")
        load = by_id.get(launch["load_id"])
        require(load in loads and quiet(load) and load["quiet"] is True,
                f"launch host: {case}")
        stdout_path = args.store_dir / "raw" / (case.replace(":", "_") + ".stdout.json")
        stderr_path = args.store_dir / "raw" / (case.replace(":", "_") + ".stderr.txt")
        require(launch["stdout_path"] == str(stdout_path)
                and launch["stderr_path"] == str(stderr_path),
                f"launch raw paths: {case}")
        stdout = raw_bytes(result["stdout"], stdout_path)
        raw_bytes(result["stderr"], stderr_path)
        require(result["wall_s"] > 0 and result["process_cpu_s"] >= 0
                and result["peak_rss_kib"] > 0
                and (result["peak_rss_kib"] <= header["rss_cap_kib"]
                     or result["cap_hit"]),
                f"process accounting: {case}")
        post_busy = not quiet(result["postcall_load"])
        require(result["postcall_busy"] == post_busy, f"post-call host: {case}")
        expected_outcome = ("censored" if result["cap_hit"]
                            or result["load_contamination"] or post_busy else
                            "parsed" if result["exit_code"] == 0
                            and result["summary"].get("identity_match")
                            and result["equal_size_match"] is not False else "failed")
        require(result["outcome"] == expected_outcome, f"outcome: {case}")
        outcomes[expected_outcome] += 1
        if expected_outcome != "parsed":
            continue
        report = json.loads(stdout)
        require(report["prime_workload_sha256"] == digest(input_path.read_bytes())
                and report["frozen_target_scalars"] == [scalar]
                and len(report["rows"]) == 1, f"report identity: {case}")
        row = report["rows"][0]
        p = int(curve["field"]["modulus"], 16)
        a, b = int(curve["a"], 16), int(curve["b"], 16)
        r = entry["r"]
        interval_size = sum(
            (x ** 3 + a * x + b) % p in roots for x in range(32))
        if arm == "kummer":
            require(result["equal_size_match"] is None,
                    f"kummer size classification: {case}")
        else:
            require(row["factor_base"]["abscissae"] == interval_size
                    and result["equal_size_match"] is True,
                    f"equal-size factor base: {case}")
        recovered = row.get("recovered")
        scalar_matches = False
        if recovered is not None:
            require(isinstance(recovered, int) and recovered >= 0,
                    f"recovered scalar type: {case}")
            scalar_matches = (
                ec_mul(recovered % r, point(curve, curve["generator"]), a, p) == target
                and recovered % r == scalar % r)
            if not scalar_matches:
                scalar_mismatches += 1
        require(row.get("verified") is not True or scalar_matches,
                f"claimed scalar verification differs: {case}")
        success = (report.get("status") == "complete"
                   and report.get("all_verified") is True
                   and row.get("verified") is True
                   and scalar_matches)
        if success:
            scalar_successes += 1
        rho = report.get("rho_reference") or {}
        require(len(rho.get("per_run", [])) == 1
                and rho["per_run"][0].get("planted") == scalar,
                f"matched rho target: {case}")
        if rho.get("all_verified") is True and rho["per_run"][0].get("verified") is True:
            rho_successes += 1
        rank = row.get("useful_rank")
        if rank and rank.get("transcript_complete"):
            rank_receipts += audit_rank(rank, curve, target, roots, case)
            rank_transcripts += 1
        require(not success or (rank and rank.get("transcript_complete") is True),
                f"successful row lacks rank transcript: {case}")
    result_cases = [record["id"].split(":", 1)[1] for record in records
                    if record["id"].startswith("result:")]
    require(result_cases == completed_cases
            and completed_cases == expected_cases[:len(completed_cases)],
            "result case order or gap")
    verdict = by_id.get("verdict")
    complete = len(completed_cases) == 576 and verdict is not None
    require(not verdict or complete, "premature verdict")
    output = {
        "schema": "pkm-z2xz8-cold-independent-replay/v1",
        "store": str(store), "store_sha256": digest(raw_store),
        "cases_recorded": len(completed_cases), "cases_expected": 576,
        "outcomes": outcomes, "rank_receipts_checked": rank_receipts,
        "rank_transcripts_checked": rank_transcripts,
        "scalar_successes_checked": scalar_successes,
        "scalar_mismatches": scalar_mismatches,
        "matched_rho_successes": rho_successes,
        "complete": complete, "claim_qualified": False,
        "reason": ("partial receipt prefix" if not complete else
                   "native work and complete-cost promotion require separate audit"),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("x") as file:
        json.dump(output, file, sort_keys=True, indent=2)
        file.write("\n")
    print(json.dumps(output, sort_keys=True))


if __name__ == "__main__":
    main()
