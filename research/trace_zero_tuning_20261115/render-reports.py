"""Render recorded Rust summaries; no new statistics or cryptographic work."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

research = Path(__file__).resolve().parent.parent
for name, arms, title in [
    ('trace_zero_projected_20261115', ['original-eager', 'v1-eager', 'projected-lazy'],
     'Product projection: partial result retained'),
    ('trace_zero_tuning_20261115', ['v1-eager', 'projected-eager', 'tuned-lazy'],
     'Compact trace-zero oracle: controlled comparison')]:
    p = research / name
    s = json.loads((p / 'summary.json').read_text())
    a = json.loads((p / 'grid-audit.json').read_text())
    q = json.loads((p / 'protocol.json').read_text())
    tuned = name.startswith('trace_zero_tuning')
    ids = q['extensions']['primary_cases']
    fig, axes = plt.subplots(1, 2, figsize=(9.3, 3.6), layout='constrained')
    for ax, metric, caption in zip(axes, ['instructions', 'oracle_instructions'],
                                  ['Whole cold process', 'Complete relation oracle']):
        for j, arm in enumerate(arms):
            # Oracle chart fixes eager context across all three algorithms.
            actual = arm.replace('-lazy', '-eager') if metric == 'oracle_instructions' else arm
            cells = [s['cells'][f'{i}/{actual}']['metrics'][metric] for i in ids]
            x = [c['median'] / 1e6 for c in cells]
            err = [[(c['median']-c['min'])/1e6 for c in cells],
                   [(c['max']-c['median'])/1e6 for c in cells]]
            ax.barh([i+(j-1)*.24 for i in range(6)], x, height=.21,
                    label=actual, color=['#526d94', '#cb9851', '#248978'][j],
                    xerr=err, error_kw={'capsize': 2, 'elinewidth': .8})
        ax.set_yticks(range(6), [f'n{n} {k}' for n in [83, 131]
                               for k in ['planted', 'uniform', 'cancel']])
        ax.invert_yaxis()
        ax.set_title(caption)
        ax.set_xlabel('Million simulated instructions (Ir)')
        ax.spines[['top', 'right']].set_visible(False)
        ax.legend(fontsize=7, loc='lower right')
    fig.savefig(p / 'instructions.svg')
    plt.close(fig)
    (p / 'plot-data.json').write_text(json.dumps(
        {'source': 'summary.json', 'cells': s['cells'], 'contrasts': s['contrasts']},
        indent=2)+'\n')
    table = []
    for i in ids:
        label = i.replace('scale-n', '').replace('-d12-', '/').replace('distinct-planted-0', 'planted').replace('cancellation', 'cancel')
        c = [x for x in s['contrasts'] if x['instance'] == i]
        whole = next(x for x in c if x['metric']=='instructions' and x['left']=='v1-eager')
        oracle = next(x for x in c if x['metric']=='oracle_instructions' and x['left']=='v1-eager')
        table.append(f'[{label}], [{whole["left_median"]:,}], [{whole["right_median"]:,}], [{whole["ratio"]:.3f}×], [{oracle["ratio"]:.3f}×],')
    candidate = 'tuned' if tuned else 'projected'
    verdict = 'MET' if a['goal_met'] else 'NOT MET'
    extra = ('''The compact path stores public field values in three words, reuses the existing multiply/reduction kernels, caches byte half-trace maps per query and replaces BigUint coordinate keys with fixed-word keys. Only x-domain lookup is built; original sign indices are preserved. All preparation is charged to the oracle. General field storage and other callers are unchanged.

Nineteen focused release tests pass: exact65-case decisions/indices/common diagnostics across six algorithm/context modes; complete F32 arithmetic, admission and exceptional group branches; wide random independent operands; fixed-word basis/order parity; product-elimination and lazy-symbolic-model equivalence. Root library check also passes. The retained v1 and projected sources execute in the same final binary [pcc:unverified].''' if tuned else '''The first projected study reduces oracle work against v1 by2.334× at n131 and2.463× at n83 on uniform controls. This fails the frozen5× oracle criterion even though both cold-process criteria pass. The entire panel is retained; the next compact-workspace study uses the same criterion and inputs [pcc:unverified].

Eleven focused tests, full-variety10, prior20, terminal2, earlier-linearized3 and root library check pass. These include complete small-field S3 equivalence, inconsistent/unique/deficient projected systems, malformed shared-x signs and lazy symbolic model sharing. The integral certificate covers only the explicitly stated polynomial expansion [pcc:unverified].''')
    best = ''
    if tuned:
        bestrows = []
        for c in s['contrasts']:
            if c['instance'].endswith('uniform') and c['metric']=='oracle_instructions' and c['left']=='projected-eager':
                bestrows.append(f'{c["instance"].split("-")[1]}: {c["ratio"]:.3f}×')
        best = '*Against the immediately preceding projected oracle:* '+', '.join(bestrows)+'. All contrasts and min/median/max values are in summary.json [pcc:unverified].'
    text = '''#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
= TITLE
2026-11-16 · PR3744 · unsigned producer evidence

#image("instructions.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 4pt,
 [Field/case], [v1 eager: whole Ir], [CAND lazy: whole Ir], [Whole ratio], [Oracle ratio],
ROWS
)

*Frozen goal: VERDICT.* Both uniform cells must have at least5× fewer complete-oracle instructions versus v1 with eager context fixed, and at least2× fewer complete cold-process instructions using CAND-lazy versus v1-eager; all outputs must match and A/A median discrepancy must be at most1%. The goal was set before either panel and is unchanged. Every positive/cancellation cell remains shown [pcc:unverified].

BEST

*Design:* all210 planned queries complete: six disclosed d12 anchors ×seven arms ×five repeats. Each has a fresh native tally process and fresh Callgrind process. Full3algorithm ×2context crossing plus identical A/A. Error bars are recorded five-repeat min/max. This is a finite exposed control panel, not a population sample. All decisions, witness indices, models and common diagnostics agree; no invalid, missing, excluded or retried rows [pcc:unverified].

*Boundary:* setup + complete oracle + post/replay/serialization equals the entire fresh-process instruction total. Every arm parses the SAME compact query payload. Earlier all65-case parsing counts are not used as a new baseline. Simulated instructions establish work reduction; native time and fastest-external performance remain unmeasured.

#pagebreak()
= Algebra, implementation and validation

On E0: y²+xy=x³+1 over odd-degree binary fields, full rational trace-kernel membership uses both Tr(x)=0 and Tr(y+xH(x))=0. Here H is half-trace. This describes the exact finite rational kernel used by the solver; it is not a general scheme-chart completeness proof.

For an actual finite residual(z,y), put t=y+zH(z). Then t²+zt=1. For trace-zero u, the S3 equation v²+zv+z²u²+1=0 is equivalent to v=t+zH(u²)+epsilon*z with epsilon in F2. Project modulo W=span(VV): only d+1 bits remain, thirteen for d12 instead of the ambient thirty-five. The original ambient diagnostics remain alongside actual projected diagnostics.

Packed parity rows can reject inconsistency early. A full-rank solution is reconstructed and checked against the COMPLETE W remainder; deficient systems fall back to the original solver/model order. Monic roots, actual supplied signs, conservative limits and original-group replay remain required. Projection setup is lazy after the first residual. Numeric membership still uses the full kernel conditions; symbolic q is materialized on demand or explicitly forced by eager arms.

EXTRA

*Cross-arithmetic audit:* a separate native checker imports no production field/group/solver code. Schoolbook polynomials and Euclid inversion recheck all65 input truths,19 negatives and all base admissions, then replay140 saved point tuples from this panel. Wide negative truths require inconsistency of every independent necessary symmetric system. Prior corrupt-point/false-refutation canaries are retained. This producer-written checker is not an independently signed verdict [pcc:unverified].

*Controls/resources:* planted and cancellation controls Found/replay; uniform controls Refuted. All28 largest-class pilot calls valid before freeze. Final phased/unphased counter calibration agrees within0.1%. Local CPU0,2GiB,600second combined calibration/pilot/freeze/primary cap, no paid launch or retry. The pilot estimate is planning only. Complete raw streams/counter dumps are archived with byte hashes and deterministic extraction/repack checks [pcc:unverified].

*Identity/custody:* E0 field moduli z^131+z^13+z²+z+1 and z^83+z^45+z²+z+1. Original workload SHA256 2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5; compact inputs retain exact point order. README links source/protocol/binary pins, rows, native summary, accepted export, audits, archive and reproduction. Earlier studies and their failed criteria remain immutable.

*Scope/sources:* this is the finite-base relation component, with fixed public controls. No Z[tau]-module advantage, asymptotic relation density, native-time or fastest-external claim. M4RI/FLINT/Magma comparison and independently signed review remain open. Existing scoreboard/timeline/runtime benchmark values stay historical. Sources checked: repository S3/Semaev/Diem, half-trace/Itoh–Tsujii and Montgomery batching. KB unavailable, wrapper127. No novelty assertion.
'''
    for token, value in [('TITLE', title), ('CAND', candidate), ('ROWS', '\n'.join(table)),
                         ('VERDICT', verdict), ('BEST', best), ('EXTRA', extra)]:
        text = text.replace(token, value)
    (p / 'report.typ').write_text(text)
    print('Rendered', p.name)
