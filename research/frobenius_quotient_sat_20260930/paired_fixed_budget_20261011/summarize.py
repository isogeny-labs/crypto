#!/usr/bin/env python3
"""Build RESULTS.md, the figures (SVG) and report.pdf from a receipts file.

    python3 summarize.py --receipts results/paired_n13.jsonl [--tag n13]

Everything numeric in RESULTS.md is computed here from the receipts, the frozen
manifest and rank.py; the hypothesis verdict is a rule applied to those numbers
(PROTOCOL.md "Hypotheses"), not prose written after the fact.  PDF renderer:
matplotlib PdfPages (no typst / LaTeX / wkhtmltopdf is installed).
"""
import argparse
import collections
import json
import os
import statistics
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.backends.backend_pdf import PdfPages  # noqa: E402
from matplotlib.patches import FancyBboxPatch  # noqa: E402

import paired_common as pc  # noqa: E402
import rank as rk  # noqa: E402

plt.rcParams.update({"svg.fonttype": "none", "svg.hashsalt": "paired-fixed-budget",
                     "pdf.fonttype": 42, "font.family": "DejaVu Sans",
                     "axes.spines.top": False, "axes.spines.right": False})
COLOR = {"A": "#1f6fb2", "B": "#d9741c"}
MARK = {"verified": "o", "unsat": "v", "timeout": "X"}
HERE = pc.HERE
ARM_NAME = {"A": "A  representatives only", "B": "B  one-hot Frobenius phases"}


def fmt(x, nd=1):
    return "n/a" if x is None else f"{x:.{nd}f}"


def load(path):
    hdr, atts = rk.load_attempts(path)
    return hdr, rk.last_per_key(atts)


def sealed_flags(sealed):
    return {tid: t["brute_force"] for tid, t in sealed["targets"].items()}


# ---------------------------------------------------------------- tables
def table(header, rows):
    out = ["| " + " | ".join(header) + " |", "|" + "|".join("---" for _ in header) + "|"]
    out += ["| " + " | ".join(str(c) for c in r) + " |" for r in rows]
    return "\n".join(out)


def headline_rows(summary, forms, scope_names):
    rows = []
    for scope in forms + ["pooled"]:
        for arm in "AB":
            d = summary.get(f"{arm}|{scope}")
            if not d:
                continue
            rows.append([scope_names.get(scope, scope), arm, d["attempts"], d["verified"],
                         d["unsat"], d["timeout"], d["skipped"], d["rejected"],
                         f"{d['rank']} / {d['columns']}", d["dependent_verified"],
                         fmt(d["wall_s"]), fmt(d["cpu_s"]),
                         fmt(d["wall_s_per_independent"]), fmt(d["cpu_s_per_independent"]),
                         fmt(d["verified_per_attempt"], 3), fmt(d["wall_s_per_attempt"])])
    return rows


def set_rows(atts, forms):
    rows = []
    for form in forms:
        for key in "abc":
            for arm in "AB":
                sel = [a for a in atts if a["form"] == form and a["set"] == key
                       and a["arm"] == arm and a["status"] != "skipped"]
                if not sel:
                    continue
                cnt = collections.Counter(a["status"] for a in sel)
                rows.append([form, key, arm, f"{cnt['verified']}/{len(sel)}", cnt["unsat"],
                             cnt["timeout"], fmt(sum(a["total_cpu_s"] for a in sel)),
                             fmt(statistics.median(a["total_cpu_s"] for a in sel))])
    return rows


def pair_rows(atts, forms):
    by = {(a["target_id"], a["arm"], a["form"]): a for a in atts}
    rows = []
    for form in forms:
        for tid in sorted({a["target_id"] for a in atts}):
            cells = [form, tid]
            for arm in "AB":
                a = by.get((tid, arm, form))
                cells += ["-", "-"] if a is None else [a["status"], fmt(a["total_cpu_s"])]
            a, b = by.get((tid, "A", form)), by.get((tid, "B", form))
            if a and b and a["status"] == b["status"] == "verified":
                cells.append(f"{b['total_cpu_s'] / a['total_cpu_s']:.2f}")
            else:
                cells.append("-")
            rows.append(cells)
    return rows


