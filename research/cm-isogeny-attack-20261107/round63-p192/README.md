# Round63: signed baby-block first response

Read report.md, paired-algebra-proof.md and scope.json. Scientific data and cost receipts are separate. The candidate uses public generator points and sqrt-sized baby/giant tables; the qualifier additionally constructs complete graphs and dense norm references, whose costs are retained. No full P192 response is evaluated.

Hydrate any .artifacts.json entries before manifest checks or replay. From this directory with sibling fixtures:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/signed-baby-target timeout 120 cargo build --release --locked --offline --manifest-path native/Cargo.toml
/tmp/signed-baby-target/release/signed-baby-response --check response.json > /tmp/response.json 2> /tmp/response.log
cmp response.json /tmp/response.json
cmp response.log /tmp/response.log
/tmp/signed-baby-target/release/signed-baby-response --full-reference --check reference.json > /tmp/reference.json 2> /tmp/reference.log
cmp reference.json /tmp/reference.json
cmp reference.log /tmp/reference.log
/tmp/signed-baby-target/release/p192 "$PWD" --check P192-blocks.json > /tmp/P192.json
cmp P192-blocks.json /tmp/P192.json
sha256sum --check dependencies.sha256
sha256sum --check manifest.sha256
```

Small counters count quadratic-field multiplications including inversion powers, inversions, and all group-add calls. P192 counters count explicit K multiplication/inversion calls and point-add calls; inverse internals are excluded. Do not combine the two models. Candidate counts retain sign guards and block recombination checks; the interior algebra reference is separately counted. Whole-qualifier totals include scalar-fixture and reference work. Time and peak memory were not measured. Claims remain unverified until a signed independent verdict.
