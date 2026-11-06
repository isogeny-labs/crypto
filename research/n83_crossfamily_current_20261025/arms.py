#!/usr/bin/env python3
"""Resolve the four exact current-source N83 high-K arms."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
ARMS = (
    ("k3300_wide3", 3300, "root11_wide3",
     "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0hd2459867be83"),
    ("k3450_wide3", 3450, "root11_wide3",
     "IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0h622efa367d76"),
    ("k3500_root4", 3500, "root4_standard",
     "IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0h0de90f01951d"),
    ("k3596_root4", 3596, "root4_standard",
     "IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h2a3353097e81"),
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest(arm):
    name, k, layout, candidate_id = arm
    if name == "k3300_wide3":
        directory = ROOT / "research/n83_k3300_current_source_prefix_20261025"
    elif name in ("k3450_wide3", "k3500_root4"):
        directory = HERE
    else:
        directory = ROOT / "research/n83_k3596_current_source_full_20261025"
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
        "schema": "n83.crossfamily-current-choice/v1",
        "source_sha256": SOURCE_SHA256,
        "ordered_candidate_ids": [arm[3] for arm in ARMS],
        "candidate_manifest_sha256": hashes,
    }
