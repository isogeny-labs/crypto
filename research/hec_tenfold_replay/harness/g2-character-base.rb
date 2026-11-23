s=File.read('kernel.cpp')
def change(s,a,b)
  raise "source mismatch" unless s.scan(a).size==1
  s.sub(a){b}
end
s=change(s,'int fibmode=0;extern "C" void sequence(int on){fibmode=on;}', 'int fibmode=0,character_mode=0;extern "C" void sequence(int on){character_mode=on;}')
s=change(s,' triples=minusarm=reorder=freqmode=fast;',' fibmode=p==127?1:2;triples=minusarm=reorder=freqmode=fast;')
s=change(s,'std::vector<Point> pts,poles;','int original_places=0; std::vector<Point> pts,poles;')
key='if constexpr(Small){if(y){poles.push_back'
s=change(s,key,<<'CPP')
if constexpr(Small){if(y){
  if constexpr(P==127 || P==257){
    constexpr int cap=(P+4)/5;
    if(character_mode==2 && original_places==cap)break;
    ++original_places;
    int g=P==127?m(x-81):m(x*x+222*x+97);
    if(character_mode && root[g]<0)continue;
  }
  poles.push_back
CPP
if ARGV.include?('crossover')
  s=change(s,'P==127 || P==257){','P==127 || P==257 || P==4093){')
  s=change(s,'cap=(P+4)/5','cap=P==4093?(P+9)/10:(P+4)/5')
  s=change(s,'m(x-81):m(x*x+222*x+97)','m(x-81):P==257?m(x*x+222*x+97):m(x-268)')
end
puts s
