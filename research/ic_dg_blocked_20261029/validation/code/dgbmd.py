#!/usr/bin/env python3
"""Write REPORT.md from work/recompute.json and the built report (validator's own code)."""
import json

V = "/dev/shm/dgb-validation"
R = json.load(open(f"{V}/work/recompute.json"))["sizes"]
REP = json.load(open(f"{V}/validator-report.json"))
CERT = json.load(open(f"{V}/replay-certificate.json"))


def ci(x, n=4):
    return f"[{x[0]:.{n}f}, {x[1]:.{n}f}]"


L = []
A = L.append
A(f"# XP-20261029-dg-blocked: independent validation ({REP['id']})")
A("")
A("Snapshot `claude/ic-dg-blocked-20261029` at `bf899979a6` (equals origin at start and end; working tree clean at start, later dirtied by another session with untracked launch/, README.md and figures/, none used as evidence: launcher receipts are read from S3); frozen protocol `cd04cfd2d1` plus `cards/RULING-P2.md` (A1). "
  "Own code under `code/`; the ecbench binary (88070375…412c) was used only for the replay. Evidence admissibility only: this authorizes nothing.")
A("")
A("## Verdicts")
A("")
A("| b | verdict | reason |")
A("|--:|:--|:--|")
for P in ("P24", "P28", "P32"):
    A(f"| {P[1:]} | **{REP['size_verdict'][P]}** | {REP['size_verdict_reasons'][P]} |")
A("")
A("## Wall statistic, recomputed from `records.jsonl` (w = 0 decision file, `--a dg --b dgb`)")
A("")
A("| b | ns/GAE dg | ns/GAE dgb | dgb/dg | aa_ratio dg-aa/dg | corrected dgb/dg | max rel diff vs committed |")
A("|--:|--:|--:|:--|:--|:--|--:|")
for P in ("P24", "P28", "P32"):
    d = R[P]["recompute"]["decision_dg__dgb"]
    A(f"| {P[1:]} | {d['ns_per_gae_a']:.3f} | {d['ns_per_gae_b']:.3f} | {d['ratio_b_over_a']:.6f} {ci(d['ci95'])} | {d['aa_ratio']:.6f} {ci(d['aa_ci95'])} | "
      f"**{d['corrected_ratio']:.6f} {ci(d['corrected_ci95'], 6)}** | {R[P]['recompute']['decision_vs_committed']['max_rel_diff']:.1e} |")
A("")
A("24 and 32 bits are descriptive only (section 6.1 fails). The w = 0.1 and w = 1 files carry an identical wall block at every size.")
A("")
A("Descriptive (decides nothing):")
A("")
A("| b | identity: dg-aa→dgb vs corrected (rel diff) | edge control dgb-edge/dg | dgb/dgb-edge | target-clustered (48) linearised | stratified target bootstrap (20,000) |")
A("|--:|:--|:--|:--|:--|:--|")
for P in ("P24", "P28", "P32"):
    rc = R[P]["recompute"]
    pc = rc["position_control"]
    cl = rc["clustered"]
    A(f"| {P[1:]} | {rc['identity_cond9_recomputed']['dg-aa__dgb_ratio']:.6f} ({rc['identity_cond9_recomputed']['rel_diff']:.1e}) | "
      f"{pc['edge_estimate_dgb-edge_over_dg']:.6f} {ci(pc['edge_ci95'])} (overlaps decision: {pc['intervals_overlap']}) | "
      f"{pc['middle_over_edge_dgb']['dgb_over_dgb-edge']:.6f} {ci(pc['middle_over_edge_dgb']['ci95'])} | "
      f"{ci(cl['target_clusters']['ci95'], 5)} | {ci(cl['stratified_target_bootstrap']['percentile_ci95'], 5)} |")
A("")
A("Per curve (corrected dgb/dg):")
A("")
for P in ("P24", "P28", "P32"):
    A(f"- {P[1:]}: " + "; ".join(f"`{s}` {v['corrected_ratio']:.4f} {ci(v['corrected_ci95'])}" for s, v in R[P]["recompute"]["per_curve"].items()))
A("")
A("Positions (measured blocks, every size): `dg` and `dgb-edge` at {0: 144, 3: 144}; `dg-aa` and `dgb` at {1: 144, 2: 144}; 144 forward and 144 reversed blocks; "
  "`dgb` precedes `dg-aa` in 144 blocks and follows it in 144.")
