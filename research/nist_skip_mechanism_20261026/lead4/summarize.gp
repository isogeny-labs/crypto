read("lib.gp");
print("# per position table: stream index label69 branches n_bits p_bits cofactor published");
KK=vecsort(vector(NP,k,[PV[k][1],PV[k][2]]));
{for(j=1,NP,my(x=mapget(POS,KK[j]));print(STREAMS[x[1]],"\t",x[2],"\t",x[3],"\t",strjoin(Vec(x[4]),"+"),"\t",x[6],"\t",x[7],"\t",x[8],"\t",x[5]))};
print("# per stream: L(prefix length), published, |E1..E5|");
{for(s=1,6,print(STREAMS[s]," L=",SL[s]," pub=",PUB[s]," |E|=",vector(5,e,#ELIG[e][s])))};
print("# totals |E|=",vector(5,e,sum(s=1,6,#ELIG[e][s]))," published inside each E=",vector(5,e,sum(s=1,6,sum(j=1,#ELIG[e][s],isPub(s,ELIG[e][s][j])))));
print("# checks: L sum=",vecsum(SL)," (expect 21753); NP=",NP);
