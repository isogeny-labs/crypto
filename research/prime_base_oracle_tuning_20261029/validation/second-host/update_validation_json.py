#!/usr/bin/env python3
"""Record the validated second-host replay (VAL-20261029-7ff9db) in the study's
VALIDATION.json and in its benchmarks mirror. Run from the repository root:

  python3 -I research/prime_base_oracle_tuning_20261029/validation/second-host/update_validation_json.py

The gate-read fields (replay_certificate, validator_report, rho_reference)
are left as they were; their pinned files are unchanged. The section 13
caveat and the promotion blocker are replaced by what the second-host
validation established, with its own caveats, and the second-host
certificate and report are pinned by sha256.
"""
import hashlib
import json
from pathlib import Path

STUDY = Path("research/prime_base_oracle_tuning_20261029")
BENCH = Path("benchmarks/20261029-prime-fb-size-20bit")
SH = STUDY / "validation/second-host"

OLD_13 = ("PROTOCOL.md section 13 requires a second-host replay (ecbench-independent-runner) before a "
          "leaderboard promotion; none exists. This validation does not meet that requirement and does not "
          "authorize promotion.")
OLD_SAME_HOST = ("Replay is same-host and same-binary (env class ECBENV3hc4d998f5ae3f, same container class as "
                 "the measurement); not an independent-host or independent-implementation replication. The "
                 "validator's own [d]G = Q check and stdlib recompute are independent of ecbench code.")


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def new_caveats(prefix: str) -> list[str]:
    return [
        "PROTOCOL.md section 13 second-host replay: established by an independent validation "
        f"(VAL-20261029-7ff9db, {prefix}validation/second-host/replay-certificate.json). The frozen holdout-P20 "
        "spec was re-executed on isolab-eks (EC2 c8a.2xlarge, AMD EPYC 9R45, L2; session ECBS1h517504d65837, "
        "ecbench 583e62f0 built from cb2ec687) with only label, measurement.rounds 2 -> 5 and "
        "measurement.isolation_required L0 -> L2 changed. All 1,008 records of ECBS1h685271327a8a have a "
        "partner with the same arm, workload, round and algorithm seed, identical in every counted, band-priced "
        "and outcome field (validator's own comparison); priced S is identical at w = 0, 0.1 and 1 on every pair; "
        "the local binary bf5eec6d replays 1,680/1,680 second-host measured runs identically; [d]G = Q holds on "
        "2,016/2,016 second-host records under the validator's own arithmetic. The statements in "
        "replay-certificate.json and validator-report.json that no second-host replay exists predate this and "
        "are superseded by it.",
        "Form of the second-host replay: a new session from the frozen spec matched record by record to the "
        "original, plus a local `ecbench verify --replay-all` of the new session; not an "
        "`ecbench verify --replay-all` receipt of the original session's bytes produced on the second host. "
        "`ecbench claim build --independent-receipt` (vs_rho claims) would need that form. The second-host "
        "binary's source revision is attested by its image (build-info.json), not rebuilt by the validator.",
        "replay_certificate above is the same-host, same-binary replay (env class ECBENV3hc4d998f5ae3f); the "
        "second-host replay is a different host class and ecbench build. Neither is an independent "
        "implementation: both hosts ran ecbench. The validators' own [d]G = Q checks and stdlib recomputes are "
        "the implementation-independent checks.",
        "The second host's older build does not emit the bitop/v1 `work` block or phase `work_native`, so no "
        "bitop/v1 figure is covered by the second-host replay. Wall times differ by host and are not compared.",
        "The second-host session's rounds 3-5 (144 extra target-round pairs per arm on the same curve, seeds "
        "the original never ran) are post-freeze data. Over its 240 pairs sel/inc is 0.836 [0.806, 0.866], "
        "0.817 [0.786, 0.847] and 0.746 [0.714, 0.779] at w = 0, 0.1 and 1, consistent with the frozen 96-pair "
        "reading; the decision-bearing numbers remain the frozen holdout's (gae_band and paired_ratio_sums above).",
    ]


def update(path: Path, indent: int, prefix: str) -> None:
    raw = path.read_text()
    d = json.loads(raw)
    cav = d["caveats"]
    if OLD_13 not in cav or OLD_SAME_HOST not in cav:
        raise SystemExit(f"{path}: expected caveats not found; refusing to edit")
    keep = [c for c in cav if c not in (OLD_13, OLD_SAME_HOST)]
    d["caveats"] = new_caveats(prefix) + keep
    if d.get("promotion_blockers") != [OLD_13]:
        raise SystemExit(f"{path}: unexpected promotion_blockers; refusing to edit")
    d["promotion_blockers"] = []
    d["promotion_note"] = (
        "No evidence blocker known to the validators remains for row icv1-fp20-tm565-6d7d1d32: the section 13 "
        "second-host replay now exists and verifies. Promotion itself is the Coordinator's decision; this file "
        "records evidence admissibility, not authorization.")
    cert = SH / "replay-certificate.json"
    rep = SH / "validation_report.yaml"
    d["second_host_replay"] = {
        "status": "verified",
        "validation_id": "VAL-20261029-7ff9db",
        "replay_certificate": {"path": f"{prefix}validation/second-host/replay-certificate.json", "sha256": sha(cert)},
        "validation_report": {"path": f"{prefix}validation/second-host/validation_report.yaml", "sha256": sha(rep)},
        "receipts_sha256sums": {"path": f"{prefix}validation/second-host/SHA256SUMS",
                                "sha256": sha(SH / "SHA256SUMS")},
        "session": f"{prefix}second-host/eks-results/session (ECBS1h517504d65837)",
        "original_session": f"{prefix}sessions/holdout-P20 (ECBS1h685271327a8a)",
        "records_matched_identical": "1008/1008",
    }
    out = json.dumps(d, indent=indent, ensure_ascii=False) + "\n"
    path.write_text(out)


def main() -> None:
    update(STUDY / "VALIDATION.json", 1, "")
    update(BENCH / "VALIDATION.json", 2, "../../research/prime_base_oracle_tuning_20261029/")


if __name__ == "__main__":
    main()
