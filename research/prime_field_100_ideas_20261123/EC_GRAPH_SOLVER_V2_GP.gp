\\ Exact component-partition versus plain elimination on emitted EC S4 rows.
p=1009; r=1013; B=16;
E=ellinit([Mod(1,p),Mod(14,p)]);
raw=readstr("research/prime_field_100_ideas_20261123/S4_MATCHED_V2_STDOUT.txt");
fb=vector(B);Q0=0;Q320=0;R0=List();R320=List();point_checks=0;

load_rows()={
  my(parts,i,x,y,z,blocks,ss,rr,idx,sgn,row,T,Q);
  for(n=1,#raw,
    parts=strsplit(raw[n]," ");
    if(parts[1]=="base",
      i=eval(strsplit(parts[2],"=")[2]);x=eval(strsplit(parts[3],"=")[2]);y=eval(strsplit(parts[4],"=")[2]);
      fb[i]=[Mod(x,p),Mod(y,p)];if(!ellisoncurve(E,fb[i]),error("base off curve"));
    );
    if(parts[1]=="target",
      z=eval(strsplit(parts[2],"=")[2]);y=eval(strsplit(parts[3],"=")[2]);
      Q=[Mod(z,p),Mod(y,p)];if(!ellisoncurve(E,Q),error("target off curve"));
      if(z==0,Q0=Q,if(z==320,Q320=Q,error("target")));
    );
    if(parts[1]=="witness",
      blocks=strsplit(raw[n]," indices=");z=eval(strsplit(blocks[1],"=")[2]);
      ss=strsplit(blocks[2]," signs=");idx=eval(ss[1]);
      rr=strsplit(ss[2]," row=");sgn=eval(rr[1]);row=eval(rr[2]);
      if(#idx!=3 || #sgn!=3 || #row!=B+1,error("row format"));
      T=ellmul(E,fb[idx[1]],sgn[1]);for(k=2,3,T=elladd(E,T,ellmul(E,fb[idx[k]],sgn[k])));
      if(T!=if(z==0,Q0,Q320),error("point witness mismatch"));point_checks++;
      if(z==0,listput(R0,row),if(z==320,listput(R320,row),error("witness target")));
    );
  );
  R0=Vec(R0);R320=Vec(R320);
  if(#R0!=24 || #R320!=33 || point_checks!=57,error("input counts"));
};

elim(rows,cols)={
  my(n=#rows,m=#cols,A,k=0,pivot,t,muladds=0,invs=0,scans=0,updates=0);
  if(n==0 || m==0,return([0,0,0,0,0]));
  A=matrix(n,m,i,j,Mod(rows[i][cols[j]],r));
  for(j=1,m,
    pivot=0;
    for(i=k+1,n,scans++;if(pivot==0 && A[i,j]!=0,pivot=i));
    if(pivot!=0,
      k++;
      if(pivot!=k,for(c=1,m,t=A[k,c];A[k,c]=A[pivot,c];A[pivot,c]=t));
      t=1/A[k,j];invs++;
      for(c=j,m,A[k,c]*=t;muladds++);
      for(i=1,n,if(i!=k && A[i,j]!=0,
        t=A[i,j];updates++;
        for(c=j,m,A[i,c]-=t*A[k,c];muladds++);
      ));
    );
  );
  [k,muladds,invs,scans,updates];
};

partition_solve(rows)={
  my(parent=vector(B,i,i),active=vector(B),first,u,v,root,labels=List(),cols,subrows,
     result,rank=0,muladds=0,invs=0,scans=0,updates=0,edge_visits=0,ncomp=0,
     nactive=0,nisolated=0,assigned=0);
  for(t=1,#rows,
    first=0;
    for(j=1,B,
      edge_visits++;
      if(rows[t][j]!=0,
        active[j]=1;
        if(first==0,first=j,
          u=first;while(parent[u]!=u,u=parent[u]);
          v=j;while(parent[v]!=v,v=parent[v]);
          if(u!=v,parent[v]=u);
        );
      );
    );
  );
  for(j=1,B,if(active[j],nactive++,nisolated++));
  for(j=1,B,
    if(active[j],
      u=j;while(parent[u]!=u,u=parent[u]);
      if(!setsearch(Set(Vec(labels)),u),listput(labels,u));
    );
  );
  ncomp=#labels;
  for(g=1,ncomp,
    root=labels[g];cols=List();subrows=List();
    for(j=1,B,if(active[j],u=j;while(parent[u]!=u,u=parent[u]);if(u==root,listput(cols,j))));
    for(t=1,#rows,
      first=0;for(j=1,B,if(first==0 && rows[t][j]!=0,first=j));
      if(first!=0,u=first;while(parent[u]!=u,u=parent[u]);if(u==root,listput(subrows,rows[t]);assigned++));
    );
    result=elim(Vec(subrows),Vec(cols));
    rank+=result[1];muladds+=result[2];invs+=result[3];scans+=result[4];updates+=result[5];
  );
  if(assigned!=#rows,error("partition lost rows"));
  [nactive,ncomp,rank,muladds,invs,scans,updates,edge_visits];
};

run_one(z,rows)={
  my(cols=vector(B,i,i),fullcols=vector(B+1,i,i),active=List(),activeplain,plain,full,block,seen);
  for(j=1,B,seen=0;for(t=1,#rows,if(rows[t][j]!=0,seen=1));if(seen,listput(active,j)));
  plain=elim(rows,cols);full=elim(rows,fullcols);block=partition_solve(rows);
  activeplain=elim(rows,Vec(active));
  if(plain[1]!=block[3] || plain[1]!=full[1] || plain[1]!=activeplain[1] || #active!=block[1],error("rank or target-column disagreement"));
  print("z=",z," rows=",#rows," active=",block[1]," components=",block[2]," base_rank=",plain[1]," full_rank=",full[1]," target_column_in_base_span=1");
  print("plain muladds=",plain[2]," inversions=",plain[3]," pivot_scans=",plain[4]," row_updates=",plain[5]);
  print("active_plain muladds=",activeplain[2]," inversions=",activeplain[3]," pivot_scans=",activeplain[4]," row_updates=",activeplain[5]," active_scan_cells=",B*#rows);
  print("partition muladds=",block[4]," inversions=",block[5]," pivot_scans=",block[6]," row_updates=",block[7]," graph_cell_scans=",block[8]);
  print("full_matrix muladds=",full[2]," inversions=",full[3]," pivot_scans=",full[4]," row_updates=",full[5]);
};

load_rows();
run_one(0,R0);
run_one(320,R320);
print("point_witness_checks=",point_checks," complete_graph_solver_control");
