#!/usr/bin/env sage -python
"""Independent Sage arithmetic and seed check for registered V2 inputs."""

import hashlib
import hmac
import json
import subprocess
from pathlib import Path

from sage.all import EllipticCurve, GF

ROOT = Path(__file__).resolve().parent
manifest_raw = (ROOT / "heldout_v2_seed_manifest.json").read_bytes()
public_raw = (ROOT / "heldout_v2_public_targets.json").read_bytes()
answers_raw = (ROOT / "heldout_v2_caller_answers.json").read_bytes()
manifest = json.loads(manifest_raw)
public = json.loads(public_raw)
answers = json.loads(answers_raw)

assert manifest["policy_id"] == "isogeny-labs/experiment-seeds/v1"
assert manifest["study"] == answers["study"] == "prime31-aps-rho-equal-core-v2"
assert hashlib.sha256(public_raw).hexdigest() == manifest["public_targets_sha256"]
assert hashlib.sha256(answers_raw).hexdigest() == manifest["caller_answers_sha256"]
assert len(public) == len(answers["records"]) == manifest["case_count"] == 48
assert manifest["replicate_ids"] == list(range(5))
assert len(manifest["streams"]) == 48 * 5
assert answers["selection_attempts"] == 48
assert answers["selection_rejections"] == []

curve = answers["curve"]
assert curve == {"p": 2147483647, "a": 847514846, "b": 2087444724,
                 "n": 2147446831, "g": [625037715, 2100627251], "cofactor": 1}
field = GF(curve["p"])
ec = EllipticCurve(field, [curve["a"], curve["b"]])
generator = ec(*curve["g"])
assert curve["n"] * generator == ec(0)

master = bytes.fromhex(manifest["master_seed_hex"])
assert len(master) == 32
domain = b"isogeny-labs/aps-equal-core/v2\x00"


def derive(phase, case, replicate=0, logical=0):
    name = phase.encode("ascii")
    message = (domain + len(name).to_bytes(2, "big") + name +
               case.to_bytes(4, "big") + replicate.to_bytes(4, "big") +
               logical.to_bytes(4, "big"))
    return hmac.digest(master, message, "sha256")


def seed64(phase, case, replicate=0, logical=0):
    return derive(phase, case, replicate, logical)[:8].hex()


dev_bytes = subprocess.check_output([
    "git", "show", "HEAD:research/prime31_aps_parallel_20261125/public_targets.json",
])
assert hashlib.sha256(dev_bytes).hexdigest() == manifest["development_targets_sha256"]
v1_bytes = subprocess.check_output([
    "git", "show", "HEAD:research/prime31_aps_parallel_20261125/heldout_public_targets.json",
])
forbidden = {tuple(q) for q in json.loads(dev_bytes) + json.loads(v1_bytes)}
assert len({tuple(q) for q in public}) == 48
assert not {tuple(q) for q in public} & forbidden

for case, (q, record) in enumerate(zip(public, answers["records"])):
    assert record["case"] == record["selection_counter"] == case
    assert record["public_Q"] == q
    digest = derive("target-scalar", case)
    assert record["target_digest_hex"] == digest.hex()
    scalar = 2 + int.from_bytes(digest, "big") % (curve["n"] - 2)
    assert scalar == record["caller_known_scalar_verifier_only"]
    assert scalar * generator == ec(*q)

arms = ("queued_aps", "folded_rho", "unfolded_rho")
seen = set()
for index, row in enumerate(manifest["streams"]):
    case, replicate = divmod(index, 5)
    assert (row["case"], row["replicate"]) == (case, replicate)
    aps = seed64("aps-source", case, replicate)
    rho = seed64("rho-walk", case, replicate)
    restart = [seed64("rho-lane-restart", case, replicate, lane)
               for lane in range(64)]
    schedule = seed64("arm-schedule", case, replicate)
    assert row["aps_source_seed_hex"] == aps
    assert row["rho_walk_seed_hex"] == rho
    assert row["rho_lane_restart_seed_hex"] == restart
    assert row["arm_schedule_seed_hex"] == schedule
    rotation = int(schedule, 16) % 3
    assert row["arm_order"] == list(arms[rotation:] + arms[:rotation])
    seeds = [aps, rho, schedule, *restart]
    assert len(seeds) == len(set(seeds))
    assert not seen.intersection(seeds)
    seen.update(seeds)
assert manifest["analysis_bootstrap_seed_hex"] == seed64("analysis-bootstrap", 0)
assert manifest["analysis_bootstrap_seed_hex"] not in seen

print(json.dumps({
    "standing": "independent V2 input and stream registration check",
    "cases": 48,
    "replicates": 5,
    "stream_records": len(manifest["streams"]),
    "checked_ec_scalar_multiplications": 48,
    "manifest_sha256": hashlib.sha256(manifest_raw).hexdigest(),
    "public_sha256": hashlib.sha256(public_raw).hexdigest(),
    "answers_sha256": hashlib.sha256(answers_raw).hexdigest(),
}, indent=2, sort_keys=True))
