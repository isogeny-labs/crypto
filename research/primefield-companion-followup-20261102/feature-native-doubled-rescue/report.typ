#set page(paper: "a4", margin: 14mm)
#set text(size: 9.5pt)
= Conditional doubled-order phase rescue
T-1879 · Prime-field compressed recovery · 2026-11-07

For odd m and 8m dividing r−1, opposite m-point orbits form one 2m-point orbit. Their three low observations combine with signs +, −, +. The new helper uses that identity after the first-radius path defers, and keeps the original state as fallback.

#image("flow.svg", width: 100%)

*Identity checks.* 10,503 scalar-mode values; 5,103 reindexed sign vectors (exhaustive for m=3, partial for m=5); 21 coefficient sections; 4,050 wrong-sign controls; 10 invalid inputs. Fifteen nonopposite public fallbacks match the original path.

#image("calls.svg", width: 100%)

*Public panel.* E/F₂₅₇: y²=x³+x+7; P=(1,3); r=281; m=5. Radii 1,280,2,3; extras 0 through 16; call cap 128. Every target/cap pair has no source-call increase and preserves every reference success. Five caps recover all 281 targets; cap 8 leaves 10 unresolved on both paths. At cap 64 no rescue context is constructed.

*Added work.* Caps 10,16,20 build 260 extra contexts and deliver 2,080 upper F0 coefficients each. Cap 32 builds 205 contexts, with 1,640 such deliveries. These exclude F1/F2 and setup. Cap misses and nonsingletons are retained. Exact-order checks, polynomial work, copies, allocations and final group verification are charged; query savings are not a runtime claim.

*Evidence.* validation01 passes on its first attempt with 94 native dependency hashes and inherited checks. PCC-20261107-67816b is a producer attestation; independent signed verdict pending. No cloud or timed workload. Original cohort unchanged. DERIVATION.md, RESULTS.md and README.md provide scope and replay.

*Sources and next.* Exact paired/direct predecessor; NIST DLMF 26.3; FFTW transform-definition conventions. Finite-field recombination is derived explicitly; KB unavailable. Next reduce or reuse the added coefficient setup, then account total arithmetic before default adoption.
