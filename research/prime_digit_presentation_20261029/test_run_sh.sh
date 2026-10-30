#!/bin/sh
# Safety test for run.sh (2026-10-29 review): a failing substitute binary, and a pre-existing OUT,
# must leave the committed results/ untouched.  Run from this directory; prints PASS or FAIL lines.
set -u
cd "$(dirname "$0")"
sum() { cat results/*.jsonl | shasum -a 256 | cut -d' ' -f1; }
before=$(sum)
T=$(mktemp -d "${TMPDIR:-/tmp}/runsh-test.XXXXXX")
fail=0
# 1. a substitute binary that always fails: run.sh stops, committed results unchanged
printf '#!/bin/sh\nexit 1\n' > "$T/failbin"; chmod +x "$T/failbin"
if DIGIT_FALL="$T/failbin" ./run.sh "$T/out1" >/dev/null 2>&1; then echo "FAIL 1: run.sh succeeded with a failing binary"; fail=1; else echo "PASS 1: run.sh exits non-zero when the binary fails"; fi
[ "$(sum)" = "$before" ] && echo "PASS 2: committed results/ unchanged after the failing run" || { echo "FAIL 2: results/ changed"; fail=1; }
# 3. an existing OUT directory is refused and left alone
mkdir "$T/exists"; echo keep > "$T/exists/A.jsonl"
if DIGIT_FALL="$T/failbin" ./run.sh "$T/exists" >/dev/null 2>&1; then echo "FAIL 3: overwrote an existing OUT"; fail=1; else [ "$(cat "$T/exists/A.jsonl")" = keep ] && echo "PASS 3: existing OUT refused and untouched" || { echo "FAIL 3: existing OUT modified"; fail=1; }; fi
# 4. no argument: usage error, nothing written
if ./run.sh >/dev/null 2>&1; then echo "FAIL 4: ran without OUT"; fail=1; else echo "PASS 4: OUT argument is required"; fi
[ "$(sum)" = "$before" ] && echo "PASS 5: committed results/ unchanged at the end" || { echo "FAIL 5: results/ changed"; fail=1; }
# 6. the new launcher issues the same commands as the launcher that produced panels B-E
printf '#!/bin/sh\necho "$@"\n' > "$T/stub"; chmod +x "$T/stub"
mkdir "$T/v1"; cp registered/run.sh.v1 "$T/v1/run.sh"; (cd "$T/v1" && DIGIT_FALL="$T/stub" sh ./run.sh >/dev/null 2>&1)
DIGIT_FALL="$T/stub" ./run.sh "$T/out2" >/dev/null 2>&1
ok=1; for p in B C D E; do cmp -s "$T/v1/results/$p.jsonl" "$T/out2/$p.jsonl" || { ok=0; echo "FAIL 6: panel $p commands differ"; fail=1; }; done
[ "$ok" = 1 ] && echo "PASS 6: panels B-E commands identical to registered/run.sh.v1"
mkdir "$T/v0"; cp registered/run.sh.v0 "$T/v0/run.sh"; (cd "$T/v0" && DIGIT_FALL="$T/stub" sh ./run.sh >/dev/null 2>&1)
cmp -s "$T/v0/results/A.jsonl" "$T/out2/A.jsonl" && echo "PASS 7: panel A commands identical to registered/run.sh.v0" || { echo "FAIL 7: panel A commands differ"; fail=1; }
rm -rf "$T"
exit $fail
