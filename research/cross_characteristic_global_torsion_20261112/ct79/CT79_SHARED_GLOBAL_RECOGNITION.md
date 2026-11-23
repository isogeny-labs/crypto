# CT79 — One coherent global 3-torsion lift recognized from encoded source coordinates [pcc:unverified]

## Result and exact scope

This is a **bounded** completion of CT69's fixed global point-field witness, CT74's 29-adic torsion lift and CT73's common-place requirement. There is now an executable two-prime chain whose *input-to-global-representative* routine consumes only encoded source coordinates, not a discrete-log label. It checks the two available algebraic candidates by reduction at **one fixed source place**, transports the recognized representative through **one fixed binary residue embedding**, and solves all public order-three targets. The global field, its two algebraic point representatives and its residue embeddings are still precomputed. Candidate enumeration is scalar-equivalent in this order-three group, and **no large-order scalable recognition algorithm or ECDLP acceleration is claimed**. [pcc:unverified]

Use the same 11a1 model as CT69:

\[
E:\ y^2+y=x^3-x^2-10x-20,\quad
H(Y)=27Y^8+108Y^7-4813Y^6-14817Y^5+162543Y^4+349907Y^3+267096Y^2+89655Y-101306819.
\]

Let \(K=\mathbf Q[\beta]/(H)\), \(X=-A(\beta)/4992581\), with \(A(Y)\) the degree-six polynomial pinned by CT69 and duplicated exactly in the executable. Fix **source prime** \(v=(29,\beta-5)\) and **binary place** \(w\) with the irreducible reduction \(H\bmod2=Y^8+Y^6+Y^5+Y^4+Y^3+Y+1\). Fix one residue coordinate embedding \(\beta\mapsto 2\) (byte-encoded) in \(\mathbf F_{256}\).

The candidate global points are \(T=(X,\beta)\) and \(-T=(X,-\beta-1)\), plus identity. This is valid because CT69 proves the division-polynomial and curve equations over \(K\), and CT79 directly confirms the *integer polynomial identity* \(A(-Y-1)=A(Y)\). **Do not independently select a new source prime or target Frobenius embedding per public point.** [pcc:unverified]

### Source-local to global recognition

The two linear roots of \(H\bmod29\) are 5 and 23. They are simple: \(H'(5)\equiv7\) and \(H'(23)\equiv22\pmod {29}\), so Hensel lifting is unique at each source prime. To precision \(29^6=594823321\), independent algorithms produce

\[
\begin{array}{c|cc}
\beta\bmod29 &\beta\bmod29^6&X\bmod29^6\\
5&425642517&25104680\\
23&169180803&25104680
\end{array}
\]

The first row exactly matches **CT74's independently recorded order-three local lift**. At the *fixed* prime \(v\), the second candidate is evaluated as \(-425642517-1\pmod {29^6}=169180803\): it is the reduction of **\(-T\) at the same v**, not the reduction of \(T\) using a switched source prime. This is the source-local/global recognition bridge achieved here. [pcc:unverified]

Given a public encoded point, the recognizer evaluates the source reductions of the two global candidates at \(v\), compares full \((x,y)\) coordinates and requires exactly one match; infinity maps to infinity. There are **two candidate tests per nonzero public input**, not a three-entry scalar-indexed output table. The list of candidate representatives itself is nevertheless equivalent to enumerating the two nonzero discrete-log possibilities, and therefore cannot establish any nontrivial complexity advantage. The program rejects three negative public inputs outside its promised subgroup. [pcc:unverified]

### End-to-end public results

| Encoded source \(Q\) | Local \(y\) modulo \(29^6\) | Binary image \(\Phi(Q)\) (byte encodings) | Source log | Target log |
|---|---:|---|---:|---:|
| \(O\) | — | \(O\) | 0 | 0 |
| \((18,5)\) | 425642517 | (19,2) | 1 | 1 |
| \((18,23)\) | 169180803 | (19,3) | 2 | 2 |

Both independent programs implement source curve addition and binary curve addition, check exact point order three, and solve target logarithms by *separate exhaustive group walks* after transport. They do not pass source-log values to the recognizer or target solver. Changing only the source prime for the second point to \(\beta\mapsto23\), while leaving binary \(\beta\mapsto2\) unchanged, incorrectly maps it to \((19,2)\) and recovers log 1 instead of 2. This is the retained **wrong-place falsifier**. [pcc:unverified]

