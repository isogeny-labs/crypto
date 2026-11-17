# H110 reproduction

Run from this directory with Python 3.11+; the experiment uses only the
standard library.

```sh
python run.py --output /tmp/h110-replay.json
```

The diagnostic clock fields may differ. The scientific-content digest must be:

```text
3397823fe3f55fe008797bb87b31324b7ae33efb5699fe479e8e58f5832db4de
```

The archived result hashes are checked with:

```sh
gzip -dc results/p2-lift-defect-cocycle.json.gz > /tmp/h110-recorded.json
sha256sum /tmp/h110-recorded.json results/p2-lift-defect-cocycle.json.gz
```

Expected hashes are `7d6c390d0156a4ed101bddd0969132a85505fa884282ef505dc4621b7852124e`
and `a69d06462068b9ad5ac16d46d3d6e29b7e05cbad0bff7a75b3dce59c5cbee7d8`.

Formal interfaces:

```sh
cd formal
lake build
```

The independent arithmetic implementation and receipt live in
[cryptanalysis#1346](https://github.com/isogeny-labs/cryptanalysis/pull/1346).
