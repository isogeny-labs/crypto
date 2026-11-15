#!/usr/bin/env python3
"""Assemble a validation-gate replay certificate for research/prime_ksum_pooled_20261029
from the `ecbench verify --replay-all` receipts in $K/replay and an own [d]G = Q check.
Stdlib only. Usage: python3 -I make_replay_certificate.py
"""
import hashlib, json, glob, os, sys

K = "/tmp/claude-0/-home-user/89d98138-21ce-5c29-8de4-5fe8ac9e919c/scratchpad/ksum"
REPO = "/home/user/crypto"
STUDY = "research/prime_ksum_pooled_20261029"
SESSIONS = ["holdout-P20", "holdout-P24", "holdout-P28", "holdout-P32",
            "train-P20", "train-P24", "train-P28", "train-P32"]


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def ec_add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0] and (P[1] + Q[1]) % p == 0: return None
    if P == Q:
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)


def ec_mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = ec_add(R, P, a, p)
        P = ec_add(P, P, a, p); k >>= 1
    return R


def is_prime(n):
    if n < 2: return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for b in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(b, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else:
            return False
    return True


def main():
    bin_sha = open(f"{K}/replay-binary.sha256").read().split()[0]
    sessions, runs, failures, curves = [], [], [], {}
    ok_all = True
    for s in SESSIONS:
        rpath = f"{K}/replay/replay-{s}.audit.json"
        rc = json.load(open(rpath))
        log = open(f"{K}/replay/replay-{s}.verify.log").read().strip().splitlines()
        sess = json.load(open(f"{K}/sessions/{s}/session.json"))
        prod = f"{REPO}/{STUDY}/audits/{s}.json"
        reps = rc.get("replays") or []
        nrep = len(reps); nrepro = sum(1 for x in reps if x.get("reproduced") is True)
        files_now = {f: sha(f"{K}/sessions/{s}/{f}") for f in
                     ("session.json", "spec.json", "plan.json", "host.json", "records.jsonl")}
        files_repo = {f: sha(f"{REPO}/{STUDY}/sessions/{s}/{f}") for f in
                      ("session.json", "spec.json", "plan.json", "host.json")}
        same_as_repo = all(files_now[f] == files_repo[f] for f in files_repo)
        sok = (rc.get("ok") is True and not rc.get("problems") and nrep > 0 and nrep == nrepro
               and rc.get("session_id") == sess["session_id"] and same_as_repo
               and files_now["records.jsonl"] == sess["records_sha256"])
        ok_all &= sok
        sessions.append({
            "session_dir": f"sessions/{s}", "session_id": sess["session_id"],
            "receipt": f"replay-{s}.audit.json (scratchpad; to be copied to validation/)",
            "receipt_sha256": sha(rpath), "end_line": log[-1] if log else None,
            "records": rc.get("records"), "verified_records": rc.get("verified_records"),
            "replays": nrep, "replays_reproduced": nrepro,
            "replay_details": sorted({x.get("detail") for x in reps}),
            "problems": rc.get("problems"), "checks": rc.get("checks"), "ok": rc.get("ok"),
            "session_files_sha256": files_now, "session_json_files_equal_repo_6edb2cb": same_as_repo,
            "records_jsonl_source": f"git blob at a0ff77014f:{STUDY}/sessions/{s}/records.jsonl",
            "records_jsonl_equals_session_records_sha256": files_now["records.jsonl"] == sess["records_sha256"],
            "producer_receipt": f"audits/{s}.json" if os.path.exists(prod) else f"audits/{s}.txt",
            "producer_receipt_sha256": sha(prod) if os.path.exists(prod) else sha(f"{REPO}/{STUDY}/audits/{s}.txt"),
            "auditor_binary_sha256": rc.get("auditor_binary_sha256"),
            "auditor_env_class_id": rc.get("auditor_env_class_id"),
            "session_ok": sok,
        })
        if not s.startswith("holdout"):
            continue
        byseq = {x["seq"]: x for x in reps}
        for line in open(f"{K}/sessions/{s}/records.jsonl"):
            r = json.loads(line)
            if r["warmup"]:
                continue
            c = r["workload"]["curve"]
            p = int(c["icv1"].split(":")[1].split("-")[1])
            G = tuple(int(v, 16) for v in c["generator"])
            Q = tuple(int(v, 16) for v in r["workload"]["target"])
            if c["slug"] not in curves:
                a = ((G[1] ** 2 - G[0] ** 3) - (Q[1] ** 2 - Q[0] ** 3)) * pow(G[0] - Q[0], -1, p) % p
                b = (G[1] ** 2 - G[0] ** 3 - a * G[0]) % p
                curves[c["slug"]] = {"p": p, "a": a, "b": b, "r": c["r"], "p_prime": is_prime(p),
                                     "r_prime": is_prime(c["r"]), "cofactor": c["cofactor"],
                                     "j_not_0_or_1728": a != 0 and b != 0,
                                     "rG_is_identity": ec_mul(c["r"], G, a, p) is None}
            cv = curves[c["slug"]]
            onc = (Q[1] ** 2 - Q[0] ** 3 - cv["a"] * Q[0] - cv["b"]) % p == 0
            d = r["outcome"].get("recovered")
            dg = d is not None and onc and ec_mul(int(d) % c["r"], G, cv["a"], p) == Q
            rep = byseq.get(r["seq"])
            if not dg or rep is None or rep.get("reproduced") is not True:
                failures.append({"session": s, "seq": r["seq"], "dG": dg, "replayed": rep is not None})
            runs.append({"session": s, "seq": r["seq"], "record_id": r["record_id"], "arm": r["arm"],
                         "method": r["method"].get("method") if isinstance(r["method"], dict) else r["method"],
                         "curve": c["slug"], "target_index": r["workload"]["target_index"], "round": r["round"],
                         "status": r["outcome"]["status"], "recovered": d,
                         "validator_dG_equals_Q": dg,
                         "reproduced": None if rep is None else rep.get("reproduced"),
                         "detail": None if rep is None else rep.get("detail")})
    curves_ok = all(v["p_prime"] and v["r_prime"] and v["j_not_0_or_1728"] and v["rG_is_identity"]
                    for v in curves.values())
    scal = not failures and curves_ok and len(runs) == 1344 + 3 * 1536
    status = "verified" if ok_all and scal else "failed"
    cert = {
        "schema": "replay-certificate/v1",
        "evidence_dir": STUDY,
        "status": status,
        "independent": True,
        "scalars_verified": scal,
        "independence": {
            "replayer": "planning subagent of a Claude Code session (claude-opus-5-5) that did not produce the study's sessions, code, analysis or README; producer receipts untouched",
            "same_binary_as_measurement": False,
            "same_host_class_as_measurement": False,
            "independent_implementation": False,
            "independent_scalar_check": True,
            "statement": "Replay of every measured deterministic run of all 8 sessions with an ecbench built from a clean git archive of 6edb2cb36d on Linux x86_64, against records recorded by ecbench-v2 (fc86ee11, macOS aarch64, 26249b4 + #2852 diff). Operation counts are host-independent; this is a different-build, different-host-class replay, not an independent implementation. Every holdout scalar re-verified [d]G = Q with this script's own affine arithmetic. The `independent: true` boolean is the writer's; the sabotage contract's signed verdict is not part of this file.",
        },
        "binary": {"path": f"{K}/target/release/ecbench", "sha256": bin_sha,
                   "source": "git archive 6edb2cb36d (isogeny-labs/crypto main, 2026-11-11), cargo build --release --locked",
                   "equals_session_binary_sha256": False,
                   "session_binary_sha256": "fc86ee11178da3bc373abecfaa1de532939637ab92fb86ecadf92e9ee1f9c195"},
        "command": "RAYON_NUM_THREADS=1 <binary> verify --dir <scratchpad>/sessions/<session> --replay-all --exit-code --out <scratchpad>/replay/replay-<session>.audit.json",
        "independent_scalar_check": {"method": "make_replay_certificate.py: a, b derived from G and a target; [d]G recomputed for every measured holdout record",
                                     "curves": curves, "measured_records": len(runs), "failures": failures},
        "sessions": sessions,
        "runs": runs,
    }
    out = f"{K}/replay-certificate.json"
    json.dump(cert, open(out, "w"), indent=1)
    print(status, "scalars", scal, "runs", len(runs), "failures", len(failures), "sha256", sha(out))
    for x in sessions:
        print(x["session_dir"], x["records"], x["verified_records"], x["replays"], x["replays_reproduced"],
              x["replay_details"], "problems", len(x["problems"] or []), "ok", x["session_ok"], x["end_line"])


if __name__ == "__main__":
    main()
