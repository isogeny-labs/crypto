\\ Replay the saved PARI ECPP certificate and reject a changed subject.
{
  my(c=read("prime263.cert"),n=9021184135396238924389891945192676814518993645089807769277009849103733654828039,good,bad);
  if(c[1][1]!=n,error("certificate subject mismatch"));
  good=primecertisvalid(c);
  if(good!=1,error("certificate rejected"));
  c[1][1]=n+2;
  bad=iferr(primecertisvalid(c),e,0);
  if(bad!=0,error("changed certificate accepted"));
  print("{\"certificate_subject_matches\":true,\"pari_certificate_accept\":true,\"changed_subject_reject\":true,\"verifier\":\"PARI/GP 2.17.2 primecertisvalid\",\"scope\":\"prime only; no isogeny verification\"}");
}
quit
