/^BEGIN degree=/ {
  split($0,a,"; polynomial="); split(a[1],b,"=");
  print "F=" a[2] "; degree=" b[2] ";";
}
/^generator=/ {
  line=$0; sub(/^generator=/,"",line); print "g=Mod(" line ",F);";
  print "checkwitness(degree,F,g);";
}
