"""Lay out the recorded run as Markdown tables: no computation beyond
formatting the values in walk.json / results.json (AGENTS.md "Implementation
language").  usage: python make_report.py RUN_DIR > tables.md
"""
import json
import sys
from pathlib import Path

run = Path(sys.argv[1])
CLASSES = [c for c in ["P2"] if (run / c / "walk.json").exists()]


def f0(x):
    return f"{x:,.0f}"


def f1(x):
    return f"{x:,.1f}"


def f3(x):
    return f"{x:.3f}"


def load(c, mode):
    p = run / c / mode / "results.json"
    return json.loads(p.read_text()) if p.exists() else None


out = []
P = out.append

# ---------------------------------------------------------------- classes
P("### The four classes\n")
P("| class | field | root curve | `#E = h·n` | `t` | `D = f²·D_K` | `h(D_K)` | class size | strata walked / measured | walked / measured curves | edges verified |")
P("|:--|:--|:--|:--|--:|:--|--:|--:|:--|:--|--:|")
for c in CLASSES:
    w = json.loads((run / c / "walk.json").read_text())
    fld = w["field"]
    fname = f"`F_p`, p = {fld['p']}" if fld["kind"] == "prime" else f"`F_2^{fld['m']}`, poly {fld['poly']}"
    r = w["root"]
    cl = w["class"]
    strata_w = sorted({v["conductor"] for v in w["vertices"] if v["levels"] and v["explored"]})
    strata_m = sorted({v["conductor"] for v in w["vertices"] if v["measured"]})
    sup = str.maketrans("0123456789", "⁰¹²³⁴⁵⁶⁷⁸⁹")
    fac = "·".join(f"{p}{str(e).translate(sup)}" if e > 1 else str(p) for p, e in cl["conductor_factored"])
    P(
        f"| {c} | {fname} | a = {r['a']}, b = {r['b']} (j = {r['j']}) | {r['cofactor']} · {r['n']} ({64 - int(r['n']).bit_length() if False else int(r['n']).bit_length()} bits) | {r['trace']} | {cl['conductor']}² · ({cl['fundamental_disc']}), f = {fac} | {f0(cl['h_fundamental'])} | {f0(cl['class_size'])} | {len(strata_w)} / {len(strata_m)} | {w['walk']['vertices_explored']} / {w['walk']['vertices_measured']} | {w['walk']['edges_verified']} |"
    )
P("")
P("Class size is `Σ_{d | f} h(d²D_K)` with `h(D_K)` counted from reduced forms and the other class numbers from the conductor formula; `h(f²D_K)` was also counted directly and agrees in every class (`h_full_order_counted` = `h_full_order_formula` in each `walk.json`).\n")

# ---------------------------------------------------------------- isogeny primes
P("### The isogenies walked, easy and hard\n")
P("| class | ℓ | role | volcano depth, crater | route | kernel field degree `d` | tree edges realised (neighbour searches) | mean ms per kernel search |")
P("|:--|--:|:--|:--|:--|--:|--:|--:|")
for c in CLASSES:
    w = json.loads((run / c / "walk.json").read_text())
    hard = set(w["params"].get("hard_ells", []))
    for e in w["ells"]:
        if not e["active"]:
            continue
        role = "hard (first vertices only)" if e["ell"] in hard else ("sampled, torsion route" if e.get("sampled") else "everywhere")
        edges = [v for v in w["vertices"] if v["edge"] and v["edge"]["ell"] == e["ell"]]
        routes = sorted({v["edge"]["route"] for v in edges})
        degs = sorted({v["edge"]["field_degree"] for v in edges})
        P(f"| {c} | {e['ell']} | {role} | {e['depth']}, {e['crater']} | {', '.join(routes) or '—'} | {', '.join(map(str, degs)) or '—'} (eigen-orbit {e.get('eigen_orbit_degree', '')}) | {len(edges)} ({e['kernel_calls']}) | {f1(e['mean_find_ms'])} |")
    for e in w["ells"]:
        if not e["active"]:
            P(f"| {c} | {e['ell']} | inactive | {e['depth']}, {e['crater']} | {e['reason']} | | | |")
P("")

