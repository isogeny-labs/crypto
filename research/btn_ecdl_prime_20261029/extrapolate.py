"""Step 3: extrapolate the measured cost of BTN index calculus over F_p to the
131-bit ECCp-131 size.

Inputs (all measured in this directory):
  widths.json      min-fill contraction width of the relation network per bit size d
  runs/ic_*.json   end-to-end solves: counts, seconds per count, relations
  first_count.json wall time of one unpinned contraction per d (the dominant count)

Model (stated, not derived): a relation costs (4d+1) contractions (Lemma 1 over
the 4d interface bits plus the total); a contraction costs at most
c * 2^width(d) table-entry operations, c calibrated on the largest measured d.
Width(d) is fitted linearly in d.  Rho reference: sqrt(pi*n/4) group operations
with n ~ 2^131.  Output: extrapolation.json and a printed table.
"""
import glob, json, math, os

H = os.path.dirname(os.path.abspath(__file__))
W = json.load(open(os.path.join(H, "widths.json")))
ds = [w["d"] for w in W]; ws = [w["width"] for w in W]
n = len(ds); md = sum(ds) / n; mw = sum(ws) / n
slope = sum((d - md) * (w - mw) for d, w in zip(ds, ws)) / sum((d - md) ** 2 for d in ds)
icpt = mw - slope * md
resid = max(abs(w - (icpt + slope * d)) for d, w in zip(ds, ws))
fc = json.load(open(os.path.join(H, "first_count.json")))
cal = max(fc, key=lambda r: r["width"])
c = cal["seconds"] / 2 ** cal["width"]  # seconds per table entry on this CPU
runs = [json.load(open(f)) for f in sorted(glob.glob(os.path.join(H, "runs", "ic_*.json")))]

def row(d):
    w = icpt + slope * d
    return dict(d=d, width=round(w, 1), log2_entries_per_count=round(w, 1),
                log2_counts_per_relation=round(math.log2(4 * d + 1), 2),
                log2_cpu_seconds_per_relation=round(w + math.log2(c) + math.log2(4 * d + 1), 1))

target = 131
rho = 0.5 * math.log2(math.pi * 2 ** target / 4)
out = dict(width_fit=dict(slope=round(slope, 3), intercept=round(icpt, 2), max_residual=round(resid, 2), points=W),
           calibration=dict(**cal, seconds_per_entry=c),
           measured_runs=runs,
           extrapolated=[row(d) for d in (8, 16, 32, 64, 131)],
           eccp131=dict(rho_log2_group_ops=round(rho, 2), btn_log2_entries_per_count=round(icpt + slope * target, 1)))
json.dump(out, open(os.path.join(H, "extrapolation.json"), "w"), indent=1)
print(f"width(d) ~ {icpt:.2f} + {slope:.3f} d   (max residual {resid:.2f}, {n} points)")
print(f"calibration: width {cal['width']} -> {cal['seconds']} s  ({c:.2e} s per entry)")
for r in out["extrapolated"]:
    print(f"d={r['d']:>3}: width {r['width']:>6}, 2^{r['log2_entries_per_count']} entries/count, "
          f"~2^{r['log2_cpu_seconds_per_relation']} CPU-s per relation")
print(f"ECCp-131: one contraction ~2^{out['eccp131']['btn_log2_entries_per_count']} entries; "
      f"Pollard rho ~2^{out['eccp131']['rho_log2_group_ops']} group operations for the whole DLP")
