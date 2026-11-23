#set page(paper: "us-letter", margin: 0.6in)
#set text(font: "Helvetica", size: 8.8pt)
#show heading: set block(above: 8pt, below: 4pt)

= Generic Prime-Order E(F_p) Index Calculus: Native `ic_autolab` + `ecbench` + `ic sweep` Study

*Summary Claim:* `PCC-20261119-85e006` (Rounds 1–6: `PCC-20261119-95e1b8`, `946051`, `e03ae8`, `8e4c59`, `7478b3`, `a5013d`; all `[pcc:unverified]`). \
*Scope:* Public synthetic prime-order curves (`prime_search` seed `59297`, cofactor 1: `icv1-fp20-t727-cd198a38`, `icv1-fp24-t1059-6df599df`, `icv1-fp28-tm3161-380f655a`, `icv1-fp32-t74357-b7396dac`), deterministic counted GAE (`S = priced GAE / sqrt(r)`) across `AGENTS.md` §5a weights `w in {0, 0.1, 1}`, local `L0` route (no wall-time claim; development loop).

== 1. One-Unit Comparison Table (`S = priced GAE / sqrt(r)`, 9,624 / 9,624 Verified)

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  inset: 3.2pt,
  align: (right, left, left, left, left, left, left, left),
  [*k*], [*Curve*], [*Arm / Track*], [*Class*], [*S (w=0 / 0.1 / 1)*], [*Holdout sel/inc (w=0 95% CI)*], [*§5a*], [*S / rho*],
  [20], [`fp20-t727`], [`rho.negation`], [ref], [1.300 / 1.408 / 2.379], [—], [—], [1.0x],
  [20], [`fp20-t727`], [`mitm2-n45` (R0)], [base], [10.345 / 11.768 / 24.575], [—], [—], [8.0x / 8.4x / 10.3x],
  [20], [`fp20-t727`], [`pkm-iso-t6-g4` (`alg`)], [base], [21.681 / 6.70e5 / 6.70e6], [—], [`rho<inc`], [15.2x / 4.3e5x / 2.5e6x],
  [20], [`fp20-t727`], [`wsum-b5-n3` (`mb` R1)], [eng], [4.479 / 5.010 / 9.787], [0.445 \[0.407, 0.487\]], [`sel<inc`], [3.2x / 3.3x / 3.8x],
  [20], [`fp20-t727`], [`tw2l-x20-j16-n2` (`gen` R4)], [eng], [2.123 / 2.305 / 3.943], [0.213 \[0.193, 0.236\]], [`sel<inc`], [1.6x / 1.6x / 1.6x],
  [24], [`fp24-t1059`], [`rho.negation`], [ref], [1.197 / 1.307 / 2.302], [—], [—], [1.0x],
  [24], [`fp24-t1059`], [`mitm2-n160` (R0)], [base], [13.300 / 15.250 / 32.800], [—], [—], [11.1x / 11.7x / 14.2x],
  [24], [`fp24-t1059`], [`wsum-b12-n3` (`mb` R2)], [eng], [3.060 / 3.422 / 6.678], [0.218 \[0.206, 0.231\]], [`sel<inc`], [3.4x / 3.4x / 3.9x],
  [24], [`fp24-t1059`], [`wwalkGg-x09-sr` (`gen` R4)], [eng], [1.946 / 2.128 / 3.771], [0.141 \[0.131, 0.151\]], [`sel<inc`], [2.0x / 2.0x / 2.0x],
  [28], [`fp28-tm3161`], [`rho.negation`], [ref], [0.899 / 0.986 / 1.774], [—], [—], [1.0x],
  [28], [`fp28-tm3161`], [`mitm2-n402` (R0)], [base], [20.640 / 23.749 / 51.724], [—], [—], [23.0x / 24.1x / 29.2x],
  [28], [`fp28-tm3161`], [`wsum-b20-n3` (`mb` R2)], [eng], [3.081 / 3.466 / 6.936], [0.156 \[0.148, 0.166\] (0.149 cum.)], [`sel<inc`], [3.3x / 3.4x / 3.7x],
  [28], [`fp28-tm3161`], [`tw2l-x20-j16-n2` (`gen` R4)], [eng], [1.726 / 1.897 / 3.432], [0.084 \[0.075, 0.093\]], [`sel<inc`], [1.8x / 1.8x / 1.9x],
  [32], [`fp32-t74357`], [`rho.negation`], [ref], [0.909 / 0.999 / 1.811], [—], [—], [1.0x],
  [32], [`fp32-t74357`], [`mitm2-n953` (R0)], [base], [31.270 / 35.981 / 78.379], [—], [—], [34.4x / 36.0x / 43.3x],
  [32], [`fp32-t74357`], [`wsum-b28-n3` (`mb` R6)], [eng], [2.966 / 3.396 / 7.269], [0.739 \[0.665, 0.822\] vs b20 (0.095 cum.)], [`sel<inc`], [2.7x / 2.8x / 3.3x],
  [32], [`fp32-t74357`], [`tw2l-x20-j16-n2` (`gen` R4)], [eng], [1.833 / 2.015 / 3.659], [0.061 \[0.056, 0.066\]], [`sel<inc`], [1.8x / 1.8x / 1.8x],
)

#image("figures/band_comparison.svg", width: 100%)

== 2. Track Ratchets and `ic sweep` TPE Studies

#grid(
  columns: (1fr, 1fr),
  gutter: 8pt,
  image("ratchet-multibase.svg", width: 100%),
  image("ratchet-generic.svg", width: 100%),
)

#image("figures/sweeper_rungs.svg", width: 100%)

*Key Findings & Limits:* (1) On `multibase` (`N >= 3` columns), multi-seed signed coefficient boxes (`wsum`, `C=3`) follow `B* = 2^(-1/2) r^(1/6)` (`B = 5, 12, 20, 28`), cutting `S` by `2.25x` (`k=20`) to `10.54x` (`k=32`) over `mitm2` and flattening the gap to `rho.negation` from `8.0x..34.4x` to `2.7x..3.4x` (`w=0`). (2) On `generic` (including fresh-per-run `C=2` `tw2l`/`wwalk`), `S` drops by `4.69x..16.39x` to `1.6x..2.0x` `rho.negation` across the entire §5a band. (3) In `ic sweep`, `prime-rootfind` `m=3` improves over `m=2` with slope `-0.399/bit` (`-28.39` projected at 80b), while `prime-algebraic` F4 hits the 240 s cap above 12 bits. No method beats `rho.negation`.