def hit_rows(summary, scopes):
    rows = []
    for scope in scopes:
        for arm in "AB":
            d = summary.get(f"{arm}|{scope}")
            if not d:
                continue
            for k, v in d["first_hit_rank"].items():
                rows.append([scope, arm, k, v["attempts"], fmt(v["wall_s"]), fmt(v["cpu_s"])])
    return rows


def relation_rows(rels, forms):
    rows = []
    for rel in rels:
        terms = " ".join(f"{'+' if t['sign'] > 0 else '-'}pi^{t['phase']}(V{t['rep']})"
                         for t in rel["terms"])
        rows.append([rel["arm"], rel["form"], rel["target_id"], terms,
                     "[" + ",".join(str(c) for c in rel["coeffs"]) + "]",
                     "yes" if rel["checks"]["all"] else "NO",
                     rel["rank_after_form_scope"]])
    return rows


def verdict(summary, atts, forms, rels_bad):
    """PROTOCOL.md rule: H1 supported only if, per formulation, arm B's seconds
    per newly independent relation are below arm A's on both clocks and every
    relation passes."""
    lines = []
    ok_all = True
    for form in forms:
        a, b = summary.get(f"A|{form}"), summary.get(f"B|{form}")
        if not a or not b or not a["rank"] or not b["rank"]:
            lines.append(f"- `{form}`: undefined (an arm has no independent relation).")
            ok_all = False
            continue
        w = b["wall_s_per_independent"] < a["wall_s_per_independent"]
        c = b["cpu_s_per_independent"] < a["cpu_s_per_independent"]
        ok_all = ok_all and w and c
        lines.append(f"- `{form}`: B / A seconds per newly independent relation = "
                     f"{b['wall_s_per_independent'] / a['wall_s_per_independent']:.2f} (wall), "
                     f"{b['cpu_s_per_independent'] / a['cpu_s_per_independent']:.2f} (CPU); "
                     f"B below A on wall: {'yes' if w else 'no'}, on CPU: {'yes' if c else 'no'}.")
    by = {(x["target_id"], x["arm"], x["form"]): x for x in atts}
    for form in forms:
        gain = [t for (t, arm, f), x in by.items()
                if f == form and arm == "A" and x["status"] == "unsat"
                and by.get((t, "B", form), {}).get("status") == "verified"]
        lines.append(f"- `{form}`: targets that arm A proved UNSAT and arm B decomposed: "
                     f"{len(gain)} ({', '.join(sorted(gain)) or 'none'}).")
    supported = ok_all and rels_bad == 0
    head = ("H1 criterion (PROTOCOL.md) **met** on this sample." if supported else
            "H1 criterion (PROTOCOL.md) **not met** on this sample "
            "(H0 is not rejected)." if rels_bad == 0 else
            "A relation failed verification: the table is void until it is explained.")
    return head, "\n".join(lines)


