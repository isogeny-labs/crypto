# Using ecdb from an agent

ecdb is the lab's curve database: a Postgres catalogue on RDS with an HTTP
API and an MCP endpoint, both served by the `ecdb-api` Lambda in us-west-2.
It holds:

- 21,080 genus-1 curves over 892 fields: the curve registry, the
  isogeny-walk outputs and the trait catalogue;
- 20,815 genus-2 and genus-3 cover curves, their Jacobians and explicit
  covers;
- 478 kernel-certified isogenies;
- 61,305 priced DLP transfer routes;
- Frobenius data, End(E), volcano levels, embedding degrees, twists and
  subgroups;
- division, modular, class and summation polynomials, GLV endomorphisms with
  decomposition lattices, typed curve relations, group structures and p-ranks;
- measured results of this repository: the IC leaderboard (28 priced curves,
  122 recipes), the progress timeline (16 series, 94 points), the boundary
  ledger, the `benchmarks/` index (385 entries with their `VALIDATION.json`),
  and 1,103 ecbench sessions with 543,290 runs and 1,082 paired comparisons;
- the sibling repositories' records:
  - cryptanalysis: the IC candidate catalogue, factor bases, identity fixtures
    and the challenge index;
  - crypto-autoresearcher: curve capsules, the trait snapshot, NIST-seed
    candidates, the endomorphism sweep and the solver fixtures;
  - cryptanalysis-rl: the tool-policy census and stage 0b;
  - ml-cryptanalysis: the rho-measured curve corpora.

  In total 10,760 records and 24,096 measurements. The 9,189 prime-field
  dataset curves among them are ecdb curves (family `dataset`) under their
  computed ICV1 slugs.
- curves generated with Sage and re-verified on import
  ([`../sage/README.md`](../sage/README.md)):
  - family `census-fp`: every curve over F_p, p < 2^10, up to isomorphism;
  - family `census-f2m`: every ordinary curve over GF(2^m), m ≤ 10;
  - families `crypto-prime`, `crypto-prime-twist`, `crypto-cm`: 32–256-bit
    curves;
  - families `pairing-bn`, `pairing-bls12`, `pairing-bls24`, `pairing-kss16`,
    `pairing-kss18`;
  - family `genus2-random`: genus-2 Jacobians.

  Filter by `family` to keep a census from swamping a search.

The design and the full table list are in [`SCHEMA.md`](SCHEMA.md). The
routes are in [`../openapi.yaml`](../openapi.yaml).

Read this page before an agent queries or writes the database. The repository
rule that points here is AGENTS.md, "Querying the curve database (ecdb)".

## When to use it

- **Before computing a curve fact,** look it up: trace, group order, cofactor,
  CM discriminant, End(E), embedding degree, twist order, l-torsion action,
  volcano level, isogeny neighbours, covers and transfer prices. One call
  replaces a pass over the 30 MB registry and trait files.
- **For "find me a curve with ..." questions:** `ecdb_search_curves`, or SQL
  over `v_curve_summary`.
- **To resolve a curve name:** ICV1 slug, EC1 alias, `curve_uid`, standard
  name (secp256k1, P-256, sect163k1), OID or any alias, through the same
  lookup that `GET /v1/curves/{ref}` uses. An ambiguous name returns 409 with
  the candidates. Pick one explicitly; never guess.

It does not replace the repository's own files as the source of record. Those
files are the registry, `traits.json`, the walk outputs and the certificates;
ecdb is imported from them at a pinned commit (`GET /v1/meta` lists the
paths, SHA-256 digests and the commit). A claim in a PR cites the repository
file or certificate. An ecdb query answer, or its id, is not that citation.

## Connecting

The endpoint and token:

```sh
export ECDB_URL=https://l6zg6lz4dr45t75p5qn5aocnme0pckrm.lambda-url.us-west-2.on.aws
export ECDB_TOKEN=$(aws secretsmanager get-secret-value --region us-west-2 \
    --secret-id ecdb/api --query SecretString --output text | jq -r .read_token)
```

- **MCP.** The repository's `.mcp.json` registers server `ecdb` at
  `$ECDB_URL/mcp` with header `Authorization: Bearer ${ECDB_TOKEN}`. Export
  `ECDB_TOKEN` before the runtime starts. Without it, the handshake and tool
  list still work, but every tool call returns an error that names the
  secret.
- **HTTP.** For example:

  ```sh
  curl -s -H "authorization: Bearer $ECDB_TOKEN" "$ECDB_URL/v1/curves/secp256k1"
  ```

Rules for the token:

- Never print it, commit it, paste it into a PR, log or task card, or pass
  it on a command line that is logged.
- Read it into an environment variable as above.
- A session without AWS credentials for secret `ecdb/api` has no access.
  Say so; do not look for the token elsewhere.

## MCP tools

