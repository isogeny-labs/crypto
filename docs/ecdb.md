# ecdb has moved

The curve database service is maintained in [isogeny-labs/ecdb](https://github.com/isogeny-labs/ecdb).
Its Rust crate, migrations, API, browser viewer, Sage generators, tests and
deployment scripts now live at the root of that repository. This repository
has no tracked `ecdb/` directory.

- [Service README and local setup](https://github.com/isogeny-labs/ecdb/blob/main/README.md)
- [Agent query guide](https://github.com/isogeny-labs/ecdb/blob/main/docs/AGENTS.md)
- [Schema and provenance](https://github.com/isogeny-labs/ecdb/blob/main/docs/SCHEMA.md)
- [AWS deployment](https://github.com/isogeny-labs/ecdb/blob/main/deploy/README.md)
- [Docker / LAN mirror](https://github.com/isogeny-labs/ecdb/blob/main/deploy/nas/README.md)
- [Sage generators](https://github.com/isogeny-labs/ecdb/blob/main/sage/README.md)

The catalogues consumed by ecdb remain in crypto. Keep the checkouts beside
each other, or set `CRYPTO_ROOT` to the crypto checkout when building the
Lambda package or running the database integration tests. For CLI imports,
pass `--repo-root /path/to/crypto`. Build the Docker image from the ecdb
checkout with:

```sh
docker build --build-context crypto=../crypto -t ecdb .
```

Earlier session logs may describe paths under `ecdb/` as they existed at the
time. For current source or deployment instructions, use the links above.