# ---------------------------------------------------------------- figures
def figure_results(atts, summary, forms, n_cols, title):
    fig, axs = plt.subplots(1, 3, figsize=(15.5, 4.9), gridspec_kw={"width_ratios": [1.35, 1.0, 1.1]})
    ax = axs[0]
    groups = [(f, arm) for f in forms for arm in "AB"]
    rng = [(-0.18, -0.06, 0.06, 0.18)]
    for gi, (form, arm) in enumerate(groups):
        sel = [a for a in atts if a["form"] == form and a["arm"] == arm and a["status"] != "skipped"]
        sel.sort(key=lambda a: a["target_id"])
        for i, a in enumerate(sel):
            xj = gi + (i - (len(sel) - 1) / 2) * 0.55 / max(len(sel) - 1, 1)
            ax.scatter(xj, a["total_wall_s"], marker=MARK[a["status"]], s=46,
                       color=COLOR[arm], edgecolor="black", linewidth=0.5, zorder=3)
        if sel:
            med = statistics.median(a["total_wall_s"] for a in sel)
            ax.hlines(med, gi - 0.33, gi + 0.33, color=COLOR[arm], linewidth=2, zorder=2)
    ax.axhline(120, color="grey", linestyle="--", linewidth=1)
    ax.text(len(groups) - 0.55, 124, "120 s CPU budget", ha="right", va="bottom", fontsize=8, color="grey")
    ax.set_yscale("log")
    ax.set_xticks(range(len(groups)))
    ax.set_xticklabels([f"{arm}\n{form.replace('_', chr(10))}" for form, arm in groups], fontsize=8)
    ax.set_ylabel("wall seconds per attempt (log)")
    ax.set_title("Every attempt, per target", fontsize=10, loc="left")
    for st, m in MARK.items():
        ax.scatter([], [], marker=m, color="white", edgecolor="black", label=st)
    ax.legend(fontsize=7, loc="lower right", frameon=False, title="outcome", title_fontsize=7)

    ax = axs[1]
    scopes = forms + ["pooled"]
    w = 0.38
    for si, sc in enumerate(scopes):
        for ai, arm in enumerate("AB"):
            d = summary.get(f"{arm}|{sc}")
            if not d or d["wall_s_per_independent"] is None:
                if d:
                    ax.text(si + (ai - 0.5) * w, 1, f"{arm}\nrank 0", ha="center", fontsize=7)
                continue
            ax.bar(si + (ai - 0.5) * w, d["wall_s_per_independent"], w * 0.92, color=COLOR[arm])
            ax.text(si + (ai - 0.5) * w, d["wall_s_per_independent"] * 1.02,
                    f"{d['wall_s_per_independent']:.0f}\nrk {d['rank']}/{n_cols}\n{d['attempts']} att",
                    ha="center", va="bottom", fontsize=6.5)
    ax.set_xticks(range(len(scopes)))
    ax.set_xticklabels([s.replace("_", "\n") for s in scopes], fontsize=8)
    ax.set_ylabel("wall s per newly independent relation")
    ax.set_title("All attempts charged", fontsize=10, loc="left")
    top = max([d["wall_s_per_independent"] or 0 for d in summary.values()] + [1])
    ax.set_ylim(0, top * 1.35)

    ax = axs[2]
    styles = {forms[0]: "-", "pooled": ":"}
    if len(forms) > 1:
        styles[forms[1]] = "--"
    for sc in scopes:
        for arm in "AB":
            d = summary.get(f"{arm}|{sc}")
            if not d:
                continue
            xs, ys = [0.0], [0]
            for k, v in sorted(d["first_hit_rank"].items(), key=lambda kv: int(kv[0])):
                xs += [v["wall_s"], v["wall_s"]]
                ys += [ys[-1], int(k)]
            xs.append(d["wall_s"])
            ys.append(ys[-1])
            ax.plot(xs, ys, styles.get(sc, "-"), color=COLOR[arm], linewidth=1.8,
                    label=f"{arm} {sc}")
    ax.set_xlabel("cumulative wall seconds, all of the arm's attempts")
    ax.set_ylabel(f"independent relations (cap {n_cols})")
    ax.set_title("Rank against cost charged", fontsize=10, loc="left")
    ax.set_ylim(-0.1, n_cols + 0.4)
    ax.legend(fontsize=7, frameon=False, loc="lower right")
    fig.suptitle(title, fontsize=11, x=0.01, ha="left")
    fig.tight_layout(rect=(0, 0, 1, 0.95))
    return fig


