"""Plot recorded summaries; never launches or implements a cryptographic algorithm."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, PageBreak

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "benchmarks/20261114-hec-anchor-forest"
OUT.mkdir(parents=True, exist_ok=True)
STUDY = ROOT / "research/hec_anchor_relations_20261114"
orbit = json.loads((STUDY / "run/orbit-summary.json").read_text())
anchor = json.loads((STUDY / "run/summary.json").read_text())
principal = json.loads((ROOT / "research/hec_relation_math_20261114/pilot/summary.json").read_text())
fig, ax = plt.subplots(figsize=(10, 4.2))
x = list(range(len(orbit)))
ax.plot(x, [r["candidate_speedup"] for r in principal], "o-", label="Unrestricted principal / prior native (cross-run)")
ax.plot(x, [r["vs_old_table"] for r in anchor], "s-", label="First anchor / prior table (cross-run)")
ax.plot(x, [r["ratio"] for r in orbit], "D-", label="Signed orbit / paired frozen table")
ax.axhline(1, color="#444444", linewidth=1)
ax.axhline(10, color="#a33a27", linestyle="--", label="10x objective")
ax.set_yscale("log")
ax.set_ylim(0.02, 15)
ax.set_xticks(x, [f'g{r["g"]}\np={r["p"]}' for r in orbit])
ax.set_ylabel("Reference median / candidate median (higher is faster)")
ax.set_title("Cold-cost exploratory pilots; separate experiments, L0 macOS")
ax.grid(axis="y", alpha=.2)
ax.legend(fontsize=8, loc="upper left")
fig.tight_layout()
fig.savefig(OUT / "pilots.svg")
fig.savefig(OUT / "pilots.png", dpi=160)

styles = getSampleStyleSheet()
styles["BodyText"].fontSize = 9
styles["BodyText"].leading = 13
story = []
def p(text):
    story.append(Paragraph(text, styles["BodyText"]))
    story.append(Spacer(1, 7))
story.append(Paragraph("Hyperelliptic principal-relation pilots", styles["Title"]))
p("2026-11-15 UTC | genus 2, 3 and 4 | source freeze 88a969080d | producer assertions [pcc:unverified]")
p("Signed-orbit reuse, coefficient-aware dense forest elimination and incremental anchor solving lower cold total cost against the frozen table controls at every pilot size. The observed median ratios range from 1.27x to 2.94x. This is an exploratory engineering and algebraic-reuse result; mathematical novelty and the 10x objective are unestablished.")
story.append(Image(str(OUT / "pilots.png"), width=6.9*inch, height=2.90*inch))
p("Each point is a ratio of five-seed medians within its own experiment. Earlier principal and first-anchor controls are cross-run comparisons, not paired with the signed-orbit experiment. Do not multiply ratios between experiments. Local L0 timings include host contention and have no isolation-based uncertainty guarantee.")
rows = [["Genus", "p", "Table (ms)", "Orbit (ms)", "Ratio"]]
for r in orbit:
    arms = {a["arm"]:a for a in r["arms"]}
    rows.append([r["g"], r["p"], f'{arms["table"]["median_ns"]/1e6:.4f}', f'{arms["orbit"]["median_ns"]/1e6:.4f}', f'{r["ratio"]:.2f}x'])
t = Table(rows, colWidths=[.7*inch,.9*inch,1.4*inch,1.4*inch,1.0*inch])
t.setStyle(TableStyle([("BACKGROUND",(0,0),(-1,0),colors.HexColor("#183a4d")),("TEXTCOLOR",(0,0),(-1,0),colors.white),("FONTSIZE",(0,0),(-1,-1),9),("BOTTOMPADDING",(0,0),(-1,-1),6),("ROWBACKGROUNDS",(0,1),(-1,-1),[colors.HexColor("#eef4f6"),colors.white])]))
story.append(t)
story.append(PageBreak())
story.append(Paragraph("Method, checks and limits", styles["Title"]))
p("Identity: interpolate h through g+1 signed rational places, A=product(x-x_i), and Q=(f-h^2)/A. Splitting Q yields a principal relation of at most 2g+1 terms. Restrict inputs to target-seeded anchors and admit at most two off-anchor places. Signed/scaled affine union-find eliminates those endpoints; the small anchor system determines the target scalar.")
p("For one tuple, h_i=(y_i/A'(x_i)) A/(x-x_i). Flipping sign j updates Q by 4 sum(i != j) sigma_i sigma_j h_i h_j/A. Cached cross terms have degree at most g-1. Gray order shares one norm division across 2^g sign classes. This is exact algebraic reuse of known interpolation mathematics, not a novelty claim.")
p("Included costs: cold square/cube/projective lookup tables as needed, original Horner catalogue, anchor selection, batch inverse differences, shuffled genus-2 tuple family, both target descents, all attempted residual factorizations, forest updates, row dedup where specified, incremental solve and final group verification. The pilot excludes instance selection and construction of the known-answer target equally in both arms. No setup is shared between solves.")
p("Correctness: signed-orbit gate completed 55 cases with 15,115 raw principal identities and 1,290 projected anchor identities checked. Timed run completed 110 solves with matching known-answer scalars. Separate unit tests compare 5,600 orbit residuals with fresh Newton interpolation, test arbitrary forest coefficients and compare incremental target determination with sparse elimination at every planted-system prefix. These are local producer self-checks, not signed independent replay.")
p("Preceding evidence: unrestricted principal pilot completed 55 gated and 330 paired solves but lost to the strongest native controls. Exact cubic/quartic tables were checked on 50,624 exhaustive and 48,000 constructed/random fixtures; table pipeline completed 220 stream/pivot gates and 660 solves. First anchor pilot completed 55 gated and 55 timed solves but gave weak or negative total gains. All raw data remain separate.")
p("Formal scope: Algebra.lean covers projective/Cardano/Ferrari polynomial identities. Forest.lean covers affine substitution, root attachment and balanced/unbalanced cycle algebra. Neither is a proof of the Rust implementation or of its running time. Lean version, mathlib commit and command output are retained in the table study run directory.")
p("Sources checked: GTTD draft, section 3.1 (cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf), on signed cycles; Magma handbook geometric/principal-divisor IC (magma.maths.usyd.edu.au/magma/handbook/text/1473); El Housni, ePrint 2026/392, on torus cube roots. Full assumptions and KB receipts are in the study protocols and derivation. Novelty is not established.")
p("Limits and next test: eleven existing instances, five seeds per instance, one macOS ARM host, no held-out growing-size confirmation or independent replay. External implementation comparison is not run; no fastest-tool or asymptotic claim is made. Reported storage is explicit arrays, not peak RSS. Preserve all rank/cap failures. Confirm frozen arms on held-out curve families and investigate relation cost before any 10x claim.")
p("Reproduction: research/hec_anchor_relations_20261114/{PROTOCOL.md,orbit_bench.rs,anchor.rs,orbit.rs,dense.rs,run/orbit-gates.txt,run/orbit-paired.txt,run/orbit-summary.json}. Earlier studies: research/hec_relation_math_20261114 and research/hec_table_oracle_20261114. This report is derived from recorded summaries; raw records are authoritative.")
SimpleDocTemplate(str(OUT/"REPORT.pdf"), pagesize=(612,792),rightMargin=42,leftMargin=42,topMargin=35,bottomMargin=35).build(story)
