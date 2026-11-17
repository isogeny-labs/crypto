# After the raster-size correction

On published headae2d6670344797830f98bc87cb62b9675f2cce82, artifacts job133837635308 passes its artifacts.py check HEAD^1 HEAD step at05:24:34Z. It then fails the next wire_workflows.py --check step:35workflows lack required store pulls or have stale pulls. The tool and relevant artifacts/validation workflows are unchanged from main; base-wiring-diff.txt is empty. This is a later masked baseline failure, not a remaining oversized-raster problem or an all-green result. Original source-head CI_BLOCKERS.md and its pinned proof subject remain unchanged. [pcc:unverified]

Decoded full response remains LOCAL at/tmp/ic-table47-native/ci-current-artifacts-133837635308.json, SHA256bc9a2c2501045d328b1d02e1501d7f5bbf2854261265252456dfa4ac11462f8a. Ordinary CI/review remain required; no gate bypass or unrelated generated-workflow rewrite. [pcc:unverified]
