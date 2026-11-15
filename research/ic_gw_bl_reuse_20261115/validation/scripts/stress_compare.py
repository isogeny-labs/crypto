#!/usr/bin/env python3
"""Compare the stress-harness outputs: REF la2 vs BLR la2 vs the oracle-audit la2 (BLR + direct grams).
usage: stress_compare.py out_ref.jsonl out_blr.jsonl out_aud.jsonl err_aud.txt > stress-summary.json"""
import json, sys, collections
ref = [json.loads(l) for l in open(sys.argv[1])]
blr = [json.loads(l) for l in open(sys.argv[2])]
aud = [json.loads(l) for l in open(sys.argv[3])]
audit = {}
cur = None
for l in open(sys.argv[4]):
    if l.startswith("TRIAL "):
        cur = int(l.split()[1])
    elif l.startswith("BLAUDIT "):
        audit[cur] = json.loads(l[8:])
assert len(ref) == len(blr) == len(aud)
S = collections.Counter()
by = collections.defaultdict(collections.Counter)
examples = []
for r, b, a in zip(ref, blr, aud):
    t = r["t"]
    S["trials"] += 1
    same_out = (r["res"], r["iters"], r["narrowed"]) == (b["res"], b["iters"], b["narrowed"])
    S["ref_blr_same_result_iters_narrowed"] += same_out
    S["aud_blr_identical_incl_counts"] += (a == b)
    au = audit.get(t)
    cls = "no_narrowing" if r["narrowed"] == 0 else "narrowed"
    if au is None:
        S["early_return_no_audit_line"] += 1
        cls = "early_return"
    else:
        if au["narrowed0"]:
            cls = "narrowed_at_iter0"
        elif au["narrow_at"]:
            last = au["iters"]
            cls = "narrowed_mid" if any(x < last - 1 for x in au["narrow_at"]) else "narrowed_last_two"
        S["id1_used"] += au["id1_used"]; S["id1_bad"] += au["id1_bad"]; S["id1_direct"] += au["id1_direct"]
        S["id2_used"] += au["id2_used"]; S["id2_bad"] += au["id2_bad"]; S["id2_direct"] += au["id2_direct"]
        S["calls_with_id1_bad"] += au["id1_bad"] > 0
        S["calls_with_id2_bad"] += au["id2_bad"] > 0
        rec = (r["lmul"] - b["lmul"] == au["skip_lmul"] - au["rec_lmul"]) and (r["ladd"] - b["ladd"] == au["skip_ladd"] - au["rec_ladd"])
        S["lmul_ladd_reconciled_exactly"] += rec
        S["audited_calls"] += 1
        by[cls]["id2_bad_calls"] += au["id2_bad"] > 0
        by[cls]["id1_bad_calls"] += au["id1_bad"] > 0
    by[cls]["trials"] += 1
    by[cls]["ref_blr_same"] += same_out
    by[cls]["ref_ok"] += r["res"] != "none"
    by[cls]["blr_ok"] += b["res"] != "none"
    by[(cls, "ell_small" if r["ell"] <= 101 else "ell_large")]["trials"] += 1
    by[(cls, "ell_small" if r["ell"] <= 101 else "ell_large")]["ref_blr_differ"] += not same_out
    if not same_out and len(examples) < 12:
        examples.append({"ref": r, "blr": b, "audit": au})
out = {"summary": dict(S), "by_class": {str(k): dict(v) for k, v in by.items()}, "first_divergent_examples": examples}
json.dump(out, sys.stdout, indent=1)
print()
