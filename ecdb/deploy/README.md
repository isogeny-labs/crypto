# Deploying ecdb on AWS (RDS + Lambda)

```
agents ──HTTPS──▶ Lambda function URL ─▶ ecdb-api (VPC) ──TLS 5432──▶ RDS PostgreSQL 16 "ecdb" (private)
                                         ecdb-admin (VPC) ──────────▶   (migrate, login roles, imports)
```

- **Database.** RDS PostgreSQL 16 `ecdb`: `db.t4g.medium`, 20 GB gp3,
  encrypted, not publicly accessible, deletion protection on, 7-day backups,
  TLS enforced (RDS default for PostgreSQL 15+).
- **Lambda functions.** Both are `provided.al2023` and both run the same
  static musl `ecdb` binary (`ecdb lambda api|admin`).
  - `ecdb-api` serves the HTTP API on a public function URL. The URL is open,
    but every data endpoint requires the bearer token; `/healthz` and
    `/openapi.yaml` do not. `POST /mcp` serves MCP on the same function: the
    handshake and tool list are open, and tool calls need the token.
  - `ecdb-admin` has no URL. Each invocation runs one action:
    `{"action": "status" | "migrate" | "bootstrap_roles"}` or
    `{"action": "import", "what": "registry" | "walks" | "traits" | "covers" | "transfers" | "certificates" | "siblings" | "derived" | "enrich" | "results" | "ecbench" | "all"}`,
    or `{"action": "import_pack", "name": "ecbench" | "generated", "data": "<base64 of one gzipped pack part>"}`.
- **Network.**
  - Lambda security group `ecdb-lambda`: egress only to 5432 on `ecdb-rds`.
  - RDS security group `ecdb-rds`: ingress only from `ecdb-lambda`.

  The functions need no internet access: the catalogues ship gzipped inside
  the package, and credentials are in their (KMS-encrypted) environment.
- **Secrets** (Secrets Manager):
  - `ecdb/master`: the RDS master login.
  - `ecdb/api`:
    - `api_password` for login `ecdb_api` (member of `ecdb_reader` only);
    - `writer_password` for login `ecdb_api_writer` (member of `ecdb_writer`);
    - `read_token` and `write_token`, the API's bearer tokens.

## One-time IAM setup (an admin)

```sh
aws iam create-role --role-name ecdb-lambda-exec \
  --assume-role-policy-document '{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"lambda.amazonaws.com"},"Action":"sts:AssumeRole"}]}'
aws iam attach-role-policy --role-name ecdb-lambda-exec \
  --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole
aws iam create-policy --policy-name ecdb-deploy --policy-document file://ecdb/deploy/iam-policy.json
aws iam attach-user-policy --user-name <deployer> --policy-arn arn:aws:iam::<account>:policy/ecdb-deploy
```

[`iam-policy.json`](iam-policy.json) is limited to us-west-2 and to
resources named `ecdb*` or tagged `project=ecdb`. It also covers the default
KMS keys (used through RDS and Lambda only) and passing the one
`ecdb-lambda-exec` role to Lambda. It grants no IAM writes and no EC2
instances. The account id in it is the lab account's; change it for another
account.

## Build and deploy

```sh
ecdb/deploy/build-lambda.sh /tmp/ecdb-lambda.zip     # static musl build + gzipped catalogues, about 12 MB
AWS_REGION=us-west-2 ecdb/deploy/deploy-aws.sh /tmp/ecdb-lambda.zip
```

`build-lambda.sh` also runs `ecdb pack`:

- the `benchmarks/` index and the sibling checkouts' files (expected beside
  this checkout) go into the zip under `data/`;
- the ecbench sessions pack to about 70 MB gzipped, over Lambda's 50 MB
  direct-upload limit, so they go to `<zip>.ecbench/` as parts of about
  2 MB.

`deploy-aws.sh` is idempotent: it reuses resources by name, updates function
code and configuration, then runs migrate, bootstrap_roles, every importer,
one `import_pack` invocation per ecbench part, and status. Re-running it after a catalogue change re-imports; imports are
idempotent. `ECDB_SKIP_IMPORT=1` skips the import. The first run waits about
10 minutes for RDS to become available.

## Loading Sage-generated curves

`ecdb/deploy/load-generated.sh OUT_DIR [set ...]` sends the finished units of
`ecdb/sage/generate.sage` output to the admin function. It splits them into
chunks of about 12 MB and makes one `import_pack` invocation per chunk. The
importer re-verifies every record and is idempotent, so rerun the script as
more units finish.

## Using it

```sh
URL=$(aws lambda get-function-url-config --function-name ecdb-api --query FunctionUrl --output text)
TOKEN=$(aws secretsmanager get-secret-value --secret-id ecdb/api --query SecretString --output text | jq -r .read_token)
curl -s -H "authorization: Bearer $TOKEN" "${URL}v1/curves/secp256k1"
```

MCP clients point at `${URL}mcp` with the header
`Authorization: Bearer $TOKEN`. In this repository, `.mcp.json` registers it
as server `ecdb` and reads the token from `ECDB_TOKEN`; see
[`../docs/AGENTS.md`](../docs/AGENTS.md).

Rotate a token or password by editing `ecdb/api`, then re-run
`deploy-aws.sh` with `ECDB_SKIP_IMPORT=1`. The script refreshes the function
environment and `bootstrap_roles` re-passwords the logins.

## Cost (us-west-2, on-demand, approximate)

| Item | Approximate cost |
|:--|:--|
| `db.t4g.medium` | about $47/month |
| 20 GB gp3 storage | about $2.30/month |
| Backups | within the free allowance at this size |
| Lambda | pennies at agent query volumes |
| Secrets Manager | $0.80/month (2 secrets) |
