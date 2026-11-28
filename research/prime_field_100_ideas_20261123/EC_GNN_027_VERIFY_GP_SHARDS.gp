\\ Independent GP scalar replay of the frozen model-selected candidate IDs.
p=1019;r=1033;B=16;H=8;N=B^3*8;K=N/4;
E=ellinit([Mod(1,p),Mod(20,p)]);G=[Mod(0,p),Mod(64,p)];
if(ellcard(E)!=r || ellorder(E,G)!=r,error("qualifier curve guard"));
smallest_point(xx)={my(v=Mod(xx^3+xx+20,p));for(yy=0,p-1,if(Mod(yy^2,p)==v,return([Mod(xx,p),Mod(yy,p)])));error("missing point")};
build_points()={my(n=0,xx=1,q);fb=vector(B);targets=vector(H);targetx=vector(H);
  while(n<B+H && xx<p,
    q=Mod(xx^3+xx+20,p);
    if(issquare(q),n++;if(n<=B,fb[n]=smallest_point(xx),targets[n-B]=smallest_point(xx);targetx[n-B]=xx));xx++;
  );
  if(n!=B+H || targetx!=[38,39,41,46,49,53,54,56],error("qualifier point rule"));
};
pointlog(P)={for(k=1,r-1,if(ellmul(E,G,k)==P,return(k)));error("qualifier point log")};
build_points();flog=vector(B,i,pointlog(fb[i]));tlog=vector(H,i,pointlog(targets[i]));
decode(id)={my(mask=id%8,t=id\8);[1+t\256,1+(t\16)%16,1+t%16,mask]};
row_for(d,u)={my(v=vector(B+H,h,0),m=d[4]);v[d[1]]+=if(m%2,-1,1);v[d[2]]+=if((m\2)%2,-1,1);v[d[3]]+=if((m\4)%2,-1,1);v[B+u]=-1;v};
is_hit(d,u)=lift(Mod(if(d[4]%2,-flog[d[1]],flog[d[1]])+if((d[4]\2)%2,-flog[d[2]],flog[d[2]])+if((d[4]\4)%2,-flog[d[3]],flog[d[3]])-tlog[u],r))==0;
rank_rows(rows)={my(V=Vec(rows),M);if(#V==0,return(0));M=matrix(#V,B+H,ii,jj,Mod(V[ii][jj],r));matrank(M)};
replay()={my(lines=readstr("/tmp/ec_gnn_027_verify_emit.reconstructed.stdout.txt"),ids=vector(H),seenLines=0,parts,prefix,u,V,seen,rows=List(),fullRows=List(),hits=0,fullHits=0,covered=0,fullCovered=0,thisHits,fullThis,d);
  for(h=1,#lines,
    parts=strsplit(lines[h],"=");
    prefix=strsplit(parts[1],"model_ids_u");
    if(#parts>=2 && #prefix==2 && prefix[1]=="",
      u=eval(prefix[2]);if(u<1 || u>H || type(ids[u])!="t_INT",error("manifest target index"));
      ids[u]=eval(parts[2]);seenLines++;
    );
  );
  if(seenLines!=H,error("manifest line count"));
  for(u=1,H,
    V=ids[u];if(#V!=K,error("manifest topK length"));seen=vector(N,i,0);thisHits=0;fullThis=0;
    for(t=1,K,
      if(V[t]<0 || V[t]>=N || seen[V[t]+1]!=0,error("manifest duplicate/range"));
      seen[V[t]+1]=1;d=decode(V[t]);
      if(is_hit(d,u),hits++;thisHits++;listput(~rows,row_for(d,u)));
    );
    for(id=0,N-1,d=decode(id);if(is_hit(d,u),fullHits++;fullThis++;listput(~fullRows,row_for(d,u))));
    if(thisHits>0,covered++);if(fullThis>0,fullCovered++);
    print("verify_target x=",targetx[u]," selected_hits=",thisHits," full_hits=",fullThis);
  );
  if(hits!=51 || covered!=4 || rank_rows(rows)!=9 || fullHits!=223 || fullCovered!=7 || rank_rows(fullRows)!=22,error("qualifier count/rank mismatch"));
  print("verify_summary selected_ids=",H*K," selected_hits=",hits," selected_coverage=",covered," selected_rank=",rank_rows(rows),
        " full_hits=",fullHits," full_coverage=",fullCovered," full_rank=",rank_rows(fullRows),
        " false_negative_hits=",fullHits-hits," rank_loss=",rank_rows(fullRows)-rank_rows(rows)," point_logs_anchored=",B+H);
};
replay();print("complete independent GNN selected-ID scalar replay");
