# E1 SAT representation-equivalence protocol

Status: **frozen before any external solver was run on a study instance**.
The earlier two-clause toolchain smoke is not a study input. This protocol
covers only the CaDiCaL/CNF and CryptoMiniSat/CNF-XOR subgate. The msolve and
cvc5 arms remain open and E2 cannot start until they are resolved.

## Question and evidence boundary

Do ordinary CNF and CNF-XOR renderings of one deterministic functional
Boolean circuit preserve exactly the projected models of the fixed F17
shared-divisor-coefficient predicate?

This is a correctness-only development validation. It records no solver
timing, work counters, or performance comparison and cannot show that SAT is
faster than enumeration, F4, rho, or any index-calculus pipeline. The current
Iso Lab commit-build workflow packages `ecbench`, not this pinned four-binary
proof toolchain. The run is therefore local and earns no EKS isolation grade.
Changing the lab image or workflow is outside this frozen subgate.

## Frozen implementation

The canonical IR is lowered once to a functional bit circuit:

- each semantic field word is a little-endian five-bit input constrained to
  be less than 17;
- ripple add/subtract, bitwise modular reduction, and schoolbook
  multiplication implement F17 arithmetic;
- AND and XOR gates have a unique output for every input assignment;
- ordinary CNF expands each XOR gate into four Tseitin clauses;
- CNF-XOR retains that same gate as one native XOR constraint;
- all other gates, assertions, variable order, and projected bits are
  byte-deterministic and shared.

Only projected field values are blocked during enumeration. A returned model
must decode every semantic bit, satisfy the canonical IR, and have a
projection in the algorithmically separate elliptic group/interpolation
oracle set. Functional auxiliary gates make the Boolean extension unique for
each complete semantic assignment; the pass condition nevertheless compares
projected sets, not auxiliary models.

The runner is
`native/src/e1.rs`. Its 40-hex source revision is supplied at launch and
recorded with the running executable's SHA-256. The run refuses an existing
output directory and every artifact writer is write-once.

## Frozen tools and calls

| Role | Source identity | Required binary SHA-256 | Invocation |
|---|---|---|---|
| ordinary CNF solver | CaDiCaL 3.0.1, `c60730422e758ef1cebe7aeddf2dda31c996bf04` | `5dba1e331369eb5543b7ee099e33d3ab360c4ae34d1c0cf07c3b878fbca33280` | `cadical --lrat --no-binary INPUT PROOF` |
| LRAT checker | `lrat-trim`, `b30f400f4ee5c32b77ee566a7c006081b521534f` | `e22e1e33818812e1aa6286578ba9cd11a26225f424c0718612784a7562dec895` | `lrat-trim INPUT PROOF` |
| CNF-XOR solver | CryptoMiniSat 5.17.0, `03954f7f7815ea3984774c95a66e7c224cd25952` | `96efc097b956e2d8f1982515de595472b17a47a25a0939fce94d923fb0d162c7` | `cryptominisat5 --threads 1 --verb 0 --xlrup 1 INPUT PROOF` |
| XLRUP checker | `cake_xlrup` from `frat-xor`, `18cd6bc17af10a9891c10a846ce966166f9fe0dc` | `56e2bc4aeddb362d632bf9f7056cfd604afb2f2e5f86d131d1d1dc554ef5b7bc` | `cake_xlrup --no-binary INPUT PROOF` |

The runner rejects any binary-hash mismatch. Each solver or checker process
has a 300-second correctness deadline. Timeout, signal, missing status,
unrecognized exit, malformed model, or unchecked proof fails the subgate.
There are no scientific retries and no timing fields are recorded.

## Frozen panel

The target panel is every affine point on
`y^2 = x^3 + 2x + 2` over F17, with `(m,t,c)=(3,3,1)`:

```text
(0,6)   (0,11)  (3,1)   (3,16)  (5,1)   (5,16)
(6,3)   (6,14)  (7,6)   (7,11)  (9,1)   (9,16)
(10,6)  (10,11) (13,7)  (13,10) (16,4)  (16,13)
```

For each target, the already-implemented coefficient enumerator and
elliptic group/interpolation oracle must first produce identical exact
projected sets. Each backend then repeats fresh-process solve, strict model
decode, canonical replay, oracle-set replay, and projected blocking until
UNSAT. The terminal UNSAT proof must verify independently. The empty
truncation of that proof must be rejected.

Positive and negative controls use the historical F17 `t=4` known witnesses
at `m=3,5,7`. Every semantic field is fixed to the known assignment for the
positive solve. For the negative solve, only `a0` is replaced by
`(a0+1) mod 17`, and every semantic field is again fixed. Each positive must
be SAT and match the exact assignment; each mutation must be UNSAT with a
checked proof whose empty truncation is rejected. The source summands must
also replay through a separate affine group law, with distinct allowed
abscissas and the declared target sum.

Parser controls require acceptance of both generated formats and rejection
of a missing terminator, a trailing constraint beyond the header count, and
native XOR passed to the ordinary-CNF parser.

## Pass, stop, and interpretation rules

Pass this SAT subgate only if all of the following hold:

1. all 36 target/backend projected model sets equal the exact oracle sets;
2. all returned models pass canonical and independent-oracle replay;
3. all 36 terminal UNSAT proofs verify and their truncations fail;
4. all six backend/fixture positives are SAT and exact;
5. all six deterministic mutations are checked UNSAT and their proof
   truncations fail;
6. all parser controls pass and every artifact/hash invariant holds.

Stop immediately on the first mismatch and preserve the partial write-once
run tree. Never repair the encoding after inspecting a failing target and
then continue under this freeze. A pass verdict is exactly
`sat_boolean_subgate_passed_msolve_cvc5_open`; it is not an E1-wide pass and
does not advance SAT as a performance lead.

## Requirement-to-evidence map

| Requirement | Evidence produced | Acceptance check |
|---|---|---|
| same mathematical predicate | canonical IR hash comments in every input; exact projected reference set in `e1-sat-panel.json` | candidate/oracle equality before solver launch and exact backend set equality |
| canonical field encoding | exhaustive 289-pair F17 add/multiply tests and all 32 five-bit range words | Rust 1.78 locked release tests |
| exact gate rendering | all truth-table rows for positive/negated AND/XOR in CNF and CNF-XOR | Rust tests plus strict DIMACS validation |
| SAT soundness | solver stdout hashes, decoded projections, canonical replay, oracle membership | every SAT attempt has both replay flags true |
| complete projected enumeration | projected blocking clauses and terminal solver attempt | exact set equality plus checked terminal UNSAT |
| UNSAT trust | LRAT/XLRUP proof, checker stdout/stderr and hashes | official checker acceptance; empty truncation rejection |
| controls | fixed m3/m5/m7 positive and `a0+1` negative trees | six exact SAT positives and six checked-UNSAT negatives |
| evidence boundary | `timing_collected=false`, local environment stated, open backend list | no speed or E1-wide claim admitted |

## Artifacts

The write-once run directory contains every exact solver input, solver
stdout/stderr, proof, checker stdout/stderr, empty truncated proof and mutation
checker logs. `e1-sat-panel.json` is the canonical aggregate. A descriptive
report, labeled diagram, and inspected PDF must be generated beside the
records before a result PR. Raw byte volume may be stored through the
checkpoint/S3 evidence path; any repository summary must retain hashes and
the durable raw location rather than silently omitting it.
