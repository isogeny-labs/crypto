#!/usr/bin/env python3
"""Independently replay source-shaped selected PKM primitive prices."""

import argparse
import hashlib
import json
import math
import os
import subprocess
from pathlib import Path

COMMIT = "a1c7d2699bd01690416274300da4b1ace3fa92e6"
MANIFEST_SHA = "99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8"
PRIMES = (("b11_c0", 2081), ("b11_c1", 2113), ("b11_c2", 2273),
          ("b14_c0", 16417), ("b14_c1", 16481), ("b14_c2", 16673))
PATHS = ("curve_word_mul", "residual_pow_mul")
REPS, ITERS = 7, 200_000


def sha(data):
    return hashlib.sha256(data).hexdigest()


def json_file(path):
    return json.loads(path.read_bytes())


def read_rows(path):
    rows, order = {}, []
    for line in path.read_text().splitlines():
        item = json.loads(line)
        key = item.get("id")
        if not isinstance(key, str) or key in rows:
            raise ValueError("duplicate or missing receipt id")
        rows[key] = item
        order.append(key)
    return rows, order


def quiet(row):
    if not isinstance(row, dict):
        return False
    loads, cpus = row.get("loadavg"), row.get("logical_cpus")
    return (row.get("competing_processes") == [] and isinstance(cpus, int)
            and cpus > 0 and isinstance(loads, list) and len(loads) == 3
            and isinstance(loads[0], (int, float)) and math.isfinite(loads[0])
            and loads[0] <= cpus)


