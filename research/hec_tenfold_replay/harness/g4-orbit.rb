require 'open3'
mode = ARGV.fetch(0)
raise 'expected packed or order' unless %w[packed order].include?(mode)
source, status = Open3.capture2('ruby', 'harness/g4-hermite-solve.rb', 'kernel', 'carry')
raise unless status.success?
header, rest = source.split('namespace screen {', 2)
packed = 'namespace hermite {' + rest.split('namespace hermite {', 2).last.split('static int coefficient,ell,mode;', 2).first
def replace(text, from, to)
  raise "expected one instance of #{from}" unless text.scan(from).size == 1
  text.sub(from) { to }
end
orbit = replace(packed, 'const int n=poles.size();', <<'CPP')
const int n=poles.size();
int width=0;
std::array<int,P> orbit{},factor{};
orbit.fill(-1);
for(const auto &q:poles){
  if(q.x==0 || orbit[q.x]>=0)continue;
  int x=q.x, weight=1;
  for(int i=0;i<3;i++){
    orbit[x]=width;
    factor[x]=weight;
    x=x*10%37;
    weight=weight*20130%30109;
  }
  ++width;
}
assert(width==11);
CPP
orbit = replace(orbit, 'std::copy_n(row.begin(),n,work.begin());',
  'for(const auto &q:poles)if(q.x)work[orbit[q.x]]+=1LL*row[q.col]*factor[q.x];')
orbit = replace(orbit, 'for(int j=n-1;j>=0;j--){int y=', 'for(int j=width-1;j>=0;j--){int y=')
orbit = replace(orbit, 'if(!audit && rank==n-1){', 'if(!audit && rank==width-1){')
orbit = replace(orbit, "for(int j=0;j<n;j++){\n    if(!piv", "for(int j=0;j<width;j++){\n    if(!piv")
orbit = replace(orbit, 'values[q.x]=kernel[q.col];', 'values[q.x]=q.x?1LL*factor[q.x]*kernel[orbit[q.x]]%L:0;')
control = mode == 'order' ? orbit : packed
candidate = mode == 'order' ? replace(orbit, 'int id=step;', 'int id=(step+3)%n;') : orbit
puts header, control.sub('namespace hermite {', 'namespace screen {'), candidate
puts <<'CPP'
static int mode;
extern "C" void select_mode(int i){mode=i;}
extern "C" int select_curve(int p,int c){assert(p==37 && c==0);return 3;}
extern "C" int quartic(int p,int*y,int*v,int,int*){
  assert(p==37);
  return (mode==0?screen::solve<37>:hermite::solve<37>)(0,30109,y,v);
}
CPP