A("")
A("## Section 6.1 and A1, rechecked")
A("")
A("| b | wall 1 exit | 2 audit | 3 counted A/A | 4 spec/build | 5 gate | 6 native A/A | 7 decision block | 8 identical work | 9 identity | A1 | counted route | counted = wall dg, dgb |")
A("|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|")
yn = lambda x: "holds" if x else "**fails**"  # noqa: E731
for P in ("P24", "P28", "P32"):
    g = R[P]["wall_gate"]
    c = R[P]["counted_gate"]
    A(f"| {P[1:]} | {yn(g['1_exit']['holds'])} ({g['1_exit']['exit_code']}) | {yn(g['2_audit']['holds'])} | {yn(g['3_counted_aa']['holds'])} | "
      f"{yn(g['4_spec_build']['holds'])} | {yn(g['5_gate']['holds'])} | {yn(g['6_native_aa']['holds'])} | {yn(g['7_decision_block']['holds'])} | "
      f"{yn(g['8_identical_work']['holds'])} | {yn(g['9_identity']['holds'])} | {yn(g['A1_packed_layout']['holds'] and c['A1_counted_dgb']['holds'])} | "
      f"{yn(c['route_holds'])} (exit {c['exit']['exit_code']}) | {yn(c['counted_equals_wall_holds'])} |")
A("")
A("Below-L2 executions (recomputed from `records.jsonl`; equal to `l2-gate.json`):")
A("")
A("- 24-bit wall: seq 1144 `dgb-edge` (903.9 µs) and seq 1145 `dgb` (913.0 µs), the same block (`icv1-fp24-tm995-ee6c519b`, target 22, round 5, reversed), "
  "28,031 GAE (the cheapest instance); blocker `L2: solve too short to admit wall time`. `dg-aa` and `dg` in that block took 1.284 and 1.270 ms. "
  "24-bit counted: seq 1145 `dgb` (911.2 µs) is an IC execution below L2, so the exit-42 clause fails.")
A("- 32-bit wall: seq 1313 `dg-aa` (`icv1-fp32-tm81759-3ad5e8c5`, target 16, round 6), blocker `L2: 3 foreign task slices on the run CPU during the solve (0 µs)`; "
  "solve 195.09 ms against 194.96 ms for `dg` in the same block. The 32-bit counted session holds (10 rho executions L1, all short solves).")
A("- 28 bits: no wall execution below L2; counted: 273 rho executions L1, all `solve too short` (allowed by the exit-42 clause).")
A("")
A("A1 (packed layout): at `be2969ce`, `build_negation_folded_prime_blocked` takes `PackedPairTable` iff `p <= u32::MAX && fb.points.len() <= 1<<16` "
  "(`ic_boundary.rs` 5754-5777); `signed_points` in the record is `fb.points.len()`. Max p: "
  + ", ".join(f"{P[1:]}: {R[P]['wall_gate']['A1_packed_layout']['max_p']}" for P in ("P24", "P28", "P32"))
  + "; signed points: " + ", ".join(f"{P[1:]}: {R[P]['wall_gate']['A1_packed_layout']['max_signed_points']}" for P in ("P24", "P28", "P32"))
  + ". Every `dgb`/`dgb-edge` record (wall and counted, warm-ups included) is in the packed regime.")
A("")
A("Not relabelling or accounting: each `dgb` and `dgb-edge` record (336 per size, 48 warm-ups) equals the `dg` record of the same curve, target and round in "
  "recovered logarithm, total GAE, every unpriced counter, every per-phase count and native map, seed, counters, and also bitop work and factor-base hashes: 0 differences. "
  "The counted session's `dg`/`dgb` records equal the wall session's. Only wall time (and, at 32 bits, RSS 80.3 → 61.9 MB) differs.")
A("")
A(f"## Replay ({CERT['status']}, independent)")
A("")
A(f"`ecbench verify --replay-all` on copies of all six sessions on this host (Intel, 4 vCPU): {CERT['totals']['replays_identical']} of {CERT['totals']['replays']} measured runs identical, "
  f"0 problems; copies equal the job SHA256SUMS and S3 originals; {REP['record_integrity']['total_records']} record seals recomputed, 0 failures.")
