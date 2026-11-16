#!/usr/bin/env python3
"""Render the exact peeled-target orbit-reuse diagnostic and its PDF."""
from __future__ import annotations

import json
import subprocess
from pathlib import Path

import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
TYPST = Path("/tmp/typst-x86_64-unknown-linux-musl/typst")


def main() -> None:
    record = json.loads((ROOT / "K1_PEELED_ORBIT_REUSE_v1.json").read_text())
    rows = record["runs"]
    assert record["status"] == "pass" and len(rows) == 6
    subgroup = json.loads((ROOT / "K1_PEELED_SUBGROUP_REUSE_v1.json").read_text())
    assert subgroup["status"] == "pass" and len(subgroup["rows"]) == 24
    assert all(subgroup["controls"].values())
    fig, ax = plt.subplots(figsize=(9.3, 4.6), layout="constrained")
    labels = [f"{r['p']}/{r['seed']}" for r in rows]
    xs = list(range(6))
    intra = [100 * r["intra_query_duplicates"] / r["finite_peels"] for r in rows]
    cross = [100 * r["cross_query_duplicates_after_local_dedup"] / r["finite_peels"]
             for r in rows]
    ax.bar(xs, intra, color="#414487", label="within a residual")
    ax.bar(xs, cross, bottom=intra, color="#35b779", label="across residuals in one stream")
    ax.set_xticks(xs, labels)
    ax.set_ylabel("Exact orbit-colliding peels / all finite peels (%)")
    ax.set_xlabel("Frozen p/seed one-target stream")
    ax.grid(axis="y", alpha=.2)
    ax.legend()
    ax.set_title("Maximum S4 solve reuse from exact 24-action peeled-target orbits")
    fig.savefig(ROOT / "k1_peeled_orbit_reuse.svg")
    fig.savefig(ROOT / "k1_peeled_orbit_reuse.png", dpi=170)
    plt.close(fig)
    group_names = ("identity", "cubic", "frob", "full24")
    group_labels = ("identity", "cubic order 3", "twist Frob order 8", "combined order 24")
    totals = subgroup["totals"]
    fig2, ax2 = plt.subplots(figsize=(8.8, 4.5), layout="constrained")
    gx = list(range(4))
    local = [totals[name]["local_duplicates"] for name in group_names]
    cross = [totals[name]["cross_query_duplicates"] for name in group_names]
    ax2.bar(gx, cross, color="#35b779", label="across residuals")
    ax2.bar(gx, local, bottom=cross, color="#414487", label="within a residual")
    ax2.set_xticks(gx, group_labels)
    ax2.set_ylabel("Exact-orbit duplicate peels in six separate streams")
    ax2.set_title("Symmetry subgroup ablation on 142,248 finite peels")
    ax2.grid(axis="y", alpha=.2)
    ax2.legend()
    fig2.savefig(ROOT / "k1_peeled_subgroup_reuse.svg")
    fig2.savefig(ROOT / "k1_peeled_subgroup_reuse.png", dpi=170)
    plt.close(fig2)
    md = ["# Can twist symmetry reuse peeled S4 systems?", "",
          "23 October 2026. This exact scalar/group diagnostic uses every residual "
          "of the six frozen p=73/97 mixed-line streams, including those after "
          "the polynomial one-target stop. Its [protocol](QUARTIC_TWIST_LINE_PROTOCOL.md) "
          "was committed as `8801c22aaa` before this calculation. "
          "[Executable source](analyze_k1_peeled_orbits.py), "
          "[machine record](K1_PEELED_ORBIT_REUSE_v1.json), "
          "[subgroup ablation](K1_PEELED_SUBGROUP_REUSE_v1.json), "
          "[figure](k1_peeled_orbit_reuse.svg), and "
          "[PDF](K1_PEELED_ORBIT_REUSE.pdf) reproduce the result.", "",
          "## Exact test", "",
          "For each frozen residual `R=[a]G+[b]Q` and every signed physical "
          "line-2 factor `T`, the diagnostic computes `R−T` modulo the prime "
          "subgroup order. It identifies peeled targets under all 24 products "
          "of the independently checked Frobenius and cubic eigenvalues. "
          "Because the line-0 base is invariant, two peeled targets in the "
          "same orbit have symmetry-equivalent S4 solution sets in principle. "
          "This is an *upper bound on exact-orbit solver-call reuse*: moving "
          "solutions back, sign reconstruction, cache lookup and canonicalization "
          "would still cost work in a native collector.", "",
          "There is also a direct obstruction to the tempting 12-fold reuse "
          "within one line-2 factor orbit. If `T′=λT` and its peeled target "
          "were the `λ` image of `R−T`, then `R−λT=λ(R−T)` would force "
          "`(1−λ)R=0`. On a prime-order subgroup, every nonidentity action "
          "has `λ≠1`, so this requires `R=O`. All 1,856 residuals tested "
          "here are nonzero. A duplicate can still occur with a *different* "
          "line-2 factor satisfying `T′=(1−λ)R+λT`; the exact count below "
          "finds those exceptional coincidences.", "",
          "The code checks that each eigenvalue group has exactly 24 elements, "
          "the signed line-2 set is closed, all 24 images of every finite "
          "peeled target have the same canonical representative, and a "
          "one-unit perturbation of the Frobenius eigenvalue breaks closure. "
          "The factor logs are the original producer's group-verified logs "
          "and each six-run independent point-keyed/BSGS audit verifies all "
          "physical base logs and the target. No polynomial solver or relation "
          "oracle runs in this diagnostic.", "",
          "![Exact 24-action peeled-target orbit collisions](k1_peeled_orbit_reuse.svg)", "",
          "## All six complete-stream results", "",
          "`Local` counts duplicates *within one residual*; `cross` counts "
          "additional duplicates in later residuals after local deduplication. "
          "The final column is the maximal exact-orbit cache ratio if an "
          "unbounded cache were shared through that one stream.", "",
          "| p/seed | Residuals | Signed L2 factors | Finite peels | Infinity | Local duplicates | Cross duplicates | Max stream cache ratio |",
          "|:--|--:|--:|--:|--:|--:|--:|--:|"]
    for r in rows:
        md.append(f"| {r['p']}/{r['seed']} | {r['residuals']:,} | "
                  f"{r['signed_l2_factors']} | {r['finite_peels']:,} | "
                  f"{r['infinity_peels']} | {r['intra_query_duplicates']} | "
                  f"{r['cross_query_duplicates_after_local_dedup']} | "
                  f"{r['max_exact_orbit_cache_ratio_across']:.6f}× |")
    md += ["", "## Identity, cubic, twist Frobenius and combined action", "",
           "A separately frozen [subgroup ablation](analyze_k1_peeled_subgroups.py) "
           "repeats the same exact count under identity, the cubic order-three "
           "action, transported Frobenius order eight, and their combined "
           "order-24 action. It checks all subgroup orders and containment, "
           "signed-factor closure, monotone unique counts and exact reproduction "
           "of the earlier combined-action record. Each row sums six separate "
           "one-target streams; a cache is not shared between seeds.", "",
           "![Subgroup ablation of peeled-target orbit reuse](k1_peeled_subgroup_reuse.svg)", "",
           "| Action | Order | Local duplicates | Cross-residual duplicates | Unique targets within streams | Ideal stream cache ratio |",
           "|:--|--:|--:|--:|--:|--:|"]
    for name, order in zip(group_names, (1, 3, 8, 24)):
        t = totals[name]
        md.append(f"| {name} | {order} | {t['local_duplicates']} | "
                  f"{t['cross_query_duplicates']} | {t['unique_within_streams']:,} | "
                  f"{t['ideal_stream_cache_ratio']:.6f}× |")
    md += ["", "The combined twist/cubic action identifies 770 more peeled targets "
           "than an identity-only cache (141,458 versus 142,228 unique targets "
           "within the six separate streams). Cubic alone identifies 87 more "
           "than identity and transported Frobenius alone 223 more. Combined "
           "action adds 683 relative to cubic alone. These are exact call-count "
           "upper bounds, with no native cache implementation or priced cost.", ""]
    md += ["", f"Across six separate streams, {record['total_finite_peels']:,} "
           f"finite peels have {record['total_intra_query_duplicates']} local "
           f"and {record['total_cross_query_duplicates']} further within-stream "
           "orbit collisions. At most 790 of 142,248 S4 calls could be "
           "eliminated by an ideal exact-orbit cache on these specific streams "
           "(1.005585× call-count ratio after local and within-stream collisions). "
           "Only two duplicate peels occur inside one residual. The count "
           "does not pool caches across different seeds or targets.", "",
           "## Interpretation and limits", "",
           "The 24-action makes the factor base and equations covariant, "
           "but a generic fixed residual has no stabilizer that makes all "
           "12 physical members of one line-2 orbit equivalent *at that same "
           "residual*. The observed two local collisions are consistent with "
           "that distinction; they do not prove a universal absence of "
           "symmetry reuse. A native canonicalized S4 cache could realize "
           "less saving once lookup, transformation and extra memory are "
           "charged. These six small-field streams do not test other "
           "factor-base shapes, target-generation rules, or larger curves. "
           "The separately audited [native thinning grid](K1_THIN_NATIVE_REPORT.md) "
           "addresses the different mechanism of reducing the number of "
           "signed line-2 peels in the first place.", ""]
    (ROOT / "K1_PEELED_ORBIT_REUSE.md").write_text("\n".join(md))
    typ = ['#set page(paper: "a4", margin: 15mm)', '#set text(size: 9pt)',
           '= Can twist symmetry reuse peeled S4 systems?',
           '23 October 2026. Six complete frozen p=73/97 one-target streams. '
           'Protocol commit 8801c22aaa; source and raw record in the study directory.',
           'For each residual and signed L2 factor, canonicalize R−T under the '
           'verified 24-element Frobenius/cubic action. Equal canonical targets '
           'have symmetry-equivalent S4 solution sets in principle. This counts '
           'possible calls eliminated before charging the cache.',
           'For a nonzero residual R, corresponding factors T and λT in one '
           'L2 orbit cannot give λ-equivalent peeled targets: '
           'R−λT=λ(R−T) would imply (1−λ)R=0, impossible for λ≠1 in '
           'the prime-order group. All 1,856 tested residuals are nonzero.',
           '#image("k1_peeled_orbit_reuse.svg", width: 100%)',
           '#table(columns: (1fr, 1fr, 1fr, 1fr, 1fr), inset: 3pt, '
           '[*p/seed*], [*peels*], [*local dup*], [*cross dup*], [*max cache ratio*],']
    for r in rows:
        typ.append(f'[{r["p"]}/{r["seed"]}], [{r["finite_peels"]:,}], '
                   f'[{r["intra_query_duplicates"]}], '
                   f'[{r["cross_query_duplicates_after_local_dedup"]}], '
                   f'[{r["max_exact_orbit_cache_ratio_across"]:.6f}],')
    typ += [')',
            'Across separate streams: 142,248 finite peels, two within-residual '
            'and 788 additional within-stream exact-orbit collisions. The '
            'ideal maximum call-count ratio is 1.005585. Group-action closure, '
            'all-image canonical equality and a perturbed-eigenvalue rejection '
            'were checked. The physical logs came from the prior independently '
            'audited BSGS records.',
            '#pagebreak()', '== Subgroup ablation',
            'Follow-up protocol commit 433df0f52d. Identity, cubic order 3, '
            'transported Frobenius order 8 and combined order 24 are compared '
            'on the same six streams. The full-24 counts reproduce the first record.',
            '#image("k1_peeled_subgroup_reuse.svg", width: 88%)',
            '#table(columns: (1fr, .7fr, .8fr, 1fr, 1fr), inset: 3pt, '
            '[*action*], [*order*], [*local*], [*cross query*], [*ideal ratio*],']
    for name, order in zip(group_names, (1, 3, 8, 24)):
        t = totals[name]
        typ.append(f'[{name}], [{order}], [{t["local_duplicates"]}], '
                   f'[{t["cross_query_duplicates"]}], '
                   f'[{t["ideal_stream_cache_ratio"]:.6f}],')
    typ += [')',
            'Combined action identifies 770 more targets than identity-only '
            'caching among 142,248 finite peels. It identifies 683 more '
            'than cubic alone. These counts exclude cache work.',
            'This is an exact orbit count, not a measured polynomial cache or '
            'whole-method speed result. Other factor bases and curves remain open.']
    (ROOT / "K1_PEELED_ORBIT_REUSE.typ").write_text("\n".join(typ) + "\n")
    subprocess.run([str(TYPST), "compile", "K1_PEELED_ORBIT_REUSE.typ",
                    "K1_PEELED_ORBIT_REUSE.pdf"], cwd=ROOT, check=True)
    print("rendered peeled-orbit reuse report")


if __name__ == "__main__":
    main()
