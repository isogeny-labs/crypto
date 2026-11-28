#!/usr/bin/env python3
"""Resumable quiet-host prices for the two selected-map modular multiply paths."""

import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import math
import platform
import subprocess
from pathlib import Path

SOURCE_COMMIT = "a1c7d2699bd01690416274300da4b1ace3fa92e6"
MANIFEST_SHA = "99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8"
PATHS = ("curve_word_mul", "residual_pow_mul")
REPS, ITERS = 7, 200_000
PROTOCOL = "PROTOCOL_Z2XZ8_SELECTED_PRIMITIVE_PRICE_20261128.md"
SOURCE = "fp_selected_primitives.rs"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load(path):
    return json.loads(path.read_bytes())


def prior_rows(path):
    rows = {}
    if path.exists():
        for line in path.read_text().splitlines():
            row = json.loads(line)
            if row["id"] in rows:
                raise ValueError("duplicate receipt id")
            rows[row["id"]] = row
    return rows


def complete(path, schema, labels):
    if not path.is_file():
        return False
    rows = prior_rows(path)
    header = rows.get("header", {})
    if header.get("schema") != schema or header.get("host") != platform.node():
        return False
    return all(any(key.startswith(f"result:{series}:{label}:a")
                   and row.get("qualifying") is True for key, row in rows.items())
               for series in ("forward", "reverse") for label in labels)


