#include <algorithm>
#include <array>
#include <cassert>
#include <cctype>
#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

// Exact bounded arithmetic/GB checker for the preregistered source fixtures.
// No timing, genericity, C-refinement or internal-F4-degree proof is claimed.
using Mon=std::array<int,3>;
using Poly=std::map<Mon,int>;
int prime=5,variables=2,referencePeak=0;
void require(bool b,const std::string& why) { if(!b) throw std::runtime_error(why); }
int mod(long long x) { return int((x%prime+prime)%prime); }
int inverse(int a) { require(a!=0,"inverse zero"); for(int b=1;b<prime;b++) if(mod(a*b)==1)return b;
  throw std::runtime_error("noninvertible"); }
std::string read(const std::string& p) { std::ifstream f(p);require(bool(f),"missing "+p);
  return {std::istreambuf_iterator<char>(f),{}}; }
std::string compact(std::string s) { s.erase(std::remove_if(s.begin(),s.end(),
  [](unsigned char c){return std::isspace(c);}),s.end());return s; }
int degree(Mon a) { return a[0]+a[1]+a[2]; }
bool lower(Mon a,Mon b) { if(degree(a)!=degree(b))return degree(a)<degree(b);
  for(int i=variables-1;i>=0;i--){if(a[i]!=b[i])return a[i]>b[i];}
  return false; }
void term(Poly& f,Mon m,int c) { int x=mod(f[m]+c);if(x)f[m]=x;else f.erase(m); }
Mon lead(const Poly& f) { require(!f.empty(),"zero leading term");Mon a=f.begin()->first;
  for(auto [m,c]:f){if(lower(a,m))a=m;}
  return a; }
int degree(const Poly& f) { return f.empty()?-1:degree(lead(f)); }
bool divides(Mon a,Mon b) { for(int i=0;i<3;i++)if(a[i]>b[i])return false;return true; }
Poly add(Poly a,const Poly& b,int sign=1) { for(auto [m,c]:b)term(a,m,sign*c);return a; }
Poly times(const Poly& f,Mon shift,int c=1) { Poly r;for(auto [key,a]:f){Mon m=key;
  for(int i=0;i<3;i++){m[i]+=shift[i];}
  term(r,m,mod(a*c));}return r; }
Poly monic(const Poly& f) { return times(f,{0,0,0},inverse(f.at(lead(f)))); }
Poly remainder(Poly f,const std::vector<Poly>& basis) { Poly r;int steps=0;
  while(!f.empty()) { require(++steps<=100000,"reduction cap");auto m=lead(f);int c=f.at(m);bool found=false;
    for(const auto& g:basis){auto n=lead(g);if(!divides(n,m))continue;
      Mon shift{};for(int i=0;i<3;i++)shift[i]=m[i]-n[i];
      f=add(f,times(g,shift,mod(c*inverse(g.at(n)))),-1);found=true;break;}
    if(!found){term(r,m,c);f.erase(m);}}
  return r; }
Poly spair(const Poly& f,const Poly& g) { auto a=lead(f),b=lead(g);Mon x{},y{};
  for(int i=0;i<3;i++){int l=std::max(a[i],b[i]);x[i]=l-a[i];y[i]=l-b[i];}
  return add(times(f,x,inverse(f.at(a))),times(g,y,inverse(g.at(b))),-1); }
std::vector<Poly> canonical(std::vector<Poly> gs) { std::vector<Poly> small,out;
  for(size_t i=0;i<gs.size();i++){bool redundant=false;auto a=lead(gs[i]);
    for(size_t j=0;j<gs.size();j++)if(i!=j&&divides(lead(gs[j]),a)&&
      (lead(gs[j])!=a||j<i))redundant=true;
    if(!redundant)small.push_back(monic(gs[i]));}
  for(size_t i=0;i<small.size();i++){std::vector<Poly> others;
    for(size_t j=0;j<small.size();j++)if(j!=i)others.push_back(small[j]);
    auto r=remainder(small[i],others);require(!r.empty(),"unexpected zero reduced generator");out.push_back(monic(r));}
  std::sort(out.begin(),out.end());return out; }
