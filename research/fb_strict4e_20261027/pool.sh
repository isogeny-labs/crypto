#!/usr/bin/env bash
# Pool summary/pairs.tsv: per-cell mean over the 4 sieve seeds of D = log2(prefix/strict),
# at band weights 0 (recorded S), 0.1 and 1; pooled mean over the 6 cells; percentile
# bootstrap over cells (B=10000, srand(20261027)). Positive = strict4e cheaper.
cd "$(dirname "$0")"
awk -F'\t' -v B=10000 '
NR>1 { c=$1"/"$2; if(!(c in seen)){seen[c]=1; cells[++nc]=c}
  d0[c]+=$9; d1[c]+=-log($11)/log(2); d2[c]+=-log($12)/log(2); k[c]++ }
END{
  srand(20261027);
  for(w=0;w<3;w++){
    for(i=1;i<=nc;i++){c=cells[i]; v[w,i]=(w==0?d0[c]:(w==1?d1[c]:d2[c]))/k[c]}
    m=0; for(i=1;i<=nc;i++) m+=v[w,i]; m/=nc; mean[w]=m
    for(b=1;b<=B;b++){s=0; for(j=1;j<=nc;j++){i=int(rand()*nc)+1; s+=v[w,i]} bs[b]=s/nc}
    gap=B; while(gap>1){gap=int(gap/2); for(i2=gap+1;i2<=B;i2++){t=bs[i2]; j2=i2; while(j2>gap && bs[j2-gap]>t){bs[j2]=bs[j2-gap]; j2-=gap} bs[j2]=t}} lo[w]=bs[int(0.025*B)+1]; hi[w]=bs[int(0.975*B)]
  }
  printf "{\n  \"unit\": \"bits: log2(S_prefix/S_strict4e); positive = strict4e cheaper\",\n  \"bootstrap\": \"percentile over %d cells, B=%d, srand(20261027)\",\n  \"cells\": [", nc, B
  for(i=1;i<=nc;i++) printf "%s{\"cell\":\"n/l=%s\",\"w0\":%.4f,\"w0.1\":%.4f,\"w1\":%.4f}", (i>1?",":""), cells[i], v[0,i], v[1,i], v[2,i]
  printf "],\n  \"pooled\": {\"w0\":{\"mean\":%.4f,\"ci95\":[%.4f,%.4f]},\"w0.1\":{\"mean\":%.4f,\"ci95\":[%.4f,%.4f]},\"w1\":{\"mean\":%.4f,\"ci95\":[%.4f,%.4f]}}\n}\n", mean[0],lo[0],hi[0],mean[1],lo[1],hi[1],mean[2],lo[2],hi[2]
}' summary/pairs.tsv > summary/pooled.json
cat summary/pooled.json
