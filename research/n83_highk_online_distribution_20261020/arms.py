#!/usr/bin/env python3
"""Exact five-arm N83 online distribution comparison."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
ARMS = (
    ("k3072_root11", 3072, "root11_dynamic",
     "IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e"),
    ("k3500_root4", 3500, "root4_standard",
     "IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea"),
    ("k3300_wide3", 3300, "root11_wide3",
     "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586"),
    ("k4300_wide3", 4300, "root11_wide3_id30",
     "IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77"),
    ("k4405_wide3", 4405, "root11_wide3_id30",
     "IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7"),
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest(arm):
    name, k, layout, candidate_id = arm
    if layout in ("root11_dynamic", "root4_standard"):
        directory = ROOT / "research/n83_online_finalists_20261018"
    elif name == "k3300_wide3":
        directory = ROOT / "research/n83_final_extended_20261018"
    else:
        directory = (ROOT / "research/n83_root4_frontier_current_20261018"
                     / "highk_20261019/identities" / f"{name}_identity")
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
            record["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256):
        raise ValueError(f"{name}: exact candidate manifest differs")
    return path, record


def choose():
    hashes = {arm[3]: digest(manifest(arm)[0]) for arm in ARMS}
    return ARMS, {"schema": "n83.online-distribution-choice/v1",
                  "source_sha256": SOURCE_SHA256,
                  "ordered_candidate_ids": [arm[3] for arm in ARMS],
                  "candidate_manifest_sha256": hashes}
