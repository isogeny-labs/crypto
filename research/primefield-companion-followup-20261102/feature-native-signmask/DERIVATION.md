# Native zero-and-parity compilation of signed character monomials

## Construction

For signed values s_a in {-1,0,1}, define Z with bit a iff s_a=0 and N
with bit a iff s_a=-1. For a monomial with an offset multiset A, compile
U_A as the OR of offset bits and V_A as their XOR. Then

    product_{a in A} s_a = 0                        if U_A & Z != 0
                          (-1)^popcount(V_A & N)  otherwise.

Proof: any zero factor annihilates the product, including an even exponent.
Otherwise each negative factor contributes one sign flip, and only the parity
of its multiplicity matters. For the empty monomial both masks vanish and
its value is one. Distinct support and odd-exponent masks are necessary:
using XOR alone would incorrectly make s_a^2=1 at s_a=0.

`Program::new` validates a caller-supplied coefficient packet once and compiles
private immutable masks. It does not certify primality or polynomial identity;
those remain the caller's obligations. `evaluate` receives only an offset-to-i8
oracle, queries the complete prefix in order, and accepts exactly -1,0,1.
Expected scalar labels and points do not enter Program. It neither parses nor
allocates during evaluation. Width0..64 is supported; larger widths are rejected.
Coefficients are canonical modulo r, with 3 <= r <= floor(u64::MAX/2). The
algebra works over this residue ring; elliptic application still requires its
prime/subgroup assumptions. Canonical addition stays below 2r <= u64::MAX-1.
No elliptic source, hidden scalar, lookup search, or feature synthesis is added.

## Exact costs and dependency

Let T be term count and L the sum of monomial lengths with multiplicity.
Construction: read L offsets, form OR/XOR masks, validate T coefficients; store
T triples (coefficient, support mask, odd mask). The existing compiled vector
owns its storage; input vectors are not retained. Width is at most64, an explicit
bound, not an arbitrary-width compressed representation claim.

Evaluation: q oracle calls and ternary validation/bit setting; T zero-mask
intersections; at most T negative-mask intersections and population counts;
at most T modular additions and sign changes. No coefficient-by-sign modular
multiplication remains. Integer/bit work, branching and source costs remain.
All terms, including zero coefficients, are retained. For the fixed repaired
program T=477 and L=1291, from its public offset lists; q=16. These are derived
counts, not timings or new experimental samples. The source oracle cost is
unchanged. No claim of reducing coefficient synthesis or the477-term storage.

## Checks executed

At precheck commit82fd48f0a683c54f6bb6b217fc2188ee3a276400, rustc1.99.1 compiled
the dependency-free unit target. One test invocation passed3 methods:

- All27 ternary inputs for a fixed polynomial containing every ordered monomial
  length0..4 over3 coordinates (including repeated offsets), compared to direct
  modular multiplication. This tests that aggregate polynomial, not every
  coefficient assignment independently.
- All479 square-set character inputs of the preserved477-term program: equal
  to direct u128 multiplication and to X; exact ascending16-call order.
- Canonical coefficient/modulus/width/offset/sign rejection, width64 high bit,
  largest admitted modulus, zero coefficient, and constant-only width0.

No fresh elliptic source or scalar recovery cohort was run. Unit-test wall
text is diagnostic only and is not a benchmark. The prior Python contract
receipts remain immutable. TSV is a jq-only projection of the exact hashed
program JSON. `check-lock.sha256`, `check-receipt.txt`, build/test logs and
manifest preserve execution provenance. No workspace Rust test pass is claimed;
only this standalone affected native unit target was compiled and tested.

## Next construction

Expose the native public sign-oracle boundary in the fresh-source runner and
port the original elliptic source without changing fixtures or query order.
For q>64, use word arrays and price ceil(q/64) mask operations per term, or
sparse masks grouped by used words. Neither implementation exists here. The
full4563-case protocol retains all prior curves and arms and still needs its
native source/runtime publication and authoritative EKS admission.

## Sources

- Preserved `feature-decoder-binding/preserved_decoder.py`, repaired program
  and source-oracle contract: the exact computation being replaced.
- [NIST DLMF27.9](https://dlmf.nist.gov/27.9): quadratic character values.
- [Rust u64](https://doc.rust-lang.org/std/primitive.u64.html#method.count_ones):
  population-count and fixed-width integer API.
- [Dumas-Pernet2018](https://arxiv.org/abs/1802.10453), revisited during lane
  selection: factorization addresses a different construction cost; no
  rank-profile or asymptotic guarantee from that work is imported here.

The sign identity is elementary algebra and bit encoding; no novelty claim.
