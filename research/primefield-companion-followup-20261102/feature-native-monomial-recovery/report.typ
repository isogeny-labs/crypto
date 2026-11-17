#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Adaptive recovery with one monomial pair
2026-11-07 · T-1740 · Native development integration

The combined character monomial is now carried through adaptive extensions and skipped-sign steps, using the preserved geometry tail and quartic decoder. Every extension returns one homogeneous pair. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== Cursor invariant
The cursor holds the previous two monomial pairs, selected multiplier x-coordinates and the geometry tail. Anchors are checked first. A first extension after a length1 window can initialize directly; later nonanchor extensions use the combined coordinate-difference recurrence. A skipped sign advances all transport state and omits only residual evaluation. Checked budgets reject overflow and exhaustion. The parity-aware −P anchor and resets across infinity are preserved. [pcc:unverified]

== Native and separate checks
The native suite passes 207 tests. Focused tests check 100 split windows, 1,375 advances and 450 skipped residuals against the frozen four-input source. Thirteen direct phase/value initializations occur in short-window extensions, matching actual branch counters. All 20 public-point recovery records agree with frozen recovery and pass forward multiplication. Their extensions retain 16 extra signs, four skips, 20 steps and 480 conversions; initial windows use 33 direct initializations and their extensions use none. [pcc:unverified]

A separate checker imports no producer modules. It reconstructs the 479-input polynomial identity with a square-set oracle and affine arithmetic, accepts the 20 records, and rejects changed scalar and initialization fields. It was authored by the producing session. Thirty-six recursive source/data hashes and binary digests accompany the raw logs and commands. No native failure; warnings retained. [pcc:unverified]

== Costs and next step
Every nonanchor extension now evaluates coordinate-difference powers, even when its residual is skipped. Geometry lanes, resultants, setup, direct initialization, allocation and verification remain. Next: matched accounting against the four-column adaptive path, with all these stages included. Unit-test duration is not performance evidence. Original 4,563-case cohort unchanged; no cloud or timing panel.

== Assumptions and sources
The inherited prime-field/subgroup, word-size, nonzero-operand and conditional seed-normalization assumptions remain. Sources: monomial-transport derivation/source, transport-recovery cursor/source/tests/generator, previously reviewed NIST DLMF23.10.3 (https://dlmf.nist.gov/23.10.E3). KB unavailable: no MCP tools or wrapper executable. No novelty-priority claim.

Claim PCC-20261107-16444b remains unsigned (local verdict not_covered). Read DERIVATION.md, receipt.json, source-lock.json, checker-receipt.json and raw logs for exact scope. Generated adaptation is reproducible with generate.py --check.
