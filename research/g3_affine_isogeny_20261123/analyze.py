#!/usr/bin/env python3
"""Summarise data/validation.jsonl against PROTOCOL.md. Writes data/summary.json and
fig_validation.svg."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PRIMES = [17, 19, 29, 31, 37, 41, 43, 47]


def main():
    rows = [json.loads(l) for l in open(os.path.join(HERE, "data", "validation.jsonl"))]
    S = {"n_kernels": len(rows), "per_prime": {}, "all": {}}
    tot = {}

    def bump(d, k):
        d[k] = d.get(k, 0) + 1
    for r in rows:
        pp = S["per_prime"].setdefault(str(r["p"]), {})
        key = f"{r['label']}|{r['status'] if r['status'] != 'ok' else 'ok'}|{r['match']}"
        bump(pp, key)
        bump(tot, key)
    S["all"] = tot
    nd = [r for r in rows if r["label"] != "degenerate" and r["informative"]]
    ok = [r for r in nd if r["status"] == "ok"]
    agree = [r for r in ok if (r["label"] == "jacobian" and r["match"] == "J") or
             (r["label"] == "twist" and r["match"] == "twist")]
    tw = [r for r in ok if r["label"] == "twist"]
    S["primary"] = {"nondegenerate_informative": len(nd), "produced": len(ok),
                    "produced_share": round(len(ok) / len(nd), 4) if nd else None,
                    "agree_with_label": len(agree), "disagree": len(ok) - len(agree),
                    "jacobian_label_produced": sum(r["label"] == "jacobian" for r in ok),
                    "jacobian_label_match_J": sum(r["label"] == "jacobian" and r["match"] == "J" for r in ok),
                    "twist_label_produced": len(tw),
                    "twist_label_match_twist": sum(r["match"] == "twist" for r in tw),
                    "twist_label_match_J": sum(r["match"] == "J" for r in tw)}
    S["uninformative_kernels"] = sum(1 for r in rows if not r["informative"])
    deg = [r for r in rows if r["label"] == "degenerate"]
    S["secondary_degenerate"] = {"n": len(deg),
                                 "vanishing_theta": sum(r["status"].startswith("vanishing") for r in deg)}
    rep = [r for r in rows if "replay" in r]
    S["G1_replay"] = {"n": len(rep), "same_N": sum(r["replay"]["same_N"] for r in rep)}
    S["G2_integral"] = {"n": len(ok), "integral": sum(r["diag"].get("integral", False) for r in ok)}
    S["n_stable_aronhold_sets_hist"] = {}
    for r in ok:
        bump(S["n_stable_aronhold_sets_hist"], str(r["diag"].get("n_stable_aronhold_sets")))
    S["row_sign_patterns_hist"] = {}
    for r in ok:
        bump(S["row_sign_patterns_hist"], str(r["diag"].get("n_row_sign_patterns_ok")))
    S["sec_total"] = round(sum(r["sec"] for r in rows), 1)
    S["success_rule"] = {
        "produced_share_ge_0.95": bool(nd) and len(ok) / len(nd) >= 0.95,
        "every_produced_matches_label": len(agree) == len(ok),
        "negative_control_informative_ge_20": len(tw) >= 20}
    S["success"] = all(S["success_rule"].values())
    json.dump(S, open(os.path.join(HERE, "data", "summary.json"), "w"), indent=1)
    print(json.dumps(S, indent=1))
    svg(rows)


def svg(rows):
    W, H = 720, 300
    bars = []
    for i, p in enumerate(PRIMES):
        rs = [r for r in rows if r["p"] == p]
        a = sum(r["label"] == "jacobian" and r["match"] == "J" for r in rs)
        b = sum(r["label"] == "twist" and r["match"] == "twist" for r in rs)
        c = sum(r["label"] == "degenerate" for r in rs)
        d = sum(r["label"] != "degenerate" and not ((r["label"] == "jacobian" and r["match"] == "J") or
                                                     (r["label"] == "twist" and r["match"] == "twist")) for r in rs)
        bars.append((p, a, b, c, d))
    cols = ["#2a6fbb", "#d9822b", "#9a9a9a", "#c0392b"]
    names = ["label Jacobian, Jac(X) = J (P_X = P_J)", "label twist, Jac(X) = J^tw (P_X = P_J(-T))",
             "degenerate (theta-null vanishes)", "mismatch or no quartic"]
    s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="12">',
         '<rect width="100%" height="100%" fill="white"/>',
         f'<text x="20" y="22" font-size="14">Explicit quartic X over F_p for B = J/L (affine L): Frobenius polynomial of Jac(X) vs the twist label</text>']
    x0, y0, bw, sc = 60, 260, 60, 4.4
    s.append(f'<line x1="{x0-5}" y1="{y0}" x2="{x0+8*80}" y2="{y0}" stroke="black"/>')
    for k in range(0, 41, 10):
        s.append(f'<text x="{x0-30}" y="{y0-k*sc+4}">{k}</text>')
        s.append(f'<line x1="{x0-5}" y1="{y0-k*sc}" x2="{x0+8*80}" y2="{y0-k*sc}" stroke="#eee"/>')
    for i, (p, *vals) in enumerate(bars):
        x = x0 + i * 80
        y = y0
        for v, ccol in zip(vals, cols):
            if v:
                s.append(f'<rect x="{x}" y="{y-v*sc}" width="{bw}" height="{v*sc}" fill="{ccol}"/>')
                s.append(f'<text x="{x+bw/2}" y="{y-v*sc/2+4}" text-anchor="middle" fill="white">{v}</text>')
            y -= v * sc
        s.append(f'<text x="{x+bw/2}" y="{y0+16}" text-anchor="middle">p={p}</text>')
    for j, (n, ccol) in enumerate(zip(names, cols)):
        s.append(f'<rect x="{x0+j%2*330}" y="{36+j//2*16}" width="10" height="10" fill="{ccol}"/>')
        s.append(f'<text x="{x0+14+j%2*330}" y="{45+j//2*16}">{n}</text>')
    s.append(f'<text x="{x0}" y="{y0+34}" fill="#555">40 kernels per prime (20 prime-order curves x 2 stable affine kernels). [pcc:unverified]</text>')
    s.append("</svg>")
    open(os.path.join(HERE, "fig_validation.svg"), "w").write("\n".join(s))


if __name__ == "__main__":
    main()
