# E0 canonical-IR implementation receipt

Date: 2026-10-31
Status: core mechanism implemented and verified on tiny fixtures; production-ladder gate remains open
Affected E1 data: none

## Outcome

`hashed-ecdl-tensor` now has a strict, versioned semantic snapshot for the
Boolean tensor graph used by the experiment portfolio. The snapshot binds the
predicate evaluator, ordered ports, construction-order graph, interface order,
pins, and provenance into stable bytes and a SHA-256 identity. Import rebuilds
an independently replayable direct-count network.

This closes the missing implementation mechanism behind E0. It does **not**
close E0 for every eventual production fixture: the final E1 generator, ladder,
fresh-process A/A runner, and two known high-arity variants listed below still
need preflight before the protocol can freeze.

## Sources and scope

Before implementation, the existing [hashed-ECDL engineering report](../../docs/hashed-ecdl.md),
the [prime-field lead review](../prime_ic_lead_review_20261031/README.md), this
study's [`PROTOCOL.md`](PROTOCOL.md), and the current network, encoder, and
contraction sources were reviewed. The algorithmic context remains the pinned
[Bochum v0.2](https://github.com/bochum-crypto/subexponential-ecdl/tree/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f)
and [OpenAI Appendix D](https://github.com/openai/math/tree/7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026)
revisions already assessed in that lead review. E0 changes representation and
verification only; it does not strengthen either source theorem or make a
novelty claim.

## Implemented representation

The binary format is `BTNIR001`, version 1. It records:

- nonempty source, generator, and revision provenance;
- vertices in construction order;
- a namespaced predicate tag and strictly sorted, unique parameters;
- exact predicate arity and an ordered edge identifier at every port;
- edges and both endpoints in construction order;
- the exact interface-edge sequence and one pin state per edge;
- a predicate semantic digest and a SHA-256 identity for the complete bytes.

Predicates through 16 ports are enumerated into packed complete truth tables.
Larger production relations must provide a typed `PredicateProgram`; that
program is the live evaluator before capture and the evaluator reconstructed
after import. Version 1 supports constants, Boolean NOT/AND/OR, comparison and
addition bits, both Fourier transform stages, Fourier pointwise multiplication,
and product-carry relations.

The parser rejects unknown versions, malformed or trailing bytes, noncanonical
integers, bad semantic digests, duplicate/unsorted parameters, incomplete or
duplicate incidence, loops, repeated interface edges, and pin-width mismatch.
This is deliberately construction-order canonicalization, not graph-isomorphism
canonical labelling.

## Design correction caught before data

The first implementation attempted complete truth tables for every production
predicate under the 16-port cap. Its representative tiny decomposition control
rejected that design: a Fourier relation has 132 ports, so its table would have
`2^132` entries. Raising a cap or hashing a closure name would not preserve
replayable semantics.

The corrected hybrid keeps exhaustive tables where they are bounded and uses a
strict serialized relation language above the cap. No E1 row, rank, solver run,
or timing measurement preceded this correction. The draft protocol records the
change explicitly.

## Correctness controls

The unit controls exercise:

| Control | Required observation |
|---|---|
| Stable fixture | Golden complete-IR SHA-256; byte-identical parse/serialize and recapture |
| Semantic round trip | Equal total count, every interface-prefix count, and least witness |
| Independent path | Original functional count equals original and rebuilt direct counts |
| Full tiny decomposition | Strict production capture, typed high-arity program, parse/rebuild/recapture byte identity |
| Endpoint mutation | Incidence validation rejects the changed edge ordering/endpoints |
| Interface permutation | Complete hash and ordered witness change |
| Predicate-parameter mutation | Predicate semantic digest rejects the snapshot |
| Predicate-port reversal | Complete hash and exact count change |
| Dropped pin | Pin-width validation rejects the snapshot |
| Parser mutations | Truncation, trailing bytes, bad magic, and corrupt typed program are rejected |
| Untagged fixture policy | Strict production export rejects it; explicit tiny-table policy admits it |

The complete command outcomes are maintained in [`VALIDATION.md`](VALIDATION.md).

## Boundaries and open obligations

- `selector_point` still uses a checked closure without a typed canonical
  operation. It is exportable only while its active arity is at most 16.
- A caller-supplied structured membership predicate or `GraphBuilder` predicate
  above 16 ports has no replayable program and is rejected, including under the
  fixture truth-table policy.
- Import intentionally drops the encoder's private `Program` optimization hints.
  The reconstructed network is a direct semantic replay instrument, not an
  optimized functional backend.
- The tests cover a hand-built Boolean fixture and the repository's full tiny
  decomposition construction. They do not yet cover every final E1 ladder row,
  cut tag, or fresh-process A/A export.
- The mirrored `cryptanalysis` crate was not changed in this checkout. Its
  source-parity integration remains a delivery obligation.
- No boundary tensor, rank, SAT/BDD output, performance comparison, relation
  yield, or ECDLP speedup has been measured or established.

## Gate status

| E0 obligation | Status |
|---|---|
| Typed, versioned semantic IR | Implemented |
| Stable bytes and SHA-256 identity | Verified on the declared tiny fixture |
| Exact count/prefix/witness round trip | Verified on the declared tiny fixture |
| Five mutation controls | Verified |
| Representative high-arity production capture | Verified on the tiny decomposition network |
| Every final E1 fixture and fresh-process A/A | Open; generator and ladder are not frozen |
| Known-rank E1 positive controls | Open; belongs to the `tensor-census` slice |
| E1 affected measurements | None; blocked by draft protocol |