### Independent checks and measured scope

- Python uses Newton–Hensel polynomial root lifting, direct polynomial/global-negation identities and shift-and-XOR field arithmetic. JavaScript independently tests **every 29-adic digit** at each precision and uses polynomial-convolution/reduction arithmetic over \(\mathbf F_{256}\). Both implement actual source and binary group laws, scalar verification and the wrong-place control.
- Python, Python \(-O\), Node and Node \(--no-opt\) output the same one-line canonical JSON. Locally recorded SHA-256: `d60a86d86f366d4b3b06c6b6bc25eb2320f26cc6f3870fd8464251e8a5ae153b`. The CI workflow checks byte equality, line count and this digest; independent PCC scientific signing is separate.
- Two source roots cost **ten Newton corrections** across five increases in precision each, versus **290 polynomial-candidate evaluations** in the digit-search auditor. For all three source subgroup elements together, recognition performs **four candidate coordinate comparisons**. The two nonidentity inputs require **three source group additions** in the direct baseline scans and **three target group additions** for destination logarithm scans. These are counts of *different operations*, **not commensurable wall-clock costs or a matched optimized-rho benchmark**. The explicit field model is provided for free in this control, whereas a cold attack must charge its computation and its coefficient heights.
- The source subgroup has order 3; direct source scanning is at most two additions per input and renders this toy **unsuitable as any evidence of an advantage over optimized rho or the existing index-calculus baselines**.

### Reproduce

```sh
D=research/cross_characteristic_global_torsion_20261112/ct79
python3 -B "$D/ct79_global_recognition.py" > /tmp/ct79-py.jsonl
python3 -O -B "$D/ct79_global_recognition.py" > /tmp/ct79-pyopt.jsonl
node "$D/ct79_global_recognition_independent.js" > /tmp/ct79-js.jsonl
node --no-opt "$D/ct79_global_recognition_independent.js" > /tmp/ct79-jsopt.jsonl
cmp /tmp/ct79-py.jsonl /tmp/ct79-pyopt.jsonl
cmp /tmp/ct79-py.jsonl /tmp/ct79-js.jsonl
cmp /tmp/ct79-py.jsonl /tmp/ct79-jsopt.jsonl
echo 'd60a86d86f366d4b3b06c6b6bc25eb2320f26cc6f3870fd8464251e8a5ae153b  /tmp/ct79-py.jsonl' | sha256sum -c
```

### Proof obligations, sources and next meaningful step

The existence and uniqueness of prime-to-residue-characteristic torsion specialization are classical, not new: A. Snowden, [*Elliptic curves over DVRs*, Lecture 8](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html), the finite-flat/étale reduction proposition and its \(E[n]\) corollary; Silverman, *Arithmetic of Elliptic Curves*, Ch. VII. The global resultant identities are independently exposed in [CT69](../ct69/CT69_EXPLICIT_POINT_FIELD_BRIDGE.md); the exact local witness in [CT74](../ct74/CT74_SOURCE_LOCAL_HENSEL.md); shared-orientation caution in [CT73](../ct73/CT73_SHARED_SPECIALIZATION.md); field-factor limitation in [CT78](../ct78/CT78_ORIENTED_TORSION_CYCLES.md). Current PRs [#3877](https://github.com/isogeny-labs/crypto/pull/3877), [#4015](https://github.com/isogeny-labs/crypto/pull/4015), and concurrent [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) were read before committing. **Shared research-KB MCP and a checked-out KB CLI were unavailable**: this is a non-exhaustive fallback, not a claim of novelty. [pcc:unverified]

The stronger unblocked gate is to construct and *algorithmically recognize* globally compatible torsion representatives for larger-order arbitrary public inputs without enumerating candidate scalar multiples, with a single second-prime residue specialization. In particular, field construction, primitive-element index/discriminant, root selection, representation heights, exact target field, total runtime, optimized-rho cold solves and existing IC baselines all remain unpriced for a cryptographically relevant subgroup. The bounded source-local/global check here does not establish that gate. No paid compute, no protected merge, and no independent signed PCC claim. [pcc:unverified]
