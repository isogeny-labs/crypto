#!/bin/bash
# compare_outputs.sh <file-form outputs> <job-form outputs>: diff of the two verbatim output files after removing only what must
# differ between two runs: UTC timestamps, wall times in ms and elapsed seconds, the pid and driver line number of bash's
# "Killed" notices. Prints the diff (empty = no other difference) and a line count.
n() { sed -e 's/[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}T[0-9:]*Z/TS/g' -e 's/ [0-9][0-9]*\(\.[0-9][0-9]*\)\{0,1\} ms/ N ms/g' \
          -e 's/elapsed=[0-9]*s/elapsed=Ns/' -e 's/line [0-9]*: *[0-9]* Killed/line N: PID Killed/' "$1"; }
diff <(n "$1") <(n "$2"); echo "diff lines: $(diff <(n "$1") <(n "$2") | wc -l)"
