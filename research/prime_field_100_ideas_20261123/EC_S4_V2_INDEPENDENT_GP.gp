\\ Independent readback of S4_MATCHED_V2_STDOUT.txt, using only emitted rows.
\\ This source does not construct S4, enumerate triples, or alter producer files.
\\ It recomputes rank by separate modular Gaussian elimination and checks
\\ every emitted signed witness in E(F_1009), including one for each target.
p=1009; r=1013; B=16;
E=ellinit([Mod(1,p),Mod(14,p)]);
lines=readstr("research/prime_field_100_ideas_20261123/S4_MATCHED_V2_STDOUT.txt");
fb=vector(B); seen=vector(B); base_count=0;
Q0=0; Q320=0; rows0=List(); rows320=List(); count0=0; count320=0;
first0=0; first320=0;

rank_from_rows(R)={
  my(n=#R,m=B+1,k=0,A,pivot,t);
  A=matrix(n,m,i,j,Mod(R[i][j],r));
  for(j=1,m,
    pivot=0;
    for(i=k+1,n,if(pivot==0 && A[i,j]!=0,pivot=i));
    if(pivot!=0,
      k++;
      if(pivot!=k,for(c=1,m,t=A[k,c];A[k,c]=A[pivot,c];A[pivot,c]=t));
      t=1/A[k,j]; for(c=j,m,A[k,c]*=t);
      for(i=1,n,if(i!=k && A[i,j]!=0,
        t=A[i,j]; for(c=j,m,A[i,c]-=t*A[k,c]);
      ));
    );
  );
  k;
};

parse_and_verify()={
  my(parts,i,x,y,z,blocks,ss,rr,idx,sgn,row,expected,T,Q);
  for(line_i=1,#lines,
    parts=strsplit(lines[line_i]," ");
    if(parts[1]=="base",
      i=eval(strsplit(parts[2],"=")[2]);
      x=eval(strsplit(parts[3],"=")[2]);
      y=eval(strsplit(parts[4],"=")[2]);
      if(i<1 || i>B || seen[i],error("duplicate/invalid base index"));
      fb[i]=[Mod(x,p),Mod(y,p)];
      if(!ellisoncurve(E,fb[i]),error("emitted base point off curve"));
      seen[i]=1; base_count++;
    );
    if(parts[1]=="target",
      z=eval(strsplit(parts[2],"=")[2]);
      y=eval(strsplit(parts[3],"=")[2]);
      Q=[Mod(z,p),Mod(y,p)];
      if(!ellisoncurve(E,Q),error("emitted target off curve"));
      if(z==0,Q0=Q,if(z==320,Q320=Q,error("unexpected target")));
    );
    if(parts[1]=="witness",
      if(base_count!=B,error("witness before full base"));
      blocks=strsplit(lines[line_i]," indices=");
      z=eval(strsplit(blocks[1],"=")[2]);
      ss=strsplit(blocks[2]," signs="); idx=eval(ss[1]);
      rr=strsplit(ss[2]," row="); sgn=eval(rr[1]); row=eval(rr[2]);
      if(#idx!=3 || #sgn!=3 || #row!=B+1,error("malformed witness"));
      expected=vector(B+1,h,0);
      for(k=1,3,
        if(idx[k]<1 || idx[k]>B || abs(sgn[k])!=1,error("bad witness term"));
        expected[idx[k]]+=sgn[k];
      );
      expected[B+1]=-1;
      if(expected!=row,error("emitted coefficient row mismatch"));
      Q=if(z==0,Q0,if(z==320,Q320,error("unexpected witness target")));
      T=ellmul(E,fb[idx[1]],sgn[1]);
      for(k=2,3,T=elladd(E,T,ellmul(E,fb[idx[k]],sgn[k])));
      if(T!=Q,error("signed EC witness mismatch"));
      if(z==0,
        listput(rows0,row); count0++; if(first0==0,first0=[idx,sgn]),
        listput(rows320,row); count320++; if(first320==0,first320=[idx,sgn]);
      );
    );
  );
  if(base_count!=B || count0!=24 || count320!=33,error("emitted count mismatch"));
  if(Q0==0 || Q320==0,error("missing target"));
};
parse_and_verify();

rank0=rank_from_rows(Vec(rows0));
rank320=rank_from_rows(Vec(rows320));
if(rank0!=4 || rank320!=6,error("modular rank disagreement"));
print("source_lines=",#lines," base_points=",base_count);
print("z=0 rows=",count0," rank_mod_1013=",rank0," first_signed_witness=",first0);
print("z=320 rows=",count320," rank_mod_1013=",rank320," first_signed_witness=",first320);
print("all_emitted_rows_and_signed_group_witnesses_checked=1");