std::vector<Poly> buchberger(std::vector<Poly> input) { std::vector<Poly> gs;referencePeak=0;
  for(auto& f:input)if(!f.empty()){referencePeak=std::max(referencePeak,degree(f));auto r=remainder(f,gs);
    if(!r.empty())gs.push_back(monic(r));}
  int pairs=0;
  for(size_t i=0;i<gs.size();i++)for(size_t j=0;j<i;j++){
    require(++pairs<=4096&&gs.size()<=128,"Buchberger cap");auto s=spair(gs[i],gs[j]);
    referencePeak=std::max(referencePeak,degree(s));auto r=remainder(s,gs);
    if(!r.empty()){referencePeak=std::max(referencePeak,degree(r));gs.push_back(monic(r));}}
  return canonical(gs); }
bool allSpairsZero(const std::vector<Poly>& gs) {
  for(size_t i=0;i<gs.size();i++){for(size_t j=0;j<i;j++){
    if(!remainder(spair(gs[i],gs[j]),gs).empty())return false;}}
  return true; }
Poly parse(std::string s) { s=compact(s);Poly f;size_t at=0;
  while(at<s.size()){int sign=1;if(s[at]=='+'||s[at]=='-'){if(s[at]=='-')sign=-1;at++;}
    size_t end=at;while(end<s.size()&&s[end]!='+'&&s[end]!='-')end++;require(end>at,"empty term");
    std::stringstream t(s.substr(at,end-at));std::string factor;int c=sign;Mon m{};
    while(std::getline(t,factor,'*')){require(!factor.empty(),"empty factor");
      auto pos=std::string("xyh").find(factor[0]);
      if(pos!=std::string::npos){require(pos<size_t(variables),"unexpected variable");int n=1;
        if(factor.size()>1){require(factor[1]=='^'&&factor.size()>2,"bad exponent");
          auto tail=factor.substr(2);require(tail.find_first_not_of("0123456789")==std::string::npos,"bad exponent digits");n=std::stoi(tail);}
        require(n<=16,"exponent cap");m[pos]+=n;
      }else{require(factor.find_first_not_of("0123456789")==std::string::npos,"unsupported factor");c=mod(c*std::stoll(factor));}}
    term(f,m,c);at=end;}
  return f; }
std::vector<Poly> parseList(const std::string& s) { std::stringstream f(s);std::string t;std::vector<Poly> r;
  while(std::getline(f,t,',')){require(!compact(t).empty(),"empty polynomial");r.push_back(parse(t));}return r; }
struct Input { std::vector<Poly> fs;int p,n; };
Input input(const std::string& file) { std::stringstream f(read(file));std::string v,p,s,line;
  std::getline(f,v);std::getline(f,p);v=compact(v);require(v=="x,y"||v=="x,y,h","input variable order");
  variables=v=="x,y"?2:3;prime=std::stoi(p);require(prime==5||prime==101,"input characteristic");
  while(std::getline(f,line)){s+=line;}
  return {parseList(s),prime,variables}; }
std::vector<Poly> solverBasis(const std::string& file) { std::stringstream f(read(file));std::string line,s;
  bool correctOrder=false,correctPrime=false,grevlex=false;
  while(std::getline(f,line)){
    if(line.rfind("#variable order:",0)==0)correctOrder=compact(line.substr(line.find(':')+1))==(variables==2?"x,y":"x,y,h");
    if(line.rfind("#field characteristic:",0)==0)correctPrime=std::stoi(line.substr(line.find(':')+1))==prime;
    if(line.rfind("#monomial order:",0)==0)grevlex=line.find("graded reverse lexicographical")!=std::string::npos;
    if(!line.empty()&&line[0]!='#')s+=line;}
  s=compact(s);require(correctOrder&&correctPrime&&grevlex,"GB ring/order header");
  require(s.size()>=3&&s.front()=='['&&s.substr(s.size()-2)=="]:","GB output envelope");
  auto out=parseList(s.substr(1,s.size()-3));for(const auto& g:out)require(!g.empty(),"zero output basis element");return out; }
int krullDimension(const std::vector<Poly>& gs) { int best=-1;
  for(int mask=0;mask<(1<<variables);mask++){bool allowed=true;
    for(const auto& g:gs){auto m=lead(g);bool contained=true;
      for(int i=0;i<variables;i++)if(m[i]&&!(mask&(1<<i)))contained=false;
      if(contained)allowed=false;}
    if(allowed)best=std::max(best,__builtin_popcount(unsigned(mask)));}
  return best; }
