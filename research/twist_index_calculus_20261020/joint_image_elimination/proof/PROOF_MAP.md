# Exact scope of mathematical evidence

No failed, interrupted or unexecuted Lean target is reported as a checked
proof. Every actual build retains the source hash, command and output.

| Statement | Exact integer evidence | Lean statement and status | Remaining bridge |
|---|---|---|---|
| Phi source-block cross product | `orbit-v1/phi.identity.json`, IDC1h90c3394925f8554f; zero remainder and +1 mutation rejected | `JointOrbitCoordinates.phi_cross`; universal commutative-ring identity; split coordinate gate v4 PASS | Nonzero denominators, field division and actual curve-map interpretation |
| Doubling source-block cross product | `orbit-v1/double.identity.json`, IDC1h6cace5d400e47150; zero remainder and mutation rejected | `JointOrbitCoordinates.double_cross`; coordinate gate v4 PASS | Same field/group bridges |
| Equality after U^12=T | Derived from the cross products | `phi_on_orbit`, `double_on_orbit`; exact orbit equality assumption, gate v4 PASS | Every actual source belongs to an allowed nonzero block; checked natively on all 21 cases |
| Quartic discriminant factorization | `orbit-v1/quartic-discriminant.identity.json`, IDC1h4254a7cf15bd8c71; exact Z identity and mutation rejection | `JointQuarticDiscriminant.quartic_discriminant`; factored gate v6 PASS on the same universal commutative-ring identity | Decoding allowed Fp roots and finite-base membership |
| Saturation excludes repetitions | Native tests plus the exact factorization | `JointQuarticDiscriminant.quartic_nonrepetition`; discriminant nonzero is an explicit hypothesis; factored gate v6 PASS | No converse over rings with zero divisors is claimed; linking the encoded inverse guard to this hypothesis remains separate |
| Total bucket mass equals candidate-list length | Discrete finite-count argument, not certified by the coordinate IDC records | `ResidualMass.total_of_bounded` and `mapped_candidates_total`; every residual label is <n; repaired uniform gate v3 PASS | Enumerating the mixed physical domain as 16*C(L,3)*L is a separate combinatorial identification |
| Equal mixed-image total mass | Both maps label the same signed candidate domain | `ResidualMass.two_images_same_mass`; uniform gate v3 PASS | Actual image injectivity, disjointness and group labelling are executable checks, not assumed formalized |
| Folded doubled triangle has zero coefficient | Predeclared post-run reconstruction of [2]P - P + psi(P) + psi^2(P) | `ZeroDoubling.folded_triangle`; one theorem, gate v1 PASS; w^2+w+1=0 is an explicit hypothesis | Actual group action, distinct allowed IDs and inclusion in saved complete zero sets require the separate native audit |
| Pure S4 components force zero discriminant | Native 34-by-31 exact rational search finds 2048*D=sum Ai*Ci; integer replay zero and +1 mutation rejected; loose coefficient certificate retained | `PureZeroComponents.component_certificate` and field consequence `coefficients_force_discriminant_zero`, with explicit 2048≠0; gate v1 PASS on both theorems, standard foundations only | Separate IDC replay pending; identify Ci as actual H4 components, Kummer basis independence and actual zero group-sum => S4 necessity; full Lean curve theorem remains open |

The quartic IDC's abstract variables `u,T,B,w` stand for four polynomial
roots in that record, not the similarly named curve parameters. The Lean
quartic theorem calls them `x,y,z,w`. Both prove the same exact integer
polynomial identity. The fifth registered IDC variable is unused there.

## Failed checks and retained history

- Verification v6 gave the proof gate the wrong path type and did not build
  the theorem file.
- The original orbit gate v2 was interrupted before a gate receipt. Its
  recovered temporary build log records a thread-creation failure.
- The one-worker combined orbit gate v3 reports an out-of-memory build
  failure under the unchanged 8 GiB limit. No theorem in that module is
  promoted on the basis of native IDC success.
- Uniform gate v2 exposed the unresolved equality
  `1+ks.length=ks.length+1`. That goal is explicitly repaired; the failed
  source and log remain in the v2 receipt. The new v3 gate is separate.
- Split polynomial targets keep the original theorem statements and use
  the algebraic solver directly. Coordinate gate v4 audits all four
  theorems successfully. The quartic module still fails allocation and is
  not promoted. Uniform gate v3 passes all 12 audited declarations,
  including the six public theorems and generated recursion equations.
- The exact failed combined and quartic source attempts are retained as
  `.lean.txt` beside their integer certificates and unchanged in their run
  metadata. They are not checked theorems. A new active standalone target,
  `orbit-v3/QuarticDiscriminant.lean`, preserves the universal ring identity
  and nonrepetition statement, using a cubic-coefficient factorization to
  reduce proof expansion. Gate v5 failed before Lean started because the
  managed service did not inherit the toolchain PATH. Gate v6 supplies the
  explicit executable paths and passes the build and all nine audited
  declarations: seven named theorems and two local AC instance definitions.
  The proof factors addition of a fourth root through cubic coefficients;
  it preserves the full original identity and nonrepetition consequence.
  The actual v6 source, build, status and axiom audit are retained in
  `results/proof-checks/quartic-factored-v6/`. No group/root/cost bridge is
  inferred from this success.
- Zero-triangle gate v1 passes its one coefficient theorem, with standard
  foundations only and zero failing or trusted-native declarations. This
  is a coefficient identity, not a formalization of the curve relation.

The repository gate allows only the standard foundations `propext`,
`Classical.choice` and `Quot.sound`. `sorryAx`, user axioms and unchecked
native evaluation are rejected. `lean4checker` is unavailable on this host.
Toolchain is pinned to Lean 4.34.1; one worker and bounded resources are
recorded by `run-proof-recovery.sh`.

## Statements not formalized

The complete S5 resultant construction, equality of elimination ideals,
finite-domain root completeness, physical relation decoding, independent
matrix rank, actual recovery, runtime and asymptotics do not have Lean
proofs here. Coefficient validation, numerical determinant probes, group
oracles and independent RREF provide bounded executable evidence for their
stated fixture scope. They do not replace these missing formal definitions.
