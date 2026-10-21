#!/usr/bin/env python3
"""Resolve the three exact current-source N83 root4 frontier arms."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
ARMS = (
    ("k3300_wide3", 3300, "root11_wide3",
     "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586"),
    ("k3500_root4", 3500, "root4_standard",
     "IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea"),
    ("k3596_root4", 3596, "root4_standard",
     "IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h3da4e8b31c3f"),
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest(arm):
    name, k, layout, candidate_id = arm
    if name == "k3300_wide3":
        directory = ROOT / "research/n83_final_extended_20261018"
    elif name == "k3500_root4":
        directory = ROOT / "research/n83_online_finalists_20261018"
    else:
        directory = HERE
    path = directory / f"candidate_{candidate_id}.json"
    record = json.loads(path.read_text())
    stripped = {key: value for key, value in record.items()
                if key != "candidate_id"}
    short = hashlib.sha256(json.dumps(stripped, sort_keys=True,
                                     separators=(",", ":"),
                                     ensure_ascii=False).encode()).hexdigest()[:12]
    if (record["candidate_id"] != candidate_id or
            not candidate_id.endswith("h" + short) or
            record["factor_base"]["actual_usable_points"] != 166 * k or
            record["factor_base"]["effective_columns"] != k or
            record["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256 or
            (layout == "root4_standard") !=
            (record["point_decomposition"]["root_slot"]["bytes"] == 4)):
        raise ValueError(f"{name}: exact candidate manifest differs")
    return path, record


def choose():
    hashes = {arm[3]: digest(manifest(arm)[0]) for arm in ARMS}
    return ARMS, {
        "schema": "n83.root4-frontier-choice/v1",
        "source_sha256": SOURCE_SHA256,
        "ordered_candidate_ids": [arm[3] for arm in ARMS],
        "candidate_manifest_sha256": hashes,
    }