def probe(data, path, p):
    if (not isinstance(data, dict) or data.get("schema") != "pkm-selected-primitive/v1"
            or data.get("path") != path or data.get("p") != p
            or data.get("reps") != REPS or data.get("iters") != ITERS
            or data.get("field_mults_per_rep") != 8 * ITERS):
        return False
    samples, checks = data.get("ns_per_mul_per_rep"), data.get("result_checks")
    if (not isinstance(samples, list) or len(samples) != REPS
            or not isinstance(checks, list) or len(checks) != REPS
            or any(type(x) not in (int, float) or not math.isfinite(x) or x <= 0
                   for x in samples)
            or data.get("ns_per_mul_median") != sorted(samples)[REPS // 2]):
        return False
    return all(type(x) is int and
               x == (1 + ((i + 1) % (p - 1))) *
               pow(1 + ((i + 2) % (p - 1)), 8 * ITERS, p) % p
               for i, x in enumerate(checks))


def raw(item):
    path = Path(item["path"])
    data = path.read_bytes()
    if len(data) != item["bytes"] or sha(data) != item["sha256"]:
        raise ValueError(f"raw content differs: {path}")
    return data


def prior_complete(path, schema, labels, host):
    rows, order = read_rows(path)
    if (not order or order[0] != "header"
            or rows["header"].get("schema") != schema
            or rows["header"].get("host") != host):
        return False
    for series in ("forward", "reverse"):
        for label in labels:
            if not any(key.startswith(f"result:{series}:{label}:a")
                       and row.get("qualifying") is True
                       for key, row in rows.items()):
                return False
    return True


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    for name in ("manifest", "checkout", "bin", "build-receipt", "store", "output"):
        ap.add_argument("--" + name, type=Path, required=True)
    ap.add_argument("--development-store", type=Path,
                    default=Path("/Volumes/SSD990-2/pkm-fp-price-20261128/receipts.jsonl"))
    ap.add_argument("--heldout-store", type=Path,
                    default=Path("/Volumes/SSD990-2/pkm-z2xz8-heldout-fp-price-20261128/receipts.jsonl"))
    args = ap.parse_args()
    here = Path(__file__).resolve().parent
    manifest, binary, build_path, store = (args.manifest.resolve(strict=True),
                                           args.bin.resolve(strict=True),
                                           args.build_receipt.resolve(strict=True),
                                           args.store.resolve(strict=True))
    source = here / "fp_selected_primitives.rs"
    protocol = here / "PROTOCOL_Z2XZ8_SELECTED_PRIMITIVE_PRICE_20261128.md"
    runner = here / "run_z2xz8_selected_primitive_price.py"
    helper = args.checkout.resolve(strict=True) / "research/pkm_nonenum_decomp_20261125/run_cold_panel.py"
    if (subprocess.check_output(["git", "-C", str(args.checkout), "rev-parse", "HEAD"],
                                text=True).strip() != COMMIT
            or subprocess.check_output(["git", "-C", str(args.checkout), "status",
                                        "--porcelain=v1", "--untracked-files=no"],
                                       text=True).strip()):
        raise ValueError("source checkout differs")
    if sha(manifest.read_bytes()) != MANIFEST_SHA:
        raise ValueError("manifest differs")
    entries = sorted(json_file(manifest)["entries"],
                     key=lambda e: (e["bits"], e["curve_index"]))
    if [(f"b{e['bits']}_c{e['curve_index']}", e["p"]) for e in entries] != list(PRIMES):
        raise ValueError("curve roster differs")
    for e in entries:
        if sha((manifest.parent / e["path"]).read_bytes()) != e["sha256"]:
            raise ValueError("workload differs")
    build = json_file(build_path)
    if (build.get("schema") != "pkm-selected-primitive-build/v1"
            or build.get("source_sha256") != sha(source.read_bytes())
            or build.get("binary_sha256") != sha(binary.read_bytes())
            or build.get("compile_command", [])[:5] !=
                ["rustc", "--edition", "2021", "-O", "-C"]
            or build.get("compile_exit_code") != 0):
        raise ValueError("build differs")
    rows, order = read_rows(store)
    if not order or order[0] != "header":
        raise ValueError("header first required")
    header = rows["header"]
    if not prior_complete(args.development_store, "pkm-explicit-fp-price/v1",
                          [f"t{t}_c{c}" for t in (4, 5) for c in range(3)],
                          header.get("host")):
        raise ValueError("development price series incomplete")
    if not prior_complete(args.heldout_store, "pkm-z2xz8-heldout-fp-price/v1",
                          [label for label, _ in PRIMES], header.get("host")):
        raise ValueError("heldout price series incomplete")
    if (header.get("schema") != "pkm-z2xz8-selected-primitive-price/v1"
            or header.get("source_commit") != COMMIT
            or header.get("manifest_sha256") != MANIFEST_SHA
            or header.get("source_sha256") != sha(source.read_bytes())
            or header.get("binary_sha256") != sha(binary.read_bytes())
            or header.get("build_sha256") != sha(build_path.read_bytes())
            or header.get("protocol_sha256") != sha(protocol.read_bytes())
            or header.get("runner_sha256") != sha(runner.read_bytes())
            or header.get("helper_sha256") != sha(helper.read_bytes())
            or header.get("development_store_sha256") != sha(args.development_store.read_bytes())
            or header.get("heldout_store_sha256") != sha(args.heldout_store.read_bytes())
            or header.get("host") != build.get("host")
            or header.get("architecture") != build.get("architecture")
            or header.get("reps") != REPS or header.get("iters") != ITERS
            or header.get("wall_cap_s") != 30 or header.get("rss_cap_kib") != 2 * 1024 * 1024):
        raise ValueError("header pin differs")
    expected_cases = [(series, path, label, p)
                      for series, cases in (("forward", [(path, label, p)
                                                         for path in PATHS for label, p in PRIMES]),
                                            ("reverse", list(reversed([(path, label, p)
                                                                       for path in PATHS for label, p in PRIMES]))))
                      for path, label, p in cases]
    cursor, selected, rejected = 1, {}, []
    for series, path, label, p in expected_cases:
        case = f"{series}:{path}:{label}"
        attempt, qualified = 1, None
        while cursor < len(order) and order[cursor] == f"pre:{case}:a{attempt}":
            if attempt > 20 or qualified is not None:
                raise ValueError(f"attempt cap/order: {case}")
            pre_id = order[cursor]
            pre = rows[pre_id]
            if pre.get("quiet") is not quiet(pre):
                raise ValueError(f"pre-call gate differs: {pre_id}")
            cursor += 1
            if quiet(pre):
                launch_id, result_id = (f"launch:{case}:a{attempt}",
                                        f"result:{case}:a{attempt}")
                if order[cursor:cursor + 2] != [launch_id, result_id]:
                    raise ValueError(f"unresolved or reordered launch: {case}")
                launch, result = rows[launch_id], rows[result_id]
                command = [str(binary), path, str(p), str(REPS), str(ITERS)]
                if (launch.get("command") != command or result.get("command") != command
                        or launch.get("pre_id") != pre_id or result.get("pre_id") != pre_id
                        or launch.get("stdout_path") != result.get("stdout", {}).get("path")
                        or launch.get("stderr_path") != result.get("stderr", {}).get("path")):
                    raise ValueError(f"launch identity differs: {case}")
                stdout = raw(result["stdout"])
                raw(result["stderr"])
                try:
                    data = json.loads(stdout)
                except ValueError:
                    data = None
                valid = (result.get("exit_code") == 0 and not result.get("cap_hit")
                         and not result.get("load_contamination")
                         and quiet(result.get("postcall_load"))
                         and result["postcall_load"].get("quiet") is True
                         and isinstance(result.get("wall_s"), (int, float))
                         and result["wall_s"] > 0
                         and isinstance(result.get("process_cpu_s"), (int, float))
                         and result["process_cpu_s"] >= 0
                         and isinstance(result.get("peak_rss_kib"), int)
                         and result["peak_rss_kib"] <= header["rss_cap_kib"]
                         and probe(data, path, p) and result.get("parsed") == data)
                if result.get("qualifying") is not bool(valid):
                    raise ValueError(f"qualification differs: {case}")
                if valid:
                    qualified = (result_id, data["ns_per_mul_median"])
                else:
                    rejected.append({"case": case, "attempt": attempt, "reason": "unqualified"})
                cursor += 2
            else:
                rejected.append({"case": case, "attempt": attempt, "reason": "busy_pre"})
            attempt += 1
        if qualified is None:
            if cursor != len(order):
                raise ValueError(f"later case before current qualified: {case}")
            print(json.dumps({"complete": False, "next_case": case,
                              "rejected_attempts": len(rejected)}, sort_keys=True))
            return
        selected[case] = qualified
    if cursor != len(order):
        raise ValueError("unrecognized trailing receipts")
    prices = {}
    for path in PATHS:
        for label, p in PRIMES:
            f = selected[f"forward:{path}:{label}"][1]
            r = selected[f"reverse:{path}:{label}"][1]
            spread = max(f, r) / min(f, r) - 1
            if spread > 0.10:
                raise ValueError(f"forward/reverse mismatch: {path}:{label}")
            prices[f"{path}:{label}"] = {"p": p, "ns_per_mul": (f + r) / 2,
                                           "forward": f, "reverse": r,
                                           "relative_spread": spread,
                                           "receipt_ids": [selected[f"forward:{path}:{label}"][0],
                                                           selected[f"reverse:{path}:{label}"][0]]}
    output = {"schema": "pkm-z2xz8-selected-primitive-selection/v1",
              "store_sha256": sha(store.read_bytes()), "manifest_sha256": MANIFEST_SHA,
              "source_sha256": sha(source.read_bytes()),
              "binary_sha256": sha(binary.read_bytes()), "prices": prices,
              "rejected": rejected}
    encoded = (json.dumps(output, sort_keys=True, indent=2) + "\n").encode()
    dest = args.output.resolve()
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists():
        if dest.read_bytes() != encoded:
            raise ValueError("frozen selection differs")
    else:
        fd = os.open(dest, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
        with os.fdopen(fd, "wb") as stream:
            stream.write(encoded)
            stream.flush()
            os.fsync(stream.fileno())
    print(json.dumps({"complete": True, "priced_classes": len(prices),
                      "rejected_attempts": len(rejected),
                      "store_sha256": output["store_sha256"]}, sort_keys=True))


if __name__ == "__main__":
    main()
