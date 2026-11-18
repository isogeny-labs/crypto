#include <algorithm>
#include <cctype>
#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

// Exact, deliberately bounded parser/checker for the registered fixtures.
// No timing or general solver-conformance conclusion is computed here.
using Mon = std::pair<int,int>;
using Poly = std::map<Mon,int>;
int p;
int mod(long long n) { return int((n%p+p)%p); }
int inv(int a) { for(int b=1;b<p;b++) if(mod(a*b)==1) return b;
  throw std::runtime_error("noninvertible coefficient"); }
void term(Poly& f,Mon m,int c) { int a=mod(f[m]+c); if(a) f[m]=a; else f.erase(m); }
Poly add(Poly a,const Poly& b,int sign=1) { for(auto [m,c]:b) term(a,m,sign*c); return a; }
Poly times(const Poly& a,Mon n,int k=1) { Poly b;
  for(auto [m,c]:a) term(b,{m.first+n.first,m.second+n.second},mod(c*k));
  return b; }
bool lessMon(Mon a,Mon b) { return a.first+a.second!=b.first+b.second ?
  a.first+a.second<b.first+b.second : a.first<b.first; }
Mon lead(const Poly& a) { if(a.empty()) throw std::runtime_error("zero basis element");
  Mon best=a.begin()->first; for(auto [m,c]:a) if(lessMon(best,m)) best=m; return best; }
bool divides(Mon a,Mon b) { return a.first<=b.first && a.second<=b.second; }
Poly monic(Poly a) { return times(a,{0,0},inv(a.at(lead(a)))); }
Poly remainder(Poly f,const std::vector<Poly>& gs) { Poly r; int steps=0;
  while(!f.empty()) { if(++steps>10000) throw std::runtime_error("reduction limit");
    auto m=lead(f); int c=f.at(m); bool reduced=false;
    for(auto& g:gs) { auto n=lead(g); if(divides(n,m)) {
      f=add(f,times(g,{m.first-n.first,m.second-n.second},mod(c*inv(g.at(n)))),-1);
      reduced=true; break; }}
    if(!reduced) { term(r,m,c); f.erase(m); }} return r; }
std::string read(const std::string& path) { std::ifstream in(path);
  if(!in) throw std::runtime_error("missing file: "+path);
  return {std::istreambuf_iterator<char>(in),{}}; }
std::string compact(std::string s) { s.erase(std::remove_if(s.begin(),s.end(),
  [](unsigned char c){return std::isspace(c);}),s.end()); return s; }
Poly polynomial(std::string s) { s=compact(s); Poly f; size_t at=0;
  while(at<s.size()) { int sign=1; if(s[at]=='+' || s[at]=='-') { if(s[at]=='-') sign=-1; at++; }
    size_t end=at; while(end<s.size() && s[end]!='+' && s[end]!='-') end++;
    if(end==at) throw std::runtime_error("empty term");
    std::stringstream ts(s.substr(at,end-at)); std::string factor; int c=sign; Mon m{0,0};
    while(std::getline(ts,factor,'*')) {
      if(factor.empty()) throw std::runtime_error("empty factor");
      if(factor[0]=='x' || factor[0]=='y') {
        int degree=1;
        if(factor.size()>1) {
          if(factor[1]!='^' || factor.size()==2) throw std::runtime_error("bad exponent");
          auto n=factor.substr(2);
          if(n.find_first_not_of("0123456789")!=std::string::npos)
            throw std::runtime_error("bad exponent digits");
          degree=std::stoi(n); }
        if(degree>8) throw std::runtime_error("exponent outside frozen bound");
        (factor[0]=='x'?m.first:m.second)+=degree;
      } else {
        if(factor.find_first_not_of("0123456789")!=std::string::npos)
          throw std::runtime_error("unsupported coefficient");
        c=mod(c*std::stoll(factor)); }
    }
    term(f,m,c); at=end;
  } return f; }
std::vector<Poly> list(std::string s) { std::vector<Poly> fs; std::stringstream in(s); std::string part;
  while(std::getline(in,part,',')) { if(compact(part).empty()) throw std::runtime_error("empty polynomial");
    fs.push_back(polynomial(part)); } return fs; }
std::vector<Poly> groebner(const std::string& path) {
  std::stringstream in(read(path)); std::string line,s; bool order=false;
  while(std::getline(in,line)) { if(line.rfind("#variable order:",0)==0)
      order=compact(line.substr(line.find(':')+1))=="x,y";
    if(!line.empty() && line[0]!='#') s+=line; }
  s=compact(s); if(!order || s.size()<3 || s.front()!='[' || s.substr(s.size()-2)!="]:")
    throw std::runtime_error("unsupported GB format or variable order");
  return list(s.substr(1,s.size()-3)); }