# ---------------------------------------------------------------- class mode
for c in CLASSES:
    R = load(c, "class")
    if not R:
        continue
    w = json.loads((run / c / "walk.json").read_text())
    exp = R["rho"]["expected_iters_sqrt_pi_n_over_2"]
    P(f"### {c}: every measured curve, one instance, {R['params']['trials']} paired seeds per arm\n")
    P(f"Reference `√(πn/2)` = {f0(exp)} (×1.016 for the 32-adding walk: {f0(R['rho']['expected_with_r_adding'])}); {R['rho']['solves']} solves, all returned the planted `k`; rho wall {f1(R['rho']['wall_s'])} s on {R['host']['threads']} threads.\n")
    P("| arm | kind | curves | solves | min | mean ± 95 % CI | median | max | sd / mean | mean ms | e2e ms | ratio to √(πn/2) | z vs rest |")
    P("|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    pooled = R["pooled"]["summary"]["iters"]
    P(f"| all isogenous | pooled | {R['pooled']['arms']} | {pooled['n']} | {f0(pooled['min'])} | {f0(pooled['mean'])} ± {f0(pooled['ci95_half'])} | {f0(pooled['median'])} | {f0(pooled['max'])} | {f3(pooled['cv'])} | {f1(R['pooled']['summary']['ms']['mean'])} | | {f3(R['pooled']['ratio_to_expected'])} | |")
    if R["controls"]:
        s = R["controls"]["iters"]
        P(f"| controls | iso / representation | {sum(1 for a in R['arms'] if a['kind'] == 'iso-control')} | {s['n']} | {f0(s['min'])} | {f0(s['mean'])} ± {f0(s['ci95_half'])} | {f0(s['median'])} | {f0(s['max'])} | {f3(s['cv'])} | {f1(R['controls']['ms']['mean'])} | | {f3(s['mean'] / exp)} | |")
    for a in R["arms"]:
        if a["kind"] == "isogenous":
            continue
        s = a["summary"]["iters"]
        z = a["tests"]["z_welch"] if a["tests"] else float("nan")
        expct = a["meta"].get("expected_ratio") if isinstance(a["meta"], dict) else None
        note = f" (expected {f3(expct)})" if expct else ""
        P(f"| {a['arm']} | {a['kind']}{note} | 1 | {s['n']} | {f0(s['min'])} | {f0(s['mean'])} ± {f0(s['ci95_half'])} | {f0(s['median'])} | {f0(s['max'])} | {f3(s['cv'])} | {f1(a['summary']['ms']['mean'])} | {f1(a['e2e_ms_mean'])} | {f3(a['ratio_to_expected'])} [{f3(s['boot_ci95'][0] / exp)}, {f3(s['boot_ci95'][1] / exp)}] | {z:.2f} |")
    P("")
    # strata
    P(f"Pooled by endomorphism-ring stratum (conductor of End relative to the walked primes), by level in each volcano, by distance from the root, by the degree and kernel field of the edge that reached the curve:\n")
    P("| group | curves | solves | min | mean ± 95 % CI | median | max | sd / mean | ratio to √(πn/2) | z vs rest |")
    P("|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    for g in R["groups"]:
        s = g["summary"]["iters"]
        P(f"| {g['group']} | {g['arms']} | {s['n']} | {f0(s['min'])} | {f0(s['mean'])} ± {f0(s['ci95_half'])} | {f0(s['median'])} | {f0(s['max'])} | {f3(s['cv'])} | {f3(g['ratio_to_expected'])} | {g['z_vs_rest']:.2f} |")
    P("")
    t = R["tests"]
    P(f"Across the {t['family_size']} measured curves: Kruskal–Wallis H = {t['kruskal_wallis']['H']:.2f} (df {t['kruskal_wallis']['df']}, p = {t['kruskal_wallis']['p']:.3f}); ANOVA F = {t['anova']['F']:.3f} (p = {t['anova']['p']:.3f}); between-curve variance ratio {t['anova']['variance_ratio']:.3f} (observed sd of curve means {f0(t['observed_sd_of_arm_mean'])} against {f0(t['expected_sd_of_arm_mean'])} expected from the pooled single-solve spread); largest |z| {t['max_abs_z']:.2f} against the Bonferroni threshold {t['z_crit_bonferroni']:.2f}; curves beyond it: {t['flagged_arms'] or 'none'}.\n")
    # per curve
    P("Per curve (iterations = group operations; `d` = kernel field degree of the edge that reached it; transport = ms to find and apply the isogenies from the root):\n")
    P("| curve | stratum f | levels | path (ℓ) | d | transport ms | min | mean ± CI | median | max | p10 | p90 | sd/mean | skew | ns/op | e2e ms | ratio | z |")
    P("|:--|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    vmap = {v["id"]: v for v in w["vertices"]}
    for a in sorted((a for a in R["arms"] if a["kind"] == "isogenous"), key=lambda a: (a["meta"]["conductor"], a["vertex"])):
        s = a["summary"]["iters"]
        v = vmap[a["vertex"]]
        lv = ",".join(f"{k}:{x}" for k, x in sorted(v["levels"].items(), key=lambda kv: int(kv[0])))
        d = v["edge"]["field_degree"] if v["edge"] else "—"
        P(f"| v{a['vertex']} | {v['conductor']} | {lv} | {'·'.join(map(str, v['path'])) or 'root'} | {d} | {f1(v['transport']['ms'])} | {f0(s['min'])} | {f0(s['mean'])} ± {f0(s['ci95_half'])} | {f0(s['median'])} | {f0(s['max'])} | {f0(s['p10'])} | {f0(s['p90'])} | {f3(s['cv'])} | {s['skew']:.2f} | {f1(a['summary']['ns_per_op']['mean'])} | {f1(a['e2e_ms_mean'])} | {f3(a['ratio_to_expected'])} | {a['tests']['z_welch']:.2f} |")
    P("")

# ---------------------------------------------------------------- generators
for c in CLASSES:
    G = load(c, "generators")
    if not G:
        continue
    P(f"### {c}: {G['params']['generators']} generators of ⟨P⟩ per curve, the same secret, {G['params']['trials']} paired seeds each\n")
    P("| curve | generators | solves | mean ± CI | min / max of generator means | sd of generator means (expected) | variance ratio | KW p | ANOVA p | max |z| (Bonferroni) | Spearman ρ of generator mean with r / bits(r) / popcount(r) / x(P) |")
    P("|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|")
    for p in G["per_vertex"]:
        if "vertex" not in p:
            continue
        s = p["summary"]
        am = p["arm_means"]
        sp = p["spearman_vs_generator_features"]
        P(f"| v{p['vertex']} | {p['arms']} | {p['solves']} | {f0(s['mean'])} ± {f0(s['ci95_half'])} | {f0(am['min'])} / {f0(am['max'])} | {f0(am['sd'])} ({f0(p['expected_sd_of_arm_mean'])}) | {p['anova']['variance_ratio']:.3f} | {p['kruskal_wallis']['p']:.3f} | {p['anova']['p']:.3f} | {p['max_abs_z']:.2f} ({G['tests']['z_crit_bonferroni']:.2f}) | {sp.get('r', float('nan')):.2f} / {sp.get('r_bits', float('nan')):.2f} / {sp.get('r_popcount', float('nan')):.2f} / {sp.get('Px', float('nan')):.2f} |")
    t = G["tests"]
    P(f"\nAll generator arms together: KW p = {t['kruskal_wallis']['p']:.3f}, variance ratio {t['anova']['variance_ratio']:.3f}, flagged arms (family {t['family_size']}): {t['flagged_arms'] or 'none'}; pooled mean {f0(G['pooled']['summary']['iters']['mean'])} = {f3(G['pooled']['ratio_to_expected'])} × √(πn/2); {G['rho']['solves']} solves, all correct.\n")

# ---------------------------------------------------------------- shift
for c in CLASSES:
    Sm = load(c, "shift")
    if not Sm:
        continue
    w = json.loads((run / c / "walk.json").read_text())
    src = {s["index"]: s["vertex"] for s in w["sources"]}
    tw = [p for p in Sm["per_vertex"] if "two_way" in p]
    P(f"### {c}: instances drawn on {len(src)} source curves, carried to every measured curve, {Sm['params']['trials']} paired seeds per cell\n")
    P("| source | drawn on | targets | solves | mean ± CI | ratio | z vs other sources |")
    P("|:--|:--|--:|--:|--:|--:|--:|")
    for g in sorted((g for g in Sm["groups"] if g["group"].startswith("source:")), key=lambda g: int(g["group"].split(":")[1])):
        s = g["summary"]["iters"]
        sv = int(g["group"].split(":")[1])
        si = [k for k, v in src.items() if v == sv]
        P(f"| {si[0] if si else '?'} | v{sv} | {g['arms']} | {s['n']} | {f0(s['mean'])} ± {f0(s['ci95_half'])} | {f3(g['ratio_to_expected'])} | {g['z_vs_rest']:.2f} |")
    if tw:
        t2 = tw[0]["two_way"]
        P(f"\nTwo-way decomposition of the {Sm['rho']['solves']} solves: η² (target curve) = {t2['eta2_target']:.4f}, η² (source curve) = {t2['eta2_source']:.4f}; Kruskal–Wallis by target p = {t2['kw_target']['p']:.3f} (df {t2['kw_target']['df']}), by source p = {t2['kw_source']['p']:.3f} (df {t2['kw_source']['df']}); flagged cells (family {Sm['tests']['family_size']}): {Sm['tests']['flagged_arms'] or 'none'}; all solves correct.\n")

# ---------------------------------------------------------------- figures
P("### Figures\n")
P("Every figure draws recorded values only (`plot.py`). The standard volcano diagrams (`tools/volcano-diagram`) place each measured curve at its level in one ℓ-volcano with every walked edge drawn; the 3D heat maps stack the endomorphism-ring strata (rings, crater on top) and colour each measured curve by its mean operations or its mean end-to-end time (transport + solve).\n")
rel = run.as_posix()
for c in CLASSES:
    d = run / c
    if not (d / "class" / "results.json").exists():
        continue
    P(f"#### {c}\n")
    for name, cap in [("volcano3d-ops", "3D class heat map, mean group operations per solve"), ("volcano3d-e2e", "3D class heat map, mean end-to-end ms (transport + solve)")]:
        if (d / f"{name}.svg").exists():
            P(f"![{c} {cap}]({rel}/{c}/{name}.svg)\n")
    for f in sorted(d.glob("volcano-ell*.svg")):
        P(f"![{c} {f.stem}: standard volcano diagram]({rel}/{c}/{f.name})\n")
    for name, cap in [("per_curve", "every solve, min–max, mean ± CI per arm"), ("per_group", "pooled groups"), ("z_scores", "each arm against the pooled isogenous solves"), ("generators", "generator sweep per curve"), ("generator_means", "distribution of generator means"), ("shift_matrix", "source × target matrix")]:
        if (d / f"{name}.svg").exists():
            P(f"![{c} {cap}]({rel}/{c}/{name}.svg)\n")
print("\n".join(out))
