# Full trace-zero variety and ECC2K-130 integration

The production library now provides full group-trace membership, point trace,
idempotent projection, an exact rational-point coordinate codec, three actual
Boolean polynomial models, factor-base intersection, and a native S3 relation
oracle. These include the second membership bit omitted by an x-trace-only
model. Local checks and the root library type-check pass [pcc:unverified].

The global algebraic variety is defined and established below. The exported
equations represent **all of its F2-rational affine points**, with the identity
handled separately. They are not a geometric defining ideal or a birational
chart over arbitrary F2-algebras. A matched natural ECC2K-130 relation-finding
comparison remains open; the bounded degree-83 x-only control returned UNKNOWN.

## Requirements and evidence

| User requirement | Delivery and status |
|---|---|
| Integrate full trace zero | Production `TraceZeroVariety`, both membership bits, full point trace, projection, coordinates and SAT integration; locally checked |
| Establish variety | Global functor, geometric splitting and dimension argument below; sourced mathematics and written derivation, no Lean proof or independent signed standing |
| Connect index calculus | Exact factor-base intersection plus `full_trace_zero_decompose`; complete intermediate-identity patterns, original-group replay and explicit caps |
| Validate ECC2K-130 | Actual degree131 equations, published G/Q membership and point-trace checks, torsion rejection and fully pinned SAT witness replay |
| Shrink relation systems / leverage beyond orbit trick | Actual Boolean model counts supplied; natural relation finding, matched solving degree, yields and performance remain unmeasured |
| Make PR | Updates existing PR3744; no merge or deployment |

## Global construction and proof map

Let k=F2, K=F_(2^n), n odd, and E=E0/k with projective equation
Y²Z+XYZ=X³+Z³, identity O=[0:1:0]. Set W=Res_(K/k)(E_K).
For every k-algebra R, W(R)=E(R tensor_k K). There is a descended group
morphism t:W -> E: over an algebraic closure it is the sum map from E^n to E.
Define the **full trace-zero variety A=t^(-1)(O)**, as a scheme-theoretic fiber.

Over the algebraic closure, the map
(P1,...,P_(n-1)) -> (P1,...,P_(n-1),-sum(Pi)) is an isomorphism
E^(n-1) -> A, inverse given by projection onto the first n-1 factors.
Consequently A is smooth, connected, projective and geometrically integral,
and is an abelian variety of dimension n-1. This geometric splitting need
not descend to a product over k. Its k-points are exactly
T_n={P in E(K): sum_(i=0)^(n-1) tau^i(P)=O}.
This establishes the global object without identifying a Boolean point-set
ideal with its positive-dimensional geometric defining ideal [pcc:unverified].

E(F2) is cyclic of order four. The trace restricts to multiplication by n
on it, hence is onto for odd n. Its kernel has order #E(K)/4. The unique
nonidentity geometric point of E[2] is (0,1), is rational over F2, and has
nonzero trace for odd n; thus T_n has odd order. Therefore T_n is precisely
the odd part 4E(K). The exact sequence and restriction-of-scalars setting
are standard; see Gorla–Massierer section2 and the prior study below.

For an affine point on E0, the existing production halving derivation gives

~~~text
P in T_n = 4E(K) iff Tr(x)=0 and Tr(y+x H(x))=0,
H(x)=sum_(j=0)^((n-1)/2) x^(2^(2j)).
~~~

Indeed a half has lambda²+lambda=x, so its existence requires Tr(x)=0.
Take lambda=H(x); its x-coordinate u satisfies
u²=y+(lambda+1)x. Then Tr(u)=Tr(y+xH(x)) when Tr(x)=0.
The half is itself halvable exactly when this second bit vanishes.
O is included explicitly. (0,1) passes the first bit and fails the second;
the order-four points fail the first. No new membership theorem or novelty
is claimed. See `BinaryCurve::is_in_4e` and its King / Eagle–Galbraith–Ong
references. The new code constructs and uses these equations.

For any P, let S=t(P). The projection is P-[n mod4]S, since n is its own
inverse modulo4; it fixes T_n and is idempotent. The different map
tau(P)-P is the cofactor image, not an idempotent projection.
The standard identity (tau-1)(1+...+tau^(n-1))=tau^n-1 explains its image.
Certificate **IDC1h3fcd7aa52221c17e** records the degree131 telescoping
polynomial identity over 2^61-1, with32 issued plus32 replay points.
It is not a characteristic-two membership certificate or formal proof.
The written telescoping proof is cancellation of adjacent powers over Z.
All new checks are local producer evidence [pcc:unverified].