bool sameBasis(std::vector<Poly> a,std::vector<Poly> b) {
  for(auto& f:a) f=monic(f);
  for(auto& f:b) f=monic(f);
  std::sort(a.begin(),a.end()); std::sort(b.begin(),b.end()); return a==b; }
bool buchberger(const std::vector<Poly>& gs) {
  for(size_t i=0;i<gs.size();i++) for(size_t j=0;j<i;j++) {
    auto a=lead(gs[i]),b=lead(gs[j]); Mon l{std::max(a.first,b.first),std::max(a.second,b.second)};
    auto s=add(times(gs[i],{l.first-a.first,l.second-a.second},inv(gs[i].at(a))),
      times(gs[j],{l.first-b.first,l.second-b.second},inv(gs[j].at(b))),-1);
    if(!remainder(s,gs).empty()) return false; } return true; }
int standardMonomials(const std::vector<Poly>& gs) { int bx=9,by=9;
  for(auto& f:gs) { auto m=lead(f); if(m.second==0) bx=std::min(bx,m.first);
    if(m.first==0) by=std::min(by,m.second); }
  if(bx==9 || by==9) throw std::runtime_error("no bounded finite staircase");
  int n=0;
  for(int x=0;x<bx;x++) for(int y=0;y<by;y++) { bool outside=true;
    for(auto& f:gs) if(divides(lead(f),{x,y})) outside=false;
    n+=outside; } return n; }
int power(int a,int n) { int r=1; while(n--) r=mod(r*a); return r; }
int value(const Poly& f,int x,int y) { int r=0;
  for(auto [m,c]:f) r=mod(r+c*power(x,m.first)*power(y,m.second));
  return r; }
int pointCount(const std::vector<Poly>& fs) { int n=0;
  for(int x=0;x<p;x++) for(int y=0;y<p;y++) { bool zero=true;
    for(auto& f:fs) if(value(f,x,y)) zero=false;
    n+=zero; } return n; }

struct Node { bool integer=false; long long n=0; std::vector<Node> a; std::string s;
  const Node& operator[](size_t i)const { return a.at(i); }
  long long number() const { if(!integer) throw std::runtime_error("expected integer"); return n; } };
Node parseNode(const std::string& s,size_t& i) { if(i>=s.size()) throw std::runtime_error("truncated array");
  Node v; if(s[i]=='[') { i++; if(i<s.size() && s[i]==']') { i++; return v; }
    for(;;) { v.a.push_back(parseNode(s,i)); if(i>=s.size()) throw std::runtime_error("truncated list");
      if(s[i]==']') { i++; return v; } if(s[i++]!=',') throw std::runtime_error("list separator"); }
  } else if(s[i]=='\'') { size_t j=s.find('\'',++i);
    if(j==std::string::npos) throw std::runtime_error("unterminated string");
    v.s=s.substr(i,j-i); i=j+1; return v;
  } else { size_t start=i; if(s[i]=='-') i++; size_t digits=i;
    while(i<s.size() && std::isdigit(static_cast<unsigned char>(s[i]))) i++;
    if(i==digits) throw std::runtime_error("integer digits");
    v.integer=true; v.n=std::stoll(s.substr(start,i-start)); return v; } }
std::pair<int,int> parametrization(const std::string& path) {
  auto s=compact(read(path)); size_t i=0; auto v=parseNode(s,i);
  if(s.substr(i)!=":" || v.a.size()!=2 || v[0].number()!=0)
    throw std::runtime_error("unsupported parametrization result");
  auto& q=v[1]; if(q.a.size()!=6 || q[0].number()!=p || q[5][0].number()!=1)
    throw std::runtime_error("unsupported parametrization shape");
  return {int(q[2].number()),int(q[5][1][0][0].number())}; }
int maxSelectedDegree(const std::string& path) { std::stringstream in(read(path)); std::string l; int d=0;
  std::regex round(R"(^\s*([0-9]+)\s+[0-9]+\s+[0-9]+\s+[0-9]+\s+x\s+[0-9]+\s+)");
  while(std::getline(in,l)) { std::smatch m; if(std::regex_search(l,m,round)) d=std::max(d,std::stoi(m[1])); }
  return d; }
std::string quote(const std::string& s) { std::string r="\""; for(unsigned char c:s) {
  if(c=='"'||c=='\\') r+='\\';
  if(c>=32) r+=char(c); else r+=' '; } return r+'"'; }
