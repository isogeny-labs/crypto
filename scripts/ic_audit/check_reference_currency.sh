#!/bin/bash
# Fails when the standing rho's source differs from the version a leaderboard table's reference was last
# assessed against (docs/ic/validation/reference-currency.json: assessed_blobs, git blob hashes, so a
# squash or merge cannot break it).  Also reports how many commits changed the rho since the table was
# measured.  Needs git and jq; the commit count needs history (fetch-depth 0).
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
cfg=${CURRENCY_CFG:-docs/ic/validation/reference-currency.json}
bad=0
for t in $(jq -r '.tables | keys[]' "$cfg"); do
  measured=$(jq -r ".tables.$t.measured_at" "$cfg")
  text=$(jq -r ".tables.$t.assessment" "$cfg")
  [ -n "$text" ] && [ "$text" != null ] || { echo "table $t: no assessment text"; bad=1; }
  for e in $(jq -r ".tables.$t.evidence[]" "$cfg"); do [ -e "$e" ] || { echo "table $t: evidence $e is missing"; bad=1; }; done
  for f in $(jq -r ".tables.$t.rho_paths[]" "$cfg"); do
    now=$(git rev-parse "HEAD:$f")
    was=$(jq -r ".tables.$t.assessed_blobs[\"$f\"] // empty" "$cfg")
    n=$(git rev-list --count "$measured..HEAD" -- "$f" 2>/dev/null || echo "?")
    if [ "$now" = "$was" ]; then echo "table $t: $f assessed at blob ${now:0:9} ($n commits since the table was measured)"
    else echo "table $t: $f is now blob ${now:0:9} but was assessed at ${was:0:9}: re-measure the table's reference or update its assessment in $cfg"; bad=1; fi
  done
done
exit $bad
