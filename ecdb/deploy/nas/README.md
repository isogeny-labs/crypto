# ecdb on a NAS (or any Docker host on your LAN)

This runs a private copy of ecdb on your own hardware:
- Postgres 16;
- the same HTTP API and MCP endpoint as the deployed service, on port 8080;
- `sync`, which copies the deployed catalogue into the local database through
  the API's bulk export.

After the copy, clients on your Wi-Fi query the NAS and never reach AWS.

Requirements:
- Docker with the compose plugin: Synology Container Manager, QNAP Container
  Station, TrueNAS apps or plain Docker all work.
- x86-64 or arm64. The image builds from source on either.
- About 4 GB of disk for the database. The first build needs about 3 GB more.

## Set up

```sh
git clone https://github.com/isogeny-labs/crypto.git && cd crypto/ecdb/deploy/nas
cp .env.example .env        # set ECDB_ADMIN_PASSWORD, ECDB_API_PASSWORD, ECDB_TOKEN
docker compose up -d --build
docker compose run --rm sync
```

`up` builds the image, which takes 10–20 minutes on a NAS CPU. It then:
1. starts Postgres;
2. runs `ecdb bootstrap`: migrations, plus the read-only login `ecdb_api`;
3. starts the API.

The catalogue is empty until `sync` runs. `sync`:
- fetches `/v1/export` from `ECDB_URL` (default: the deployed Lambda URL);
- checks that the remote schema version matches the image's migrations;
- copies all tables inside one transaction, about 2.3 GB at schema version 10;
- moves the identity sequences past the copied ids;
- refuses to commit unless every table's row count matches the remote.

A failed sync leaves the previous contents in place.

`ECDB_TOKEN` is the read token of the deployed API. It is the `read_token`
field of the AWS Secrets Manager secret `ecdb/api`:

```sh
aws secretsmanager get-secret-value --region us-west-2 --secret-id ecdb/api \
  --query SecretString --output text | jq -r .read_token
```

Keep `.env` on the NAS; `.gitignore` covers it.

## Use it from the LAN

| What | Where |
|:--|:--|
| API | `http://<nas>:8080/v1/curves/secp256k1`, `/v1/meta`, `/openapi.yaml` |
| Health | `http://<nas>:8080/healthz` |
| MCP (streamable HTTP) | `http://<nas>:8080/mcp` |

Point Claude Code at the local MCP endpoint:

```sh
claude mcp add --transport http ecdb-nas http://<nas>:8080/mcp
# with ECDB_LAN_TOKEN set:
claude mcp add --transport http ecdb-nas http://<nas>:8080/mcp --header "Authorization: Bearer $ECDB_LAN_TOKEN"
```

Reads are open to anyone who can reach port 8080, unless `ECDB_LAN_TOKEN` is
set in `.env`. Writes are off: the API connects as `ecdb_api`, a member of
`ecdb_reader`, and `POST /v1/sql` runs as `ecdb_reader` with the usual timeout
and row cap. Don't forward the port to the internet.

## Refresh

The deployed catalogue grows when new curves are generated or loaded. To
pull it again:

```sh
ECDB_SYNC_REPLACE=true docker compose run --rm sync
```

A replacing sync holds exclusive locks on the tables until it commits, so
the local API waits on it for those few minutes. To refresh nightly, add a
NAS task-scheduler or cron entry:

```sh
cd /path/to/crypto/ecdb/deploy/nas && git pull -q && docker compose build -q && \
  docker compose up -d && ECDB_SYNC_REPLACE=true docker compose run --rm sync
```

If `sync` reports a schema version mismatch, the deployed service has
migrated ahead of your image. Run `git pull` and `docker compose up -d --build`
first; `bootstrap` applies the new migrations.

## Without the deployed copy

The image also carries the repository catalogues, so a local database can be
filled without AWS. That gives the curated curves but not the Sage-generated
census or isogeny graphs:

```sh
docker compose run --rm bootstrap import --repo-root /srv/crypto all
```

`ecdb/sage/README.md` regenerates the census curves, which are seeded and
reproducible; `ecdb import generated` then loads them.

## Back up and reset

```sh
docker compose exec db pg_dump -U ecdb_admin -Fc ecdb > ecdb.dump   # backup
docker compose down && rm -rf pgdata                                # wipe; next `up` starts empty
```
