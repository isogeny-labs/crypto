#!/usr/bin/env python3
"""CT326: strict SOURCE-BOUND two-stage feasibility arithmetic on CT241.

This only evaluates previously frozen per-child records; it does not run a
native ECDLP solver, independently replay timing, or establish a speedup.
"""
import copy
import hashlib
import json
from pathlib import Path
from statistics import median

HERE = Path(__file__).resolve().parent
PANEL = HERE / "ct241_f103_native_15_pairs.jsonl"
ORIGINAL_GIT_BLOB = "61d8aa3f86ef5672faf09c9fc638e5fe9594f429"
BASES = (12, 16, 18, 20, 22)
PHASES = ("setup_ns", "relation_online_la_ns", "backsolve_ns",
          "recovery_ns", "verification_ns")


def need(condition, message):
    if not condition:
        raise ValueError(message)


def validate_rows(rows):
    need(type(rows) is list and len(rows) == 15, "expected 15 genuine rows")
    need({(r.get("base"), r.get("round")) for r in rows} ==
         {(b, i) for b in BASES for i in (0, 1, 2)},
         "missing, duplicate, or extra arm/round")
    for r in rows:
        b, i = r["base"], r["round"]
        need(r.get("order") ==
             ("IC_BSGS" if i != 1 else "BSGS_IC"),
             "changed preregistered arm ordering")
        need(r.get("pcc") == "unverified", "PCC standing mutated")
        need(r.get("memory") == "vmhwm_processwide", "memory scope mutated")
        for key in PHASES + ("candidate_ns", "direct_bsgs_ns",
                             "candidates", "target_candidates",
                             "rank", "lp_pivots", "determined_lp"):
            need(type(r.get(key)) is int and r[key] >= 0,
                 "missing or malformed " + key)
        c, ref = r["candidate_ns"], r["direct_bsgs_ns"]
        need(c > 0 and ref > 0, "zero baseline or candidate cost")
        need(sum(r[x] for x in PHASES) == c, "phase sum is not cold cost")
        need(r["rank"] == b and r["lp_pivots"] ==
             r["determined_lp"] == 22-b, "wrong rank/pivots")
        need(r["target_candidates"] == 27, "target candidate drift")
        need(r["candidates"] == (589 if b == 12 else 381),
             "source relation budget drift")
        need(abs(r.get("ratio_direct_over_candidate", -1) -
                 ref/c) < 2e-10, "paired ratio drift")
        need(0 < r["relation_online_la_ns"] < c,
             "nonpositive/nonfull relation phase")
    return True


def load_exact(raw):
    sha = hashlib.sha1(b"blob " + str(len(raw)).encode("ascii") +
                       b"\0" + raw).hexdigest()
    need(sha == ORIGINAL_GIT_BLOB, "source Git blob mismatch")
    try:
        rows = [json.loads(x) for x in raw.decode("utf-8").splitlines() if x]
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise ValueError("invalid source JSONL") from exc
    validate_rows(rows)
    return rows


def frontier(rows):
    """Necessary-only factors for T'=R/s_R + O/s_O < B.

    These gates are not jointly sufficient. Models assume the same fixed
    workloads/phase boundary. Redesigning the algorithm invalidates them.
    """
    result = []
    for b in BASES:
        panel = [r for r in rows if r["base"] == b]
        rel = [r["relation_online_la_ns"] / r["direct_bsgs_ns"]
               for r in panel]
        rest = [(r["candidate_ns"]-r["relation_online_la_ns"]) /
                r["direct_bsgs_ns"] for r in panel]
        residual_ns = [r["candidate_ns"]-r["relation_online_la_ns"]
                       for r in panel]
        need(min(rel) > 1 and min(rest) > 1,
             "single-phase-free negative gate no longer holds")
        result.append({
            "base": b,
            "paired_cold_rows": len(panel),
            "relation_only_necessary_factor_median": round(median(rel), 6),
            "nonrelation_only_necessary_factor_median": round(median(rest), 6),
            "relation_only_necessary_factor_min": round(min(rel), 6),
            "nonrelation_only_necessary_factor_min": round(min(rest), 6),
            "free_relation_residual_ns_median": int(median(residual_ns)),
            "single_phase_free_still_loses_all_rows": True,
        })
    return result


def mutation_controls(raw, rows):
    # Byte mutations must reject even if a modified JSONL remains parseable.
    changes = (raw[:-1], raw + b" ", raw.replace(b'"base":12',
               b'"base":13', 1), raw.replace(b'"pcc":"unverified"',
               b'"pcc":"verified"', 1))
    for changed in changes:
        need(changed != raw, "mutation was a no-op")
        try:
            load_exact(changed)
        except ValueError:
            pass
        else:
            raise ValueError("modified source bytes accepted")

    for name, mutate in (
        ("duplicate", lambda x: x.__setitem__(0, x[1])),
        ("phase", lambda x: x[0].__setitem__("relation_online_la_ns", 1)),
        ("baseline", lambda x: x[0].__setitem__("direct_bsgs_ns", 0)),
        ("rank", lambda x: x[0].__setitem__("rank", 11)),
        ("order", lambda x: x[0].__setitem__("order", "BSGS_IC")),
        ("pcc", lambda x: x[0].__setitem__("pcc", "verified")),
        ("missing", lambda x: x.pop()),
    ):
        altered = copy.deepcopy(rows)
        mutate(altered)
        try:
            validate_rows(altered)
        except ValueError:
            pass
        else:
            raise ValueError("semantic mutation accepted: " + name)
    return len(changes)+7


def main():
    raw = PANEL.read_bytes()
    rows = load_exact(raw)
    mutations = mutation_controls(raw, rows)
    output = {
        "study": "CT326_F103_PARETO_RECORD_ONLY",
        "pcc": "unverified",
        "original_git_blob": ORIGINAL_GIT_BLOB,
        "original_rows": len(rows),
        "mutation_rejections": mutations,
        "data": frontier(rows),
        "scope": "necessary factors for same-phase hypothetical costs; no timing replay",
    }
    print(json.dumps(output, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
