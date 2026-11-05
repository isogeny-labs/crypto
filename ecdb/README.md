# ecdb

A PostgreSQL catalogue (RDS-ready, Postgres 15+) of elliptic curves, curves
of higher genus, Jacobians, isogenies, covers and DLP transfers, with an HTTP
API and an MCP endpoint that agents query. A deployed instance runs on AWS
(see [`deploy/README.md`](deploy/README.md)); how agents use it is in
[`docs/AGENTS.md`](docs/AGENTS.md). The design and the reasons behind it are in
[`docs/SCHEMA.md`](docs/SCHEMA.md); the API is described in
[`openapi.yaml`](openapi.yaml), which `GET /openapi.yaml` also serves.

It is a standalone Rust crate (its own `[workspace]` and lock file), outside
the root library's build graph.

## Quick start (local)

```sh
cd ecdb
cargo build --release
export DATABASE_URL='postgres://postgres@localhost:5432/ecdb' ECDB_PG_TLS=disable

./target/release/ecdb migrate                      # schema ec, eight migrations
./target/release/ecdb import --repo-root .. all    # registry, walks, traits, covers, transfers, certificates, derived, enrich
./target/release/ecdb serve                        # http://0.0.0.0:8080

curl -s localhost:8080/v1/meta
curl -s localhost:8080/v1/curves/secp256k1
curl -s 'localhost:8080/v1/curves?family=koblitz&field_bits_min=160'
curl -s -X POST localhost:8080/v1/sql -H 'content-type: application/json' \
     -d '{"sql": "SELECT cm_discriminant, count(*) FROM v_curve_summary GROUP BY 1 ORDER BY 2 DESC LIMIT 10"}'
```

`ecdb import` subcommands:

- `registry`, `walks [files...]`, `traits`, `covers`, `transfers`,
  `certificates`, `derived` and `enrich` run one importer each; `all` runs
  all of them in dependency order.
- `enrich` computes from the imported rows, in Rust (`src/ec.rs`,
  `src/import/enrich.rs`):
  - Phi_l for l up to 19 (from `src/cryptanalysis/curve_traits/modular_polynomials.txt`);
  - the 13 class-number-one H_D, each checked against every curve with that j;
  - Semaev S_2 and S_3;
  - division polynomials psi_2..psi_5 per prime-field genus-1 curve;
  - GLV endomorphisms for j = 0 and 1728, with the eigenvalue checked on the
    generator and a reduced decomposition lattice;
  - twist, isomorphism and model-change relations;
  - cyclic group structures where gcd(n, q - 1) decides them;
  - p-ranks and Hasse invariants.
- `lmfdb-av-fq <file>` loads an LMFDB `av_fq_isog` JSON export (isogeny
  classes of abelian varieties over finite fields, any dimension). Each label
  is recomputed from its Weil polynomial and must match.

Imports are idempotent. Each prints a JSON report of row counts and of every
record it refused, with the reason.

## Endpoints

| Method and path | Purpose |
|:--|:--|
| `GET /v1/meta` | counts, schema version, pinned input files (path, SHA-256, commit) |
| `GET /v1/schema` | every table and view with column types, comments, foreign keys and CHECKs; enums; functions |
| `GET /v1/curves` | filter and sort over `ec.v_curve_summary`, by field, genus, model, family, trace, j, CM discriminant, class number, End, subgroup size, cofactor, embedding degree, twist security, anomalous / supersingular, name and more |
| `GET /v1/curves/{ref}` | the full curve document; `{ref}` is an id, ICV1 slug or string, EC1 alias or `curve_uid`, standard name, OID or any alias |
| `GET /v1/curves/{ref}/isogenies` | isogenies from or to the curve, with kernel polynomials and certificates |
| `GET /v1/curves/{ref}/graph?depth=&ell=` | the recorded isogeny graph around the curve |
| `GET /v1/curves/{ref}/transfers` | priced DLP transfer routes for the curve |
| `GET /v1/curves/{ref}/covers` | covers onto (or from) the curve, with the cover's model and map |
| `GET /v1/curves/{ref}/traits` | verbatim producer trait records |
| `GET /v1/curves/{ref}/relations` | quadratic twists, isomorphisms and model changes to other curves |
| `GET /v1/curves/{ref}/endomorphisms` | Frobenius, GLV (with eigenvalue) and subfield-tau endomorphisms, GLV decomposition lattices |
| `GET /v1/curves/{ref}/division-polynomials` | psi_2..psi_5 in x, ascending coefficients over the curve's field |
| `GET /v1/isogenies/{id}` | one isogeny |
| `GET /v1/isogeny-classes`, `GET /v1/isogeny-classes/{label}` | isogeny classes over F_q by Weil polynomial, any dimension |
| `GET /v1/fields`, `GET /v1/fields/{id}` | field representations |
| `GET /v1/transfers` | transfer routes across all curves |
| `GET /v1/polynomials/{sha256}` | the content-addressed polynomial store |
| `GET /v1/modular-polynomials?level=&coefficients=` | Phi_l over Z, l = 2..19 |
| `GET /v1/class-polynomials`, `GET /v1/summation-polynomials` | H_D for class number one; Semaev S_2, S_3 |
| `POST /v1/sql` | one read-only query |
| `POST /v1/evidence`, `POST /v1/isogenies` | token-gated agent writes |
| `POST /mcp` | MCP (streamable HTTP, JSON responses, stateless): nine tools over the routes above |

