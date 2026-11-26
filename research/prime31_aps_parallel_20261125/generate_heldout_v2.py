#!/usr/bin/env python3
"""Freeze V2 public targets and every logical random stream before solving."""

import hashlib
import hmac
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent
P = 2_147_483_647
N = 2_147_446_831
A = 847_514_846
B = 2_087_444_724
G = (625_037_715, 2_100_627_251)
STUDY = "prime31-aps-rho-equal-core-v2"
MASTER_HEX = "8c2877bd7cd585c2f20ed182aaaae327ff576b3f313f3704c03ba75a0242b26c"
MASTER = bytes.fromhex(MASTER_HEX)
COUNT = 48
REPLICATES = list(range(5))
DOMAIN = b"isogeny-labs/aps-equal-core/v2\x00"
ARMS = ("queued_aps", "folded_rho", "unfolded_rho")


def on_curve(point):
    if point is None:
        return True
    x, y = point
    return 0 <= x < P and 0 <= y < P and (y * y - x * x * x - A * x - B) % P == 0


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    z, w = right
    if x == z and (y + w) % P == 0:
        return None
    if x == z:
        slope = ((3 * x * x + A) * pow(2 * y, P - 2, P)) % P
    else:
        slope = ((w - y) * pow((z - x) % P, P - 2, P)) % P
    u = (slope * slope - x - z) % P
    return u, (slope * (x - u) - y) % P


def multiply(k, point):
    result = None
    while k:
        if k & 1:
            result = add(result, point)
        point = add(point, point)
        k >>= 1
    return result


def derive(phase: str, case: int, replicate: int = 0, logical: int = 0) -> bytes:
    name = phase.encode("ascii")
    message = (DOMAIN + len(name).to_bytes(2, "big") + name +
               case.to_bytes(4, "big") + replicate.to_bytes(4, "big") +
               logical.to_bytes(4, "big"))
    return hmac.new(MASTER, message, hashlib.sha256).digest()


def seed64(phase: str, case: int, replicate: int = 0, logical: int = 0) -> str:
    return derive(phase, case, replicate, logical)[:8].hex()


def write_json(path: Path, value) -> str:
    raw = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode()
    path.write_bytes(raw)
    return hashlib.sha256(raw).hexdigest()


def main() -> None:
    assert len(MASTER) == 32 and on_curve(G) and multiply(N, G) is None
    development_bytes = subprocess.check_output([
        "git", "show", "HEAD:research/prime31_aps_parallel_20261125/public_targets.json",
    ])
    development = {tuple(q) for q in json.loads(development_bytes)}
    v1_path = ROOT / "heldout_public_targets.json"
    v1_bytes = (v1_path.read_bytes() if v1_path.exists() else
        subprocess.check_output([
            "git", "show",
            "HEAD:research/prime31_aps_parallel_20261125/heldout_public_targets.json",
        ]))
    v1 = {tuple(q) for q in json.loads(v1_bytes)}
    public = []
    answers = []
    rejected = []
    used = set()
    counter = 0
    while len(public) < COUNT:
        digest = derive("target-scalar", counter)
        scalar = 2 + int.from_bytes(digest, "big") % (N - 2)
        q = multiply(scalar, G)
        reason = ("duplicate_scalar" if scalar in used else
                  "development_Q" if q in development else
                  "superseded_V1_Q" if q in v1 else
                  "invalid_Q" if q is None or not on_curve(q) else None)
        if reason:
            rejected.append({"counter": counter, "digest_hex": digest.hex(),
                             "reason": reason})
        else:
            case = len(public)
            used.add(scalar)
            public.append(list(q))
            answers.append({"case": case, "selection_counter": counter,
                            "target_digest_hex": digest.hex(),
                            "caller_known_scalar_verifier_only": scalar,
                            "public_Q": list(q)})
        counter += 1
    streams = []
    stream_seeds = set()
    for case in range(COUNT):
        for replicate in REPLICATES:
            aps = seed64("aps-source", case, replicate)
            rho = seed64("rho-walk", case, replicate)
            restarts = [seed64("rho-lane-restart", case, replicate, lane)
                        for lane in range(64)]
            schedule = seed64("arm-schedule", case, replicate)
            all_seeds = [aps, rho, schedule, *restarts]
            assert len(all_seeds) == len(set(all_seeds))
            assert not any(seed in stream_seeds for seed in all_seeds)
            stream_seeds.update(all_seeds)
            rotation = int(schedule, 16) % len(ARMS)
            streams.append({
                "case": case,
                "replicate": replicate,
                "aps_source_seed_hex": aps,
                "rho_walk_seed_hex": rho,
                "rho_lane_restart_seed_hex": restarts,
                "arm_schedule_seed_hex": schedule,
                "arm_order": list(ARMS[rotation:] + ARMS[:rotation]),
            })
    analysis_seed = seed64("analysis-bootstrap", 0)
    assert analysis_seed not in stream_seeds
    public_sha = write_json(ROOT / "heldout_v2_public_targets.json", public)
    answers_sha = write_json(ROOT / "heldout_v2_caller_answers.json", {
        "standing": "caller-only public benchmark answers",
        "study": STUDY,
        "curve": {"p": P, "a": A, "b": B, "n": N, "g": list(G),
                  "cofactor": 1},
        "selection_attempts": counter,
        "selection_rejections": rejected,
        "records": answers,
    })
    write_json(ROOT / "heldout_v2_seed_manifest.json", {
        "policy_id": "isogeny-labs/experiment-seeds/v1",
        "study": STUDY,
        "schema_version": 2,
        "master_seed_hex": MASTER_HEX,
        "master_seed_selection": "openssl rand -hex 32, recorded before V2 solver outcomes",
        "kdf": "HMAC-SHA-256",
        "kdf_message": "ASCII domain isogeny-labs/aps-equal-core/v2 plus NUL; phase length u16 big-endian, phase ASCII, case u32 big-endian, replicate u32 big-endian, logical lane u32 big-endian",
        "seed64_encoding": "first eight digest bytes, big-endian u64, lowercase 16-digit hex",
        "target_scalar_law": "2 + full 256-bit target-scalar digest mod (n-2); reject duplicate scalar, development Q, V1 Q, invalid Q",
        "solver_prng": "native SplitMix64 with constants pinned in prime31_aps_one_relation.rs; rho jump and initial-lane source is shared; per-logical-lane restart state is explicit",
        "case_count": COUNT,
        "replicate_ids": REPLICATES,
        "arms": list(ARMS),
        "stream_relationships": "Public Q shared across arms and replicates. APS source independent of rho walk. Folded and unfolded rho share walk and lane-restart seeds. Scheduling seed is independent of solver streams.",
        "analysis_bootstrap_seed_hex": analysis_seed,
        "public_targets_sha256": public_sha,
        "caller_answers_sha256": answers_sha,
        "development_targets_sha256": hashlib.sha256(development_bytes).hexdigest(),
        "selection_attempts": counter,
        "selection_rejections": rejected,
        "streams": streams,
    })


if __name__ == "__main__":
    main()
