# Protected-target integration of the adapted two-LP finder

Selected after the published fixed-cohort result28: rank29 for the existing
full-smooth reduced-base control and rank114 for adapted2LP plus graph.
That cohort does not supply a complete basis or an original logarithm.
[pcc:unverified]

Sources checked(KB): protectedtarget/descent and negativekernel/modulus
queries unavailable, /tmp/ic-two-lp-dlp-kb-search{1,2}.log. Read JVprimary
§3.3/§6 and existing Dec6/Nagao/verify_dec, public spec_from_curve,
generic affine graph certificates and RelationSystem. The latter maintains
incremental reduced row echelon form and reports when a target coordinate
is uniquely determined. This is reuse, with no novelty claim.
[pcc:unverified]

For a verified residual decomposition aG'+bQ'=sum epsilon_i B_i, retain
small-base coordinates, put omitted abscissae in the LP namespace, and put
the target at protected small column small_count with coefficient-b and
right-hand side a, all modulo the prime subgroup order. Reconstruct and
verify the Jacobian equation before acceptance and again before filtering.
Original sieve rows stay homogeneous. Six-point descent rows may contain
up to six LPs; the existing hypergraph filter supports this case.
[pcc:unverified]

The filter handles original partial and affine descent rows together. It
replays every output's source combination and RHS before returning rows.
The target never enters the LP namespace. A native test uses an actual
Jacobian affine-equation fixture to check the protected target, nonzero RHS
and damaged-source refusal; it deliberately does not claim elliptic
transport or a full ECDLP result. [pcc:unverified]

The full attack rebuilds a cover/base, collects a bounded unique degree-nine
B cohort through the verified2LP API, then searches verified Nagao descents.
After two accepted descents it filters/replays and inserts complete rows
into the existing incremental RelationSystem. More accepted descents can
resolve a still-underdetermined target; row count is never treated as rank.
A determined candidate must pass actual elliptic [d]G=Q. A failed projected
certificate or determined scalar replay is recorded and stops this path.
There is no original full-smooth solve fallback. [pcc:unverified]

The library API accepts an explicit bounded option set. None for b_count
means one enumeration of the finite degree-nine B space. Default option
values do not constitute an experiment specification: the actual
small fraction, B/residual/descent bounds, instance/seed and limits must be
frozen separately before any scientific call. Collection caps, rejected
candidates, duplicate work, every projection/solve attempt, stream advances,
all original sources and accepted descents remain in the report. The final
proof contains affine inputs and every output certificate; earlier attempts
are reconstructible from the original rows and recorded descent prefixes.
[pcc:unverified]

The public-input Rust example separates fixture generation from workers.
Only public field/curve/order/G/Q are deserialized by the workers, with
unknown fields rejected, including nested point fields. The planted scalar
stays in the parent fixture. Workers rebuild transfer with Spec.d=0 and
return the actual recovered scalar, checked on the elliptic curve. Original
IC uses the same public challenge and pinned incidence/shared recipe.
Nested work uses one claimed Rayon thread. [pcc:unverified]

Cold counters cover actual transfer/model/base/relations/descent/graph/solve
and scalar replay. Legacy unit-report calibration is separately emitted
as excluded harness primitives/sampling; captured process wall includes it.
Selected modular RNG events and known native integer
prices stay separate. Native containers, generator internals, incremental
RREF scans/fixed-factor internals and cohort hashing/serialization remain
explicit gaps; every complete total/comparative flag stays null/false.
No speed, strongest-external, fullDLP or rho-parity result is asserted before
the separate prospective full-recovery protocol. Native source checks pass:
22sieve/2LP/target tests(twoexisting ignored),12generic/MSTfilter,3existing
incrementalRREF controls,newpublicinput1/oldworker1,actualthreeexamplebuilds
andstrictlibrary/threeexampleClippy. The metadata-only calibration emission
was separately rebuilt/retested/linted. Standalonefull qualifier14checks
(12shared+2new) andoriginalcomponent12/build/strictClippy pass. Original28
summary replays byte-identically after sharing helper visibility. The actual
public-target call is separately frozen in Amendment24 and not yet run.
[pcc:unverified]
