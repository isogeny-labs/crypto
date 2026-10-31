# Read-only source map and compatibility gap

Consumer inspected at88707e16b99f54976cfbcbe78c0f49379ab69da7; producer runtime
at58bd5ab64424f7710540e0aea502225de6e6c438. During final capture the producer
HEAD was1e8096394fdd51a2e33fd47b6b21ef47d848602f after its owner's documentation/
evidence merge; the scoped runtime diff against58bd was empty. Both observed
worktrees were clean. Exact inspected file hashes are in
consumer-source.sha256 and producer-source.sha256. All paths below are real
source/doc paths, not proposed implementation evidence.

Consumer root: /home/ec2-user/crypto-ml-consumer-root-certificates-20261028.
Standalone crate: research/ml_dataset_independent_validation_20261028/native.

- native/src/audit.rs:495 audit_cell checks manifest/path/hash/config/provenance;
  664 positive certificates;689 root-specific call;699 root_certificates;
  791 audit;897 static_features;1042 export_rows. There is no generic-v1/v2
  native filter, source-combination or modular-rank replay at this baseline.
- native/src/common.rs:11 MAX_JSON=64MiB;14 open_regular;99 read_json_bytes;
  106 bounded_bytes; unique-key Visitor and artifact helpers preserve byte
  integrity without mathematical filter claims.
- native/src/arithmetic.rs:20 Point;22 decimal;71 is_prime;159 Field;
  260 Curve;394 public_curve;513 signed_domain;594 verify_certificate.
  Curve::add/neg/point are crate-visible; Curve::mul and PublicCurve.generator/
  declared_order are private. Existing field/group algebra can be reused through
  narrow crate-local accessors; public curve validation does not prove declared
  subgroup order/cardinality. Field::prime requires p>3.
- native/src/roots.rs:549 sorted_value;562 value_hash;702+ verify_dataset.
  Existing root code shows the required preserve_order correction but is not
  a sparse modular relation or rank checker.
- native/src/policy.rs:99 bind_base independently binds recipe/config/size/caps
  and sampling provenance, not complete construction or cyclic membership.
- native/src/transport.rs is manifest-driven S3 fetch and separately labeled
  producer replay; no arithmetic reuse is available or needed there.
- native/tests/saved_roots.rs and tests/producer_policies.rs provide retained
  sealed-fixture/rehash test patterns. Reuse patterns, not old acceptance claims.
- README.md, NATIVE_MIGRATION.md, POLICY_COMPATIBILITY.md and
  ROOT_CERTIFICATE_DESIGN.md define current narrow labels and unchanged static
  feature contracts. Historical Python files are not an execution option.

Producer root: /home/ec2-user/crypto-ml-generic-filter-trace-20261028.

- src/ml_dataset/relations_generic.rs:21 schema constants;28 Limits;60 SignedPoint;
  66 Domain;99 Pool;178 Pair;217 Attempt;231 make_row;258 equation;304 check_attempt;
  345 Rank;353 reduce;397 Batch;404 batch;450 recovered;917 Bindings;961 TraceTotals;
  1018 BatchBinding;1027 BatchTerminal;1033 TracedBatch;1055 bounded_json_payload;
  1103 TranscriptV2;1130 query_rng;1145 filter_options;1175 traced_batch;
  1233 outcome_v2;1283 solve;1463 shape_v2;1499 verify_v2;1631 verify_v2_labels;
  1718 expected_filter_payloads. These are reference contracts, not imports.
- src/ml_dataset/relations.rs:generic module/dispatch; solve fallback around412.
  Successful native Koblitz Frobenius setup stays on its own legacy path.
- src/ml_dataset/mod.rs:403 JSON_INPUT_MAX_BYTES=32MiB;1835
  verify_generic_filter_artifacts establishes exact transcript/batch/outcome
  binding before producer's full replay. Consumer needs its own equivalent hook.
- src/cryptanalysis/large_prime_filter.rs:PartialRelation, CompleteRelation,
  LargePrimeFilterOptions, LargePrimeFilterReport define wire shapes.
- src/cryptanalysis/large_prime_filter/trace.rs:SCHEMA, WORK_SCOPE, BYTE_SCOPE,
  FilterTraceLimits, RowRef, RowSnapshot, Reservation, Transition, FilterEvent,
  FilterRun define native wire/accounting. Input hash is typed order, not sorted
  JSON binding. The consumer must own its types and arithmetic.
- src/cryptanalysis/large_prime_filter/trace/check.rs describes supplied singleton
  choice, static core scoring, admission priority, multiplicatively checked
  inverse/cancellation, source AXPY and acknowledgment semantics. Do not copy or
  import this producer checker and call it independent.
- src/cryptanalysis/ec_index_calculus.rs:1210
  gaussian_eliminate_mod_n_particular gives free-variable-zero/determined-column
  semantics. The consumer needs a separate bounded prime-modulus reducer.
- Cargo.lock pins rand0.8.8/rand_chacha0.3.1/rand_core0.6.4 for exact query-stream
  interpretation; future StdRng defaults are not a stable substitute.
- research/ml_dataset_generic_filter_trace_20261028/DESIGN.md records original
  producer boundaries. The native source map/program design references Cavallar,
  CWI MAS-R0012 sections2.1–2.2 and official CADO merge implementation as prior
  filtering context, not ECDLP correctness/cost theorems. Existing consumer
  arithmetic references SEC1 v2 and its prime/irreducibility sources. This task
  introduces no new algorithm or literature claim.

No tests/builds/fixtures/cloud operations ran during this inventory. This design
is a concrete implementation contract, not compatibility evidence. Proposed
checker admission caps and future fixture execution remain parent decisions.
