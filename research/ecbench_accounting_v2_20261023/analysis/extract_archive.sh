#!/bin/bash
# Re-create sessions/j0-frozen-main from its verbatim archive, to re-run compare_all.sh / cross_all.sh / inventory.sh.
# Do NOT commit the extracted directory: the branch binary's audit rejects it by design (README / REPORT, "j = 0 floor").
set -eu
cd "$(dirname "$0")/.."
sha256sum -c archive/j0-frozen-main.tar.gz.sha256
tar -xzf archive/j0-frozen-main.tar.gz -C sessions
