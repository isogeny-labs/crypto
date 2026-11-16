default(parisizemax,2000000000);
\\ EXPLORATORY totals over all recorded CSV rows (written after the main run). Reads out/stage1_m*.csv, out/stage2_m*.csv.
rd(f) = { my(L=readstr(f),names=[],rows=List(),s,v);
  for(k=1,#L, s=L[k]; if(#s==0,next); v=Vec(s);
    if(v[1]=="#", if(#s>10 && concat(vector(10,j,v[j]))=="# fields: " && #names==0, names=strsplit(concat(vector(#v-10,j,v[j+10])),",")); next);
    listput(rows,eval(Str("[",s,"]"))));
  [names,Vec(rows)] }
cix(names,nm) = { for(q=1,#names,if(names[q]==nm,return(q))); error(nm) }
med(v) = my(w=vecsort(v),n=#w); if(n%2,w[(n+1)/2],(w[n/2]+w[n/2+1])/2);
{
my(T1=Map(),T2=Map(),n1=0,n2=0,tot=vector(40),nm1,nm2,cr=List(),wr=List(),wrp=List(),crp=List());
my(f1=["m","l","pop","k","seed","N0V","N1V","np_same","np_cross","rat_same","rat_cross","viol","triples","triples_formula","f4_ok","rootfail","x3zero","pc2_rat","pc2_diff"]);
my(S1=vector(#f1),S2n=["N0V","N1V","nT0","nT1","E_solves0","S_solves0","Esh_solves0","E_rel0","S_rel0","S_k0pure0","S_cross0","S_k1pure0","mism","mism_sh","E_solves1","S_solves1","E_rel1","S_rel1","S_k1pure1","S_cross1","S_k0pure1","cls_S","cls_S_t1","cls_S_used","cls_S_t1_used","cls_E","cls_E_used"],S2=vector(27),nrows1=0,nrows2=0);
for(i=1,4, my(m=[11,13,17,19][i],a=rd(Str("out/stage1_m",m,".csv")),b=rd(Str("out/stage2_m",m,".csv")));
  for(r=1,#a[2], nrows1++; for(j=1,#f1, S1[j]+=a[2][r][cix(a[1],f1[j])]));
  for(r=1,#b[2], nrows2++; my(row=b[2][r]);
    for(j=1,#S2n, S2[j]+=row[cix(b[1],S2n[j])]);
    listput(cr,row[cix(b[1],"S_solves0")]/row[cix(b[1],"E_solves0")]);
    my(rr=List()); for(rep=0,2, my(tE=row[cix(b[1],"tE1")+4*rep],tS=row[cix(b[1],"tS1")+4*rep]); if(tE>0,listput(rr,tS/tE)));
    if(#rr>0, listput(wr,med(Vec(rr))))));
print("TOTAL stage1 V-draw rows=",nrows1," stage2 V-draw rows=",nrows2);
for(j=6,#f1, print("TOTAL stage1 ",f1[j],"=",S1[j]));
for(j=1,#S2n, print("TOTAL stage2 ",S2n[j],"=",S2[j]));
print("TOTAL all-draw median counted-solve ratio S/E=",Strprintf("%.4f",med(Vec(cr)))," n=",#cr);
print("TOTAL all-draw median CPU-time ratio S/E=",Strprintf("%.4f",med(Vec(wr)))," n=",#wr);
}
quit