## Actual descended rational-point equations

Work in the polynomial basis of the validated irreducible field polynomial.
Write Tr(x)=sum(t_i x_i), q(x)=Tr(xH(x)); t_0=1 for odd n.
Field squaring and half-trace are F2-linear, so q is quadratic Boolean ANF.
Expand y²+xy+x³+1 into n field-coordinate equations. Variables are Boolean:
the ring is F2[v]/(v_i²+v_i), not an ordinary unconstrained polynomial ring.

| Mode | Variables | Equations | Boolean degree bound | Substitution |
|---|---:|---:|---:|---|
| Raw | 2n | n+2 | 2 | Full x,y plus both trace equations |
| Compact | 2n-2 | n | 3 | x0=sum_(i>0)t_i x_i; y0=sum_(i>0)t_i y_i+q(x) |
| Factored | 2n-1 | n+1 | 2 | Same x0; y0=sum_(i>0)t_i y_i+h; h+q(x)=0 |

These substitutions are reversible on satisfying affine points; the codec
checks curve and full membership during reconstruction. Together with O
they give the full rational-point set. They do not parametrize arbitrary
2n-2 bit strings by points. Boolean field equations are implicit, so n
displayed equations do not imply a geometric dimension of n-2.

Constructed counts, not solver timings (one model for each row):

| n | Mode | Variables | Equations | Term occurrences | Max degree | Distinct AND auxiliaries |
|---:|---|---:|---:|---:|---:|---:|
|83|Raw|166|85|53707|2|10292|
|83|Compact|164|83|63644|3|20468|
|83|Factored|165|84|53042|2|10127|
|131|Raw|262|133|100717|2|25676|
|131|Compact|260|131|117552|3|43517|
|131|Factored|261|132|99684|2|25415|

The compact elimination removes two input bits but can add monomials and
SAT auxiliary gates. It supplies no measured performance advantage.
The earlier x-only chain counts182/178 concern another representation and
are not comparable with these full x/y single-point models.

For ECC2K-130, n=131 and f(z)=z^131+z^13+z²+z+1. The geometric dimension
is130 and |T_131|=680564733841876926932320129493409985129, already the
challenge subgroup. Removing the cofactor does not reduce that subgroup.
See [summary](ecc2k130-factored-summary.json), [diagram](construction.svg)
and [PDF](report.pdf) [pcc:unverified].

## Integration and contracts

- `src/binary_ecc/trace_zero.rs`: public model/codec/trace/projection APIs.
  Admission checks E0, odd3<=n<=131, Rabin irreducibility and declared
  total cardinality. A proper selected odd subgroup is allowed; full kernel
  cardinality remains #E/4. Infinity has a separate coordinate variant.
- `src/cryptanalysis/koblitz_index_calculus.rs`:
  `build_full_trace_zero_subspace_factor_base` intersects the original
  points with T_n and reduces the basis to V intersect ker(Tr).
  `full_trace_zero_decompose` invokes the native full-point oracle.
  These explicit APIs are opt-in; no existing CLI strategy is switched.
- `src/cryptanalysis/binary_semaev_chain_sat.rs`: exact BigUint circuits now
  admit degree131, attach both full membership equations and original curve
  coordinates to each summand, and replay them. Generic u64 FieldStructure
  degree guards remain. Raw mode is used by this SAT adapter.
- `src/cryptanalysis/trace_zero_relations.rs`: arity2..9, all2^(m-2)
  intermediate-identity patterns, finite x-domain constraints, all actual
  base lifts by x, and final original-group sum. A rejected x tuple is
  blocked across all its possible lifts. Only exhaustion of all patterns
  yields Refuted; allocation/domain/model/conflict/pattern/deadline caps
  return Inconclusive. Returned indices refer to the supplied base.

ANF-to-SAT emits one shared AND variable for each distinct monomial prefix
and an XOR row for each equation. Both wire validity and total allocation
are preflighted. This is the direct logical equivalence of AND gates and
ANF rows, additionally tested against exhaustive finite-field models.
Known full-point-pinned m=3 witness P+(-P)+P=P uses an identity intermediate:
n83 gives31540 SAT variables/92296 clauses/421 XORs;
n131 gives78076/230560/661. These counts include full membership constraints,
but do not measure an unpinned relation search [pcc:unverified].

