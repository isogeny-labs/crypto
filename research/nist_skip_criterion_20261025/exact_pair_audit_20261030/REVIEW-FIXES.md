# Corrections to PR 2981 review

30 October 2026. These corrections address review comments 4426242371 and
4426242364 on revision 3331b40a7ff9065658c2dfee7bb63101bf503999.

The original active flatten.py path violated the repository's native-language
requirement even though it only copied integers. copy-score-fields.cpp and
build-native.sh replace that execution path; REPRODUCE.md invokes native code
only. json-c 0.18 parses the JSON; C++ validates selected field types, labels,
cohort size, score range, unique identities and output freshness. No statistical
or cryptographic arithmetic moves into a scripting language. Historical
flatten.py and sources/heldout.py remain bound, inactive source snapshots and
must not be executed. Retaining their bytes does not erase the original violation.

The first native build passes with g++ 14.2.0, C++17, -O2, -Wall, -Wextra,
-Werror and -pedantic. All 68 generated rows match the frozen TSV exactly
(SHA256 d64817340813111618807b4b382014bca27c418560778b7018c1194e11067531).
Six malformed fixtures are refused with exit1 and no output, and an overwrite
attempt is refused with the existing bytes unchanged. Native jq creates the
fixtures; no Python is executed. See test-native-preparation.sh and
review-evidence/. The unchanged statistical kernel is not rerun for an input
that matches byte for byte. No new mathematical result or formal proof is claimed.

The original 1064052-byte archive, SHA256
3bedeed0581f3e6ac286be9b69d340055e2c9ea8c83c9c1561aa4e78af1327f6,
is now at the content-addressed S3 URI in DELIVERY.json and .artifacts.json.
The conditional write used If-None-Match:* and the full-object SHA256 checksum.
A complete GET matched the original archive byte for byte; fresh extraction
checked the explicit member allowlist and all 140 member size/hash bindings
(141 regular members including MEMBERS.json). S3 access is authenticated through
the lab artifact store, not advertised as anonymous. Retrieval and extraction
commands are in REPRODUCE.md. No credentials enter these files or the archive.

The payload contains public-source exact-audit records only, including the
first-run binaries, logs, corrupted controls, and report. The earlier denied
39-member auxiliary paper payload remains excluded and was not retried.
This upload at 18:29 UTC provides later custody, not a pre-run checkpoint,
canonical experiment gate, repair of source-study deviations, or new sampling.
The first conditional PUT was rejected for an invalid checksum after local xxd
was found absent; no accepted object resulted. The corrected checksum was
computed before the successful write and independently checked on readback.

All 21 original preparation bindings remain unchanged. Original RESULTS.md,
REPRODUCE.md, DELIVERY.json and the original 13-entry RESULTS-SHA256SUMS are
retained under historical/pre-review/. The original manifest's paths are relative
to the study at revision 3331b40; only its three documentary files change now.
The current RESULTS-SHA256SUMS explicitly binds their amended current versions.
To check the original manifest against retained documentary snapshots and the
unchanged mathematical files from the study directory:

```sh
awk '{name=$2; if (name=="RESULTS.md" || name=="REPRODUCE.md" || name=="DELIVERY.json") name="historical/pre-review/" name; printf "%s  %s\n",$1,name}' historical/pre-review/RESULTS-SHA256SUMS | sha256sum -c -
```

The merge of main preserves both complete session-log additions after Copilot
did not respond for over an hour. Only log markers were removed; no frozen
evidence conflict existed. An initial conservative prefix check refused to
resolve because the base log was not a complete byte prefix; no content was
discarded. The eventual log diff against main is exactly our 93 added lines.
The mechanically merged progress exporter retains main's additions and the
new pass-through allocation inventory. No gate, CI configuration or protection
is changed. Current-head hosted checks and independent review remain required
before the user's conditional auto-merge approval can be exercised.
