#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Paired-radius parity supports
T-1878 · Prime-field compressed recovery · 2026-11-07

The paired initializer separates the even and odd parts of the folded C0 polynomial. Its first gcd uses Y = V² and phase degree n/2; it then restores the allowed V orientations exactly.

#image("flow.svg", width: 100%)

*Checked construction.* Seven scalar-field/order configurations; 36,464 support equalities, including 4,984 equal-observation and 31,480 unequal-observation controls. There are 35,657 empty supports. All 3,501 candidate states agree with sequential direct updates. Validation02 also checks 28 rejected inputs and 10 public fallback comparisons.

#image("calls.svg", width: 100%)

*Public recovery.* E over F₂₅₇: y² = x³ + x + 7; P = (1,3); r = 281; m = 5. Radii 1, 280, 2, 3; extra offsets 0 through 16; call cap 128. Both adapters recover all targets at both degree caps. These totals include endpoint exits and source calls; they are not elapsed-time measurements. The second orbit costs 241 additional calls at cap 64 and saves 2 at cap 16.

*Costs and limits.* Four n-entry folds and n+1 coefficient deliveries remain. Norm filtering and orientation restoration add polynomial work; temporary lifting can reach degree n. Equal observations may retain both signs. Source setup, order checks, allocation/copy work and final verification stay charged.

*Evidence.* validation01 passed but omitted new-adapter later-update counts; changed sources are retained. validation02 corrects the counter and passes the expanded checks. 91 native dependency pins; no compiler/native failure or cloud/timing run. PCC-20261107-6848bb is an unsigned producer attestation; independent signed verdict pending.

*Sources and next.* Exact direct-phase predecessor and NIST DLMF 26.3 (dlmf.nist.gov/26.3); KB unavailable. Next use paired initialization only after a first-radius failure, to preserve early recoveries. DERIVATION.md and RESULTS.md give proof, replay and scope.
