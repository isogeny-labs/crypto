\\ Exact local controls for 42, 55, 61, 62 on public E(F_1009) rows.
\\ Reads producer rows but does not use the producer's relation checker.
p=1009; r=1013; B=16;
E=ellinit([Mod(1,p),Mod(14,p)]);
raw=readstr("research/prime_field_100_ideas_20261123/S4_MATCHED_V2_STDOUT.txt");
fb=vector(B); Q0=0; Q320=0; count0=0; count320=0; accepted=0; tampered_rejected=0;

eval_relation(row,Q)={
  my(T=ellmul(E,Q,row[B+1]));
  for(j=1,B,if(row[j]!=0,T=elladd(E,T,ellmul(E,fb[j],row[j]))));
  T;
};

verify_stream()={
  my(parts,i,x,y,z,blocks,ss,rr,row,changed,first);
  for(n=1,#raw,
    parts=strsplit(raw[n]," ");
    if(parts[1]=="base",
      i=eval(strsplit(parts[2],"=")[2]);
      x=eval(strsplit(parts[3],"=")[2]);
      y=eval(strsplit(parts[4],"=")[2]);
      fb[i]=[Mod(x,p),Mod(y,p)];
      if(!ellisoncurve(E,fb[i]),error("base point off curve"));
    );
    if(parts[1]=="target",
      z=eval(strsplit(parts[2],"=")[2]);
      y=eval(strsplit(parts[3],"=")[2]);
      if(z==0,Q0=[Mod(z,p),Mod(y,p)],if(z==320,Q320=[Mod(z,p),Mod(y,p)],error("target")));
    );
    if(parts[1]=="witness",
      blocks=strsplit(raw[n]," indices=");
      z=eval(strsplit(blocks[1],"=")[2]);
      ss=strsplit(blocks[2]," signs=");
      rr=strsplit(ss[2]," row="); row=eval(rr[2]);
      if(#row!=B+1 || row[B+1]!=-1,error("row format"));
      if(z==0,count0++;if(Q0==0,error("target absent")),if(z==320,count320++;if(Q320==0,error("target absent")),error("witness target")));
      if(eval_relation(row,if(z==0,Q0,Q320))!=[0],error("valid row rejected"));
      accepted++;
      changed=row;
      first=0;for(j=1,B,if(first==0 && row[j]!=0,first=j));
      if(first==0,error("empty factor-base row"));
      changed[first]++;
      if(eval_relation(changed,if(z==0,Q0,Q320))==[0],error("tampered row accepted"));
      tampered_rejected++;
    );
  );
  if(count0!=24 || count320!=33 || accepted!=57 || tampered_rejected!=57,error("row count"));
};
verify_stream();

R=matrix(1,1,i,j,r); Rmod=matrix(1,1,i,j,Mod(r,r));
rankQ=matrank(R);rankFr=matrank(Rmod);
if(rankQ!=1 || rankFr!=0,error("real/modular rank guard"));
half_mod=lift(Mod(1,r)/Mod(2,r));
if(half_mod!=507,error("rational reduction guard"));

int32max=2^31-1; dotcap=floor(int32max/(255^2));
if(dotcap!=33025 || dotcap*255^2>int32max || (dotcap+1)*255^2<=int32max,error("accumulator bound"));

print("rank_control rational_rank_[1013]=",rankQ," modular_rank_F1013_[1013]=",rankFr," rational_half_reduced_mod_1013=",half_mod);
print("int8_unsigned_dot_signed32_cap=",dotcap," last_safe_sum=",dotcap*255^2," first_overflow_sum=",(dotcap+1)*255^2);
print("public_ec_rows z0=",count0," z320=",count320," accepted=",accepted," one_coefficient_tampered_rejected=",tampered_rejected);
print("complete_guarded_controls");
