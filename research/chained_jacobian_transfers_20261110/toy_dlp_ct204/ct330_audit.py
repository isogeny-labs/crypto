#!/usr/bin/env python3
"""CT330 fail-closed *record-only* verifier for three native source-pinned pairs.

Does not independently rerun Rust or verify physical ECDLP runtime.
"""
import argparse
import json
import math
from pathlib import Path

PREFIX = "CT330_PAIRED_JSON "
EXPECTED_ORDER = ("RHO_BSGS", "BSGS_RHO", "RHO_BSGS")
KEYS = {
    "round", "order", "p", "subgroup_order", "target_x", "target_y",
    "rho_log", "bsgs_log", "rho_total_ns", "bsgs_total_ns",
    "rho_setup_ns", "rho_walk_ns", "rho_verify_ns", "rho_unclassified_ns",
    "rho_ec_adds", "rho_walk_steps", "rho_restarts",
    "rho_canonicalisations", "rho_negations", "rho_collisions",
    "rho_useless_collisions", "rho_table_entries", "rho_table_capacity",
    "bsgs_original_partial_adds", "bsgs_ct329_full_adds",
    "vmhwm_kb_processwide", "pcc",
}


def audit(text):
    # Refuse duplicate JSON keys, rather than trusting json's last-value semantics.
    def unique_pairs(pairs):
        output = {}
        for key, value in pairs:
            if key in output:
                raise ValueError("duplicate JSON key: " + key)
            output[key] = value
        return output

    lines = [line[len(PREFIX):] for line in text.splitlines()
             if line.startswith(PREFIX)]
    if len(lines) != 3 or text.count("CT330_THREE_PAIRED_Q73_COMPLETE") != 1:
        raise ValueError("expected exactly three pairs and exactly one terminal marker")
    data = [json.loads(line, object_pairs_hook=unique_pairs) for line in lines]
    ratios = []
    for i, row in enumerate(data):
        if set(row) != KEYS:
            raise ValueError("unexpected/missing record keys")
        fixed = {
            "round": i, "order": EXPECTED_ORDER[i], "p": 103,
            "subgroup_order": 109, "target_x": 42, "target_y": 71,
            "rho_log": 73, "bsgs_log": 73, "bsgs_original_partial_adds": 17,
            "bsgs_ct329_full_adds": 34, "pcc": "unverified",
        }
        for key, expected in fixed.items():
            if row[key] != expected or type(row[key]) is not type(expected):
                raise ValueError("mismatched fixture/control: " + key)
        ints = [k for k in KEYS if k not in fixed]
        if any(not isinstance(row[k], int) or isinstance(row[k], bool)
               or row[k] < 0 for k in ints):
            raise ValueError("invalid negative/nonintegral timing/counter")
        if min(row["rho_total_ns"], row["bsgs_total_ns"],
               row["rho_setup_ns"], row["rho_ec_adds"],
               row["rho_restarts"]) <= 0:
            raise ValueError("missing timed solve/operation")
        if not (1 <= row["rho_restarts"] <= 64):
            raise ValueError("rho restart bound")
        if not (row["rho_table_entries"] <= row["rho_table_capacity"]
                and row["rho_table_entries"] <= 64 * 4 * 109):
            raise ValueError("rho table bound")
        if (row["rho_setup_ns"] + row["rho_walk_ns"] +
            row["rho_verify_ns"] + row["rho_unclassified_ns"]
                != row["rho_total_ns"]):
            raise ValueError("rho phase-accounting mismatch")
        if row["rho_walk_steps"] > row["rho_restarts"] * 4 * 109:
            raise ValueError("rho walk-work cap mismatch")
        if row["rho_negations"] > row["rho_canonicalisations"]:
            raise ValueError("rho negation accounting")
        ratios.append(row["bsgs_total_ns"] / row["rho_total_ns"])

    median = sorted(ratios)[1]
    geometric = math.exp(sum(math.log(x) for x in ratios) / len(ratios))
    return data, ratios, median, geometric


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--native-log", required=True)
    args = ap.parse_args()
    data, ratios, median, geometric = audit(
        Path(args.native_log).read_text(encoding="utf-8"))
    for i, (row, ratio) in enumerate(zip(data, ratios)):
        print("CT330_PAIRED_AUDIT round={} order={} rho_ns={} bsgs_ns={} bsgs_over_rho={:.12f}".format(
            i, row["order"], row["rho_total_ns"], row["bsgs_total_ns"], ratio))
    print("CT330_AUDIT_PASS pairs=3 median_bsgs_over_rho={:.12f} geo_bsgs_over_rho={:.12f} pcc=unverified".format(
        median, geometric))


if __name__ == "__main__":
    main()