def figure_mechanism(n, n_cols):
    fig, ax = plt.subplots(figsize=(11, 4.6))
    ax.axis("off")
    ax.set_xlim(0, 22)
    ax.set_ylim(0, 9.6)

    def box(x, y, w, h, text, fc, fs=8.5, ec="#333333"):
        ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.05,rounding_size=0.15",
                                    fc=fc, ec=ec, linewidth=1))
        ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=fs)

    def arrow(x0, y0, x1, y1):
        ax.annotate("", xy=(x1, y1), xytext=(x0, y0), arrowprops=dict(arrowstyle="->", lw=1.1))

    ax.text(0.1, 9.2, "Mechanism claimed for phase-aware summation, and what is measured here",
            fontsize=10.5, fontweight="bold")
    for lane, (y, arm) in enumerate(((5.2, "A"), (0.9, "B"))):
        c = "#dbe9f6" if arm == "A" else "#fbe6d3"
        ax.text(0.1, y + 3.1, ARM_NAME[arm], fontsize=9.5, color=COLOR[arm], fontweight="bold")
        box(0.1, y + 0.6, 2.6, 1.6, "target R\n(fixed, frozen)", "#eeeeee")
        if arm == "A":
            mid = "4 summands, x_i = v_i\nv_i in a 4-dim span\n(%d valid reps)" % n_cols
            eff = "%d admissible x\n(effective factor base)" % n_cols
        else:
            mid = ("4 summands, x_i = v_i^(2^k_i)\none-hot phase e_i (%d bits each)\n"
                   "adds %d AND gates per bit of x" % (n, n))
            eff = "%d admissible x\n(%d reps x %d phases)" % (n_cols * n, n_cols, n)
        box(3.7, y + 0.4, 5.4, 2.0, mid, c)
        box(10.1, y + 0.4, 3.9, 2.0, "S3 chain + line/final\nequation, CryptoMiniSat\n1 thread, 120 s CPU", "#f3f3f3")
        box(15.0, y + 0.4, 3.0, 2.0, eff, c)
        box(18.9, y + 0.4, 3.0, 2.0,
            "relation row over\n%d rep columns\ncoeff +-1" % n_cols if arm == "A" else
            "relation row over\nthe SAME %d columns\ncoeff +-lambda^k mod r" % n_cols, "#e8f3e8")
        arrow(2.8, y + 1.4, 3.6, y + 1.4)
        arrow(9.2, y + 1.4, 10.0, y + 1.4)
        arrow(14.1, y + 1.4, 14.9, y + 1.4)
        arrow(18.1, y + 1.4, 18.8, y + 1.4)
    ax.text(0.1, 0.05, "Measured: seconds per newly independent verified relation (every attempt charged).  "
            "Not measured: end-to-end cost, S, any size beyond n = %d." % n,
            fontsize=8.5, style="italic")
    fig.tight_layout()
    return fig


# ---------------------------------------------------------------- PDF
def md_blocks(text, width=104):
    """Very small markdown -> list of (kind, lines) for monospace pagination."""
    import textwrap
    blocks, lines, i = [], text.split("\n"), 0
    while i < len(lines):
        ln = lines[i]
        if ln.startswith("|"):
            tbl = []
            while i < len(lines) and lines[i].startswith("|"):
                tbl.append(lines[i])
                i += 1
            rows = [[c.strip().replace("**", "").replace("`", "") for c in r.strip("|").split("|")]
                    for r in tbl if not set(r.replace("|", "").strip()) <= set("-: ")]
            ncol = max(len(r) for r in rows)
            rows = [r + [""] * (ncol - len(r)) for r in rows]
            wid = [max(len(r[c]) for r in rows) for c in range(ncol)]
            total = sum(wid) + 3 * ncol
            fs = 7.0 if total <= 112 else 5.6 if total <= 150 else 4.6
            blocks.append(("table", fs, ["   ".join(c.ljust(w) for c, w in zip(r, wid)) for r in rows]))
            continue
        if ln.startswith("#"):
            blocks.append(("h", 10.5 if ln.startswith("# ") else 9, [ln.lstrip("# ").strip()]))
        elif ln.strip() == "":
            blocks.append(("gap", 7, [""]))
        else:
            txt = ln.replace("**", "").replace("`", "")
            ind = "    " if ln.lstrip().startswith(("-", "*")) else ""
            wrapped = textwrap.wrap(txt, width, subsequent_indent=ind) or [""]
            blocks.append(("p", 7.2, wrapped))
        i += 1
    return blocks


