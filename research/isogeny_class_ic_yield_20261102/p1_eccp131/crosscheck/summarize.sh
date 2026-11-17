#!/usr/bin/env bash
# Tables of CROSSCHECK.md from the rows of run_crosscheck.sh.   Usage: summarize.sh ROWS.jsonl...
cat "$@" | jq -s -r '
 "primes\t" + ([.[]|.label|split("|")[0]]|unique|join(" ")),
 "systems\t\(length)   F4-comparable (p < 2^32)\t\([.[]|select(.f4.complete)]|length)   failures\t\([.[]|.failures|length]|add)",
 "solution sets truth==f4\t\([.[]|select(.solution_sets=="equal")]|length)   ladder planted-unique\t\([.[]|select(.solution_sets=="planted_unique")]|length)",
 "dense-reference systems agree\t\([.[]|select(.dense_reference=="agree")]|length)   (system,degree) pairs\t\([.[]|select(.dense_reference=="agree")|.dense.rows|length]|add)   mismatches\t\([.[]|select(.dense_reference=="MISMATCH")]|length)",
 "",
 "config\tkind\tn\tengine D_solve\tf4 solving_degree\tf4 solving_degree_max\tf4 degree_reached\tK\tf4 dim\tengine n_std",
 (group_by([.config,.kind])|.[]|select(.[0].f4.complete) | [ .[0].config, .[0].kind, (map(select(.f4.complete))|length),
   ([.[]|select(.f4.complete)|.engine.D_solve]|unique|tostring), ([.[]|select(.f4.complete)|.f4.solving_degree]|unique|tostring),
   ([.[]|select(.f4.complete)|.f4.solving_degree_max]|unique|tostring), ([.[]|select(.f4.complete)|.f4.degree_reached]|unique|tostring),
   ([.[]|select(.f4.complete)|.K_closure]|unique|tostring), ([.[]|select(.f4.complete)|.f4.dim]|unique|tostring),
   ([.[]|select(.f4.complete)|.engine.nstd_final]|unique|tostring) ] | @tsv)'
