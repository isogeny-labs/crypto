# H109 reproduction

Run from this directory with Python 3.11+.

```sh
python -m venv .venv
. .venv/bin/activate
python -m pip install -r requirements.txt
python run.py --output /tmp/h109-replay.json
```

The wall-clock fields are diagnostic and may differ. The runner records a
scientific-content digest that excludes clock and environment fields; it must
equal:

```text
3991b25d2a2ddac0ebb1d25370c8b110051a40acca64a6bda3dd3baba4191b69
```

The archived measurement can be decompressed and hash-checked with:

```sh
gzip -dc results/torsion-division-derivative.json.gz > /tmp/h109-recorded.json
sha256sum /tmp/h109-recorded.json results/torsion-division-derivative.json.gz
```

Expected hashes are `940123c44c6194b247fab3596a9ed2e331ee7e01b2fb8830fc7e8465e354993a`
and `f4f815d9938794f814b6f289c78729e1b571bb2ff4c897829d6d84d8508b5bf4`.

For the formal interfaces:

```sh
cd formal
lake build
```

The independent standard-library verifier and its replay receipt live in
[isogeny-labs/cryptanalysis#1342](https://github.com/isogeny-labs/cryptanalysis/pull/1342).
