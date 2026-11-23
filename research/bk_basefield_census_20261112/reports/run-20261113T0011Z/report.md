---
title: "Separate BK boundary pilot"
fontsize: 10pt
geometry: margin=17mm
mainfont: DejaVu Serif
---

All four registered calls complete and the canonical export/file check
passes. The complete p<14 population contains 506 coefficient models
across six prime fields: 178 general F2/F3 models and 328 short models.
There are 262 representative/model rows. Reference and A/A emit identical
270-row JSONL files; this tests determinism on the same host.
[pcc:unverified]

![](full-torsion.pdf)

The figure copies exact native SQL counts. Models are weighted by their
short scaling orbits; the eligible populations differ by r. There is no
random-sample or probability inference. The identity is included in every
torsion count. [pcc:unverified]

The native implementation counts points and every prime-factor kernel.
SQL independently reconstructs all 328 short scaling members, 84 polynomial
point/2/3-torsion counts and 34 modular orders. All 25 integrity checks pass.
The pilot has 212 qualified prime-to-characteristic k=1 cases and zero
mismatches. Dropping the condition yields 331 failures, including 122
with r² dividing #E. This reproduces a source-known omitted-condition
boundary. [pcc:unverified]

The positive and negative control calls both run the four fixed known
fixtures, including the original F19:y²=x³+x+6, #E=18, r=3 example.
On F7:y²=x³+2, the correct count9 differs from the deliberately corrupted
polynomial count3. A separate copied-weight SQL corruption exits1 at the
registered CHECK constraint. Canonical data remain unchanged.
[pcc:unverified]

Protocol dad642117df8 was committed49c8a1f2 before data. Exact source,
binary/runtime pins, all outputs, failures and clocks are retained;
every scientific clock is invalid. The separately frozen main population
includes these fields as part of a complete universe; it is not a held-out
confirmation sample. No outcome-dependent tuning or retry occurred.
[pcc:unverified]
