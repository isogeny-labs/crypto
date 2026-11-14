"""Aggregate the L4/L5/L6 (strong) and L4-weak checks.  [pcc:unverified]"""
import json, glob, os, sys, collections
O = sys.argv[1]
agg = collections.OrderedDict()
for f in sorted(glob.glob(os.path.join(O, 'L4_*.jsonl'))):
    fam = os.path.basename(f)[3:-6]
    rows = [json.loads(l) for l in open(f) if l.strip()]
    hb = [r for r in rows if r.get('hbasis_claim', True)]
    nh = [r for r in rows if not r.get('hbasis_claim', True)]
    a = {"targets": len(rows),
         "hbasis_rows": len(hb),
         "exact_DXL_eq_pred": sum(1 for r in hb if r.get('D_XL') is not None and r['D_XL'] == r['pred']),
         "non_hbasis_rows": len(nh),
         "non_hbasis_pred_le_DXL": sum(1 for r in nh if r.get('pred_le_DXL')),
         "L5_checked": sum(1 for r in rows if 'L5_ok' in r), "L5_ok": sum(1 for r in rows if r.get('L5_ok')),
         "L6u_ok": sum(1 for r in rows if r.get('L6_unreduced_ok')), "L6r_ok": sum(1 for r in rows if r.get('L6_reduced_ok')),
         "degenerate_targets": sum(1 for r in rows if r['delta'] == -1),
         "D_range": [min(r['D_XL'] for r in rows if r.get('D_XL') is not None), max(r['D_XL'] for r in rows if r.get('D_XL') is not None)] if rows else None}
    agg[fam] = a
wagg = collections.OrderedDict()
for f in sorted(glob.glob(os.path.join(O, 'L4w_*.jsonl'))):
    fam = os.path.basename(f)[4:-6]
    rows = [json.loads(l) for l in open(f) if l.strip()]
    wagg[fam] = {"targets": len(rows),
                 "XL_eq_pred": sum(1 for r in rows if r['agree_XL_pred']),
                 "fast_eq_pred": sum(1 for r in rows if r['agree_fast']),
                 "pred_none(degenerate)": sum(1 for r in rows if r['D_weak_pred'] is None),
                 "D_weak_range": [min((r['D_weak_XL'] for r in rows if r['D_weak_XL']), default=None), max((r['D_weak_XL'] for r in rows if r['D_weak_XL']), default=None)]}
lit = [json.loads(l) for l in open(os.path.join(O, 'weak_d32_literal.jsonl')) if l.strip()]
out = {"strong_L4_L5_L6": agg, "weak_L4": wagg,
       "weak_literal_d16_d32": [{k: r[k] for k in ('d', 'fast_delta_weak', 'generic_weak', 'D_pred', 'XL_weak_at_Dpred_minus_1', 'XL_weak_at_Dpred')} for r in lit]}
print(json.dumps(out, indent=1))
json.dump(out, open(os.path.join(O, 'L4_summary.json'), 'w'), indent=1)