| Tool | What it returns | REST route |
|:--|:--|:--|
| `ecdb_schema` | tables and views with comments; with `table`, its columns, types, keys and CHECKs | `GET /v1/schema` |
| `ecdb_search_curves` | filtered, sorted page of `v_curve_summary` (field, genus, model, family, trace, j, CM, class number, End, subgroup bits, cofactor, embedding degree, twist security, ...) | `GET /v1/curves` |
| `ecdb_get_curve` | the whole curve document | `GET /v1/curves/{ref}` |
| `ecdb_curve_detail` | one part: `isogenies`, `graph` (with `depth`, `ell`), `transfers`, `covers`, `traits`, `relations`, `endomorphisms`, `division-polynomials`, `measurements` (leaderboard rows, ecbench arms, dataset measurements, sibling records) | `GET /v1/curves/{ref}/{part}` |
| `ecdb_isogeny_class` | one isogeny class over F_q (any dimension) by label `g.q.a1_..._ag` or id | `GET /v1/isogeny-classes/{label}` |
| `ecdb_transfers` | transfer routes across all curves, by route kind, verdict, target and minimum ratio | `GET /v1/transfers` |
| `ecdb_polynomials` | `modular` (Phi_l, l = 2..19), `class` (H_D, class number one) or `summation` (Semaev S_2, S_3) | `GET /v1/*-polynomials` |
| `ecdb_results` | `leaderboard` (filters `family`, `regime`, `validation`; `recipes=true`), `progress` (`series`), `boundary` (`regime`, `stage`), `benchmarks` (`listed`, `q`) | `GET /v1/ic/*`, `GET /v1/benchmarks` |
| `ecdb_ecbench` | a page of ecbench sessions (filters `method`, `curve`, `label`, `path`); with `session_id`, its per-arm summaries and comparisons | `GET /v1/ecbench/sessions[/{id}]` |
| `ecdb_sql` | one read-only SELECT over schema `ec` (15 s timeout, 5,000 rows) | `POST /v1/sql` |
| `ecdb_record_evidence` | attach a claim, with its artifact, to a curve; needs the write token | `POST /v1/evidence` |

Start with `ecdb_schema` (no argument) to see the relations, then
`ecdb_schema {"table": "..."}` before writing SQL against a table you have
not used.

Without MCP, `scripts/ecdb.sh` (in each repository that reads ecdb) does the
same over HTTP: `ecdb.sh curve <ref> [part]`, `ecdb.sh sql '<query>'`,
`ecdb.sh results leaderboard 'family=A'`, `ecdb.sh ecbench [session]`, and
`ecdb.sh check`, which reports whether ecdb's pinned copy of the current
repository's files matches the checkout.

## Reading the answers

- **Exact integers.** Values beyond 2^53 are decimal strings. Compare them as
  integers, never as floats. Field elements are strings in the field's
  encoding: decimal for F_p, the recorded bit or hex encoding for F_{2^m}
  (see the `field` object).
- **Status.** Every derived value carries a status from `ec.fact_status`:
  `proved`, `probable`, `bounded`, `heuristic`, `conjectural`, `refuted`,
  `unknown`, `not_evaluated`, `not_applicable`. Report the status with the
  value. `probable` (for
  example, a primality test without a certificate) is not `proved`. Unknown
  is `null`, never 0 or false.
- **Transfers.** `dlp_transfer` rows are formula estimates under the cost
  model in their provenance parameters. A large `ratio_log2` is a lead for
  the research process (AGENTS.md "The rule: boundary, table, ratio" and
  §5a). It is not a measured speedup and never goes into the IC leaderboard
  or `benchmarks/`.
- **Relations.**
  - Computed relations exist only for j ≠ 0, 1728 and t ≠ 0, between curves
    over the same field with equal j: traces t and −t give `quadratic_twist`;
    equal traces give `isomorphism` (same model) or `birational_model_change`
    (e.g. Montgomery or Edwards against Weierstrass).
  - Sextic and quartic twists at j = 0, 1728 are not recorded yet.
  - None of them transports a discrete logarithm. A walk needs an `isogeny`
    row with its kernel certificate, as the typed curve-link rules require.
- **GLV.** An endomorphism of kind `glv` with a non-null `eigenvalue` was
  checked on the curve's recorded generator: φ(G) = [λ]G. The matching
  `scalar_decomposition` row holds a reduced basis of the decomposition
  lattice. A scalar-multiplication speedup is a separate, measured claim.
- **Division polynomials.** These are in x only, with ascending coefficients:
  - n = 2 is ψ_2² = 4x³ + b2 x² + 2 b4 x + b6;
  - odd n is ψ_n;
  - n = 4 is ψ_4/ψ_2.

  Only prime-field curves and n ≤ 5 are stored.
- **Provenance.** Each row points to an `ec.source`: a repository path with
  its SHA-256 and commit, or a computation (`ecdb:enrich`, `agent:<name>`).
  Rows another agent wrote through `ecdb_record_evidence` are claims with
  that agent's status, not established results.