def verify_probe(out, path, p):
    if (not isinstance(out, dict) or out.get("schema") != "pkm-selected-primitive/v1"
            or out.get("path") != path or out.get("p") != p
            or out.get("reps") != REPS or out.get("iters") != ITERS
            or out.get("field_mults_per_rep") != 8 * ITERS):
        return False
    samples, checks = out.get("ns_per_mul_per_rep"), out.get("result_checks")
    if (not isinstance(samples, list) or len(samples) != REPS
            or not isinstance(checks, list) or len(checks) != REPS
            or any(not isinstance(x, (int, float)) or isinstance(x, bool)
                   or not math.isfinite(x) or x <= 0 for x in samples)
            or out.get("ns_per_mul_median") != sorted(samples)[REPS // 2]):
        return False
    return all(type(check) is int and
               check == (1 + ((i + 1) % (p - 1))) *
               pow(1 + ((i + 2) % (p - 1)), 8 * ITERS, p) % p
               for i, check in enumerate(checks))


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    for name in ("checkout", "manifest", "bin", "build-receipt", "store"):
        ap.add_argument("--" + name, required=True, type=Path)
    ap.add_argument("--development-store", type=Path,
                    default=Path("/Volumes/SSD990-2/pkm-fp-price-20261128/receipts.jsonl"))
    ap.add_argument("--heldout-store", type=Path,
                    default=Path("/Volumes/SSD990-2/pkm-z2xz8-heldout-fp-price-20261128/receipts.jsonl"))
    ap.add_argument("--series", choices=("forward", "reverse"), required=True)
    ap.add_argument("--max-new", type=int, default=0)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()
    if args.max_new < 0:
        ap.error("negative --max-new")
    here = Path(__file__).resolve().parent
    checkout = args.checkout.resolve(strict=True)
    manifest = args.manifest.resolve(strict=True)
    binary = args.bin.resolve(strict=True)
    build_path = args.build_receipt.resolve(strict=True)
    store = args.store.resolve()
    helper_path = checkout / "research/pkm_nonenum_decomp_20261125/run_cold_panel.py"
    spec = importlib.util.spec_from_file_location("pkm_cold_helpers", helper_path)
    helper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helper)
    helper.WALL_CAP_S = 30
    if (subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"],
                                text=True).strip() != SOURCE_COMMIT
            or subprocess.check_output(["git", "-C", str(checkout), "status",
                                        "--porcelain=v1", "--untracked-files=no"],
                                       text=True).strip()):
        ap.error("source checkout differs")
    if sha(manifest.read_bytes()) != MANIFEST_SHA:
        ap.error("manifest differs")
    entries = sorted(load(manifest)["entries"], key=lambda e: (e["bits"], e["curve_index"]))
    expected = [(11, 0, 2081), (11, 1, 2113), (11, 2, 2273),
                (14, 0, 16417), (14, 1, 16481), (14, 2, 16673)]
    if [(e["bits"], e["curve_index"], e["p"]) for e in entries] != expected:
        ap.error("roster differs")
    for e in entries:
        if sha((manifest.parent / e["path"]).read_bytes()) != e["sha256"]:
            ap.error("workload differs")
    build = load(build_path)
    if (build.get("schema") != "pkm-selected-primitive-build/v1"
            or build.get("host") != platform.node()
            or build.get("architecture") != platform.machine()
            or build.get("source_sha256") != sha((here / SOURCE).read_bytes())
            or build.get("binary_sha256") != sha(binary.read_bytes())
            or build.get("compile_command", [])[:5] !=
                ["rustc", "--edition", "2021", "-O", "-C"]
            or build.get("compile_exit_code") != 0):
        ap.error("standalone build differs")
    dev_labels = [f"t{t}_c{c}" for t in (4, 5) for c in range(3)]
    labels = [f"b{b}_c{c}" for b, c, _ in expected]
    dev_done = complete(args.development_store, "pkm-explicit-fp-price/v1", dev_labels)
    heldout_done = complete(args.heldout_store, "pkm-z2xz8-heldout-fp-price/v1", labels)
    cases = [(path, f"b{e['bits']}_c{e['curve_index']}", e["p"])
             for path in PATHS for e in entries]
    header = {"id": "header", "schema": "pkm-z2xz8-selected-primitive-price/v1",
              "source_commit": SOURCE_COMMIT, "manifest_sha256": MANIFEST_SHA,
              "development_store_sha256": sha(args.development_store.read_bytes()) if dev_done else None,
              "heldout_store_sha256": sha(args.heldout_store.read_bytes()) if heldout_done else None,
              "source_sha256": sha((here / SOURCE).read_bytes()),
              "binary_sha256": sha(binary.read_bytes()), "build_sha256": sha(build_path.read_bytes()),
              "protocol_sha256": sha((here / PROTOCOL).read_bytes()),
              "runner_sha256": sha(Path(__file__).read_bytes()),
              "helper_sha256": sha(helper_path.read_bytes()),
              "host": platform.node(), "architecture": platform.machine(),
              "reps": REPS, "iters": ITERS, "wall_cap_s": 30,
              "rss_cap_kib": helper.RSS_CAP_KIB,
              "created_utc": dt.datetime.now(dt.timezone.utc).isoformat()}
    if args.dry_run:
        print(json.dumps({"series": args.series, "cases": cases,
                          "development_complete": dev_done, "heldout_complete": heldout_done,
                          "header": header}, sort_keys=True))
        return
    if not dev_done or not heldout_done:
        print(json.dumps({"waiting_for_prior_price_series": True,
                          "development_complete": dev_done,
                          "heldout_complete": heldout_done}, sort_keys=True))
        return
    store.parent.mkdir(parents=True, exist_ok=True)
    with (store.parent / "receipts.lock").open("a+") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        prior = prior_rows(store)
        if not prior:
            helper.append(store, header)
            prior["header"] = header
        elif {k: v for k, v in prior["header"].items() if k != "created_utc"} != \
                {k: v for k, v in header.items() if k != "created_utc"}:
            raise ValueError("frozen header differs")
        if args.series == "reverse":
            for path, label, _ in cases:
                prefix = f"result:forward:{path}:{label}:a"
                if not any(k.startswith(prefix) and v.get("qualifying") for k, v in prior.items()):
                    raise ValueError(f"forward incomplete: {path}:{label}")
            cases.reverse()
        launched = 0
        for path, label, p in cases:
            case = f"{args.series}:{path}:{label}"
            if any(k.startswith(f"result:{case}:a") and v.get("qualifying")
                   for k, v in prior.items()):
                continue
            attempt = 1 + sum(k.startswith(f"pre:{case}:a") for k in prior)
            if attempt > 20:
                raise RuntimeError(f"attempt cap: {case}")
            for n in range(1, attempt):
                if f"launch:{case}:a{n}" in prior and f"result:{case}:a{n}" not in prior:
                    raise RuntimeError(f"unresolved launch: {case}:a{n}")
            if args.max_new and launched >= args.max_new:
                return
            pre = helper.quiet()
            pre["quiet"] = (not pre["competing_processes"]
                            and pre["loadavg"][0] <= pre["logical_cpus"])
            pre_id = f"pre:{case}:a{attempt}"
            helper.append(store, {"id": pre_id, **pre})
            prior[pre_id] = pre
            if not pre["quiet"]:
                print(json.dumps({"waiting_for_quiet_host": case, "pre_id": pre_id}))
                return
            command = [str(binary), path, str(p), str(REPS), str(ITERS)]
            raw = store.parent / "raw"
            raw.mkdir(parents=True, exist_ok=True)
            stem = case.replace(":", "_") + f"_a{attempt}"
            stdout, stderr = raw / (stem + ".stdout"), raw / (stem + ".stderr")
            launch_id = f"launch:{case}:a{attempt}"
            helper.append(store, {"id": launch_id, "command": command,
                                  "pre_id": pre_id, "p": p, "path": path,
                                  "stdout_path": str(stdout), "stderr_path": str(stderr)})
            prior[launch_id] = {"command": command}
            result = helper.run_one(command, stdout, stderr)
            post = helper.quiet()
            post["quiet"] = (not post["competing_processes"]
                             and post["loadavg"][0] <= post["logical_cpus"])
            result.update({"id": f"result:{case}:a{attempt}", "command": command,
                           "p": p, "path": path, "pre_id": pre_id, "postcall_load": post})
            try:
                parsed = load(stdout)
            except (OSError, ValueError):
                parsed = None
            result["parsed"] = parsed
            result["qualifying"] = bool(
                result["exit_code"] == 0 and not result["cap_hit"]
                and not result["load_contamination"] and post["quiet"]
                and verify_probe(parsed, path, p))
            helper.append(store, result)
            prior[result["id"]] = result
            launched += 1
            if not result["qualifying"]:
                raise RuntimeError(f"unqualified attempt retained: {case}")
    print(json.dumps({"series": args.series, "new_launches": launched, "store": str(store)}))


if __name__ == "__main__":
    main()
