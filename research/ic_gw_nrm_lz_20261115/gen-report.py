#!/usr/bin/env python3
"""Writes REPORT.typ from analysis.json. usage: gen-report.py"""
import json, os
H = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(H, "analysis.json")))
QS = [262111, 524287, 1048507, 2097091, 4194271]; KS = [18, 19, 20, 21, 22]
R = a["ratios"]; G = a["gates"]; g = a["gap_k22"]
def tab(arm, y):
    rows = "".join(f"[{q} ({k})], " + ", ".join(f"[{R[f'{arm}/{y}/{q}/{w}/0.0']['geomean']:.4f} ({R[f'{arm}/{y}/{q}/{w}/0.0']['max']:.4f})]" for w in (0.0, 0.5, 1.0)) + ",\n" for q, k in zip(QS, KS))
    return f"#table(columns: 4, align: left,\n[q (k)], [w_add = 0], [w_add = 0.5], [w_add = 1],\n{rows})\n"
d = a["decision"]
yn = lambda b: "improvement" if b else "not an improvement"
t = ["#set page(margin: 2cm)\n#set text(size: 10pt)\n= Normalised points and Lanczos dot reuse in the plane-quartic IC\n",
 "_Study ic\\_gw\\_nrm\\_lz\\_20261115. All numbers \\[pcc:unverified\\], L0 local run (EKS budget hold), counted cost only._\n",
 "*Hypotheses.* NRM: normalised factor-base and residual points (first coordinate 1) skip the products by that coordinate in relation finding. LZ\\_REUSE: scalar Lanczos (arms D, DL1) takes $v^T v_(\"prev\")$ and $w_i^T y$ from the local and global A-orthogonality identities. Candidate = BLR1 plus the two env-gated changes; REF = BLR1 (sha256 e236855d7fef...); 5 arms x 30 cells (k = 18..22, seeds 1-6) = 150 units, c#sub[D] = 1.7.\n",
 f"*Gates.* G0 (A0 = REF) {'pass' if G['G0']['pass'] else 'FAIL'}; G1 (only the allowed fields change) {'pass' if G['G1']['pass'] else 'FAIL'}; G2 (all ok, exit 0) {'pass' if G['G2']['pass'] else 'FAIL'}; G3 (all s.\\* statistics equal) {'pass' if G['G3']['pass'] else 'FAIL'}.\n",
 '#image("nrmlz.png", width: 100%)\n',
 f"*Decision (frozen rule: ratio < 1 at all 30 cells and w_add in 0, 0.5, 1).* BOTH: DL {yn(d['BOTH']['DL']['improvement'])} (max {d['BOTH']['DL']['max_ratio']:.4f}), DL1 {yn(d['BOTH']['DL1']['improvement'])} (max {d['BOTH']['DL1']['max_ratio']:.4f}), D {yn(d['BOTH']['D']['improvement'])} (max {d['BOTH']['D']['max_ratio']:.4f}); also below 1 for every w#sub[h]. NRM alone: all three arms improve. LZ alone: only DL1 and D (DL is unchanged, ratio exactly 1).\n"]
for y in ("DL", "DL1", "D"):
    t.append(f"*BOTH/REF, {y} cold M-eq*, geomean (max) over seeds 1-6, $w_h = 0$:\n" + tab("BOTH", y))
t.append(f"NRM only, DL geomean at w_add = 0 / 1: {R['NRM/DL/4194271/0.0/0.0']['geomean']:.4f} / {R['NRM/DL/4194271/1.0/0.0']['geomean']:.4f} at k = 22 (relations.mul ratio {a['relations_mul_ratio']['NRM']['4194271']:.4f}); LZ only, DL1 at w_add = 0 / 1: {R['LZ/DL1/4194271/0.0/0.0']['geomean']:.4f} / {R['LZ/DL1/4194271/1.0/0.0']['geomean']:.4f} at k = 22 (DL1 linear\\_algebra.lmul ratio {a['lmul_ratio']['LZ/DL1']['4194271']:.4f}).\n")
t.append(f"*Gap to the matched negation-map rho at k = 22* (frozen band; REF reproduces its IC numbers to $<10^(-9)$: {'yes' if g['ref_reproduces_band'] else 'no'}): rho/IC (DL) at w = 0, 0.1, 1 GAE goes from {g['rho_over_ic']['REF/DL/0']:.1f}, {g['rho_over_ic']['REF/DL/0.1']:.1f}, {g['rho_over_ic']['REF/DL/1']:.1f} (REF) to {g['rho_over_ic']['BOTH/DL/0']:.1f}, {g['rho_over_ic']['BOTH/DL/0.1']:.1f}, {g['rho_over_ic']['BOTH/DL/1']:.1f} (BOTH). Distance remaining IC/rho: DL {1/g['rho_over_ic']['BOTH/DL/0']:.5f}, {1/g['rho_over_ic']['BOTH/DL/0.1']:.5f}, {1/g['rho_over_ic']['BOTH/DL/1']:.5f}; DL1 {1/g['rho_over_ic']['BOTH/DL1/0']:.5f}, {1/g['rho_over_ic']['BOTH/DL1/0.1']:.5f}, {1/g['rho_over_ic']['BOTH/DL1/1']:.5f}. Not validated: no replay certificate, validator report or new matched-rho run, so no leaderboard row moves.\n")
open(os.path.join(H, "REPORT.typ"), "w").write("\n".join(t))
