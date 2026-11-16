# On-demand structured queries, not all-target cache extrapolation

UNEXECUTED changed experiment. This note adds no run, does not edit any frozen source/input/proof/closure/raw evidence, and claims neither novelty nor an elliptic-curve chart. A future defensive checker will explicitly assert canonical target-coordinate ranges in addition to row uniqueness; the current frozen batch construction already emits range(q) squared and is not changed.

## Distinguish the two contracts

The measured joint-Frobenius worker solves ALL q squared target pairs for a tiny quadratic field K of order q=p squared. It caches q decoded first coordinates and emits q squared complete rows. Its time and memory are dominated partly by batch output, and its O(q) cache is not appropriate for large fields. The point-query formula in the proof has a different contract: for one given (t,u), return every ordered (x,y) in Fp squared satisfying t=x+theta*y and u=x squared+theta*y squared. Under the declared basis and odd characteristic, its root set is empty or the unique reconstructed pair.

For each query, the generic formula computes t^p, x=(t+t^p)/2 and y=(t-t^p)/(2theta), verifies x,y lie in the prime subfield, then checks u. It uses O(log p) field multiplications under binary exponentiation and a constant number of field elements once field constants are prepared. The full field/domain validation and constant preparation are real costs. No all-t cache or source-image table is permitted in the point-query arm.

## Strong comparator and changed mechanism

For the particular fixed polynomial basis (1,theta), the strongest simple comparator is direct coordinate inversion: if encode(a+b theta)=a+p b, then x=coord(t)[0] and y=coord(t)[1]. Check u by independent pair arithmetic and emit that exact pair or the empty set. This is a valid coordinate identification for this LINEAR first equation, not a claim that integer labels multiply as field elements. It must be measured alongside any generic Frobenius implementation. Merely beating an enumerated source-image index would not establish the best specialized performance.

A third arm can prepare the Fp-linear Frobenius matrix once from the declared modulus/basis, independently check its action, then apply it per query instead of binary exponentiation. In the trace-zero quadratic basis theta^p=-theta, the matrix is diag(1,-1); exploiting this classical structure may recover generic-backend efficiency but cannot be presented as a new rootfinding theorem. Setup and matrix validation are charged. Changing the field representation invalidates cached constants and matrices.

If generic exponentiation loses, do not repeat it with a longer timeout. Compare generic powers, charged linear Frobenius, and direct-coordinate inversion on the SAME declared queries. The useful question is whether a field-agnostic interface can retain near-coordinate cost without materializing the field or the source image. There is no predicted gain over coordinate inversion.

## Concrete bounded next test

Correctness first: preserve separate small fields GF9,GF25,GF49 with the explicitly reviewed nonsquare moduli and prime-subfield source domain. Use all declared point queries in a fresh version, independently compare against exhaustive source indexing, and include zero/repeated-coordinate sources, empty targets, basis changes, invalid modulus, characteristic-two rejection and canonical-coordinate checks. The source construction is a finite-field system, not an elliptic decomposition.

Only after those checks, a separately frozen wide-query diagnostic may cover the five requested NIST primes with K=Fp[theta]/(theta squared-d). Pin the existing prime certificates/preset sources; choose and freeze one independently certified nonsquare d per field before comparative timing. Field construction and nonsquare verification must be included in the paid pipeline or explicitly reused and charged as common setup. Full width is not hardness evidence.

Freeze a bounded query rule before timing:32 queries per field, mixing t=0,1,theta,1+theta and deterministic public hash-derived t values. For half the queries construct u from a declared source; for the other half add a fixed nonzero field element to the compatible u, preserving proven emptiness. Do not reveal expected roots to solver inputs. Independently verify every output and completeness using the proved coordinate-linear bijection plus a different pair-arithmetic implementation. The wide oracle must not allocate a q-element table: the previous TableOracle is forbidden there by an explicit allocation preflight.

Compare three fixed alternating cold repetitions per arm. Charge input parsing, field/domain validation, constants/matrix setup, all32 query solves, independent checks and complete output serialization. Store every query and output, setup/solve/verification stage costs, full worker lifetime and per-process RSS. Bound each worker by90 seconds,4GiB per-process address space, one core and a declared query/operation cap. Any cap or late report is incomplete with null timing ratios. No same-session fourth experiment is authorized by this plan.

## Required bridge to the actual program

The first equation's invertible source-linear map is the strong premise. For an actual guarded chart, identify its coordinate matrix over the declared source subfield, prove its rank and domain membership, and preserve every singular rank stratum. Full column rank gives a unique possible source vector; a rank defect leaves a kernel family that must be completely solved or reported UNKNOWN. Rank is not assumed generic. Prove poles, exceptional fibers and original auxiliary/ghost branch coverage before using this branch in the original all-roots solver.

Ordinary binary/Koblitz and the actual GF(2^83) target need their own chart proof. The odd trace formula cannot cross characteristic two, and the simple coordinate argument alone does not solve the hard target. This plan supplies neither a genuine F4/F5/F6 comparison nor the20-instance qualified release gate or requested8x result.
