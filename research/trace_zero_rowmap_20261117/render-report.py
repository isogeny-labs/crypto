"""Lay out recorded Rust results; no cryptographic or statistical computation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

p = Path(__file__).resolve().parent
s = json.loads((p/'summary.json').read_text())
a = json.loads((p/'grid-audit.json').read_text())
q = json.loads((p/'protocol.json').read_text())
ids = q['extensions']['primary_cases']
fig, axes = plt.subplots(1, 2, figsize=(9.4, 3.5), layout='constrained')
for ax, metric, title in zip(axes, ['instructions','oracle_instructions'],
                            ['Complete cold query','Complete algebraic relation oracle']):
    for j, arm in enumerate(['baseline','lookup']):
        v = [s['cells'][f'{i}/{arm}']['metrics'][metric] for i in ids]
        ax.barh([i+(j-.5)*.32 for i in range(6)], [x['median']/1e6 for x in v],
                height=.29, color=['#55759b','#24866f'][j], label=arm,
                xerr=[[(x['median']-x['min'])/1e6 for x in v],
                      [(x['max']-x['median'])/1e6 for x in v]],
                error_kw={'capsize':2,'elinewidth':.8})
    ax.set_yticks(range(6), [f'{n} {k}' for n in ['E0 n83','ECC2K-130']
                           for k in ['planted','uniform','cancel']])
    ax.invert_yaxis(); ax.set_title(title, fontsize=10)
    ax.set_xlabel('Million simulated instructions (Ir)')
    ax.spines[['top','right']].set_visible(False)
    ax.legend(fontsize=8,loc='lower right')
fig.savefig(p/'instructions.svg');fig.savefig(p/'instructions.png',dpi=150);plt.close(fig)
(p/'plot-data.json').write_text(json.dumps({'source':'summary.json','cells':s['cells'],
                                          'contrasts':s['contrasts']},indent=2)+'\n')
table=[]
for i in ids:
    c=next(x for x in a['goal_criteria'] if x['instance']==i)
    b=s['cells'][i+'/baseline']['metrics']['instructions']['median']
    v=s['cells'][i+'/lookup']['metrics']['instructions']['median']
    label=i.replace('scale-n','').replace('-d12-','/').replace('distinct-planted-0','planted').replace('cancellation','cancel')
    table.append(f'[{label}], [{b:,}], [{v:,}], [{100*c["whole_removed_fraction"]:.2f}%], [{100*c["oracle_removed_fraction"]:.2f}%], [{100*c["aa_relative_median_difference"]:.6f}%],')
text='''#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Index-calculus S3 row evaluation: measured reduction
2026-11-17 · TZR packed projected rows · unsigned qualified-build evidence

#image("instructions.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto,auto), inset: 4pt,
 [Field/case], [Baseline cold Ir], [Lookup cold Ir], [Cold removed], [Oracle removed], [A/A],
ROWS
)

*Observed:* the ECC2K-130 uniform control uses22.87% fewer complete-oracle instructions and11.21% fewer complete-query instructions against the current optimized baseline. The n83 control uses31.58%/16.77% fewer. All90 planned executions complete with exact matching outputs, witness indices/order, models, common/projected diagnostics and field ledgers. Every saved relation passes a separate polynomial-arithmetic replay [pcc:unverified].

*Frozen criterion: MET.* BOTH uniform cells require at least5% fewer oracle instructions and1% fewer whole-query instructions; all six A/A discrepancies<=1% and no other whole-query cell may regress>1%. All six cells pass. Immediate cancellation does not construct the lookup; its small count differences are compiled-path effects, not evidence of table evaluation. Keep every control [pcc:unverified].

*Design:* six disclosed d12 queries ×three arms ×five repetitions. Baseline/lookup/baseline-aa share one executable, actual current field/context code, qualified support and identical compact query bytes. Lazy symbolic context is fixed. Error bars show recorded min/max across five repeats; A/A is repeatability/nuisance control, not population variance. No independent-target or asymptotic inference.

*Accounting chain:* C_old=setup+oracle+post. Replace ONLY fixed binary-linear row evaluation inside the oracle, including every per-query table build. C_new has the same boundaries and answers. No removed parsing, free symbolic/base preparation or batch tail. Whole Ir includes startup/input, context/base/admission, all preparation/search, roots/signs/replay/output and exit. Oracle/setup/post sum exactly to whole process. Ir is simulated instruction work; native runtime is unmeasured.

#pagebreak()
= Exact map, validation and build qualification

The existing trace-zero/subspace S3 oracle already uses13 unknown bits at d12. This replacement evaluates those SAME coefficient and RHS rows. With t=y+zH(z), each coefficient row is F2-linear in z, and its RHS projection is independently F2-linear in t. The quadratic computation of t stays unchanged and paid.

Pack up to16 rows into four u64 words, four disjoint16-bit coefficient lanes each. For every four-bit input chunk, build all16 XOR combinations of its basis images; RHS projections have separate u16 images. XOR one entry per chunk to evaluate all packed rows together. Disjoint lanes and XOR introduce no carry or row permutation. Wider-than16 coefficient systems and rows after16 retain exact scalar parity. Original elimination, rank/model order, full W membership, monic roots, supplied signs and original-group replay remain.

Each query pays its own tables when the existing projector is first needed. Table payload is ceil(n/4) ×16 ×(32+2) bytes:17,952 at n131 and11,424 at n83, excluding existing maps/vector headers. This formula is not a peak-RSS measurement. Full numeric admission keeps both Tr(x)=0 and Tr(y+xH(x))=0; the factor base and target are fixed.

*Checks:*21 focused release checks pass, including every input basis bit and independent random pairs at widths1/13/16, width>16 refusal, deliberately inserted16 zero leading rows forcing scalar-tail evaluation, complete small S3 model classifications and exact65-case current/lookup parity. The post-run Rust auditor independently recomputes every summary range from raw rows, the unique90-row grid, all criteria, phase sums and identical field ledgers [pcc:unverified].

A separate native checker imports no production field/group/solver code. Schoolbook polynomial arithmetic and Euclid inversion recheck65 known truths,19 negative proofs and all base admissions, then replay all60 saved tuples from this panel. Wide negatives require every necessary system independently inconsistent. This producer-written cross-check is not independently signed verification [pcc:unverified].

*Qualification:* current main SAT source has unclosed delimiters; broad root check also fails hashed-ecdl-tensor metrics imports. The focused build uses ACTUAL current numerical solver/field/curve/trace-kernel code, with byte-pinned SAT/chain support from d7a006293. Those support solvers are not executed in this numeric relation backend. Both arms share the same qualification. No full-root or current SAT build success is claimed. Initial/facade/workspace failures are retained.

*Execution:*12 positive/uniform pilot calls pass before freeze, with final phased/unphased calibration within0.1%. CPU0/2GiB/600seconds combined calibration/pilot/freeze/primary, zero paid resources/retries. Source SOURCE; protocol PROTOCOL; executable BINARY. Exact rows, inputs, archives, audits, accepted export and reproduction are linked in README. Raw archives extract/hash/repack identically [pcc:unverified].

*Identity/sources:* ECC2K-130 is icv1-f2m131-tm22283658519494248867-115e0dc5. Matched E0 n83 is icv1-f2m83-tm6151469093347-debefd74, used because this model supports E0; standard K1-83 is E1. Existing runtime/leaderboard values stay historical. KB unavailable127; reviewed repository S3/half-trace, symmetric subspace oracle, Four Russians tables and setup regressions. Established lookup technique; no novelty, relation-density, full-pipeline, fastest-external or native-time claim.
'''
text=text.replace('ROWS','\n'.join(table)).replace('SOURCE',q['freeze']['commit']).replace('PROTOCOL',q['freeze']['protocol_sha256']).replace('BINARY',q['arms'][0]['instrument']['binary_sha256'])
(p/'report.typ').write_text(text)
print('Rendered recorded-value graph/report source')
