# Primary N83 chained-S3 XCNF control

The [K1182/866 same-query gate](K1182_RESULT.md) evaluates the larger
signed-Frobenius domains against the first ordinary public query. Its
presealed 5,000, 100,000, and 1,000,000-conflict calls and independent
receipt audit preserve the bounded point-decomposition outcome.
The [paired K600 ordinary-query gate](NATURAL_RESULT.md) extends this
known-witness control to the same first public query on Gray and hash pools.
Its sealed one-million-conflict calls preserve both bounded outcomes and
their full raw receipts.
The [phase-prefix gate](PHASE_RESULT.md) measures how many known Frobenius
phases the external solver can leave free on the same fixed witness.
The [external Gauss panel](GAUSS_RESULT.md) checks native-XOR matrix settings
against bracketed default calls on the unpinned witness formula.

The existing 5-summand, signed-Frobenius-domain chained-S3 encoding exports
to native-XOR XCNF, and CryptoMiniSat 5.14.7 returns a model when the known
witness's orbit choices are supplied. An independent Python verifier checks
all 714,776 CNF clauses, all 4,980 XOR rows, the field S3 identities, and
the five curve points' ordered sum against the frozen public target. The
choice-pinned call took 125.253 ms of local wall time at a 5,000-conflict cap.
Fixing only the five known Frobenius phases also returned a checked model
in 149.543 ms. With only the five representatives fixed, the formula
remained `UNKNOWN_solver_cap` through 100,000 conflicts. The unpinned formula
reached 1,000,000 conflicts in 73.263 s with `UNKNOWN_solver_cap`.

## Requested scope and measured scope

| Requirement | Status and evidence |
| --- | --- |
| Find the best complete factor base for the primary N83 DLP | Open. The primary a=0 curve has no observed natural relation yield, rank, final relation linear algebra, or verified one-target online comparison across bases. |
| Validate the higher-arity SAT encoding on its retained public witness | Complete for the exact identity-mask-zero, five-summand formula and domains below; see the model and independent replay in `evidence/`. |
| Measure the unpinned SAT access path | Three bounded local stage calls, each `UNKNOWN_solver_cap`; five additional known-answer choice controls isolate the search behavior below. |
| Establish a Tier 1 speed comparison | Open. These arm64 macOS calls are correctness and stage diagnostics, not an isolated same-point IC/rho pair. |

The field is GF(2^83) in the polynomial basis with modulus
`z^83 + z^45 + z^2 + z + 1`. The binary curve is
`y^2 + xy = x^3 + 1`, with subgroup order
`2417851639230796216685689` and cofactor 4. This control uses the exact
`known_n83_k0()` source model, the public five-point witness with SHA-256
`fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538`,
and the retained `domains.json` with SHA-256
`63accc5b9dc9de6327c5b07e93b3e62717f8376a96c3f91f7bb70b77095f23bb`.
The first three summands have 1,182 small-domain orbit representatives each;
the last two have 866 residual-domain representatives each. These are domain
counts, not an `fb<B>` point count. The exported formula pins five summand x
blocks and three partial-sum x blocks to the retained witness, applies the
worker's unsigned first-pair order, and uses `identity_mask=0`.

`candidate_id`, natural relation count, selected best total runtime, and
one-target speedup remain null. The witness is a known-answer control. The
previous first-pattern group replay is retained at
`verification/known-witness-first-pattern-replay-20261014/replay.json` in the
factor-base sweep. Its old `replay.py --check` currently fails at the pinned
hash of `src/cryptanalysis/koblitz_large_prime.rs`, because that source has
changed since the old receipt; `evidence/prior_replay.log` records this. The
new verifier reads the unchanged hashed witness fixture and computes the
curve group sum independently of that source.

## Bounded calls

All eight calls used one local arm64 macOS process, CryptoMiniSat 5.14.7,
`--threads=1`, native XORs, and the indicated conflict cap. Wall time includes
solver input loading. The exporter separately recorded 322.022 ms; that is
local setup, not an online DLP interval. Exact raw stdout, stderr, command,
binary hash, cap, exit status, and machine receipt are under `evidence/`.

| Formula | Conflict cap | Wall ms | Status | Interpretation |
| --- | ---: | ---: | --- | --- |
| original choice-free witness formula | 5,000 | 324.597 | `UNKNOWN_solver_cap` | no model at cap |
| same | 100,000 | 7,418.862 | `UNKNOWN_solver_cap` | no model at cap |
| same | 1,000,000 | 73,263.109 | `UNKNOWN_solver_cap` | no model at cap |
| first selector-pin attempt, unsorted representative order | 5,000 | 117.696 | `UNSAT_unverified` | source-layout mistake; retained as a failed attempt |
| corrected selector and phase pins, sorted representative order | 5,000 | 125.253 | `SAT_unverified` at producer; independent replay `PASS_EXTERNAL_XCNF_AND_ORIGINAL_FIELD` | full model checked |
| sorted representative pins only | 5,000 | 246.643 | `UNKNOWN_solver_cap` | phase variables free |
| same | 100,000 | 2,928.439 | `UNKNOWN_solver_cap` | phase variables free |
| known phase pins only | 5,000 | 149.543 | `SAT_unverified` at producer; independent replay `PASS_EXTERNAL_XCNF_AND_ORIGINAL_FIELD` | representative selectors free |

