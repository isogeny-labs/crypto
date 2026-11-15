#!/usr/bin/env bash
set -euo pipefail
for d in /var/tmp/pencil-comparator-word-20261101/snapshot /var/tmp/independent-product-review-20261101/snapshot /var/tmp/pencil-mumford-validity-20261031-v1/dependency-union /var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union;do
 find "$d" -maxdepth 1 \( -name '*.lean' -o -name '*.olean' \) -print0
done | sort -z | xargs -0 sha256sum
