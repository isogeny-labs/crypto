# aggregate.awk -- reads recorded rows (out/T1*.txt out/T2*.txt out/T3.txt out/T4.txt out/T5.txt out/controls.txt) and prints key=value lines.
BEGIN{FS="|"}
function famof(k){return (k==1)?"odd":((k==2)?"binord":"binss")}
function ispow2(n){ if(n<1)return 0; while(n%2==0)n=n/2; return (n==1) }
function addflags(tier,fl,   a,n,i){ if(fl=="OK")return; n=split(fl,a,","); for(i=1;i<=n;i++) ftype[tier "." a[i]]++ }
FNR==1{ nfiles++ }
($1=="T1"||$1=="T2"||$1=="T2S"||$1=="T3"){
  tier=$1; q=$2; kind=$3; w=$5+0; t=$6+0; nE=$7+0; nEt=$8+0; g2=$9+0; cop=$11+0;
  if(tier=="T1"){ fl=$24; litfail=($22+0==0)?1:0; bij=$23+0 } else { fl=$16; litfail=$15+0; bij=-1 }
  fam=famof(kind); key=tier "." fam "." q;
  keys[key]=1; cnt[key]++; W[key]+=w;
  if(fl!="OK"){viol[key]++; totviol[tier]++; if(index(fl,"ORA")>0) ora[tier]++}
  addflags(tier,fl);
  if(g2>1){ng[key]++; Wg[key]+=w}
  if(cop==1){ncop[key]++; Wcop[key]+=w}
  if(cop==1&&g2>1){ncg[key]++}
  if(cop==1&&litfail){ncl[key]++; totncl[tier]++}
  if(litfail){nlit[key]++; Wlit[key]+=w}
  if(litfail!=(g2>1)){off[key]++; totoff[tier]++}
  if(nE%2==0){Wev[key]+=w}
  if(t==0){nt0[key]++; if(g2>1)nt0g[key]++; if(litfail)nt0l[key]++}
  tab[tier "." litfail "." (g2>1)]++; tabW[tier "." litfail "." (g2>1)]+=w;
  g2d[tier "." fam "." g2]++;
  if(tier=="T1"){ if((bij==1)!=(g2==1)){m7[tier]++}; if(bij==1)nbij[key]++ }
  if(fam=="binss"&&t==0){ ss0[tier "." q]++; if(litfail==0)ss0lit[tier "." q]++; if(tier=="T1"&&bij==1)ss0bij[q]++; if(cop==0)ss0nc[tier "." q]++ }
  if(fam=="odd"&&q>=101&&q<=199&&tier=="T2"){ P5W+=w; P5E+=(nE%2==0)?w:0 }
  if(fam=="odd"&&ispow2(q+1)){ mers[tier "." q]=1; mersn[tier "." q]++; mersg[tier "." q]+=(g2>1); mersl[tier "." q]+=litfail; merscg[tier "." q]+=(cop==1&&g2>1) }
  if(fam=="odd"&&t==0){ t0odd[tier]++; if(g2==1)t0oddbad[tier]++ }
  if(fam=="binord"){ binord[tier]++; if(g2!=2)binordbad[tier]++ }
  if(fam=="binss"){ binss[tier]++; if(g2!=1)binssbad[tier]++ }
}
$1=="WITNESS"{ nwit++; wit[nwit]=$0 }
$1=="T4"{ g2=$9+0; iso=$15+0; nat=$16; q=$2;
  n4[g2==1]++; if(iso==1)i4[g2==1]++; if(nat=="MISMATCH")nat4++;
  k4=q; c4[k4]++; if(g2==1){c4a[k4]++; if(iso==1)c4ai[k4]++} else {c4b[k4]++; if(iso==1)c4bi[k4]++}
  if(g2==1&&iso!=1)t4bad++ }
