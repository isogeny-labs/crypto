# Received-symbol curve identity and bounded quotient

24 November 2026. Result scope: conditional algebra and one fixed native
interpolation box on original Curve25519 torus-coset rows.

The55-column evaluation matrix has rank54 and one kernel direction, exactly
the universal received-symbol identity. Removing that direction leaves a
54-column space of rank54 in both recorded global sign orientations. All216
fixed raw/map/eightfold-image replays agree. The signs are related checks,
not independent experiments. [pcc:PCC-20261124-fce173]

## Algebra and hypotheses

For the raw Montgomery point v²=u³+A*u²+u, set w=-u*v and tau=z.a!=0.
The raw branches are u=(-A+A*epsilon/tau)/2, epsilon²=1; characteristic2
is excluded in the guarded chart theorem. Write beta=A²(tau²-1)/(4tau²).
Then u²+A*u+beta=0 and

    w²=(1-beta)*((A²-beta)*u+A*beta)
    F=w⁴+A(1-beta)(A²-3beta)w²+beta³(1-beta)²=0.

With J=4tau² and B=A²(tau²-1), clear denominators:

    G=J⁵w⁴+A*J³*(J-B)*(A²J-3B)*w²+B³*(J-B)²
    H_epsilon=32tau⁵w²-A³(epsilon-tau)³*(J-B)
    G=H_(+1)*H_(-1).

Four explicit Lean declaration bindings cover the received quartic, its
clearing, the guarded branch quadratic and branch factorization. The focused
project gate has4named/0trusted endpoints. Local PCC verification passes
inside the pinned Lake environment. Bare file-target invocation lacked the
Mathlib search path; its failed record remains separate. The declarations
are generic conditional CommRing/Field algebra, with standard axioms, and
leave native arithmetic, group/compiler semantics and native rank uninstantiated.
[pcc:PCC-20261123-09b3cb]

The separate maintained identity record IDC1h5d7a5b64ec47d105, described in identity-v1/README.md,
evaluates the unconditional branch-factor polynomial over its format's
2^61-1 field. Its optional Lean source pointer is not executed by that
checker. It accepts32+32 points at syntactic degree20, rejects plus-one
checking and false issuance, and preserves an existing-output collision. Its
22 maintained module tests/fmt/build pass; strict Clippy retains five inherited
canonical SHA style findings without an exemption or library edit. Neither a universal equation nor its factorization discovers a
target-dependent section. No irreducibility or function-field normalization
is inferred. [pcc:unverified]

## Exact native box and evidence

Original parameters are p=2^255-19,A486662,a60824497,q213156431,K20633,
j1,t0..107. The metadata fixture is a labelled projection of the prior frozen
rank capture; original full auxiliary witnesses remain in that capture.
Current native controls replay norm/power/labels/map/curve/eightfold image
and the received-symbol role on each recorded sign. They do not create a
fresh original Circuit qualification. [pcc:PCC-20261124-fce173]

The declared monomials tau^i*w^j have0<=i<=10,0<=j<=4, ordered w-major.
The coefficient of tau^10*w^4 in G is1024, nonzero in the actual field.
Subtracting that G multiple deletes this one pivot and gives the54-column
vector-space complement. This is a bounded complement, not a complete ideal
membership or Gröbner procedure. Full matrices, kernels, selected original
minors, determinants and inverses are retained. Gaussian and separate Bareiss
recurrences agree, sharing field primitives/producer custody. Native small
controls include9 monic shifts,36 corruptions,6 admission refusals and a
known determinant23mod101. [pcc:PCC-20261124-fce173]

| Recorded orientation | Rows | Ambient columns/rank | Kernel dimension | Complement columns/rank |
| --- | ---: | ---: | ---: | ---: |
| Canonical raw sign |108|55/54|1|54/54|
| Global opposite sign |108|55/54|1|54/54|

[diagram.svg](diagram.svg) and [ranks.svg](ranks.svg) render these retained
values; render.py performs layout only. No timing, fitted scaling, uncertainty
interval or total cost is measured here. Canonical scoreboards and existing
curve/performance diagrams are unchanged: no admitted performance datum or
new map edge is added. [pcc:unverified]

## Flow, custody and acceptance

One final native call after source publication and immutable qualification
exits0 with221 records,2 sign boxes,108 parameters,216 raw replays and zero
decoder calls/new roots. Three release tests,fmt and strict all-target Clippy
passed before qualification. No native source edit/test followed it. The
102-input context is frozen point95 plus7new inputs in FOUR constituent
archives, not one monolithic archive. Compiled5 hashes and the retained binary
pre/post match. Distinct write-once BUILD/PRECALL/POSTCALL and shell positive0/
collision1 records remain. [pcc:PCC-20261124-fce173]

Full raw1498854bytes SHA6cb32aef661f4d2638ca97910301ae9f801658c59cbf0cc7b4a0aa695a4b6c2c
roundtrips through one350820-byte archive SHA549660a3323bd56694c49dc8a1e27f580b16f022aa7f93421290632d73f3bfa6.
Actual artifact policy selects it; inspected bytes remain local/untracked
until a genuine store receipt and replay. The native statement binds source,
summary and inventory, not unavailable remote raw bytes. All standing is
producer unsigned; independent signed rank/field validation remains open.
PUBLISHING.md records the unchanged store, journal and immutable-record
dependencies. No credential or upload retry was made. [pcc:unverified]

Mathlib's maintained full-cache prefetch exhausted disk. The final gate
completed after moving only a rebuildable cache to RAM with full per-file
readback; dependency source pins and research evidence stayed unchanged.
CACHE_RECOVERY.json identifies the inventory. The RAM cache is ephemeral,
so a later environment may need to rebuild it. Failed cache/import/build
attempts are retained. [pcc:unverified]

| Requirement | Current evidence / remaining step |
| --- | --- |
| Received-symbol coefficient identities | Four conditional Lean endpoints; native fixed controls |
| Separate automatic curve kernels | Bounded55/54 quotient classified in two related sign cases |
| Useful target-dependent section constraint | Open; return to normalized five-coefficient selection |
| q,K-dependent discovery/cold cost | Open; require a non-enumerating selection/root rule |
| Useful yield,family,private selection,rank,kernel,recovery | Original contract remains open |
| Full raw remote publication | Pending genuine access/receipt/replay |

The tested box supplies no nonuniversal normalized kernel. This rejects
counting its universal kernel as a section constraint, scoped to these
native rows and this box. Larger/weighted/branch-aware spaces and other
coordinate/order/list routes remain separate. NEXT.md retains the actual
unknown-section acceptance object. [pcc:unverified]

## Sources checked

Original map/index and section-normalization sources; existing theory-v1
energy draft, which already contains the genus-two cover proposal; Cheng,
[cs/0507026](https://arxiv.org/pdf/cs/0507026), section2/Theorem1 for the
divisor/subset-sum context; official [FLINT division/resultants](https://flintlib.org/doc/fmpz_mpoly/index.html);
the pinned Mathlib monic-division/evaluation laws listed in INSTANCE_GATE.md;
and the maintained identity-certificate guide. The Cheng hardness family is
not a lower bound for this fixed algorithm. KB unavailable: wrapper127 and
intended query are retained in PROTOCOL.md. No novelty for classical
elimination, quotient arithmetic or interpolation is claimed. [pcc:unverified]