int staircase(const std::vector<Poly>& gs) { if(krullDimension(gs)<0)return 0;if(krullDimension(gs)>0)return -1;
  std::array<int,3> bounds{1,1,1};
  for(int i=0;i<variables;i++){bounds[i]=17;for(auto& g:gs){auto m=lead(g);bool pure=true;
    for(int j=0;j<variables;j++)if(i!=j&&m[j])pure=false;
    if(pure)bounds[i]=std::min(bounds[i],m[i]);}require(bounds[i]<=16,"finite staircase cap");}
  int count=0;for(int i=0;i<bounds[0];i++)for(int j=0;j<bounds[1];j++)for(int k=0;k<bounds[2];k++){
    bool ok=true;for(auto& g:gs)if(divides(lead(g),{i,j,k}))ok=false;count+=ok;}return count; }
int hilbert(const std::vector<Poly>& gs,int d) { int count=0;
  for(int i=0;i<=d;i++)for(int j=0;j<=d-i;j++){int k=d-i-j;if(variables==2&&k)continue;
    bool ok=true;for(auto& g:gs)if(divides(lead(g),{i,j,k}))ok=false;count+=ok;}return count; }
int power(int x,int n){int r=1;while(n--)r=mod(r*x);return r;}
int value(const Poly& f,Mon x){int a=0;for(auto [m,c]:f){int b=c;for(int i=0;i<variables;i++)b=mod(b*power(x[i],m[i]));a=mod(a+b);}return a;}
int rationalZeros(const std::vector<Poly>& fs){int count=0;
  for(int x=0;x<prime;x++){for(int y=0;y<prime;y++){
    for(int h=0;h<(variables==3?prime:1);h++){bool zero=true;
      for(auto& f:fs){if(value(f,{x,y,h}))zero=false;}
      count+=zero;}}}
  return count;}
int selectedDegree(const std::string& path){std::stringstream in(read(path));std::string line;int best=-1;
  std::regex row(R"(^\s*([0-9]+)\s+[0-9]+\s+[0-9]+\s+[0-9]+\s+x\s+[0-9]+\s+)");
  while(std::getline(in,line)){std::smatch m;if(std::regex_search(line,m,row))best=std::max(best,std::stoi(m[1]));}return best;}

// Polynomial-basis source checks, independently of msolve and the GB parser.
struct Field { int p,n; };
struct E { std::array<int,3> c{}; bool operator==(const E&)const=default; };
Field field{5,2};
int fm(long long a){return int((a%field.p+field.p)%field.p);}
E elt(int a,int b=0,int c=0){return {{fm(a),fm(b),fm(c)}};}
E plus(E a,E b){for(int i=0;i<3;i++)a.c[i]=fm(a.c[i]+b.c[i]);return a;}
E neg(E a){for(auto& x:a.c)x=fm(-x);return a;}
E minus(E a,E b){return plus(a,neg(b));}
E mul(E a,E b){std::array<int,5> c{};for(int i=0;i<field.n;i++)for(int j=0;j<field.n;j++)c[i+j]=fm(c[i+j]+a.c[i]*b.c[j]);
  for(int i=2*field.n-2;i>=field.n;i--){if(field.n==2)c[i-2]=fm(c[i-2]+2*c[i]);
    else{c[i-3]=fm(c[i-3]-c[i]);c[i-2]=fm(c[i-2]-c[i]);}}return elt(c[0],c[1],field.n==3?c[2]:0);}
E pow(E a,int n){E b=elt(1);while(n){if(n&1)b=mul(b,a);a=mul(a,a);n>>=1;}return b;}
E div(E a,E b){require(b!=elt(0),"source field divide zero");int q=1;for(int i=0;i<field.n;i++)q*=field.p;return mul(a,pow(b,q-2));}
bool curvePoint(E a,E b,E x,E y){return pow(y,2)==plus(plus(pow(x,3),mul(a,x)),b);}
void checkHomogenization(const std::string& base,const std::string& affine,const std::string& homogeneous){
  auto a=input(base+"/inputs/"+affine+".ms"),h=input(base+"/inputs/"+homogeneous+".ms");
  require(a.p==h.p&&a.n==2&&h.n==3&&a.fs.size()==h.fs.size(),"homogenization shape");
  for(size_t i=0;i<a.fs.size();i++){Poly expected;
    for(auto [key,c]:a.fs[i]){Mon m=key;require(degree(m)<=2,"homogenization input degree");m[2]=2-degree(m);term(expected,m,c);}
    require(expected==h.fs[i],"literal degree-two homogenization");}}
