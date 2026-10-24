\\ Replay explicit witnesses without a logarithm search or class-number oracle.
verify_witnesses(relations) = {
  my(d=-6649310858224389603,p=34081013,n=68162026,I0=Qfb(1,1,1662327714556097401),
    C,A,G,T,seen=Map(),z,right);
  C=[Qfb(3,3,554109238185365801),Qfb(7,5,237475387793728201),
    Qfb(11,7,151120701323281583),Qfb(13,5,127871362658161339),
    Qfb(19,9,87490932345057759),Qfb(23,17,72275118024178151),
    Qfb(53,27,31364673859549011),Qfb(61,23,27251274009116353)];
  A=C[2]; G=qfbpow(A,2);
  if(!isprime(p) || qfbpow(A,n)!=I0 || qfbpow(A,p)==I0 || G==I0 || qfbpow(G,p)!=I0,return(0));
  T=[C[1],qfbpow(A,p),qfbpow(C[3],p),qfbpow(C[8],p)];
  for(j=1,4,if(qfbpow(T[j],2)!=I0,return(0)));
  for(mask=0,15,
    z=I0;for(j=1,4,if(bittest(mask,j-1),z=qfbcomp(z,T[j])));
    if(mapisdefined(seen,Str(z)),return(0));mapput(seen,Str(z),1)
  );
  for(i=1,8,
    if(relations[i][1]<0 || relations[i][1]>=p || relations[i][2]<0 || relations[i][2]>=16,return(0));
    right=qfbpow(G,relations[i][1]);
    for(j=1,4,if(bittest(relations[i][2],j-1),right=qfbcomp(right,T[j])));
    if(right!=C[i],return(0))
  );
  return(1)
};
check_all() = {
  my(relations=[[0,1],[17040507,2],[24833214,4],[17619985,2],
    [31141112,7],[734461,3],[2966921,4],[22645521,8]],bad);
  if(!verify_witnesses(relations),error("explicit relation witness rejected"));
  bad=relations;bad[4][1]++;
  if(verify_witnesses(bad),error("corrupted exponent accepted"));
  bad=relations;bad[5][2]=bitxor(bad[5][2],1);
  if(verify_witnesses(bad),error("corrupted torsion mask accepted"));
  print("{\"schema\":\"prime61.class-relation-witness-check/v1\",\"result\":\"PASS\",\"relations_checked\":8,\"torsion_products\":16,\"corruption_controls_rejected\":2,\"logarithm_search\":false,\"arithmetic_backend\":\"same PARI library; separate witness checker\",\"whole_class_number_certified\":false}")
};
iferr(check_all(),failure,print(Str(failure));quit(1));
quit(0);