def write_pdf(md_text, figures, out_path, title):
    with PdfPages(out_path) as pdf:
        blocks = md_blocks(md_text)
        fig = None
        y = 0
        def newpage():
            f = plt.figure(figsize=(8.5, 11))
            f.text(0.06, 0.975, title, fontsize=7, color="grey")
            return f, 0.95
        fig, y = newpage()
        for kind, fs, lines in blocks:
            lh = fs * 1.55 / 792
            need = lh * len(lines) + (lh if kind == "h" else 0)
            if y - need < 0.04:
                pdf.savefig(fig)
                plt.close(fig)
                fig, y = newpage()
            for ln in lines:
                fig.text(0.06, y, ln, fontsize=fs, family="DejaVu Sans Mono",
                         fontweight="bold" if kind == "h" else "normal", va="top")
                y -= lh
        pdf.savefig(fig)
        plt.close(fig)
        for f in figures:
            pdf.savefig(f)
        d = pdf.infodict()
        d["Title"] = title
        d["Author"] = "Claude Code (research harness output)"


SCALING = """\
## Scaling to m = 51, 53, 83, 131

Nothing in this section is a result of this study. It lists what is already
recorded and what a harness would need. No projection is made.

Known from the parent harness (`../README.md`, `../results/`):

| field | representatives | admissible x with phases | phase model (recorded) | source |
|---|---|---|---|---|
| n = 13, s = 4 | 5 | 65 | about 3.6k variables, 2.5k AND gates; solved at 300 s | `../README.md`, this study |
| n = 19, s = 6 | 22 | 418 | about 7.5k variables, 5.4k AND gates; 0/4 planted at 1200 s for each of two forms | `../README.md` |

- Recorded fact: with phases unknown, none of 8 planted n = 19 attempts finished
  in 1200 s (about 4.9k s total per form), while the n = 13 planted targets
  finished in a median of about 50 s at 300 s. That is a lower bound from
  timeouts, not a fitted rate.
- Recorded fact: representatives only at n = 19 timed out on all 12 completed
  300 s attempts.
- Not known: the variable and gate counts of this encoding at m = 51, 53, 83
  and 131 were not computed here, and the factor-space dimension `s` that makes
  a four-summand decomposition possible at those sizes was not determined.
  `encode.py` could report them (`Model.size()`) without solving; that
  measurement was not run.
- Not known: whether phase-aware encoding helps once the solver can finish at
  all. At n = 13 both arms finish, which is the only regime compared here.

What the harness would need before any claim at those sizes:

1. A native (Rust or C++) circuit builder and runner. `AGENTS.md` requires this
   for new research comparisons; this harness is Python and the m = 83 gate in
   `AGENTS.md` section 8a also needs the frozen m = 83 curve, basis and
   subgroup manifest (`z^83 + z^45 + z^2 + z + 1`, odd part prime).
2. Moduli and subgroup data for n = 51, 53, 83, 131 in the field code
   (`verify_claims.py` carries moduli for 13, 19, 29 and 131 only), plus
   discrete-log machinery that does not tabulate the whole subgroup: the
   brute-force log table used here has `r` entries, which is impossible at
   these sizes. Relation verification would instead be the curve-sum check
   only, or a known-log planted instance.
3. Instances where the phase-aware arm can finish. At n = 19 neither arm did
   within the recorded budgets, so a paired comparison there needs longer
   budgets or a better encoding first (relative or binary phase encodings are
   planned, not implemented, in PR #1630).
4. A relation-collection design with more than one target per factor base:
   rank saturates at the number of orbit columns, so measuring seconds per
   independent relation at scale needs many targets of the form `[a]G + [b]Q`
   against a column count that grows with the factor base.
5. Full-cost accounting under `AGENTS.md` sections 2 and 8: precomputation,
   failed attempts, linear algebra, final scalar recovery, matched rho. This
   study prices one stage on toy fields.
"""


