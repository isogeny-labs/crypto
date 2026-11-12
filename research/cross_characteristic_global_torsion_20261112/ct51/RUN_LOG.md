# CT51 session handoff - 2026-11-12 19:xx UTC

## What was done (commands, files, features)

- Checked current root crypto PR #3877 (head c7af55d as inspected), cross-characteristic PR #4012 (head changed from e0a0d172 to 29f99038 while inspecting), cryptanalysis CT50 PR #1514, CI of exact CT50 and cross-characteristic parent, AGENTS.md / CLAUDE.md / CONDUCTOR.md, existing local/global degree-floor proof and problem record. Cross-repo GitHub search covered division polynomial, global torsion and characteristic two. Research-KB MCP and Conductor tool were not present; fallback searches were used. crypto-autoresearcher AGENTS.md fetch failed Unicode decoding.
- Selected a separate feature branch from 29f99038 to avoid changing the concurrently advanced PR #4012 head; no native #3877 files touched.
- Derived an exact global field K for 3-torsion of Cremona 11a1: f3(X)=3X^4-4X^3-60X^2-237X-21, Y^2+Y=X^3-X^2-10X-20. Mod 2: X^4+X+1 irreducible; Tr_(F16/F2)(X^3+X^2)=1, proving [K:Q]=8 and residue degree 8 above 2. Mod 29: X=18,Y=5 gives a degree-one specialization. This is a fixed-instance common global 3-torsion bridge, not a uniform map.
- Ran local Python normal and -O and independent JS normal and --no-opt: source/target orders 30/225, all 9 subgroup addition pairs, 3/3 public toy logs, 8/8 geometric order-three points, generator orientation negative controls passed. Executed exact committed JS file inside the connector V8 runtime with PASS output. Python published code has a separate bounded CI replay workflow; the exact committed Python file has not been independently executed outside CI and its standing is [pcc:unverified].
- Source receipts: primary references Masser--Wuestholz 1993 DOI 10.1112/blms/25.3.247, Lozano-Robledo 2015 DOI 10.4171/RMI/870; inherited root PR #3877, PR #4012, and problems/lift-reduce-second-prime-transfer.md. No novelty assertion.

## Decisions made (and why)

- Model the same *global nonrational* torsion point over two residue characteristics, advancing beyond the earlier five-element rational-torsion bridge.
- Keep subgroup inverse specialization explicit and precomputed. Do not confuse a sign-swapped generator with a nonhomomorphism: sign swap is a homomorphism but fails the designated-generator check.
- Source-to-target field cardinalities differ and the target uses a degree-eight extension; necessary field-degree growth is part of the cost, not an advantage.
- Remain within local exact arithmetic, no EKS Spot or paid benchmark launch. CI replay is verification of code execution only, not independent PCC scientific signing.

## Open threads (unfinished, pick up next time)

- Inspect exact-head CI and PCC; if checks fail, correct committed code with a new commit. Require independent signed PCC before calling the result verified.
- Find a non-enumerative algorithm for arbitrary input lifts or a rigorous growth obstruction for engineered higher-dimensional global A/K. A surjective mod-N Galois image for a fixed elliptic E/Q implies degree N^2-1 for any nonzero N-torsion point, but that premise must be checked in each intended family and does not rule out varying A/K.
- Complete native cross-factor and genus-two Cantor corpus on its existing branches without duplicating them. Full-cost comparative benchmarking remains blocked until a nontrivial candidate transfer algorithm exists.
- Retarget stacked PR to main only after parent PR #4012 merges and review gates permit.

## Errors (what went wrong, how it was handled)

- Initial local Python arithmetic used integer bit patterns for negative coefficients in characteristic two, incorrectly evaluating psi3. Corrected by distinguishing field elements from scalar integer coefficients and reran normal/-O; all tests then passed.
- Initial CT51 README math backslashes were eaten by JS string escaping; replaced malformed math with explicit inline expressions in a follow-up commit.
- JS torsion enumeration initially used .filter(psi3), selecting nonroots. Corrected to .filter(z=>psi3(z)===0) and executed the exact committed code successfully.
- KB, Conductor binary and crypto-autoresearcher AGENTS.md remained unavailable; no attempt to bypass access controls or overwrite concurrent heads.