A("")
A("Launcher receipts were read from their S3 originals (`runs/<job>/launcher/<id>/`), not from the evidence directory's `launch/` folders, which are untracked at `bf899979a6` "
  "(written by another session after the snapshot push; byte-equal to S3). Each receipt's spec equals the frozen spec; one pod per Job, no restart; `status.json` pod, node and exit equal `job-exit.json`. "
  "Launcher state is `succeeded`/0 for 28-bit wall and `failed`/23 for the five exit-42 sessions, as the v3 protocol anticipates.")
A("")
A("## Matched rho (counted session, rho-neg), mean S at w = 0 / 0.1 / 1")
A("")
A("| curve | dg = dgb | rho-neg | dgb/rho-neg (distance remaining) |")
A("|:--|:--|:--|:--|")
for P in ("P24", "P28", "P32"):
    for s, v in R[P]["band_counted"]["per_curve"].items():
        m = v["mean_S"]
        A(f"| `{s}` | {m['dgb']['0']:.3f} / {m['dgb']['0.1']:.3f} / {m['dgb']['1']:.3f} | {m['rho-neg']['0']:.4f} / {m['rho-neg']['0.1']:.4f} / {m['rho-neg']['1']:.4f} | "
          f"{v['ic_over_rho']['dgb']['0']:.2f} / {v['ic_over_rho']['dgb']['0.1']:.2f} / {v['ic_over_rho']['dgb']['1']:.2f} |")
A("")
A("rho-neg < dg = dgb at all three weights on every curve. Strongest available: every curve has p and r prime, cofactor 1, j ∉ {0, 1728} "
  "(recomputed from a, b solved on G and targets; equals the ICV1 j), so Aut(E) = {±1}; negation is the only fold (no Frobenius fold over F_p; "
  "Frobenius discriminants have large squarefree parts). Records show `automorphisms_used` 2.")
A("")
A("## Phase sums (measured runs, ms; descriptive)")
A("")
A("| b | oracle_setup dg → dgb | target_PDP dg → dgb | relations dg → dgb | solve dg → dgb |")
A("|--:|:--|:--|:--|:--|")
for P in ("P24", "P28", "P32"):
    x, y = R[P]["phase_sums_wall"]["dg"], R[P]["phase_sums_wall"]["dgb"]
    A(f"| {P[1:]} | {x['phase_wall_ms']['oracle_setup']:.1f} → {y['phase_wall_ms']['oracle_setup']:.1f} | {x['online_phases_ms']['target_PDP']:.1f} → {y['online_phases_ms']['target_PDP']:.1f} | "
      f"{x['phase_wall_ms']['relations']:.1f} → {y['phase_wall_ms']['relations']:.1f} | {x['solve_wall_ms']:.1f} → {y['solve_wall_ms']:.1f} |")
A("")
A("The main session's 28-bit reading (set-up 1769 → 1000 ms, target PDP 1062 → 1428 ms) is confirmed. Target PDP wall rises at every size, consistent with a slower probe on the packed table; "
  "with the smaller-than-predicted set-up saving at 24 and 28 bits, that is why every corrected ratio lies above its section 7 prediction (24: 0.81-0.87, 28: 0.77-0.84, 32: 0.78-0.85).")
A("")
A("## Discrepancies with the main session")
A("")
A("None in the numbers: 0.9300 [0.9266, 0.9333], the below-L2 executions (24: seq 1144 `dgb-edge`, 1145 `dgb`; 32: seq 1313 `dg-aa`, '3 foreign task slices (0 µs)'), "
  "the record-by-record identity and the phase sums all reproduce. Additions: at 24 bits the counted session also fails its route (counted `dgb` seq 1145, 911.2 µs), "
  "which VERDICT.md states and the main-session summary omits; the 28-bit per-curve ratios differ beyond their intervals.")
A("")
A("## Before promotion")
A("")
for p in REP["promotion_blockers"]:
    A(f"- {p}")
A("")
A("## Files")
A("")
A("`validator-report.json`, `replay-certificate.json`, `VALIDATION.draft.json` (rows: the two 28-bit curves), `code/` (replay.sh, fetch_launcher.sh, dgbstats.py, dgbval.py, dgbseal.py, dgbbuild.py, gatecheck.sh, dgbmd.py), `launcher-receipts/` (S3 request.json and result.json per session), "
  "`work/` (recompute.json, seals.json, gatecheck.txt), `replay/` (audit and logs per session), `frozen/` (protocol, approval, ruling, specs, source excerpts), `SHA256SUMS`.")
open(f"{V}/REPORT.md", "w").write("\n".join(L) + "\n")
print("\n".join(L))
