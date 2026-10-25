// Independent prime-field arithmetic and headline recomputation. No crypto_lib.
// C++ is intentional: a separately written implementation checks the Rust output.
#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <random>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>
using U=uint64_t;
U mm(U a,U b,U p){return (__uint128_t)a*b%p;}
U pw(U a,U k,U p){U z=1;for(;k;k>>=1,a=mm(a,a,p))if(k&1)z=mm(z,a,p);return z;}
bool prime(U n){if(n<2)return false;for(U q:{2,3,5,7,11,13,17,19,23,29,31,37}){if(n%q==0)return n==q;} U d=n-1;int s=0;while(!(d&1)){d>>=1;s++;}for(U a:{2,325,9375,28178,450775,9780504,1795265022}){if(a%n==0)continue;U x=pw(a%n,d,n);if(x==1||x==n-1)continue;bool ok=false;for(int i=1;i<s;i++){x=mm(x,x,n);if(x==n-1){ok=true;break;}}if(!ok)return false;}return true;}
struct P{U x=0,y=0;bool inf=true;bool operator==(const P&b)const{return inf==b.inf&&(inf||(x==b.x&&y==b.y));}};
struct C{U p,a,b;bool on(P z)const{return z.inf||mm(z.y,z.y,p)==(mm(mm(z.x,z.x,p),z.x,p)+mm(a,z.x,p)+b)%p;}
P add(P x,P y)const{if(x.inf)return y;if(y.inf)return x;if(x.x==y.x&&(x.y+y.y)%p==0)return {};U numerator=x==y?(3*mm(x.x,x.x,p)+a)%p:(y.y+p-x.y)%p;U denominator=x==y?2*x.y%p:(y.x+p-x.x)%p;U s=mm(numerator,pw(denominator,p-2,p),p);U z=(mm(s,s,p)+2*p-x.x-y.x)%p;return {z,(mm(s,(x.x+p-z)%p,p)+p-x.y)%p,false};}
P mul(P x,U k)const{P z;for(;k;k>>=1,x=add(x,x))if(k&1)z=add(z,x);return z;}};
U integer(const std::string&s){return std::stoull(s,nullptr,s.rfind("0x",0)==0?16:10);}
std::vector<std::string> fields(const std::string&s){std::vector<std::string>v;std::stringstream ss(s);std::string x;while(std::getline(ss,x,','))v.push_back(x);return v;}
void require(bool b,const char*why){if(!b)throw std::runtime_error(why);}
struct Agg{long double sum=0;U n=0;void add(double v){sum+=v;n++;}double mean()const{return double(sum/n);}};
using Cell=std::tuple<int,int,int>;
double quant(std::vector<double>v,double q){std::sort(v.begin(),v.end());return v[std::llround((v.size()-1)*q)];}
void interval(const std::vector<double>&logs,U seed){std::mt19937_64 rng(seed);std::uniform_int_distribution<int>pick(0,int(logs.size())-1);std::vector<double>bs;for(int k=0;k<10000;k++){double z=0;for(size_t j=0;j<logs.size();j++)z+=logs[pick(rng)];bs.push_back(std::exp(z/logs.size()));}double z=0;for(double x:logs)z+=x;std::cout<<"\"geometric_mean\":"<<std::exp(z/logs.size());for(auto a:std::array<std::pair<const char*,double>,3>{{{"ci90",.05},{"ci95",.025},{"ci98",.01}}})std::cout<<",\""<<a.first<<"\":["<<quant(bs,a.second)<<","<<quant(bs,1-a.second)<<"]";}
int main(int argc,char**argv){try{require(argc==3,"A.csv B.csv required");std::cout<<std::setprecision(17);std::map<Cell,Agg> cells;std::map<std::pair<int,std::string>,Agg> controls;std::set<std::tuple<U,U,U,U,U,U>> checked;std::map<std::string,U>statuses;U rows=0,verified=0,mutation_rejections=0;std::array<U,2>panel_rows{};
for(int panel=0;panel<2;panel++){std::ifstream f(argv[panel+1]);require(bool(f),"CSV unreadable");std::string line;std::getline(f,line);while(std::getline(f,line)){auto v=fields(line);require(v.size()==18,"CSV columns");rows++;panel_rows[panel]++;statuses[v[14]]++;C c{integer(v[5]),integer(v[6]),integer(v[7])};U r=integer(v[8]);P g{integer(v[9]),integer(v[10]),false},q{integer(v[11]),integer(v[12]),false};require(c.on(g)&&c.on(q),"point off curve");auto key=std::make_tuple(c.p,c.a,c.b,r,g.x,g.y);if(checked.insert(key).second){require(prime(c.p)&&prime(r),"composite parameter");require(c.mul(g,r).inf,"wrong subgroup order");}
if(v[14]!="verified"){continue;}
verified++;U k=integer(v[13]);require(k<r&&c.mul(g,k)==q,"scalar recovery mismatch");require(!(c.mul(g,(k+1)%r)==q),"mutated scalar accepted");mutation_rejections++;double s=std::stod(v[15]),gae=std::stod(v[16]);require(std::abs(s-gae/std::sqrt(double(r)))<1e-10*std::max(1.,std::abs(s)),"S conversion mismatch");int ci=std::stoi(v[0]),volcano=std::stoi(v[1]),level=std::stoi(v[2]);if(panel==0&&v[4]=="rho-neg")cells[{ci,level,volcano}].add(s);if(panel==0&&(v[4]=="rho-plain"||v[4]=="rho-neg-aa"))controls[{ci,v[4]}].add(s);
}}
require(panel_rows[0]==41856&&panel_rows[1]==1664,"planned panel count mismatch");require(statuses.size()==1&&statuses["verified"]==rows,"incomplete or failed panel");std::cout<<"{\"schema\":\"conductor-gap.independent/v1\",\"implementation\":\"standalone C++20; no shared arithmetic or analyzer\",\"same_host\":true,\"rows\":"<<rows<<",\"verified\":"<<verified<<",\"unique_curve_generators\":"<<checked.size()<<",\"mutated_scalar_rejections\":"<<mutation_rejections<<",\"bootstrap\":\"independent mt19937_64, 10000 whole-class draws, seed 2026102522+level\",\"levels\":[";
for(int l=0;l<5;l++){if(l)std::cout<<",";std::vector<double>logs;std::cout<<"{\"level\":"<<l<<",\"class_ratios\":[";for(int ci=0;ci<8;ci++){for(int volcano=0;volcano<4;volcano++)require(cells[{ci,l,volcano}].n==192,"level imbalance");double floor=0;for(int volcano=1;volcano<4;volcano++)floor+=cells[{ci,l,volcano}].mean()/3;double ratio=floor/cells[{ci,l,0}].mean();logs.push_back(std::log(ratio));if(ci)std::cout<<",";std::cout<<ratio;}std::cout<<"],";interval(logs,2026102522+l);std::cout<<"}";}
std::vector<double>pos;for(int ci=0;ci<8;ci++)pos.push_back(std::log(controls[{ci,"rho-plain"}].mean()/controls[{ci,"rho-neg-aa"}].mean()));std::cout<<"],\"positive_control\":{";interval(pos,2026102527);std::cout<<"},\"verdict\":\"all arithmetic checks passed; means independently recomputed; bootstrap is an independent Monte Carlo check\"}\n";return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<"\n";return 1;}}
