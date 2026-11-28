# Seven-draw RNG state equivalence

This is exact source-model analysis of an existing published reconstruction,
not a new curve search, primality replay, point count or scientific launch.
It tests a necessary link in the proposed explanation of the seven discarded
FreeLIP outputs before six known May1997 base points.

## Sources checked before implementation

- Original FreeLIP1.1 `lip.c` at lrobot/freelip commit
  `00898a4814d2cc1244bf348682be6ed10dda35f0`, Git blob
  `a86ce8ae5be49e0f35999a2c87f52e1718ebdb81`: zrstarts/zrandom,
  zcomposite, zmcomposite, zprobprime and zrandomb.
- [Published reconstruction](https://www.anthropic.com/research/ecdsa-curve-origins#claude-found-some-base-points-too):
  the six recovered seeds and seven-discard recipe; code dedicated CC0.
- [Independent seed enumeration](https://words.filippo.io/seeds-solved/) [pcc:unverified]:
  order-bit-width screen and unexplained remaining eligible positions.
- Existing `../freelip_replay_20261027/{PROTOCOL.md,REPORT.md,source-audit.json}`
  already proposes seven primality calls as a possible mechanism and records
  all21correct orders accepted. This analysis makes that existing conjecture
  more precise; it is not a novelty claim or a new historical reconstruction.
- Existing generator/b comparison and exact allocation audit. Their source,
  results and proof subjects are untouched. KB/Conductor binaries and MCP are
  absent locally; repository sources and the primary source are the fallback.
  `docs/CONDUCTOR.md` documents the unavailable-binary warning fallback.

## Fixed question and checks

For M=2^107−1 and g=3^121 mod M, does each of the first seven states from
each published seed lie below M−2^26+1? Seeds are20000,20420,20507,20509,
20512,20517, fixed from the source before executing the check. All42rows are
mandatory. No parameter/offset search, RNG reseeding, curve arithmetic or
legacy primality code is executed. Each state is calculated by both bit-serial
modular multiplication by g and121modular triplings. The latter intermediate
is below3M; the former is below2M, both safely below unsigned128capacity.
Boundary controls distinguish strict acceptance from rejection at equality.

This is a local bounded arithmetic/code check under the existing-results
analysis/development rule, not an experiment checkpoint or EKS qualification.
No frozen Python, AWS identity, new point count, scalar search, upload or paid
job is involved. Build with C++17, Wall/Wextra/Werror; execute with a10second
watchdog. Raw build/status/stdout/stderr and byte bindings accompany the report.

## Why this addresses the hypothesis

zrandom(b) updates the saved state by g modulo M until it is below
B_b=M−(M mod b), then returns state mod b without replacing the saved state
with that residue. For1<=b<=2^26, B_b>=M−2^26+1. Thus a state below this
universal threshold is accepted on the first advance for every allowed bound.
Induction then equates seven calls with bound2^26 and seven calls with any
allowed primality-test bounds from the same seed.

For a multi-limb prime, assuming correct library arithmetic, zprobprime(n,7)
performs a deterministic base2 test and seven random Montgomery trials. Its
trial bounds are within the range above. zcomposite(&n,1,2) uses no random
base. This supplies a precise sufficient mechanism for the warmup state.
It does not identify the original caller or establish that it used this mechanism.
An extra randomized prime check between seeding and point selection would
consume additional calls and invalidate the simple seven-call caller model.

The check does not recompute historical G, certify the original C implementation,
or explain omitted curve labels. Even equal post-warmup states only imply equal
subsequent deterministic streams when all later sampling/packing rules match.
Historical generators for the omitted curves remain unknown. The existing
correct-order primality replay accepts both labels, so this mechanism alone
cannot reject the omitted curves. No complete skip criterion is claimed.
