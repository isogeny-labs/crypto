#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Index-calculus S3 row evaluation: measured reduction
2026-11-17 · TZR packed projected rows · unsigned qualified-build evidence

#image("instructions.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto,auto), inset: 4pt,
 [Field/case], [Baseline cold Ir], [Lookup cold Ir], [Cold removed], [Oracle removed], [A/A],
[83/planted], [31,611,467], [29,445,307], [6.85%], [17.60%], [0.000117%],
[83/uniform], [40,805,623], [33,962,805], [16.77%], [31.58%], [0.000125%],
[83/cancel], [24,910,969], [24,860,333], [0.20%], [0.90%], [0.000245%],
[131/planted], [39,974,973], [39,214,706], [1.90%], [5.73%], [0.000185%],
[131/uniform], [52,068,848], [46,231,195], [11.21%], [22.87%], [0.000117%],
[131/cancel], [32,885,116], [32,834,844], [0.15%], [0.81%], [0.000143%],
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

*Execution:*12 positive/uniform pilot calls pass before freeze, with final phased/unphased calibration within0.1%. CPU0/2GiB/600seconds combined calibration/pilot/freeze/primary, zero paid resources/retries. Source 45b0279c0280b5cb2394cf7710a07464142f42ec; protocol 7703daa3af347a76a52bd0bc5b5587516e658ecb04c0b5c887d6b7d550aadfcf; executable 7380fefb6583d434734d2cbffae23fb16b8a10282a9bcf5b67461687a5cc9601. Exact rows, inputs, archives, audits, accepted export and reproduction are linked in README. Raw archives extract/hash/repack identically [pcc:unverified].

*Identity/sources:* ECC2K-130 is icv1-f2m131-tm22283658519494248867-115e0dc5. Matched E0 n83 is icv1-f2m83-tm6151469093347-debefd74, used because this model supports E0; standard K1-83 is E1. Existing runtime/leaderboard values stay historical. KB unavailable127; reviewed repository S3/half-trace, symmetric subspace oracle, Four Russians tables and setup regressions. Established lookup technique; no novelty, relation-density, full-pipeline, fastest-external or native-time claim.
