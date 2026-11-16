# First pursued lead: general contraction to ECDLP

Source inspection, 2026-10-30. No independent kernel check, proof of the
general theorem, new mathematical identity or correctness theorem is claimed.
Read [sources.json](sources.json) with this map.

| Interface | Source and observation | Acceptance still needed |
| --- | --- | --- |
| ECDL reduction | Bochum v0.2 Theorem 2 imports OpenAI D.1; Theorem 4 uses growing arity and fixed cofactor | Exact general tensor theorem, not merely a SAT consequence |
| OpenAI routing | formalization.yaml lists Final.lean:exact_count_and_sat; comparator loads SubexponentialSAT.Main, importing Final | Independently build pinned dependencies and audit declarations/axioms |
| OpenAI endpoint | Fixed-width CNF counting, primitive-machine time and all-prefix visited space | General implicit-entry endpoint or quantitative reduction preserving the ECDL size saving |
| Comparator semantics | Challenge statement has a placeholder; solution has proof terms; config lists standard permitted axioms and enable_nanoda=false | Placeholder is not a missing solution; no local kernel result obtained |
| Alternate source | Anthropic MainResults defines ETHIsFalse/MainTheorem; Solution provides terms via CountedRoute, NamesInOrder and Wrap | Independent source/cost checking and an actual engine port; not already the Rust backend |
| Entry work | D.4.6 retains immutable creation descriptions/precision and shares references | General recursive entry-work and retained-space bound against native callbacks |
| Native admission | Setup::new/parameters and Counter::count use finite fallback at public machine sizes | Distinguish forced controls from an admitted scalable execution; preserve guards |
| Finite plans | width.rs:Plan::validate rechecks pins, partition and intermediate ranks | Rank cap is not a smaller-packet proof or complete runtime estimate |
| Encoding | encoding.rs uses implicit closures and arithmetic metadata | Uniform degree/wire/description/construction bit-work bounds, including pins and exceptional cases |
| Sampling | Counter::sample documents completion weighting; selector counts have their own contract | Track projected words, cofactor preimages, auxiliary witnesses and seed distribution |
| Whole reduction | Remark 2 separately prices oracle calls, elimination and memory | Retain all terms, failed attempts and rank/invertibility probability |

## Avoid false shortcuts

Fixed-k formal counting is not the same endpoint as arbitrary implicit tensor
contraction. Its exponent uses encoded variables. A circuit simulation may
introduce enough auxiliary variables to lose the intended saving; the missing
item is the appropriate quantitative bound, not mere computability.

Native fallback is a representation boundary, not a counterexample to an
asymptotic existence theorem. Passing fallback tests does not verify recursion.
Generic selector enumeration and coordinate-aware tensor predicates also have
different computational models; coordinate access alone proves no advantage.

Readable proof terms, listed axioms and source hashes do not replace checking
the pinned dependencies. No Lean executable was present in this environment.

## Next packet: implicit-entry cost

### Additional source trace completed in this round

`Tensor::entry` charges an entry, checks creation precision and, for a packet,
evaluates `packet.entry` in its stored ring before lifting. `PacketTensor`
contains its ring, local coefficient table and incoming/outgoing mode data;
it has no parent tensor callback. `child` retains old vertices by Arc and
constructs new packet data from the Cauchy kernel and selected modes. Thus
the suspected *nested parent callback inside each newly created packet* is
not present in this representation. This is a source finding, not a general
complexity proof or an assertion that all descriptions are cheap.

The concrete remaining cost interface is coefficient extraction:
`PacketTensor::entry` performs a depth-first choice traversal over rows and
incoming exponents. `evaluate_at` enumerates weighted mode tuples and group
children; strict mode checks edge/degree shrinkage. Audit the total traversal,
coefficient/table preparation, retained data and bit work together. Meter
charges bound an admitted execution but do not independently prove the
source's uniform asymptotic work bound. Original Boolean predicates also
need the source's evaluation-cost hypothesis; an opaque user closure does
not supply it by its type alone.

Follow one retained tensor and one new packet tensor through creation, lift,
child evaluation and reconstruction. Record parent-description references,
creation precision, port count, coefficient bit size, temporary storage and
predecessor evaluations. Compare the source entry-work induction with native
callbacks; include weighted-mode enumeration and depth-first accumulation.

Success requires a complete invariant with checked dependencies, or an exact
missing assertion/counterexample. A bounded control is regression evidence,
not the general induction. New algebraic results require the repository's
identity/proof rules; this map records existing obligations only.

## Cost record before a later run

Bind revision, encoding/backend, curve/subgroup, seeds and protocol. Price setup,
decoding, support checks, predicates, unsuccessful queries, useful rank,
elimination, memory and verification. Keep Limit/timeout unknown. Compare
unchanged workloads and independently replayed answers against the frozen
native baseline and strongest matched rho. Do not combine historical units,
compare reused setup with a cold reference, or claim fastest without external
comparisons.