## Validation, retained attempts and reproduction

The focused facade imports the actual production arithmetic, curve, SAT,
S3 and full-model source. It contains no replacement arithmetic or mocked
group types. Dependencies build without their unrelated unit-test graphs.

~~~sh
cargo test --locked --manifest-path research/full_trace_zero_variety_20261109/native/Cargo.toml -- --nocapture --test-threads=1
cargo check --lib --no-default-features
cargo run --locked --manifest-path research/full_trace_zero_variety_20261109/native/Cargo.toml -- factored /tmp/new-full-model.json
cargo run --locked --manifest-path research/full_trace_zero_variety_20261109/native/Cargo.toml --bin certify_trace_identity
~~~

The exporter uses create-new semantics. The optional file contains each
equation as a list of monomials, each a sorted list of zero-based variable
indices; empty monomial means1. All equations equal0, operations are XOR/AND.
Raw ordering is x0..x_(n-1),y0..y_(n-1); compact/factored ordering is
x1..x_(n-1),y1..y_(n-1), with the helper last in factored mode.
Without the path it prints only compact summary metadata. The generated
large equation file is intentionally not committed.

Ten local tests cover exhaustive small kernels n3/5/7/9 with sizes1/11/29/127,
all assignments for raw/compact/factored at n3/5, actual n83/131 models on
public fixtures and torsion cosets, malformed admission/caps, exact factor
base intersection, degree131 order-two UNSAT, wide fully pinned SAT witnesses,
complete n5 relation comparisons and certificate replay/tamper rejection.
The tenth test is a **bounded x-only control**: it accepts UNKNOWN as a retained
budget outcome, not as a solution. At n83, one second/2000 conflicts gives
UNKNOWN. No unpinned degree131 natural relation is reported.

The earlier unbounded wide control was interrupted with SIGINT after90+
seconds. Its raw output is retained in
[checks/interrupted-wide-control.txt](checks/interrupted-wide-control.txt).
The replacement keeps the bounded UNKNOWN separate from fully pinned SAT
controls. Exact commands and the environment are in [checks/commands.txt](checks/commands.txt),
test output in [checks/native-tests.txt](checks/native-tests.txt),
root type-check in [checks/root-cargo-check.txt](checks/root-cargo-check.txt),
and dependency hashes in [SOURCES.sha256](SOURCES.sha256).

Flow: full model construction, exact rational membership and local correctness
checks attempted and completed; initial unbounded wide control interrupted;
bounded control completed UNKNOWN; natural matched relation finding, timings,
solving degree and full DLP not attempted here. Local builds are development
validation, not benchmark records. Experiment execution remains blocked on
task-specific checkpoint/S3 credentials; no EKS or paid job was launched.
Earlier whole-root test-suite failures and the unrelated benchrec gate remain
documented on PR3744; this successful library type-check does not clear them.

## Sources and standing

Sources checked (KB): connector absent; repository wrapper exits127. Fallback:
existing `BinaryCurve::is_in_4e` derivation/prior-art links, primary
[Gorla–Massierer](https://arxiv.org/pdf/1405.1059),
[trace-zero representation caveats](https://arxiv.org/pdf/1403.0126),
[Kosters–Yeo](https://arxiv.org/pdf/1503.08001), and
[Galbraith–Granger–Merz–Petit](https://sacworkshop.org/SAC20/files/preproceedings/18-IndexCalculus.pdf).
The [prior matched-layout audit](../trace_zero_relation_check_20261109/README.md)
remains a separate result with its original limits.

No new formal proof or signed independent PCC verdict is supplied. The
integral telescoping certificate cannot certify the characteristic-two
Boolean model; its evidence is the written construction and local checks.
Remaining global software work is an explicit scheme-level descended ideal
or chart valid over arbitrary algebras. Remaining performance work is a
matched natural relation-finding run with complete budgets and original-group
replay. No claim of that scope is made [pcc:unverified].

Canonical graph review: docs/index-calculus-scoreboard.html,
docs/ic/progress-timeline.json, docs/curves and docs/performance-gains retain
their existing values because no measured performance/ranking changed. The
previous study's layouts.svg remains historical; this study's construction.svg
shows the new point-model counts. No website or scoreboard publication occurs.
