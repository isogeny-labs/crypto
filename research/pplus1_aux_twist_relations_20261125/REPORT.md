# Smooth auxiliary fibers for prime-field relations

**Status, 2026-11-25:** bounded native falsification panel. The three usable
auxiliary fibers produce verified relations on the original curve, but each
costs more counted target-curve additions than both the matched small-x panel
and every same-target rho trial in this fixture. This is a lower-bound screen:
it omits several setup and field operations, polynomial solving at realistic
size, and sparse linear algebra. It therefore gives no end-to-end speedup claim.

## Construction and proof boundary

For the prime-order target $E(\mathbb F_p)=\langle P\rangle$, let
$Q=[k]P$. For each auxiliary construction, enumerate field elements $x$,
retain only those for which $x^3+Ax+B$ is a square on **E**, and choose
target points $F_i=(x_i,y_i)$ and $-F_i$. Every recorded row comes from a
checked equality $aP+bQ=\sum_j\epsilon_j F_{i_j}$ on E. In logs, it is
$bk-\sum_j\epsilon_j\log_P F_{i_j}=-a\pmod r$. Since E has prime order,
each nonidentity $F_i$ lies in its target subgroup and has nonzero scalar
modulo r. No auxiliary-point map is treated as a group homomorphism. The
only map needed is field x selection followed by target-curve lifting.

The torus arm uses the norm-one subgroup of $\mathbb F_{p^2}^{*}$, of order
$p+1=128$ here, and $x=z+z^{-1}$. A fixed order-8 coset has eight distinct
images; the checked Dickson recurrence $D_8(x)=c$ characterizes its fiber.
The unrelated elliptic arm uses an order-8 coset on $y^2=x^3+x$, then its x
coordinates. The twist arm uses an order-5 coset on the target quadratic
twist; normalization to the twist's $d y^2=f(x)$ model yields zero target
lifts, as quadratic characters require. The shifted twist arm applies the
first field shift giving two lifts. That shift has no homomorphism property;
its rows are valid only through the target-curve check. Both signs and all
duplicate pair representations are retained as rows, while matrix rank counts
independent information.

## Native panel and falsifiers

The frozen protocol is in [PROTOCOL.md](PROTOCOL.md) with the native stream
amendment in [AMENDMENT_NATIVE.md](AMENDMENT_NATIVE.md). Public fixture:
$p=127$, $E:y^2=x^3+x+26$, $r=131$, $P=(0,36)$, $Q=(12,49)$,
and withheld verification scalar 37. Each eligible arm uses two x columns,
both signs, pair arity 2, the same 1024-query stream, and stops at rank 3.
The independent Rust replay checked every point identity, coefficient/sign,
rank and recovered scalar. Raw stream, bases, rows, summary and rho trials are
in [native_run_v1](native_run_v1/). These are **measured local diagnostic**
counts. Group additions count the native runner's scalar arithmetic and row
point verification, but omit x-lift tests and finite-field setup. Wall times
are excluded from comparisons because this was not an EKS-controlled run.

| Arm | Raw fiber | Target x lifts | Queries to rank 3 | Raw rows | Target additions | Auxiliary additions | Verdict |
|:--|--:|--:|--:|--:|--:|--:|:--|
| small x, matched base | 2 | 2 | 10 | 6 | 206 | 0 | control |
| p+1 torus | 8 | 5 | 32 | 3 | 613 | 0* | cost gate fails |
| unrelated curve | 8 | 3 | 35 | 5 | 676 | 84 | cost gate fails |
| raw twist | 5 | 0 | 0 | 0 | 0 | 34 | lift gate fails |
| shifted twist | 5 | 4 | 29 | 4 | 560 | 34 | cost gate fails |

*Torus finite-field work is uncounted in the addition column. Twist shift
search, x lifts, pair-table creation outside target additions, and linear
algebra are also not fully priced. Thus the auxiliary total costs are strictly
higher than the displayed lower bounds. The rho control has 16 verified runs,
29–113 counted group additions (median 71.5), using a plain three-way walk.
The matched small-x arm is an exhaustive arity-2 relation control, with the
same two factor-base columns; the auxiliary variants have no measured yield
advantage at this size. The plot [counted target work](figures/work.svg) shows
the lower bounds and rho range.

**Cheap falsifier per candidate.** Torus: verify exact norm-one order, eight
distinct Dickson roots, then count target lifts and independent rows per
complete work. Unrelated curve: verify smooth subgroup/coset and distinct x
fiber, then target lifts and row yield including auxiliary setup. Raw twist:
the target-lift count is zero for the normalized x values. Shifted twist:
check all shifted target lifts and charge the shift scan; its valid rows still
lose the counted target-work screen. These results apply to this fixture and
stream; they do not prove a general lower bound.

## Published-method and larger-curve screen

[Petit, Kosters and Messeng, PKC 2016, §§3.1–3.5](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf)
already describe smooth auxiliary curves as *factor-base constructors*. Their
summation-polynomial relation must still be solved on the target curve.
[Semaev's summation-polynomial method](https://eprint.iacr.org/2004/031)
provides that target relation condition. Our arity-2 exhaustive pair table is
only a small-field oracle for the relation step, not a measured Gröbner or
resultant implementation. At fixed two-column base, the three auxiliary arms
need 29–35 queries for rank, versus 10 for the small-x control. The larger
fiber alone does not enlarge the matched base.

The existing [factor-base encyclopedia](../../docs/ic/factor-base-encyclopedia/README.md)
records prior Dickson and isogeny measurements, including a P-256 Dickson
tower cost model reported there as 394.5 times its rho baseline. That is
**prior evidence**, not a measurement made in this PR, and its workload and
model differ from this exact panel. A P-256 scaling claim would require a
fresh, same-target EKS run with polynomial solving, setup, recombination and
linear algebra charged. AWS credential validation returned
`InvalidClientTokenId` in this session, so the repository's EKS performance
gate could not run. The local count screen rejects all new arms before that
expensive stage, and no new relation solver was built.

## Reproduce

From the repository root, run
`cargo test --manifest-path research/pplus1_aux_twist_relations_20261125/native/Cargo.toml`,
then `cargo run --release --manifest-path research/pplus1_aux_twist_relations_20261125/native/Cargo.toml --bin prime-aux-relations-panel -- /tmp/prime-aux-run`,
then `cargo run --release --manifest-path research/pplus1_aux_twist_relations_20261125/native/Cargo.toml --bin verify-relations -- /tmp/prime-aux-run`.
The old Python files and JSON remain under [historical-python](historical-python/)
for provenance; they are excluded from the native result.
