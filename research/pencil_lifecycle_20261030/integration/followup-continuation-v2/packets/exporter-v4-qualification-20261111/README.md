# Review and execution map

Read PLAN.md,commands.json,admission-draft.json and runtime-provenance.json. The runtime files are concrete derivatives of the existing durable supervisor, including its phase-aware target directory monitor. Source application is implemented by materialize.py using source-overlay.tar.gz, overlays.json and expected-source-manifest.json; source-layer-bindings.json records v1/v4 application origins and pre/post hashes. No Git mutation occurs at runtime.

Expanded overlay/ is ignored staging material for review; publish its exact archive through canonical artifact storage. Large JSON manifests likewise require canonical storage. No upload is performed here. Do not stage expanded overlay directly.

Prospective entrypoint: /usr/bin/python3 -B durable-launcher.py after separately reviewed tracked admission. All hashing/staging/resolver/build/tests are inside the one bounded native attempt.