def build_results(args, hdr, atts, summary, rels, vsum, manifest, sealed, forms):
    n_cols = len(manifest["factor_space"]["representatives"])
    n = manifest["curve"]["n"]
    flags = sealed_flags(sealed)
    ntar = len(flags)
    dec_a = sum(f["decomposable_reps_only"] for f in flags.values())
    dec_b = sum(f["decomposable_phases_free"] for f in flags.values())
    bad = sum(d["relations_failed_check"] for d in summary.values() if True)
    head, vlines = verdict(summary, atts, forms, vsum["rank_summary_relations_failed_check"]
                           + sum(1 for r in rels if not r["checks"]["all"]))
    scope_names = {f: f for f in forms}
    scope_names["pooled"] = "both forms pooled"
    skipped = [a for a in atts if a["status"] == "skipped"]
    spent = sum(a["total_cpu_s"] for a in atts if a["status"] != "skipped")
    pa, pb = summary.get("A|pooled"), summary.get("B|pooled")
    hdr_rows = headline_rows(summary, forms, scope_names)
    t_head = table(["scope", "arm", "attempts", "verified", "UNSAT", "timeout", "skipped",
                    "rejected tuples", "rank", "dependent verified", "wall s", "CPU s",
                    "wall s / new indep.", "CPU s / new indep.", "verified / attempt",
                    "wall s / attempt"], hdr_rows)
    md = f"""\
# Phase-aware Frobenius summation: paired fixed-budget comparison, n = {n}

Stage diagnostic on a toy field. `S`, end-to-end cost and speedup are unset.
No ECDLP speedup is claimed, and nothing here transfers to another size.
Protocol: [`PROTOCOL.md`](PROTOCOL.md) (registered 2026-10-11, addendum 2026-10-26).
Related draft PR #1630 (`PHASE_EXPERIMENTS.md`) asks for exactly this
controlled comparison; this directory does not depend on its branch.

## What ran

- Curve `y^2 + xy = x^3 + 1` over `F_(2^{n})`, odd subgroup order r = {manifest['curve']['odd_subgroup_order']}
  (prime), lambda = {args['lambda']} (root of `lambda^2 + lambda + 2` mod r with `pi(G) = [lambda]G`).
- Factor space: seeded {manifest['factor_space']['s']}-dimensional subspace of `ker Tr` with
  {n_cols} representatives ({manifest['factor_space']['admissible_x_phases_free']} admissible x once
  Frobenius phases are free). Basis `{', '.join(manifest['factor_space']['basis'])}`; the same space as `../README.md`.
- {ntar} frozen targets ({', '.join(f"{k}: {len(v['targets'])}" for k, v in manifest['target_sets'].items())});
  manifest SHA-256 `{hdr['manifest_sha256']}`.
- Arms: **A** representatives only; **B** one-hot Frobenius phases. Formulations: {', '.join(f'`{f}`' for f in forms)}.
  Per attempt: {hdr['budget_cpu_s']:g} s solver CPU, one CryptoMiniSat thread
  (pycryptosat {hdr['pycryptosat']}), one process, ABBA order, fresh solver per attempt.
- Host: {hdr['cpu_model']}, {hdr['cpu_count']} cores shared with other jobs, {hdr['platform']}, Python {hdr['python']}.
  Load average at start {hdr['loadavg_start'][0]:.2f}. Wall seconds include any contention;
  CPU seconds are the solver clock.
- Command: `{hdr['command']}`
- Attempts run: {sum(1 for a in atts if a['status'] != 'skipped')} of {len(atts)} scheduled;
  skipped by the CPU cap: {len(skipped)}. Total attempt CPU: {spent:.0f} s ({spent / 60:.1f} min).
- Audit: `verify.py` {'PASSED' if vsum['pass'] else 'FAILED'}: {vsum['relations_checked']} relations checked on the curve,
  {vsum['unsat_checked_against_brute_force']} UNSAT answers checked against exhaustive pair matching,
  {len(vsum['problems'])} problems.{(' Notes: ' + '; '.join(vsum['notes'])) if vsum['notes'] else ''}

## Structural fact (brute force, no solver)

Of the {ntar} frozen targets, **{dec_a}** are decomposable as a signed sum of four distinct
representatives (arm A's search space) and **{dec_b}** as a signed sum of four distinct admissible
x-coordinates once phases are free (arm B's). Source: `witness_sealed.json`, read only by `verify.py`.
Arm B's effective factor base is **{n}x larger** than arm A's ({n_cols * n} against {n_cols} x-coordinates)
while the relation matrix keeps the same {n_cols} columns. That is the claimed mechanism: equal linear
algebra, larger decomposable set. It is also why arm A can only ever return relations on the phase-0
planted targets here, and why a comparison that only counts solved targets favours arm B before any
timing is read.

## Headline: seconds per newly independent verified relation

Every attempt of the arm is charged (verified, UNSAT, timeout). "New independent" is incremental rank of the
coefficient rows over Z/{manifest['curve']['odd_subgroup_order']} in receipt order; rank cannot exceed {n_cols}.

{t_head}

{head}

{vlines}

Rank is capped at {n_cols}, so attempts made after an arm reaches full rank still add cost and no rank;
the table below gives the cost charged when each rank was first reached.

## Cost charged when each rank was first reached

{table(['scope', 'arm', 'rank reached', 'attempts charged', 'wall s', 'CPU s'], hit_rows(summary, forms + ['pooled']))}

## By target set

Set a: planted with random Frobenius phases. b: planted with all phases 0. c: uniform random in the odd subgroup.
CPU s are totals over the attempts shown; the last column is the median per attempt.

{table(['form', 'set', 'arm', 'verified / attempts', 'UNSAT', 'timeout', 'CPU s total', 'CPU s median'], set_rows(atts, forms))}

## Paired attempts, same target and formulation

Last column: B / A total CPU where both arms verified.

{table(['form', 'target', 'A status', 'A CPU s', 'B status', 'B CPU s', 'B / A CPU'], pair_rows(atts, forms))}

## Verified relations

Each row is a relation `log R = sum c_j log V_j` over the orbit representatives with `c_j` the sum of
`+-lambda^k` over the summands using representative j. Checks: terms recomputed on the curve, signed sum equals the
target, coefficients equal the lambda-power sums, and the log identity holds mod r (exhaustive log table).

{table(['arm', 'form', 'target', 'terms', 'coefficients mod r', 'all checks', 'rank after (form scope)'], relation_rows(rels, forms))}

## Limits of this evidence

- One timing trial per cell: no repeat of any attempt. Solver runs are deterministic given the clause order, so
  repetition would vary mainly the CPU clock; run-to-run spread was not measured.
- {ntar} targets, {len(forms)} formulation(s), one factor space, one field. The 4 + 4 + 4 target design makes arm A
  unable to produce a relation on 8 of 12 targets by construction (structural fact above).
- The relation matrix has {n_cols} columns, so "independent relations" saturates at {n_cols}; the per-independent-relation
  figures are therefore dominated by how many attempts each arm spends before and after saturation. Both are shown.
- The targets carry no `[a]G + [b]Q` structure, so no unknown-`Q` column is modelled (PROTOCOL.md addendum 3).
- Wall time on a shared host. CPU seconds are the stated budget clock. Neither prices linear algebra, the factor-base
  build, or scalar recovery; this is a stage diagnostic, not `S`, and no rho comparison was run.
- The hypothesis rule above is applied to this sample only; with these counts it is a description, not an inference.
- The harness is Python (legacy tooling under `AGENTS.md`); the SAT backend is native CryptoMiniSat. A native rerun is a
  prerequisite for any performance claim.

## Graphs checked

`docs/index-calculus-scoreboard.html`, `docs/ic/progress-timeline.json`, `docs/ic/leaderboard.json`: unchanged. This is a
stage diagnostic with no whole-pipeline `S`, no ratio to a boundary, and no matched rho, so it adds no point under
`AGENTS.md` section 7a. The study-local figures are new.

## Figures and sources

- `figure_{args['tag']}.svg` (source: `summarize.py`, data: `results/paired_{args['tag']}.jsonl`), `mechanism.svg`.
- Receipts: `results/paired_{args['tag']}.jsonl`; relations: `results/relations_{args['tag']}.jsonl`; rank data:
  `results/rank_{args['tag']}.json`; audit: `results/verify_{args['tag']}.json`.
- Code: `freeze_targets.py`, `run_paired.py`, `rank.py`, `verify.py`, `summarize.py`, `test_paired.py`;
  parent harness `../encode.py`, `../run.py`, `../brute_check.py`.
- Report PDF: `report.pdf`, built with matplotlib PdfPages (no typst, LaTeX, wkhtmltopdf or weasyprint is installed;
  `pandoc` has no PDF engine).

"""
    return md + SCALING


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--receipts", default=os.path.join(HERE, "results", "paired_n13.jsonl"))
    ap.add_argument("--tag", default="n13")
    ap.add_argument("--manifest", default=pc.MANIFEST)
    ap.add_argument("--sealed", default=pc.SEALED)
    ap.add_argument("--no-pdf", action="store_true")
    a = ap.parse_args(argv)
    res = os.path.join(HERE, "results")
    manifest = pc.load_manifest(a.manifest)
    with open(a.sealed) as fh:
        sealed = json.load(fh)
    ctx = pc.Ctx(manifest)
    hdr, atts = load(a.receipts)
    forms = hdr["forms"]
    rels, summary = rk.build(ctx, atts)
    with open(os.path.join(res, f"relations_{a.tag}.jsonl"), "w") as fh:
        for r in rels:
            fh.write(json.dumps(r, sort_keys=True) + "\n")
    with open(os.path.join(res, f"rank_{a.tag}.json"), "w") as fh:
        json.dump({"lambda": ctx.lam, "r": ctx.r, "rep_logs": ctx.rep_logs, "scopes": summary},
                  fh, indent=1, sort_keys=True)
        fh.write("\n")
    vpath = os.path.join(res, f"verify_{a.tag}.json")
    with open(vpath) as fh:
        vsum = json.load(fh)
    info = {"lambda": ctx.lam, "tag": a.tag}
    md = build_results(info, hdr, atts, summary, rels, vsum, manifest, sealed, forms)
    with open(os.path.join(HERE, "RESULTS.md" if a.tag == "n13" else f"RESULTS_{a.tag}.md"), "w") as fh:
        fh.write(md)
    n_cols = len(ctx.reps_x)
    f1 = figure_results(atts, summary, forms, n_cols,
                        f"n = {ctx.n}: arm A (reps only) against arm B (phases), identical frozen targets, "
                        f"{hdr['budget_cpu_s']:g} s CPU budget, one thread")
    f1.savefig(os.path.join(HERE, f"figure_{a.tag}.svg"), format="svg")
    f2 = figure_mechanism(ctx.n, n_cols)
    f2.savefig(os.path.join(HERE, "mechanism.svg"), format="svg")
    if not a.no_pdf:
        write_pdf(md, [f2, f1], os.path.join(HERE, "report.pdf" if a.tag == "n13" else f"report_{a.tag}.pdf"),
                  f"Paired fixed-budget phase comparison, n = {ctx.n}  (stage diagnostic, toy field)")
    plt.close("all")
    print("wrote RESULTS.md, figure_%s.svg, mechanism.svg%s" % (a.tag, "" if a.no_pdf else ", report.pdf"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
