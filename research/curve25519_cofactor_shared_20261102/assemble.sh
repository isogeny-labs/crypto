#!/bin/bash
# README.md = head + generated tables + tail (sed fills the test lines from files if present).
cd "$(dirname "$0")"
A=analysis
{ cat README.head.md; cat $A/table.md; cat README.mid.md; cat $A/ratios.md; cat README.trace.md; cat $A/trace.md; cat README.tail.md; } > README.md
