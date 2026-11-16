# Additional ordinary-curve controls for the critical guessing cells

2026-10-25, before new curve/fixture output. Preserve every original b=1
fixture, selection and active run. These additional controls keep the
critical (m,n,ell) cells (3,22,8), (3,25,9), (4,17,6), (4,18,6), and the
four basis families. They change the curve coefficient b and target law;
they are not a same-instance speed comparison with the earlier fixtures.

For each field use the existing native random_binary_instance(n,seed,16)
factory. Starting at seed 202610250100+n, examine at most 64 successive
factory seeds. Retain every returned model/rejection or constructor failure;
choose the first a=0 model with b of full field degree and a certified
odd prime subgroup. Choosing a=0 avoids an automatic trace-class support
failure of trace-zero subspace bases on the cofactor-two a=1 model. Actual
factor-base support must still be checked; do not assume nonemptiness.
The native factory's internal candidate enumeration is source-pinned but
not exposed as an independently logged curve-space census or priced here.

Freeze curve/source/binary identities and the exact field polynomial,
a,b,#E,r,h,G. Independently check primality, #E=r*h, gcd(r,h)=1, nonzero
G on the curve and [r]G=O. The factory counts every field abscissa exactly;
this is fixture preparation, not a cheap or fully priced IC construction.

Generate 128 draws per family/cell, with basis seed 202610250003 and
target seed 202610250001. Use the existing make_basis implementation.
Targets are paired across families at each (m,n,ell,draw). Hash the curve
identity and those indices, excluding the family and basis, to a uniform
field-x draw and an independent y-root choice. Lift, clear the cofactor,
reject O, and verify [r]R=O, with at most 4096 hash attempts. No scalar is
supplied to the decomposition solver or known by this target constructor.

Record all 2,048 planned cases, including SAT, UNSAT, empty bases, direct
hits, cap/failure outcomes and duplicate x-only queries. For every case,
count the finite S3 chain using the native b-aware quadratic oracle;
independently count actual ordered signed prime-subgroup decompositions
by group MITM, including repeated points and infinity intermediate sums.
Verify each returned group witness by actual additions. These two counts
have different contracts and need not agree: finite-chain variables do
not represent infinity intermediates. Preserve both classifications.

Predeclared follow-up selection per file: first four distinct x-only
queries with Boolean count zero, actual point count zero, at least two
base points and no direct base hit; first two with both Boolean and point
counts positive as true SAT controls. Retain quota shortages explicitly.
No extra draws, cancelled cases or favourable-only means are authorized.

Each file is the original experiment's natural cell/family input unit,
now with 128 draws. Apply artifact thresholds to the actual files; do not
split them to evade the store policy. Fixtures are deterministic; keep
resource/timing diagnostics separately. Curve construction and each
cell/family generation use a 2-GiB address-space and 900-second local cap.
All prices and complete-DLP costs remain unknown. No paid VM is launched.

The b-aware chain/count helpers retain b=1 defaults for old inputs, and
qualified inputs must explicitly carry curve_b. Check nonunit b against
the independent library for m=3--6 and small fields before generation.
Build a separate frozen instrument for future generic-b jobs. Never feed
these fixtures to the old b=1-only frozen instrument. All larger closure,
all-branch guessing, lifting/verification costs and qualified performance
requirements remain open until actually executed and independently replayed.

Exact deterministic encoding, frozen before output: SHA-256 domains are the
literal UTF-8 strings `ic-qualified-basis/v1`, `ic-qualified-target-x/v1`,
and `ic-qualified-target-sign/v1`, each followed by one zero byte. Append
all integer words as u64 little endian, then the UTF-8 string byte length
as u64 little endian and its bytes. For the basis the words are
(202610250003,m,n,ell,draw), and the string is the family. Its first eight
digest bytes as u64 little endian seed make_basis. For both target hashes
the words are (202610250001,m,n,ell,draw,attempt), and the string is the
qualified curve record SHA-256. The first eight x-digest bytes masked to n
bits give x; bit zero of the sign digest selects the library's ordered root
list (modulo its length). Attempts start at zero. The curve digest is the
native canonical UTF-8 JSON hash before adding its own digest/EC1 fields;
the exact record, generator and unregistered EC1 record id are retained.
The native library's packed point key distinguishes both signs and O at
these n<=25 fields; all equal-key point matches are additionally checked
for exact coordinate equality by the MITM. Group counts and witnesses use
this point key, never just x. No numerical results exist at this freeze.
