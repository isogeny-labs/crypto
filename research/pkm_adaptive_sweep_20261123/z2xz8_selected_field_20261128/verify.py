#!/usr/bin/env python3
"""Replay the selected-tower field ledger against the earlier raw maps."""

import argparse
import hashlib
import json
import subprocess
from pathlib import Path

SOURCE = "a1c7d2699bd01690416274300da4b1ace3fa92e6"
BINARY = "2b06842604a3afc2f75931d25e5c6d472a6f8a822f7dd8478e7a5e6842ead025"
BUILD = "a2ea69ea4d96c3d5b83c8e93b8911ba115738e95aeb432db8f0e9357c12ad5ab"
DIAGNOSTIC = "cf9c571fca58ad7956d5ca24ab20762c1ac893d1b67d84ae848c5983ffd6da97"
DIAGNOSTIC_RAW = "2f98e4fe257096fd1b39b6ee53124b9e144e64bb138a9c4e72e6be8e980a30dd"
MAP_REPLAY = "c83a0006e6e048cf85b82ddd1d99cbd862c8078a861ddabc79bca5b1a84fdde4"
PREFLIGHT = "4e849aebeb3da8047b7d69fa4c953483dd7d3bb6a7d45659f2971abd0a195cea"
MANIFEST = "99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8"
OLD_DIAGNOSTIC_RAW = "1ce2716d94d1b099642edba38a2e2d517ee492c9325b31fb370fc896c6e7aa5c"
FIELD_KEYS = (
    "selection_curve_check_fp_muls",
    "selection_direct_fp_muls",
    "selection_pow_fp_muls",
    "selection_rhs_fp_muls",
    "velu_direct_fp_muls",
    "velu_pow_fp_muls",
)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_bytes())


def same_old_native(old, new):
    for key, value in old.items():
        if new.get(key) != value:
            raise ValueError(f"old native counter changed: {key}")


