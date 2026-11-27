#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_scan_call_instrument_20261127
test ! -e "$s/qualification-data.tar.xz"
test ! -e "$s/qualification-workers.tar.xz"
taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=2147483648 --fsize=1073741824 -- tar -C /dev/shm -c -I 'xz -T1 -9 --lzma2=dict=64MiB' -f "$s/qualification-data.tar.xz" relation-scan-call-transport-20261127 relation-scan-call-neutral-20261127 relation-scan-call-neutral-analysis-20261127 relation-scan-call-hypothetical20-20261127 relation-scan-call-hypothetical20-analysis-20261127 relation-scan-call-canary-clock-20261127 relation-scan-call-canary-clock-analysis-20261127 relation-scan-call-canary-duplicate-20261127 relation-scan-call-canary-duplicate-analysis-20261127 relation-scan-call-unqualified-refusal-20261127 > "$s/raw/archive-data.stdout" 2> "$s/raw/archive-data.stderr"
taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=2147483648 --fsize=1073741824 -- tar -C /dev/shm -c -I 'xz -T1 -9 --lzma2=dict=64MiB' -f "$s/qualification-workers.tar.xz" relation-scan-call-workers-20261127 > "$s/raw/archive-workers.stdout" 2> "$s/raw/archive-workers.stderr"