## Measured results

These rows are copies of the producers' frozen files, with their source file
and SHA-256. They are not new measurements. Read them under the repository's
own rules:

- **Leaderboard.** `ic_board_row.best_ratio_rho` is IC / rho:
  - it is the best recipe's cold cost against the matched rho of the row's
    family, in that family's unit (`ic_family.unit`; table A batched-addition
    units, table B GAE, table C historical batch);
  - compare ratios only within one family;
  - `validation_status` is the gate's standing, and
    `unvalidated (pre-gate)` rows are not admitted results;
  - quote a row with its gap to rho, as the distance remaining (the success
    criterion), never as a pass or fail.
- **ecbench runs.**
  - `s` is cost in GAE per sqrt(r).
  - `lower_bound = true` means native work went unpriced; it is listed in
    `unpriced`.
  - Rank two arms only where the order holds at 0, 0.1 and 1 GAE per
    unpriced operation (AGENTS.md §5a).
  - `v_ecbench_arm` excludes warm-up runs.
  - A session's replay audit, when it has one, is in
    `ecbench_session.audit`.
- **Sibling measurements.** `measurement` keeps each producer's metric and
  unit:
  - ml-cryptanalysis S of plain and automorphism-class rho, with SEM and
    seed count;
  - cryptanalysis-rl group operations and ratio to expected rho.

  Compare within one dataset and metric only.
- **Sibling records.** `external_curve_record.match` says how a record was
  linked:
  - `ec1` and `curve_uid`: exact representation;
  - `icv1_slug`: same model, possibly another representation;
  - `computed_icv1`: a dataset curve named by its computed slug;
  - `standard_name`: a unique standard name;
  - `none`: kept unlinked.

  Never link records by j or by group order.
- **Citations.** A claim cites the producer's file or run, which `source`
  names, not the ecdb row.

## SQL

`ecdb_sql` and `POST /v1/sql` run one statement inside a READ ONLY
transaction as role `ecdb_reader`, with `search_path = ec, public`. Useful
starting points:

- `v_curve_summary` has one row per curve with the common filters.
- `v_curve_summary` also carries `jacobian_id` and `isogeny_class_id`:
  - `isogeny_class_fq` (Weil polynomial, order, p-rank) and
    `frobenius_genus1` (trace, discriminants, CM, conductor, class number) key
    on `isogeny_class_id`;
  - `subgroup` keys on `av_id` (the Jacobian);
  - `endomorphism_ring` and `curve_relation` key on the curve;
  - `isogeny` and `dlp_transfer` key on `source_curve_id`.
- `polynomial` holds content-addressed coefficients (JSON).
  `division_polynomial`, `modular_polynomial`, `class_polynomial` and
  `summation_polynomial` link to it.

```sql
-- prime-field curves with CM by -3 and a subgroup of at least 200 bits
SELECT icv1_slug, standard_names, field_bits, subgroup_bits
  FROM v_curve_summary
 WHERE cm_discriminant = -3 AND field_kind = 'prime' AND subgroup_bits >= 200
 ORDER BY field_bits;

-- the twist of a curve, with its trace
SELECT c2.icv1_slug, r.kind
  FROM curve_relation r JOIN curve c1 ON c1.id = r.source_curve_id JOIN curve c2 ON c2.id = r.target_curve_id
 WHERE c1.icv1_slug = '<slug>' AND r.kind = 'quadratic_twist';
```

Large result sets: use `LIMIT` and `OFFSET`. Output beyond 5,000 rows is
cut, and the response carries `"truncated": true`.

## Writing

Writes need `write_token` from the same secret, and go only through
`ecdb_record_evidence` (`POST /v1/evidence`) and `POST /v1/isogenies`.

- Writes are records of claims. They do not establish results.
- Write only what a run or certificate in a PR supports.
- Give the artifact's repository path and SHA-256, and the true status.
- Never write to correct an imported row. The fix belongs in the repository
  file it came from, followed by a re-import (`deploy/README.md`).

## Keeping it current

The deployed data are as of the commit in `GET /v1/meta`. Redeploying
re-imports every catalogue and the computed enrichment:
`ecdb/deploy/build-lambda.sh`, then `ecdb/deploy/deploy-aws.sh`
(`deploy/README.md`).

- Do this after a change to anything ecdb imports:
  - `docs/curves/registry.json`, `traits.json`, `covers.json`,
    `jacobian-transfers.json`, the certificates, or a walk output;
  - `docs/ic/leaderboard.json`, `progress-timeline.json`,
    `boundary_targets.json` or `benchmarks/`;
  - a new ecbench session;
  - the sibling files listed in `ecdb/src/import/siblings.rs`.

  `scripts/ecdb.sh check` lists the files whose checkout differs from what
  ecdb holds.
- A new importer or producer goes in `ecdb/src/import/`, in Rust, with a test
  that checks its values (the repository's no-Python rule applies).
