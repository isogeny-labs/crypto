#!/usr/bin/env bash
# Assembles RESULTS.md = hand-written head + auto-generated tables (every number in the tables comes from results/*.tsv via the analyzer) + tail.
cd "$(dirname "$0")"
{
  cat prose_head.md
  echo
  cat results/auto_tables.md
  echo
  cat prose_tail.md
} > RESULTS.md
