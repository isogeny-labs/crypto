#!/bin/bash
jq -S '.report | walk(if type=="object" then with_entries(select(.key|test("_ns$|_ms$|^ns_per_|wall|phases_ns"; "i")|not)) else . end)' "$1"
