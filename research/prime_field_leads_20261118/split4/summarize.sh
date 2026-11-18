#!/usr/bin/env bash
set -euo pipefail
export LC_ALL=C
awk -F, '
$1=="query" {
    id=$2 OFS $3 OFS $4 OFS $5
    online[id]+=$19; mults[id]+=$13; inversions[id]+=$14; visits[id]+=$16
    lookups[id]+=$17; compares[id]+=$18; count[id]+=$9; calls[id]++
}
$1=="left" {
    id=$2 OFS $3 OFS $4 OFS $5
    left[id]=$19; lf[id]=$13; li[id]=$14; bytes[id]=$11+$12; keys[id]=$10
}
$1=="right" {
    id=$2 OFS $3 OFS $4
    right[id]=$19; rf[id]=$13; ri[id]=$14; rb[id]=$11
}
END {
    for (id in online) {
        split(id,a,OFS); r=a[1] OFS a[2] OFS a[3]
        if (calls[id]!=24 || !(r in right) || !(id in left)) exit 1
        print id,(left[id]+right[r]+online[id])/1e6,left[id]/1e6,right[r]/1e6,
            online[id]/1e6,lf[id],rf[r],mults[id],li[id],ri[r],inversions[id],
            visits[id],lookups[id],compares[id],count[id],keys[id],bytes[id],rb[r]
    }
}
' "${1:-measurements.csv}" | sort -k1n -k2 -k3n -k4