$1=="T5"{ p=$2; t=$5+0; gg=$8+0; fl=$15; n5++; if(fl!="OK")v5++; addflags("T5",fl);
  if(gg>1)gg5++; if(t!=0&&gg>1)gg5nt0++; if((t==0))t05++;
  if($13!="NA"){ s=$13; gsub(/[\[\] ]/,"",s); split(s,a,","); nv1+=a[1]; ne1+=a[2]; nb1+=a[3]; nc1+=a[4]; if(a[5]>=0){ if(t==0){r1t0++} else {r1nt0++} } if(t==0&&$13!="NA")n1t0++ }
  if($14!="NA"){ s=$14; gsub(/[\[\] ]/,"",s); split(s,a,","); nv3+=a[1]; ne3+=a[2]; nb3+=a[3]; nc3+=a[4]; if(a[5]>=0){ if(t==0){r3t0++} else {r3nt0++} } if(t==0)n3t0++ ; n3++ }
}
($1=="PC1"||$1=="PC2"||$1=="PC3"||$1=="NC1"||$1=="NC2"||$1=="NC3"){ ctag=$1; nrow=NF; res=$NF; cn[ctag]++; if(res=="FAIL")cf[ctag]++ }
$1=="NC2"&&NF<=5{ nc2na++ }
END{
  # per-key lines
  n=0; for(k in keys){ klist[++n]=k } 
  # simple insertion sort of keys
  for(i=2;i<=n;i++){ v=klist[i]; j=i-1; while(j>=1 && klist[j]>v){klist[j+1]=klist[j]; j--} klist[j+1]=v }
  for(i=1;i<=n;i++){ k=klist[i];
    printf "%s.classes=%d\n%s.weight=%d\n%s.violations=%d\n%s.g2gt1=%d\n%s.weight_g2gt1=%d\n%s.coprime=%d\n%s.coprime_and_g2gt1=%d\n%s.coprime_and_literal_fails=%d\n%s.literal_fails=%d\n%s.weight_literal_fails=%d\n%s.offdiag=%d\n%s.weight_nE_even=%d\n%s.t0_classes=%d\n%s.t0_g2gt1=%d\n",
      k,cnt[k],k,W[k],k,viol[k]+0,k,ng[k]+0,k,Wg[k]+0,k,ncop[k]+0,k,ncg[k]+0,k,ncl[k]+0,k,nlit[k]+0,k,Wlit[k]+0,k,off[k]+0,k,Wev[k]+0,k,nt0[k]+0,k,nt0g[k]+0 }
  # tier totals
  nt=split("T1 T2 T2S T3",tl," ");
  for(i=1;i<=nt;i++){ t=tl[i];
    printf "%s.M1_violations=%d\n%s.M2_offdiag=%d\n%s.M3_coprime_and_literal_fails=%d\n%s.M5_oracle_disagreements=%d\n",t,totviol[t]+0,t,totoff[t]+0,t,totncl[t]+0,t,ora[t]+0;
    printf "%s.table.litholds_g2eq1=%d\n%s.table.litholds_g2gt1=%d\n%s.table.litfails_g2eq1=%d\n%s.table.litfails_g2gt1=%d\n",t,tab[t ".0.0"]+0,t,tab[t ".0.1"]+0,t,tab[t ".1.0"]+0,t,tab[t ".1.1"]+0;
    printf "%s.M7_bij_vs_g2=%d\n",t,m7[t]+0;
    printf "%s.odd_t0_classes=%d\n%s.odd_t0_g2eq1=%d\n",t,t0odd[t]+0,t,t0oddbad[t]+0;
    printf "%s.binord_classes=%d\n%s.binord_g2ne2=%d\n%s.binss_classes=%d\n%s.binss_g2ne1=%d\n",t,binord[t]+0,t,binordbad[t]+0,t,binss[t]+0,t,binssbad[t]+0 }
  for(f in ftype) printf "flagcount.%s=%d\n",f,ftype[f]
  # S6 contrast, supersingular t=0
  for(k in ss0) printf "ss_t0.%s.classes=%d\nss_t0.%s.lemma_holds=%d\nss_t0.%s.odd_parts_not_coprime=%d\n",k,ss0[k],k,ss0lit[k]+0,k,ss0nc[k]+0
  for(k in ss0bij) printf "ss_t0.T1.%s.sum_map_bijective=%d\n",k,ss0bij[k]
  # Mersenne q
  for(k in mers) printf "mersenne.%s.classes=%d\nmersenne.%s.g2gt1=%d\nmersenne.%s.literal_fails=%d\nmersenne.%s.coprime_and_g2gt1=%d\n",k,mersn[k],k,mersg[k],k,mersl[k],k,merscg[k]
  # P5
  if(P5W>0) printf "P5.weighted_fraction_nE_even_p101_199=%.6f\nP5.total_weight=%d\nP5.even_weight=%d\n",P5E/P5W,P5W,P5E
  # T4
  printf "T4.g2eq1_classes=%d\nT4.g2eq1_abstract_iso=%d\nT4.g2gt1_classes=%d\nT4.g2gt1_abstract_iso=%d\nT4.native_mismatch=%d\nT4.g2eq1_not_iso=%d\n",n4[1]+0,i4[1]+0,n4[0]+0,i4[0]+0,nat4+0,t4bad+0
  # T5
  printf "T5.classes=%d\nT5.rows_with_flags=%d\nT5.gcd_gt1=%d\nT5.gcd_gt1_and_t_ne0=%d\nT5.t0_classes=%d\n",n5+0,v5+0,gg5+0,gg5nt0+0,t05+0
  printf "T5.bfs1.nodes_total=%d\nT5.bfs1.edges_total=%d\nT5.bfs1.invalid_candidates=%d\nT5.bfs1.neighbour_card_ne_nE=%d\nT5.bfs1.t0_classes_reaching_Eprime=%d\nT5.bfs1.tne0_classes_reaching_Eprime=%d\n",nv1+0,ne1+0,nb1+0,nc1+0,r1t0+0,r1nt0+0
  printf "T5.bfs3.classes=%d\nT5.bfs3.nodes_total=%d\nT5.bfs3.edges_total=%d\nT5.bfs3.invalid_candidates=%d\nT5.bfs3.neighbour_card_ne_nE=%d\nT5.bfs3.t0_classes=%d\nT5.bfs3.t0_classes_reaching_Eprime=%d\nT5.bfs3.tne0_classes_reaching_Eprime=%d\n",n3+0,nv3+0,ne3+0,nb3+0,nc3+0,n3t0+0,r3t0+0,r3nt0+0
  # controls
  nct=split("PC1 PC2 PC3 NC1 NC2 NC3",ct," "); for(i=1;i<=nct;i++) printf "control.%s.rows=%d\ncontrol.%s.fails=%d\n",ct[i],cn[ct[i]]+0,ct[i],cf[ct[i]]+0
  printf "control.NC2.no_coprime_partner_rows=%d\n",nc2na+0
  printf "witness.count=%d\n",nwit+0
  for(i=1;i<=nwit&&i<=40;i++) printf "witness.%d=%s\n",i,wit[i]

  # ---- cross-checks between tiers ----
  nx=0; allx=1; nx3=0; all3=1;
  for(k in keys){ np=split(k,pp,".");
    if(pp[1]=="T1" && (pp[2]=="odd"||pp[2]=="binord")){ k2="T2." pp[2] "." pp[3];
      if(k2 in keys){ nx++; ok=(cnt[k]==cnt[k2] && W[k]==W[k2] && ng[k]==ng[k2] && Wg[k]==Wg[k2] && nlit[k]==nlit[k2] && ncop[k]==ncop[k2] && Wev[k]==Wev[k2]);
        if(!ok){allx=0; printf "xcheck.T1vsT2.%s.%s=DIFF\n",pp[2],pp[3]} } }
    if(pp[1]=="T3" && pp[2]=="odd"){ k2="T2.odd." pp[3]; nx3++;
      ok=((k2 in keys) && W[k]==W[k2] && Wg[k]==Wg[k2] && Wev[k]==Wev[k2] && Wlit[k]==Wlit[k2] && Wcop[k]==Wcop[k2] );
      printf "xcheck.T3vsT2.p%s=%s;curves=%d;T3_g2gt1=%d;T2_weighted_g2gt1=%d;T3_nE_even=%d;T2_weighted_nE_even=%d\n",pp[3],(ok?"EQUAL":"DIFF"),W[k],Wg[k],Wg[k2],Wev[k],Wev[k2];
      if(!ok)all3=0 } }
  printf "xcheck.T1vsT2.compared_q=%d\nxcheck.T1vsT2.all_equal=%s\nxcheck.T3vsT2.compared_p=%d\nxcheck.T3vsT2.all_equal=%s\n",nx,(allx?"true":"false"),nx3,(all3?"true":"false")
  # ---- verdicts (computed from the counts above) ----
  v1=(totviol["T1"]+totviol["T2"]+totviol["T2S"]+totviol["T3"]==0 && nat4==0 && ora["T1"]+ora["T2"]+ora["T2S"]+ora["T3"]==0 && m7["T1"]==0);
  mersok=(merscg["T1.7"]>0 && merscg["T1.31"]>0 && merscg["T1.127"]>0 && merscg["T2.7"]>0 && merscg["T2.31"]>0 && merscg["T2.127"]>0);
  offall=(totoff["T1"]+totoff["T2"]+totoff["T2S"]+totoff["T3"]);
  v2=(offall==0 && mersok);
  v3=(binord["T1"]>0 && binord["T2"]>0 && binordbad["T1"]+binordbad["T2"]==0 && binss["T1"]>0 && binss["T2"]>0 && binssbad["T1"]+binssbad["T2"]+binssbad["T2S"]==0);
  v4=(t0odd["T1"]>0 && t0odd["T2"]>0 && t0oddbad["T1"]+t0oddbad["T2"]+t0oddbad["T3"]==0);
  f5=(P5W>0)?P5E/P5W:-1; v5=(f5>=0.62 && f5<=0.72);
  v6=(ftype["T5.HOM_GCD"]+ftype["T5.HOMNEQ"]+ftype["T5.F5_REACH"]+ftype["T5.F5_REACH3"]+ftype["T5.NBRCARD"]+ftype["T5.NBRCARD3"]+ftype["T5.GCDDIV"]+ftype["T5.GCDPRIME"]==0 && nc1+nc3==0 && r1nt0+r3nt0==0 && cf["PC3"]+0==0 && cn["PC3"]>0);
  v7=(t4bad==0 && n4[1]>0);
  ctl=(cf["PC1"]+cf["PC2"]+cf["PC3"]+cf["NC1"]+cf["NC2"]+cf["NC3"]==0 && cn["PC1"]>0 && cn["PC2"]>0 && cn["PC3"]>0 && cn["NC1"]>0 && cn["NC2"]>0 && cn["NC3"]>0);
  printf "verdict.P1_zero_violations=%s\nverdict.P2_literal_fails_iff_g2gt1_and_mersenne_witnesses=%s\nverdict.P3_binary=%s\nverdict.P4_t0_odd=%s\nverdict.P5_in_range_informational=%s\nverdict.P6_hom_isogeny=%s\nverdict.P7_abstract_iso_g2eq1=%s\nverdict.T3_equals_T2=%s\nverdict.T1_equals_T2_overlap=%s\nverdict.controls_all_pass=%s\n",
    (v1?"true":"false"),(v2?"true":"false"),(v3?"true":"false"),(v4?"true":"false"),(v5?"true":"false"),(v6?"true":"false"),(v7?"true":"false"),(all3?"true":"false"),(allx?"true":"false"),(ctl?"true":"false")
  allok=(v1&&v2&&v3&&v4&&v6&&v7&&all3&&allx&&ctl);
  printf "verdict.H_sharp_prediction_confirmed=%s\n",(allok?"true":"false")
  # one-sided 95%% upper bound on violation rate for the SAMPLE tier (0 violations of n): 1-0.05^(1/n)
  ns=cnt["T2S.binss.256"]+cnt["T2S.binss.512"];
  if(ns>0) printf "sample.T2S.n=%d\nsample.T2S.violations=%d\nsample.T2S.upper95_rate_if_zero=%.6g\n",ns,totviol["T2S"]+0,1-exp(log(0.05)/ns)
}
