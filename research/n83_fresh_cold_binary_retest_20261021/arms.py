#!/usr/bin/env python3
"""Resolve the extended holdout arms from the sealed six-arm evidence."""

import hashlib
import json
from pathlib import Path

from research.n83_online_finalists_20261018 import run as six_run
from research.n83_online_finalists_20261018 import select as six_select

HERE = Path(__file__).resolve().parent
SOURCE_SHA256 = six_run.SOURCE_SHA256
SMALL = (
    ("k992_root11", 992, "root11_dynamic",
     "IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0ha6a31a64b52b"),
    ("k1024_root11", 1024, "root11_dynamic",
     "IC1N83Ckb1a1fb169984PDP4rootRCguidedLAgaussTDdirectISO0h644bda8bfc36"),
)
MID = (
    ("k3300_wide3", 3300, "root11_wide3",
     "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586"),
)
ONLINE_FALLBACK = six_run.ARMS[1][3]  # K2304
COLD_FALLBACK = six_run.ARMS[0][3]    # K1107


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest(candidate_id):
    directory = (HERE if candidate_id in {arm[3] for arm in (*SMALL, *MID)}
                 else six_run.HERE)
    path = directory / f"candidate_{candidate_id}.json"
    record = json.loads(path.read_text())
    stripped = {key: value for key, value in record.items()
                if key != "candidate_id"}
    short = hashlib.sha256(json.dumps(stripped, sort_keys=True,
                                     separators=(",", ":"),
                                     ensure_ascii=False).encode()).hexdigest()[:12]
    if (record["candidate_id"] != candidate_id or
            not candidate_id.endswith("h" + short) or
            record["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256):
        raise ValueError(f"exact candidate manifest differs: {candidate_id}")
    return path, record


def choose(validated_path, selection_path):
    validated = json.loads(validated_path.read_text())
    selection = json.loads(selection_path.read_text())
    if (validated.get("schema") != "n83.online-finalists-validated/v1" or
            validated.get("source_sha256") != SOURCE_SHA256 or
            validated.get("targets_sha256") != six_run.TARGETS_SHA256 or
            validated.get("candidate_ids") != [arm[3] for arm in six_run.ARMS] or
            selection != six_select.select(validated)):
        raise ValueError("six-arm evidence or frozen selector differs")
    if (validated.get("evidence_status") != "PASS_INDEPENDENT_REPLAY" or
            validated.get("different_cloud_instances") is not True or
            validated.get("same_candidate_bases_across_strata") is not True):
        raise ValueError("six-arm provider and arithmetic evidence not admitted")
    online = selection["online"]["selected_candidate_id"]
    cold = selection["cold"]["selected_candidate_id"]
    if (online is not None and online not in six_select.IDS or
            cold is not None and cold not in six_select.IDS):
        raise ValueError("selected six-arm ID is absent")
    requested = (*[arm[3] for arm in (*SMALL, *MID)],
                 online or ONLINE_FALLBACK, cold or COLD_FALLBACK)
    by_id = {arm[3]: arm for arm in (*SMALL, *MID, *six_run.ARMS)}
    arms = tuple(by_id[item] for item in dict.fromkeys(requested))
    if len(arms) not in (4, 5):
        raise ValueError("extended holdout requires four or five unique arms")
    manifest_hashes = {arm[3]: digest(manifest(arm[3])[0]) for arm in arms}
    choice = {
        "schema": "n83.fresh-cold-binary-retest-choice/v1",
        "source_sha256": SOURCE_SHA256,
        "upstream_run_id": validated["run_id"],
        "upstream_head_sha": validated["head_sha"],
        "upstream_validated_sha256": digest(validated_path),
        "upstream_selection_sha256": digest(selection_path),
        "upstream_admission_status": selection["admission_status"],
        "online_fallback_used": online is None,
        "cold_fallback_used": cold is None,
        "ordered_candidate_ids": [arm[3] for arm in arms],
        "candidate_manifest_sha256": manifest_hashes,
    }
    return arms, choice