void sourceChecks(const std::string& base){field={5,2};E beta=elt(0,1),A=beta,D=elt(4),c=plus(elt(1),beta),t=div(A,D);
  require(plus(A,pow(c,2))==plus(elt(1),mul(D,pow(c,2))),"Edwards actual target");
  std::array<E,4> coeff{minus(pow(c,2),elt(1)),minus(t,pow(c,2)),
    minus(plus(mul(elt(2),pow(c,2)),mul(mul(elt(2),minus(elt(1),t)),c)),mul(elt(2),t)),mul(t,minus(pow(c,2),elt(1)))};
  require(coeff==std::array<E,4>{elt(2,2),elt(2,2),elt(2),elt(1,3)},"Edwards source descent coefficients");
  auto ed=input(base+"/inputs/edwards-affine.ms");require(ed.p==5&&ed.n==2&&ed.fs.size()==2,"Edwards input shape");
  for(int k=0;k<2;k++){Poly g;term(g,{0,2,0},coeff[0].c[k]);term(g,{2,0,0},coeff[1].c[k]);
    term(g,{0,1,0},coeff[2].c[k]);term(g,{0,0,0},coeff[3].c[k]);require(g==ed.fs[k],"actual Edwards input binding");}
  auto top=input(base+"/inputs/edwards-top.ms");require(top.p==5&&top.n==2&&top.fs.size()==2,"top input shape");
  for(int k=0;k<2;k++){Poly g;for(auto [m,v]:ed.fs[k])if(degree(m)==2)term(g,m,v);require(g==top.fs[k],"actual highest-part binding");}
  field={101,3};for(int x=0;x<101;x++)require(fm(x*x*x+x+1)!=0,"cubic irreducibility roots");
  E a=elt(44,52,60),b=elt(58,87,74);require(plus(mul(elt(4),pow(a,3)),mul(elt(27),pow(b,2)))!=elt(0),"JV discriminant");
  std::array<E,2> xs{elt(85,84,37),elt(21,94,16)},ys{elt(15,3,86),elt(41,34,80)};
  std::array<std::array<E,5>,2> expected{{{elt(100,29,59),elt(32,34,27),elt(55,71,31),elt(17,83,48),elt(81,16,32)},
    {elt(59,78,61),elt(59,14,69),elt(57,20,40),elt(80,89,40),elt(77,11,12)}}};
  for(int i=0;i<2;i++){E x=xs[i];require(curvePoint(a,b,x,ys[i]),"JV actual target");
    std::array<E,5> got{pow(x,2),mul(elt(-2),x),minus(mul(elt(-2),mul(a,x)),mul(elt(4),b)),
      minus(mul(elt(-4),pow(x,2)),mul(elt(2),a)),minus(pow(a,2),mul(elt(4),mul(b,x)))};
    require(got==expected[i],"JV exact source coefficient descent");
    auto source=input(base+"/inputs/"+(i?"jv-split-affine":"jv-empty-affine")+".ms");
    require(source.p==101&&source.n==2&&source.fs.size()==3,"JV input shape");
    for(int k=0;k<3;k++){Poly g;term(g,{2,0,0},got[0].c[k]);term(g,{1,1,0},got[1].c[k]);
      term(g,{1,0,0},got[2].c[k]);term(g,{0,1,0},got[3].c[k]);term(g,{0,0,0},got[4].c[k]);
      if(k==0){term(g,{0,2,0},1);}
      require(g==source.fs[k],"actual JV source input binding");}}
  E ux=elt(6),uy=elt(35,93,77),vx=elt(63),vy=elt(2,66,1);
  require(curvePoint(a,b,ux,uy)&&curvePoint(a,b,vx,vy),"JV actual factor-base points");
  E slope=div(minus(vy,uy),minus(vx,ux)),rx=minus(minus(pow(slope,2),ux),vx);
  E ry=minus(mul(slope,minus(ux,rx)),uy);require(rx==xs[1]&&ry==ys[1],"JV actual factor-base sum");
  checkHomogenization(base,"edwards-affine","edwards-homogeneous");
  checkHomogenization(base,"jv-empty-affine","jv-empty-homogeneous");
  checkHomogenization(base,"jv-split-affine","jv-split-homogeneous");
}
void calibration(bool negative){prime=5;variables=2;auto input=parseList("x^2,y-1"),good=buchberger(input);
  require(staircase(good)==2&&allSpairsZero(good),"positive dual-number control");
  auto wrong=buchberger(parseList("x^2,y-2"));bool detects=false;
  for(auto& f:input)if(!remainder(f,wrong).empty())detects=true;
  require(detects,"wrong constant detector");
  auto reduced=buchberger(parseList("x,y-1"));require(staircase(reduced)==1,"multiplicity negative control");
  std::cout<<"{\"kind\":\""<<(negative?"negative":"controls")<<"\",\"dual_length\":2,\"simple_length\":1,\"wrong_constant_detected\":true,\"pass\":true}\n";}
