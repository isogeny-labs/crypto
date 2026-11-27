\\ Exact four-bit energy diagnostic for proposals 43, 44 and 48.
\\ Two frozen public EC triples, eight sign patterns each, target Q=(0,425).
p=1009;
E=ellinit([Mod(1,p),Mod(14,p)]);
Q=[Mod(0,p),Mod(425,p)];
F3=[Mod(4,p),Mod(456,p)]; F5=[Mod(10,p),Mod(32,p)]; F12=[Mod(29,p),Mod(35,p)];
F4=[Mod(5,p),Mod(12,p)]; F11=[Mod(25,p),Mod(23,p)];
T1=[F3,F5,F12]; T2=[F4,F5,F11];

signsum(T,mask)={
  my(s1=if(mask%2,-1,1),s2=if((mask\2)%2,-1,1),s3=if((mask\4)%2,-1,1));
  elladd(E,elladd(E,ellmul(E,T[1],s1),ellmul(E,T[2],s2)),ellmul(E,T[3],s3));
};

run()={
  my(energy=vector(16),coeff,marked=List(),marked_count=0,maxdeg=0,highterms=0,degree,neighbor,flat=0,adjacent_marked);
  if(!ellisoncurve(E,Q),error("target off curve"));
  for(j=1,3,if(!ellisoncurve(E,T1[j]) || !ellisoncurve(E,T2[j]),error("base off curve")));
  for(mask=0,15,
    energy[mask+1]=if(signsum(if(mask<8,T1,T2),mask%8)==Q,0,1);
    if(energy[mask+1]==0,marked_count++;listput(marked,mask));
  );
  if(marked_count!=2,error("changed EC mark set"));
  coeff=vector(16,i,energy[i]);
  \\ Boolean-lattice Mobius inversion: unique multilinear polynomial.
  for(bit=0,3,for(mask=0,15,
    if((mask\2^bit)%2,coeff[mask+1]-=coeff[mask-2^bit+1]);
  ));
  for(mask=0,15,
    degree=0;for(bit=0,3,degree+=(mask\2^bit)%2);
    if(coeff[mask+1]!=0,
      if(degree>maxdeg,maxdeg=degree);
      if(degree>2,highterms++);
    );
    if(energy[mask+1]!=0,
      adjacent_marked=0;
      for(bit=0,3,
        neighbor=if((mask\2^bit)%2,mask-2^bit,mask+2^bit);
        if(energy[neighbor+1]==0,adjacent_marked=1);
      );
      if(!adjacent_marked,flat++);
    );
  );
  print("ec_candidates=16 marked=",marked_count," marked_masks=",Vec(marked));
  print("exact_indicator_multilinear_degree=",maxdeg," nonzero_terms_above_quadratic=",highterms);
  print("nonmatching_masks_without_better_one_bit_neighbor=",flat);
  print("complete_sign_energy_control");
};
run();
