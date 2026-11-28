\\ Recompute the exact subject before reading the saved certificate.
{
  my(n=9551,u0=0,u1=1,u,c,cert,bad);
  for(k=2,n,u=-u1-2*u0;u0=u1;u1=u);
  c=abs(u1); cert=read("conductor.cert");
  if(type(cert)!="t_VEC" || cert[1][1]!=c,error("wrong certificate subject"));
  if(!primecertisvalid(cert),error("certificate replay rejected"));
  cert[1][1]=c+1;
  bad=iferr(primecertisvalid(cert),e,0);
  if(bad,error("changed subject accepted"));
  print("{\"subject_matches\":true,\"certificate_replay_accept\":true,\"changed_subject_reject\":true,\"scope\":\"prime only; no isogeny evaluation\"}");
}
quit