def check_field(native, p):
    for key in FIELD_KEYS:
        if native.get("aux_" + key + "_uncharged", 0) <= 0:
            raise ValueError(f"missing positive field counter: {key}")
    rhs = 3 * (32 + native["aux_selection_x_scanned"]
               + 32 * native["aux_selection_cosets_tested"])
    if native["aux_selection_rhs_fp_muls_uncharged"] != rhs:
        raise ValueError("curve RHS multiplication count differs")
    exponent = p - 2
    pow_muls = exponent.bit_length() + exponent.bit_count()
    expected_selection_pow = (2 + native["aux_curves_tried"]
                              - native["aux_curves_singular"]) * pow_muls
    if (native["aux_selection_pow_fp_muls_uncharged"] != expected_selection_pow
            or native["aux_velu_pow_fp_muls_uncharged"] != 9 * pow_muls
            or native["aux_velu_direct_fp_muls_uncharged"] != 61):
        raise ValueError("selector or Vélu arithmetic count differs")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--checkout", type=Path, required=True)
    ap.add_argument("--binary", type=Path, required=True)
    args = ap.parse_args()
    here = Path(__file__).resolve().parent
    root = here.parent
    checkout = args.checkout.resolve(strict=True)
    binary = args.binary.resolve(strict=True)
    build = here / "BUILD.json"
    diag = here / "DIAGNOSTIC.json"
    diag_raw = here / "DIAGNOSTIC_RAW.json"
    maps = here / "MAP_REPLAY.json"
    preflight = root / "z2xz8_preflight_archive"
    old_diag = root / "z2xz8_build_archive/DIAGNOSTIC_RAW.json"
    manifest = root / "z2xz8_holdout/manifest.json"
    for path, expected in ((build, BUILD), (diag, DIAGNOSTIC),
                           (diag_raw, DIAGNOSTIC_RAW), (maps, MAP_REPLAY),
                           (preflight / "receipts.jsonl", PREFLIGHT),
                           (manifest, MANIFEST),
                           (old_diag, OLD_DIAGNOSTIC_RAW), (binary, BINARY)):
        if sha(path) != expected:
            raise ValueError(f"pinned content changed: {path}")
    if (subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"],
                                text=True).strip() != SOURCE
            or subprocess.check_output(["git", "-C", str(checkout), "status",
                                        "--porcelain=v1", "--untracked-files=no"],
                                       text=True).strip()):
        raise ValueError("source checkout differs or has tracked edits")
    b = read(build)
    if (b["source_commit"] != SOURCE or b["binary_sha256"] != BINARY
            or b["prime_tower_sha256"] != sha(checkout / "src/cryptanalysis/ic_framework/prime_tower.rs")
            or b["pkm_semaev_sha256"] != sha(checkout / "src/cryptanalysis/ic_framework/pkm_semaev.rs")
            or not b["full_checkout"] or not b["checkout_clean"]
            or b["cargo_exit_code"] != 0):
        raise ValueError("build receipt differs")
    entries = read(manifest)["entries"]
    primes = {f"b{e['bits']}_c{e['curve_index']}": e["p"] for e in entries}
    if len(primes) != 6 or sorted(primes.values()) != [2081, 2113, 2273, 16417, 16481, 16673]:
        raise ValueError("held-out curve roster differs")
    old_rows = {r["id"]: r for r in (json.loads(s) for s in
                (preflight / "receipts.jsonl").read_text().splitlines())}
    replay = read(maps)
    if (replay["source_commit"] != SOURCE or replay["binary_sha256"] != BINARY
            or replay["build_receipt_sha256"] != BUILD
            or replay["prior_preflight_store_sha256"] != PREFLIGHT
            or len(replay["checks"]) != 6):
        raise ValueError("map replay receipt differs")
    seen = set()
    for check in replay["checks"]:
        key = check["case"]
        if key in seen or key not in primes:
            raise ValueError("missing or duplicate map case")
        seen.add(key)
        prior_launch = old_rows["launch:" + key]
        if check["command"] != [str(binary)] + prior_launch["command"][1:]:
            raise ValueError(f"map command differs: {key}")
        old_path = preflight / "raw" / (key + ".stdout.json")
        new_path = here / "map_raw" / (key + ".stdout.json")
        stderr = here / "map_raw" / (key + ".stderr.txt")
        if (sha(old_path) != old_rows["result:" + key]["stdout"]["sha256"]
                or sha(new_path) != check["stdout_sha256"]
                or sha(stderr) != check["stderr_sha256"]):
            raise ValueError(f"map raw hash differs: {key}")
        old, new = read(old_path), read(new_path)
        a, n = old["rows"][0]["factor_base"], new["rows"][0]["factor_base"]
        if (old["prime_workload_sha256"] != new["prime_workload_sha256"]
                or old["frozen_target_scalars"] != new["frozen_target_scalars"]
                or not (a["abscissae"] == n["abscissae"] == check["base_abscissae"])
                or not (a["cost"]["group_ops"] == n["cost"]["group_ops"]
                        == check["base_group_ops"])):
            raise ValueError(f"map identity or group work differs: {key}")
        same_old_native(a["cost"]["native"], n["cost"]["native"])
        check_field(n["cost"]["native"], primes[key])
        if check["field_counters"] != {
            "aux_" + k + "_uncharged": n["cost"]["native"]["aux_" + k + "_uncharged"]
            for k in FIELD_KEYS
        }:
            raise ValueError(f"map field receipt differs: {key}")
    if seen != set(primes):
        raise ValueError("six-map coverage differs")
    old, new = read(old_diag), read(diag_raw)
    a, n = old["rows"][0], new["rows"][0]
    if (old["prime_workload_sha256"] != new["prime_workload_sha256"]
            or old["frozen_target_scalars"] != new["frozen_target_scalars"]
            or a["factor_base"]["abscissae"] != n["factor_base"]["abscissae"]
            or a["factor_base"]["cost"]["group_ops"] != n["factor_base"]["cost"]["group_ops"]
            or a["decomposition"]["setup"]["group_ops"] != n["decomposition"]["setup"]["group_ops"]):
        raise ValueError("F4 diagnostic identity or group work differs")
    same_old_native(a["factor_base"]["cost"]["native"], n["factor_base"]["cost"]["native"])
    same_old_native(a["decomposition"]["setup"]["native"], n["decomposition"]["setup"]["native"])
    base_native = n["factor_base"]["cost"]["native"]
    oracle_native = n["decomposition"]["setup"]["native"]
    for key in FIELD_KEYS:
        if (base_native["aux_" + key + "_uncharged"]
                != oracle_native["pkm_oracle_aux_" + key + "_uncharged"]):
            raise ValueError(f"F4 oracle rebuild field work differs: {key}")
    if read(diag)["field_counters"] != {
        key: base_native["aux_" + key + "_uncharged"] for key in FIELD_KEYS
    }:
        raise ValueError("F4 diagnostic field receipt differs")
    print(json.dumps({"schema": "pkm-selected-field-replay/v1",
                      "source_commit": SOURCE, "binary_sha256": BINARY,
                      "maps_checked": len(seen), "f4_oracle_parity": True,
                      "map_receipt_sha256": MAP_REPLAY,
                      "diagnostic_receipt_sha256": DIAGNOSTIC}, sort_keys=True))


if __name__ == "__main__":
    main()