Integers beyond 2^53 come back as decimal strings. Unknown values are
`null` with a status, never 0. An ambiguous name returns 409 with the
candidates.

## Configuration

| Variable | Default | Meaning |
|:--|:--|:--|
| `DATABASE_URL` | (required) | read connection: a login that is a member of `ecdb_reader` only |
| `ECDB_PG_TLS` | `require` | `disable` for local sockets |
| `ECDB_PG_CA_FILE` | webpki roots | PEM bundle of trusted CAs. RDS needs the Amazon RDS bundle (`global-bundle.pem`) |
| `ECDB_LISTEN` | `0.0.0.0:8080` | |
| `ECDB_READ_TOKEN` | unset | bearer token required on reads; unset means reads are open |
| `ECDB_WRITE_TOKEN`, `ECDB_WRITE_DATABASE_URL` | unset | enable `POST /v1/evidence` and `POST /v1/isogenies`; the write login is a member of `ecdb_writer` |
| `ECDB_SQL_ROLE` | unset | role `POST /v1/sql` switches to (`ecdb_reader`) |
| `ECDB_SQL_TIMEOUT_MS`, `ECDB_SQL_MAX_ROWS` | 15000, 5000 | limits of `POST /v1/sql` |
| `ECDB_MIGRATE_ON_START` | false | run pending migrations before serving |
| `ECDB_POOL_SIZE` | 16 | connections per pool |

## Deploying on RDS

1. **Create the instance.** Postgres 15 or later (the schema uses
   `UNIQUE NULLS NOT DISTINCT`). Allow the `pg_trgm` and `btree_gin`
   extensions; both are on the RDS allow list. Turn on
   `rds.force_ssl = 1`. A full import of this checkout is about 340 MB with
   indexes, so `db.t4g.medium` with 20 GB gp3 is enough to start.
2. **Migrate** as the master user or a schema owner. This creates schema
   `ec` and the NOLOGIN group roles `ecdb_reader` and `ecdb_writer`:

   ```sh
   export DATABASE_URL='postgres://admin:...@<endpoint>:5432/ecdb' ECDB_PG_CA_FILE=global-bundle.pem
   ecdb migrate && ecdb import --repo-root /path/to/crypto all
   ```
3. **Create the logins the service uses**, each in exactly one group role.
   `POST /v1/sql` can reset `role` to the session user inside its own
   read-only transaction, so the read login must not hold more than
   `ecdb_reader`.

   ```sql
   CREATE ROLE ecdb_api LOGIN PASSWORD '...' IN ROLE ecdb_reader;
   CREATE ROLE ecdb_api_writer LOGIN PASSWORD '...' IN ROLE ecdb_writer;
   ```
4. **Run the service.** Use the container in `Dockerfile` on ECS / Fargate,
   EKS or an EC2 host in the database's VPC. Inject the passwords from
   Secrets Manager into `DATABASE_URL` / `ECDB_WRITE_DATABASE_URL`, set
   `ECDB_SQL_ROLE=ecdb_reader`, and set tokens if the API is reachable
   beyond the agents' network. A later step can add RDS IAM authentication
   tokens; the pool takes passwords from the URL today.

## Tests

```sh
cargo test --lib            # field arithmetic, identities, labels, JSON shaping
ECDB_TEST_DATABASE_URL='postgres://postgres@localhost/ecdb_test' cargo test --test api
```

The integration test needs a database whose name ends in `_test`. It drops
and recreates schema `ec` there, migrates twice (the second run must be a
no-op), imports the registry, walks and traits from this checkout (the
registry twice, to show idempotence), runs `derived` and `enrich`, then
queries through the router and the MCP endpoint.
