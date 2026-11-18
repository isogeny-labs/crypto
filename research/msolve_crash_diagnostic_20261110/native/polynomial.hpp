// Derived from frozen multiplicity analyzer; widened products for F65537.
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
int inv(int a) { for(int b=1;b<p;b++) if(mod(1LL*a*b)==1) return b;
  throw std::runtime_error("noninvertible coefficient"); }
void term(Poly& f,Mon m,int c) { int a=mod(f[m]+c); if(a) f[m]=a; else f.erase(m); }
Poly add(Poly a,const Poly& b,int sign=1) { for(auto [m,c]:b) term(a,m,sign*c); return a; }
Poly times(const Poly& a,Mon n,int k=1) { Poly b;
  for(auto [m,c]:a) term(b,{m.first+n.first,m.second+n.second},mod(1LL*c*k));
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
      f=add(f,times(g,{m.first-n.first,m.second-n.second},mod(1LL*c*inv(g.at(n)))),-1);
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
int power(int a,int n) { int r=1; while(n--) r=mod(1LL*r*a); return r; }
int value(const Poly& f,int x,int y) { int r=0;
  for(auto [m,c]:f) r=mod(r+1LL*c*power(x,m.first)*power(y,m.second));
  return r; }
