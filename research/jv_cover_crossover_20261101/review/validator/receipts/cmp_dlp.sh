#!/usr/bin/env bash
# field-by-field comparison of two single-row jv_cover dlp JSON records, time fields excluded
a=$1; b=$2
ex() { grep -o '"[a-z_0-9]*": [-0-9.e+]*\|"[a-z_0-9]*": true\|"[a-z_0-9]*": false\|"[a-z_0-9]*": "[a-z:0-9]*"' "$1" | grep -v '"f4_ms"\|"lin_ms"\|"wall_ms"'; }
diff <(ex $a) <(ex $b) > /dev/null && echo "ALL NON-TIME FIELDS IDENTICAL" || { echo "DIFFERING FIELDS:"; diff <(ex $a) <(ex $b); }
echo "time fields: A $(grep -o '"f4_ms": [0-9.]*\|"lin_ms": [0-9.]*\|"wall_ms": [0-9.]*' $a | tr '\n' ' ')"; echo "             B $(grep -o '"f4_ms": [0-9.]*\|"lin_ms": [0-9.]*\|"wall_ms": [0-9.]*' $b | tr '\n' ' ')"