int main(int argc,char** argv){try{
  require(argc==3,"usage: analyze STUDY --controls|--negative|--main");std::string base=argv[1],mode=argv[2];
  if(mode=="--controls"||mode=="--negative"){calibration(mode=="--negative");return 0;}
  require(mode=="--main","mode");sourceChecks(base);std::cout<<"{\"kind\":\"source\",\"edwards\":true,\"jv\":true,\"pass\":true}\n";
  std::vector<std::string> fixtures{"control","edwards-affine","edwards-top","edwards-homogeneous","jv-empty-affine","jv-empty-homogeneous","jv-split-affine","jv-split-homogeneous"};
  int failures=0;
  for(auto& id:fixtures)for(int rep=0;rep<2;rep++){bool pass=true;std::string prefix=base+"/runs/main/"+id+(rep?"-aa":"");
    try{auto f=input(base+"/inputs/"+id+".ms");require(std::stoi(read(prefix+".exit"))==0,"solver exit");
      auto expected=buchberger(f.fs);int peak=referencePeak;auto got=solverBasis(prefix+".out");
      require(canonical(got)==expected,"independent reduced GB mismatch");require(allSpairsZero(got),"output S-pairs");
      for(auto& h:f.fs)require(remainder(h,got).empty(),"input remainder");
      int dim=krullDimension(got),length=staircase(got),zeros=rationalZeros(f.fs),maxdegree=-1;
      for(auto& h:got)maxdegree=std::max(maxdegree,degree(h));
      if(id=="control")require(dim==0&&length==4&&zeros==1,"positive solver control");
      if(id=="edwards-affine")require(dim==0&&length==2&&zeros==1,"Lean Edwards known answer");
      if(id=="edwards-top")require(dim==1&&zeros==9,"nonregular top known answer");
      if(id=="jv-empty-affine")require(length==0&&zeros==0,"source empty answer");
      if(id=="jv-split-affine")require(length==1&&zeros==1,"source split answer");
      if(rep)require(read(prefix+".out")==read(base+"/runs/main/"+id+".out"),"A/A exact GB bytes");
      std::cout<<"{\"fixture\":\""<<id<<"\",\"replicate\":"<<rep<<",\"p\":"<<prime<<",\"variables\":"<<variables
        <<",\"basis_size\":"<<got.size()<<",\"basis_degree\":"<<maxdegree<<",\"initial_krull_dimension\":"<<dim
        <<",\"affine_length_if_finite\":"<<length<<",\"rational_zeros\":"<<zeros<<",\"reference_buchberger_peak\":"<<peak
        <<",\"msolve_selected_pair_degree\":"<<selectedDegree(prefix+".stdout")<<",\"hilbert_0_to_12\":[";
      for(int d=0;d<=12;d++){if(d)std::cout<<",";std::cout<<hilbert(got,d);}std::cout<<"],\"pass\":true}\n";
    }catch(const std::exception& e){pass=false;std::cout<<"{\"fixture\":\""<<id<<"\",\"replicate\":"<<rep<<",\"pass\":false,\"error\":\""<<e.what()<<"\"}\n";}
    if(!pass)failures++;}
  std::cout<<"{\"kind\":\"integrity\",\"source_checks\":1,\"solver_records\":16,\"failures\":"<<failures<<",\"pass\":"<<(failures?"false":"true")<<"}\n";
  return failures?1:0;
}catch(const std::exception& e){std::cerr<<e.what()<<"\n";return 2;}}
