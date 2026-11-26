# 2026-11-23 — compact Semaev projective tensor

PR #5087 https://github.com/isogeny-labs/crypto/pull/5087. Predecessor #5058 is merged; no maintenance of its branch.

## What was done (commands, files, features)
- Read AGENTS.md, ACTIVE, latest lane log, existing core and field implementation, findings for #5023/#5044/#5058; repository search for Veronese returned no hit, not evidence of novelty. New task branch and draft before implementation.
- Built normalized projective S3 Veronese tensor, explicit quadratic-extension and geometric projection arms, plus shared-square line variant. Local source blobs for veronese.py, veronese_bench.py, veronese_fall.py exactly matched fetched GitHub blobs.
- PYTHONPATH=/mnt/data python -m summation_split.veronese_bench: 200/200 exact affine-slice sets over p=5,7,11,19; 2100 decisions, 225 positive polynomial certificates. 26 successful GB solves all matched; 6 split worker timeouts at m=5. Last two timeouts come from separate identical-seed replay, marked run 2b. All completed paired split timings slower than monolithic. No F4 measurement.
- PYTHONPATH=/mnt/data python -m summation_split.veronese_fall: exact direct Macaulay ranks at D=2,3, Prime31 and smaller p. Local normalization yields new direct fall at D=3, independent of m. Added implied quadratics in a follow-up diagnostic: all D=3 new fall dimensions zero in tested cells. Recorded derivation, data, replay script and SVG.
- Files summation_split/VERONESE_FINDINGS.md, veronese*.py/csv/svg; append link in existing FINDINGS.md. All [pcc:unverified], no independent runner.

## Decisions made (and why)
- Geometric projection eliminates the need to require all auxiliaries rational. Only original field restriction is charged when extracting Fp outputs; explicit Fp2 arm gives rational coordinate witnesses. No factor-base map changed.
- A local degree-3 fall is a provable raw presentation fact, not a regularity or solving-degree bound. Preclosing the implied equations removes all observed degree-3 falls. Preserve negative runtime panel.
- Keep draft because required two-parameter speedup and F4 evidence are absent. No merge or paid compute executed.

## Open threads (unfinished, pick up next time)
- Genuine F4 or instrumented Macaulay elimination on matched slices with at least two free parameters, both preclosed and balanced-tree controls, 50 exact projected sets per cell; include field membership and extraction in cost. Actual d_F4, memory, independent replay/PCC.
- Integrate bounded rank diagnostic into benchmark only under explicit definition; no fake solving-degree plot.

## Errors (what went wrong, how it was handled)
- Sage/FGb/msolve/Singular, research-KB service/wrapper and conductor CLI unavailable; SymPy local diagnostics only, no EKS access. Runtime cap interrupted last two arms; reran identical seed worker and preserved provenance. Cached numeric Lagrange interpolation for gate efficiency.
- Prior sources: Semaev 2004 https://eprint.iacr.org/2004/031, Kosters–Yeo 2015 https://arxiv.org/abs/1503.08001, Kousidis–Wiemers 2019 https://arxiv.org/abs/1906.05594 (different Weil descent hypothesis); repository prior PRs #5023, #5044, #5058.