struct Fixture {std::string id,input,basis; int characteristic,dimension,points;};
int main(int argc,char** argv) { try {
  if(argc!=2 && argc!=3) throw std::runtime_error("usage: analyze STUDY_DIR [--preflight]");
  std::string base=argv[1];
  if(argc==3) {
    if(std::string(argv[2])!="--preflight") throw std::runtime_error("unknown mode");
    p=5;
    auto gs=groebner(base+"/evidence/preflight/gb.out");
    auto param=parametrization(base+"/evidence/preflight/param.out");
    bool good=sameBasis(gs,list("x+y-2,y^2-2*y")) && buchberger(gs) &&
      standardMonomials(gs)==2 && param==std::pair<int,int>{2,2};
    for(auto& f:list("x*y,x+y-2")) good=good && remainder(f,gs).empty();
    // A wrong constant must be detected even though the resulting basis is a GB.
    auto wrong=list("x+y-1,y^2-y");
    bool detectsWrong=!remainder(polynomial("x+y-2"),wrong).empty() && !sameBasis(gs,wrong);
    bool rejectsZero=false;
    try { (void)monic(polynomial("5*x")); } catch(const std::runtime_error&) { rejectsZero=true; }
    good=good && detectsWrong && rejectsZero;
    std::cout<<"{\"scope\":\"in-sample preflight c=2 only\",\"pass\":"
      <<(good?"true":"false")<<",\"detects_wrong_constant\":"<<(detectsWrong?"true":"false")
      <<",\"rejects_zero_basis_element\":"<<(rejectsZero?"true":"false")<<"}\n";
    return good?0:1;
  }
  std::vector<Fixture> fixtures={
    {"double","x*y,x+y","x+y,y^2",5,2,1},
    {"split","x*y,x+y-1","x+y-1,y^2-y",5,2,2},
    {"simple","x,y","x,y",5,1,1},
    {"degree3","x^2+x,y^2+y,x*y+x+y","x,y",2,1,1}};
  int failures=0;
  for(auto& f:fixtures) for(int rep=0;rep<2;rep++) { p=f.characteristic;
    std::cout<<"{\"fixture\":"<<quote(f.id)<<",\"replicate\":"<<rep<<",\"p\":"<<p;
    auto input=list(f.input); int points=pointCount(input);
    std::cout<<",\"points\":"<<points<<",\"expected_points\":"<<f.points;
    bool good=points==f.points;
    std::string prefix=base+"/evidence/runs/"+f.id+"-"+std::to_string(rep);
    try {
      auto actualInput=read(base+"/inputs/"+f.id+".ms"); std::stringstream is(actualInput);
      std::string vars,characteristic,rest,line; std::getline(is,vars); std::getline(is,characteristic);
      while(std::getline(is,line)) rest+=line;
      if(compact(vars)!="x,y" || std::stoi(characteristic)!=p || list(rest)!=input)
        throw std::runtime_error("input does not match frozen fixture");
      if(std::stoi(read(prefix+"-gb.exit"))!=0 || std::stoi(read(prefix+"-param.exit"))!=0)
        throw std::runtime_error("solver nonzero exit");
      auto gs=groebner(prefix+"-gb.out"); bool basis=sameBasis(gs,list(f.basis));
      bool spairs=buchberger(gs), reduces=true;
      for(auto& g:input) if(!remainder(g,gs).empty()) reduces=false;
      int dim=standardMonomials(gs); auto [qdim,degree]=parametrization(prefix+"-param.out");
      int selected=maxSelectedDegree(prefix+"-gb.stdout");
      good=good&&basis&&spairs&&reduces&&dim==f.dimension&&qdim==f.dimension&&degree==f.points;
      std::cout<<",\"basis_matches\":"<<(basis?"true":"false")
        <<",\"buchberger\":"<<(spairs?"true":"false")<<",\"inputs_reduce\":"<<(reduces?"true":"false")
        <<",\"standard_monomials\":"<<dim<<",\"expected_dimension\":"<<f.dimension
        <<",\"msolve_qdim\":"<<qdim<<",\"parameter_degree\":"<<degree
        <<",\"selected_degree_max_exploratory\":"<<selected;
    } catch(const std::exception& e) { good=false; std::cout<<",\"error\":"<<quote(e.what()); }
    std::cout<<",\"pass\":"<<(good?"true":"false")<<"}\n"; if(!good) failures++;
  } return failures?1:0;
} catch(const std::exception& e) { std::cerr<<e.what()<<"\n"; return 2; } }
