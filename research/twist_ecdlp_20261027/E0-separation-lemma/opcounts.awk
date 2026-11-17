# opcounts.awk -- OPERATION COUNTS derived from recorded raw rows (no new computation, no instrumentation).
# Each count is a closed-form function of recorded fields (q, nE, nEt, n2) and of the loop structure of the frozen scripts
# (lib_e0.gp: ptsFq, fullenum, pairstats; t1class, t2class). Field multiplications inside GP are NOT instrumented and are not reported.
BEGIN{FS="|"}
function add(tag,q,nE,nEt,n2){
  cls[tag]++
  if(tag=="T1"){
    pair[tag]+=nE*nEt; if(n2!=nE*nEt) mism[tag]++
    scan[tag]+=q*q                 # x in F_{q^2}: one ordinate (cubic evaluation + quadratic solve) each, fullenum
    pairavoid[tag]+=0
  } else {
    pairavoid[tag]+=nE*nEt-nEt     # pair-loop additions the B-only check does not perform (hypothetical nE*nEt vs nEt)
  }
  ord[tag]+=2*q                    # ptsFq(E), ptsFq(E'): q ordinates each
  bcand[tag]+=nEt                  # phi(B) candidates tested (israt vs 2B=O) in T1 bseen and T2/T3 loop
  pinv[tag]+=nE                    # phi^{-1}(A) membership tests (S9)
}
/^T1\|/ { add("T1",$2,$7,$8,$12); next }
/^T2\|/ { add("T2",$2,$7,$8,0); next }
/^T2S\|/{ add("T2S",$2,$7,$8,0); next }
/^T3\|/ { add("T3",$2,$7,$8,0); next }
END{
  n=split("T1 T2 T2S T3",tags," ")
  for(i=1;i<=n;i++){t=tags[i]; if(!(t in cls)) continue
    printf "ops.%s.classes=%d\n",t,cls[t]
    printf "ops.%s.ordinate_evals_ptsFq=%d\n",t,ord[t]
    printf "ops.%s.phiB_candidates_tested=%d\n",t,bcand[t]
    printf "ops.%s.phiinv_membership_tests=%d\n",t,pinv[t]
    if(t=="T1"){
      printf "ops.T1.pair_additions_AplusB=%d\n",pair[t]
      printf "ops.T1.pair_additions_vs_n2_mismatches=%d\n",mism[t]+0
      printf "ops.T1.ordinate_evals_fullenum_Fq2=%d\n",scan[t]
      tot=pair[t]+scan[t]+ord[t]
      printf "ops.T1.total_counted_ops=%d\n",tot
    } else {
      printf "ops.%s.pair_additions_avoided_vs_T1_loop=%d\n",t,pairavoid[t]
      printf "ops.%s.total_counted_ops=%d\n",t,ord[t]+bcand[t]+pinv[t]
    }
  }
}
