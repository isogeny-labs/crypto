# P-256 transfer search, 2026-11-15

## User request and acceptance

"Make pr and find a hidden transfer that leads to speed up."

The prior framework PR #4209 is merged. This follow-on search must produce a
regular PR, concrete maps or bounded exclusions, and evidence for any claimed
advantage. A valid map, a smaller representation or a model estimate does not
satisfy the speedup requirement. Keep that requirement open unless a matched
complete-cost solve demonstrates it.

| Requirement | Acceptance evidence |
|---|---|
| PR | Regular PR with source, report, diagram, inspected PDF and checks |
| Hidden/newly useful transfer | Explicit working-field map; exact source subgroup; kernel and recovery |
| Speedup | Applicable named destination solver; paired complete-cost comparison and replay |

## Literature and lookup receipt before implementation

- KB unavailable: no MCP tools; scripts/research-kb status exits 127.
- ecdb guide read from current main. No ecdb token, AWS credentials or AWS CLI
  is available; do not seek a token elsewhere. Use targeted repository sources.
- Read ecdlp-surfaces and its higher-genus reference, report-evidence,
  research-visuals, endomorphism-rules and transfer-assessment methodology.
- Smith arXiv:0806.2995 sections 3–8, especially Proposition 5: its
  absolute-simplicity hypothesis does not hold for our C3 Jacobian. Its
  rational-kernel count alone does not imply a smooth Jacobian quotient.
- Existing smith_kernel_coverage, smith_prop24, p256_isogeny_campaign,
  p256_isogeny_task and isogeny/velu modules. The existing degree-11 campaign
  is prior work; do not relabel an isogeny walk as a newly discovered attack.
- P-256 identity from p256_isogeny_campaign::P256_ICV1:
  icv1-fp256-t89188191154553853111372247798585809583-f188c491.
  Exact p,b,n,G are inherited from the native study's parameters(), with a=-3.

## Predictions and discriminating checks

1. **C3 all rational (2,2,2) kernels.** Enumerate all 135 geometric
   Lagrangians, retain Frobenius-stable ones using the actual-field audit.
   For each of the three elliptic degree-two covers, form its E[2] image
   from the Weierstrass fibers. Test whether every stable kernel contains
   exactly one such isotropic plane. If true, prove the quotient has a
   principally polarized elliptic factor and is decomposable. If false,
   preserve the uncovered kernels as candidates for a full quotient.
2. **C2 Richelot.** Determine the actual sextic factor type with native
   arbitrary-precision polynomial arithmetic. Enumerate all 15 matchings
   under that Frobenius action. Identify the deck-involution pairing and
   test whether any other rational kernel remains.
3. **Elliptic escape.** Compute the degree-three division polynomial's
   rational-root factor. If it has roots, construct the normalized Vélu
   map and dual, bind the actual P-256 subgroup, and check polynomial
   identities, order, additivity and dual composition. A valid map is a
   candidate, not evidence of a speedup. If the root factor is empty,
   preserve that exact bounded exclusion and try another concrete route.

Controls: all 135/105 and 15 combinatorial counts; conjugation of branch-point
labels; a generic case that is not forced into the decomposable verdict;
small-field exhaustive maps with kernel/identity exceptions; negative
coefficient/map controls; actual-field known-answer points, never production
targets. Failures and amended predictions must stay in the work log.

## Amendment before the symmetry-image check

The first three routes produced a split-kernel obstruction and a standard
elliptic transfer control. Test an additional candidate rather than stop there:
mix the two E copies in J(C3), using a cube root lambda modulo n, to obtain
a finite n-subgroup stable under the order-three source automorphism. Check
actual divisors, order, additivity and the eigenvalue relation. An ideal
orbit-count reduction is not a measured runtime gain; include transport,
group arithmetic, canonicalization and short-cycle handling before any claim.
Compare a componentwise E x E realization as an explicit cost control.

Also screen the genus-two elliptic companion F2 over the original field:
in its shifted model it has a=-3 and b=b_P256^2-2. Compare its exact
Frobenius trace modulo 3 using the rational part of psi_3 and, where that
part is linear, the kernel ordinate's square class. Different traces modulo
3 exclude a nonzero same-field elliptic homomorphism without computing a
full group order. Matching or ambiguous signatures remain unresolved.

Local work consists of exact correctness tests, not a timed experiment.
At most two minutes per correctness command and 30 native CPU-minutes for
the initial search. No key-recovery campaign or cloud spend is launched.
An actual benchmark must use the required EKS/isolation protocol. Direct AWS
access is absent; the skill also documents a GitHub-dispatch route, which must
be checked before calling measurement infrastructure blocked. No benchmark is
launched until a candidate has a specified destination solver and workload.
No local wall time becomes a speed claim.

## Environment and ownership

Conductor is unavailable (command not found). Scope: this new transfer_search
directory, its PCC claim registrations, a focused problem record if a material
dependency persists, and the shared session log. Do not edit frozen prior
reports, source, results or claim files. Rust is the active research language.