The phase-only control has 35 extra unit clauses, SHA-256
`89348c5797536847d558596c13144262ae5bce010b19c3dc84c223226ad52379`.
The representative-only control has five extra units, SHA-256
`e70ba46885f35a8b0088e3f5cc15bc626e032ff7fe821580a46ded945cca916d`.
The observed fixed-witness access bottleneck is searching Frobenius phase
choices with this encoding and solver. These controls use the known witness;
the next ordinary-query panel must measure whether a phase-aware algorithm
improves natural relation yield per charged unit.

The source `constrain_frobenius_orbit_domains` sorts representatives before
allocating selector variables. The first sidecar indexed their input order,
so it fixed the wrong selector literals. `pin_orbit_choices.py` now matches
the source ordering. This correction changed only the 40 extra choice unit
clauses; the common exported XCNF remained byte-identical. The failed
formula and receipt remain frozen.

The original XCNF has 101,636 variables, 714,736 CNF clauses, and 4,980 XOR
rows (13,202,037 bytes), SHA-256
`c2cc36eee7a72dbc583a217a0d42d02c74d5d3fde241536941b6614fc565b81e`.
The corrected formula adds 40 choice units and has SHA-256
`8569c449f00b0aab6e1b7457ec1383e663e1ae3260c456c56b47dd032dd7a4e5`.
All five XCNF byte streams are stored as deterministic gzip files in
`evidence/`; their uncompressed hashes are in their export receipts.

## Reproduce and verify

From the repository root, with CryptoMiniSat 5.14.7 available:

```sh
P=research/n83_chain_xcnf_control_20261019
D=research/koblitz_n83_factor_base_sweep_20261008/verification/known-witness-ordered-sat-20261014/domains.json
CARGO_TARGET_DIR=/path/to/writable/target cargo run --release --offline --example n83_chain_xcnf_control -- "$D" /tmp/n83-instance.xcnf
python3 "$P/pin_orbit_choices.py" /tmp/n83-instance.xcnf "$P/evidence/export.json" "$D" /tmp/n83-pinned.xcnf /tmp/n83-pinned.json
python3 "$P/pin_orbit_choices.py" /tmp/n83-instance.xcnf "$P/evidence/export.json" "$D" /tmp/n83-phase.xcnf /tmp/n83-phase.json --mode phase
python3 "$P/run_external.py" /tmp/n83-pinned.xcnf /tmp/n83-cms --seconds 30 --conflicts 5000
python3 "$P/verify.py" /tmp/n83-pinned.xcnf /tmp/n83-cms/solver.stdout "$D" /tmp/n83-pinned.json
```

The local files in `/tmp` must be fresh because export and run tools use
exclusive creation. To verify the committed model directly, decompress
`evidence/pinned_sorted.xcnf.gz` and
`evidence/cms_pinned_sorted_30s_5k/solver.stdout.gz`, then pass their
decompressed paths, the retained domains, and
`evidence/pinned_sorted_export.json` to `verify.py`. The checked result was:

```json
{"clauses":714776,"curve_point_sum_verified":true,"fixed_blocks":8,"status":"PASS_EXTERNAL_XCNF_AND_ORIGINAL_FIELD","variables":101636,"xcnf_sha256":"8569c449f00b0aab6e1b7457ec1383e663e1ae3260c456c56b47dd032dd7a4e5","xors":4980}
```

`python3 "$P/audit.py"` decompresses and hashes all five archived formulas,
checks all eight first-attempt solver receipts against raw output, and replays
both SAT models through the independent XCNF, field, and curve verifier.
The exporter rebuilt from formatted source SHA-256
`d33cbdc4986f33cd6a173770e5b02562f61937b7f2017f51568fff2c614fb559`
regenerated the original XCNF byte-for-byte. The rebuilt local executable
had SHA-256 `419e4a8cbaf199a6ca0a411808737a51206c81d30d657f7b3d81b3407fef36ef`.
The complete release library gate passed 4,399 tests, with 118 ignored and
zero failures; the example test target compiled. Logs are in `evidence/`.

The next discriminating experiment is a frozen ordinary-query XCNF panel
across the primary candidate bases, with conflict and wall caps, then natural
verified relation yield and useful-rank measurement. A complete base
selection additionally needs final matrix solving, target descent, and a
same-point rho pair under the benchmark tier contract.
