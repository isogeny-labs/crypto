#!/usr/bin/env bash
# Compare the validator's per-record uncharged re-derivation (from a session's
# raw records.jsonl) with the committed runs/uncharged/<spec>.csv.xz and
# runs/extract/<spec>.csv.xz. Usage: unc_compare.sh STUDY SPEC RECORDS_JSONL
set -eu
STUDY=$1; SPEC=$2; REC=$3; HERE=$(dirname "$0")
jq -r -f "$HERE/unc_from_records.jq" "$REC" > "$REC.mine.tsv"
awk -F'\t' -v spec="$SPEC" '
  FNR == 1 { fileno++ }
  fileno == 1 { k = $1 SUBSEP $2 SUBSEP $3 SUBSEP $4; g[k] = $5; s[k] = $6; u[k] = $7; n1++; next }
  fileno == 2 { if (FNR == 1) next; split($0, f, ","); k = f[2] SUBSEP f[3] SUBSEP f[4] SUBSEP f[5]
                n2++; if (!(k in u)) { miss++; next } seen[k] = 1
                if (g[k] + 0 != f[6] + 0) dg++; if (s[k] + 0 != f[7] + 0) ds++; if (u[k] + 0 != f[8] + 0) du++; next }
  fileno == 3 { if (FNR == 1) next; gsub(/"/, ""); split($0, f, ","); if (f[12] != "false") next
                k = f[7] SUBSEP f[8] SUBSEP f[9] SUBSEP f[11]; n3++; if (!(k in u)) { miss3++; next }
                if (g[k] + 0 != f[13] + 0) eg++; if (s[k] + 0 != f[14] + 0) es++ }
  END { for (k in u) if (!(k in seen)) extra++
        printf "{\"spec\":\"%s\",\"records_measured\":%d,\"uncharged_rows\":%d,\"extract_measured_rows\":%d,\"uncharged_rows_without_record\":%d,\"records_without_uncharged_row\":%d,\"extract_rows_without_record\":%d,\"total_gae_mismatch\":%d,\"s_mismatch\":%d,\"unc_total_mismatch\":%d,\"extract_ops_gae_mismatch\":%d,\"extract_S_mismatch\":%d}\n", spec, n1, n2, n3, miss + 0, extra + 0, miss3 + 0, dg + 0, ds + 0, du + 0, eg + 0, es + 0 }
' "$REC.mine.tsv" <(xz -dc "$STUDY/runs/uncharged/$SPEC.csv.xz") <(xz -dc "$STUDY/runs/extract/$SPEC.csv.xz")
rm -f "$REC.mine.tsv"
