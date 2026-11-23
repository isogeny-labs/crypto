#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Root-free affine descent in the actual subgroup
16 November 2026 · hec_fibre_pencil_20261116 / projected descent gate

== Exact selector and verification
For a full-degree squarefree nonbranch-support divisor $(u,v)$, the residual norm has degree $g+1$. A bucket with $g+1$ distinct points under $(y-v)/u$ therefore supplies all its roots. Production need not form each candidate norm or compute a gcd. The gate separately forms every admitted norm, checks original principal group identities, and verifies emitted decompositions at the actual ell. This is classical algebra; no novelty or speed claim.

The graph component equation is projected to odd-prime torsion so branch-place 2-torsion is handled correctly. A gate-only certificate h annihilates every catalogue point after multiplication by ell and is coprime to ell. All emitted rows satisfy [h]target = sum alpha_j[h]P_j. Multipliers are1,2,1 for genera2,3,4. Certificate work is not free production setup or a point-count theorem. Candidate input is a public divisor, with no expected scalar.

== Once-only panel outcomes
Source79f304759cf7, prior checkpoint20261116T065252Z; source/import/binary freeze passes. Execution exits0 with empty stderr. Six fixed direct-profile targets, four prefix-anchor policies each. Actual orders61667,4620611,16790591; inputs and certificate checks are in DESCENT-INPUTS.txt. No fresh-holdout claim.

#image("../../DESCENT-COUNTS.svg",width:100%)

Five projected decompositions, fifteen no-component outcomes, four precondition failures. All selected witnesses are single fibres; multi-fibre projected cancellation is not exercised by this panel. At the Compact cardinality rule (H13/36/22), only one of six targets decomposes. Anchor membership is a prefix, not Compact's target-seeded selection.

Genus3 target1 has $u=(x+35)^2(x+37)$ over F211 and violates squarefree support. Its four precondition failures are retained. The existing unrestricted descent already splits it with zero additions. No failed input or anchor policy is replaced or rerun.

#pagebreak()
== Reference controls and selected work counters
The unrestricted existing point-add descent completes all24policy calls. The six inputs repeat four times; additions per target are:

#table(columns:4,inset:5pt,
[Genus],[Target0 adds],[Target1 adds],[Candidate inversions per eligible scan],
[2 / F251],[1],[3],[123],
[3 / F211],[5],[0],[103],
[4 / F61],[6],[33],[35–36])

The anchor-restricted point-add control completes20of24calls, retaining four2048-add caps. All successful reference decompositions reconstruct the supplied divisor in the original group. Unrestricted reference has a different output-support contract; restricted reference changes the sampling. Field inversions and group additions are different units and are not divided to claim a speedup.

Eligible candidate calls scan123/104/36catalogue entries. Counters omit additional polynomial precondition checks, allocations and graph assembly; they are selected counters, not complete operation accounting. Catalogue/table setup, certificate search and group verification are gate prerequisites and would need proper treatment in any complete timing protocol. There is no wall-time measurement or independent scientific validation here. Source descent and complete scalar recovery are not implemented by this routine.

== Conditional guard-and-fallback work identity
For a guard that tries this pencil and invokes the identical reference with unchanged input/seed after failure,

$W_("hybrid")=W_("common")+W_("pencil")+W_("reference"),$
$W_("baseline")=W_("common")+W_("reference").$

With nonnegative additive charged costs, a failed scan retains all baseline work and adds positive pencil work. It cannot give tenfold arithmetic-work improvement on those inputs. Both genus2targets and both genus4targets fail the recorded size-rule pencil; one genus3target is rejected by its precondition. This is a conditional architectural exclusion, not a runtime regression estimate or a measured complete hybrid solver.

Changing anchor membership, function family or intermediate-state reuse would be a different algorithm. The result does not exclude every fibre technique. Repeated-support handling alone cannot fix the four observed genus2/genus4 no-component cases. A successor requires a structural mechanism that yields useful output at the target-seeded anchor scale, with failed pencils and all source/target costs charged against Compact. The mathematical tenfold objective stays open.

== Artifacts and limits
DESCENT.tsv retains every policy, selected counter and bounded control. DESCENT-SUMMARY.tsv and DESCENT-TOTALS.txt are reduced by native descent_reduce.rs; its figure is layout over fixed recorded counts. DESCENT-PROTOCOL.md and DESCENT-FREEZE.sha256 identify the pre-run contract. Shared field/group code remains a dependency. The earlier graph gate validates component algebra but is not substituted for missing multi-fibre actual-subgroup cases.

No timing record, benchmark-index/leaderboard admission, independent VALIDATION.json or performance claim. Existing canonical graphs remain unchanged. Literature/KB receipts and inaccessible primary-source gap are retained in THEORY.md/SESSION.md; no priority claim. Every report page is visually inspected. PR4357 remains unmerged.
