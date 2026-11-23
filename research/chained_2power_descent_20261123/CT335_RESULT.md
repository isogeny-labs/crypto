# CT335 — Prime31 split genus-two Richelot quotients and dual-algebra controls

**2026-11-23 UTC. Executed, source-pinned finite mathematics, unsigned [pcc:unverified].** This is a bounded continuation of [CT334](CT334_RESULT.md), on the existing [draft #4997](https://github.com/isogeny-labs/crypto/pull/4997). No new scalar-solver or authenticated original-model run occurred.

## Result

At `p=2147483647`, take the already frozen **smooth** fully split source `C:y²=f(x)=prod_{a=1}^6(x-a)` from CT334 JSON blob `01be355bf299c01419ac3a43a53adc4f8f7d61ea`. Exhaust all **15** unordered pairings of these six roots. For each, let `g1,g2,g3` be the three corresponding monic quadratics and let `delta` be the determinant of their constant/linear/quadratic coefficient rows.

A new source-bound finite calculation gives **14** nonzero-determinant kernels producing **14 squarefree degree-six Richelot codomain equations** over the same Fp; the remaining pairing `(1,6)|(2,5)|(3,4)` has `delta=0`, hence a decomposable PPAV quotient rather than a Richelot Jacobian codomain via the nondegenerate formula. These are **15 rational kernels**, not 15 smooth Jacobian destinations. The 14 nonzero cases each passed a direct polynomial gcd test and the reverse-bracket diagnostic.

For the canonical pairing `(1,2)|(3,4)|(5,6)`, the kernel quadratics (coefficient order constant, linear, quadratic; all residues mod p) are

```text
g1 = [2, 2147483644, 1]
g2 = [12, 2147483640, 1]
g3 = [30, 2147483636, 1]
delta = 32

h1 = [1207959549, 268435457, 1879048191]
h2 = [268435458, 536870910, 536870912]
h3 = [671088639, 1342177280, 1879048191]
H = h1*h2*h3 =
[1204813827, 400556024, 1637875718, 100663293,
 1480589312, 1971322879, 8388608]
```

The source and target sextics are smooth. Under the chosen `h_i=(g'_{i+1}g_{i+2}-g_{i+1}g'_{i+2})/delta` normalization, every one of the **14** nondegenerate cases returns the *same three original source quadratics* after a second, dual bracket operation. The measured coefficient identity is `delta*delta_dual = -2 mod p` in every case. This is a strong **finite polynomial** control, **not** a verified implementation of point/divisor images or `dual(phi)∘phi=[2]` in Jacobian arithmetic.

## Why it matters and what it does not prove

By the standard Richelot theorem, each nonzero-determinant kernel defines a rational degree-four polarized `(2,2)` isogeny `phi:J(C)->J(C')`; the associated quotient equation was computed explicitly. Because the kernel is 2-primary, its restriction to any **odd-order** subgroup is injective. Thus an *already available* prime-to-two DLP encoding inside `J(C)` remains injective across such a step. No elliptic source encoding into this particular Jacobian was constructed here, and point-image evaluation or transfer costs have **not** been implemented. No claim of a faster complete ECDLP solver follows.

The degeneracy guard prevents incorrectly promoting a base-field-rational kernel to a smooth codomain. The 14/1 split applies only to this one explicit CT334 root configuration, **not** to every curve with six rational Weierstrass roots, all twists or any arbitrary transfer chain.

## Reproducibility and independent-path comparison

- **Preregistered** [CT335_PROTOCOL.md](CT335_PROTOCOL.md), commit `d67ac361416083307ab825bb93bcd66b350a8547`, before the first CT335 arithmetic calculation.
- Committed finite BigInt/field source [`ct335_richelot_exact.js`](ct335_richelot_exact.js), Git blob `57cd05fceec6522011599c694c96550521e8c9a7`, commit `359b54b2ed0baa3b5c6a09ef620f640aebd70e22`. Generated full [CT335_EXACT_RESULT.json](CT335_EXACT_RESULT.json), 22,025 characters, blob `c1d10330006212726fda29188e690d041bd42583`, commit `540faba44aa67d936d96c8728dcd2b153e6ea2ff`. Both exact blobs and summary were fetched back through GitHub. The JS study function was executed in the coordinator's V8 tool; Node CLI replay is a **reproduction command**, not an additionally observed native CI result:

```sh
node research/chained_2power_descent_20261123/ct335_richelot_exact.js > /tmp/CT335.replay.json
cmp /tmp/CT335.replay.json research/chained_2power_descent_20261123/CT335_EXACT_RESULT.json
```

- **Different producer implementation**: a separately written standalone Python Fp polynomial/Euclidean algorithm was compiled and executed with `python3` and `python3 -O`; its two full outputs were byte-identical. SHA256 of local Python source `79c1f684e261b5428cff71c59f2d86ed9d98d7a328b204e812a3a5b23bd08b88`, local raw SHA256 `b3a7819d6fb2a03b43c1029c44b0e9b1f316ec60dbffefd96ad15b1cb21d3d5c`. This program is not the committed JS code, so **do not claim the Python source or raw blob was published on GitHub**.
- Comparing all 15 exact pairings, `g`, `delta`, `h`, `H`, dual quadratics and dual determinants, canonical 4,492-byte array serialization had the **same FNV-1a 64-bit digest** `8edf6357e7942b9a` in committed JS JSON and local Python output. This is a deterministic comparison receipt, **not** a cryptographic signature or validator verdict.
- A distinct SymPy `Poly(..., modulus=p)` / `Matrix.det` implementation confirmed 14 smooth, one decomposable, all 14 reverse identities; it rejected a modified h-coefficient. Local SymPy checker SHA256 `db243e84a7ebc646c34ac848f5cc584239945535369fe83c8fc47caf0c02b8f3`. No external independent person, signed PCC or original native model was involved. Both JavaScript and Python paths rejected three real source-data mutations and accepted an unchanged control.

## Ownership and next gate

One coordinator; **zero native worker agents** available in this tool environment. This used bounded source-only finite arithmetic on public model coefficients, not a timed native experiment. The mathematics package produced equations and the kernel/odd-order injectivity criterion; the Rust norm-point/Cantor package was deferred to its existing owner; validation was coordinator-authored across different implementations, **not** independently signed. No EKS/paid run, shared native build, protected merge, benchmark rerun, original fixture custody or full-cost claim. This is not an elapsed-time parallel speed gain.

The next distinct step is independent **divisor-image evaluation and homomorphism/dual-composition checks**, with exceptional divisors, inversions and Cantor reduction. For the owner-reserved main objective, authenticated **695/701** replay and physically priced `C169/C13a/C13b/N2_actual/D` plus all transfer/setup/LA/memory against optimized same-host rho, BSGS and strongest existing IC remain mandatory. Retain draft #4997, stacked #4992 and #5005, original EKS-only/paid STOP/custody and required CI/review/PCC holds.

Primary sources: [Kunzweiler (2024), §3.2](https://link.springer.com/article/10.1007/s10623-024-01366-1), [Magma Richelot handbook](https://magma.maths.usyd.edu.au/magma/handbook/text/1615) and [Visser slides](https://warwick.ac.uk/fac/sci/maths/people/staff/visser/genus2crypto.pdf). The repo research-KB MCP and wrapper/Conductor were not accessible in this connector-only runtime; CT331–CT334, CT333/A1, #4992 and #5005 were the available internal fallback. No absence/novelty claim was made.
