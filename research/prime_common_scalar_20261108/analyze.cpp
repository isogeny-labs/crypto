#include <gmpxx.h>
#include <algorithm>
#include <cmath>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>

using Z=mpz_class;
using Q=mpq_class;
void need(bool b,const std::string& s) { if(!b)throw std::runtime_error(s); }
std::string read(const std::string& p) {
    std::ifstream f(p);need(bool(f),"cannot open "+p);
    return {std::istreambuf_iterator<char>(f),std::istreambuf_iterator<char>()};
}
std::vector<std::string> split(const std::string&s,char c) {
    std::vector<std::string> r;size_t start=0;
    while(true) {size_t end=s.find(c,start);r.push_back(s.substr(start,end-start));
        if(end==std::string::npos)break;
        start=end+1;}
    return r;
}
std::map<int,int> hist(const std::string&s,int q) {
    std::map<int,int> h;if(s.empty())return h;
    for(auto&v:split(s,';')) {
        auto p=split(v,':');need(p.size()==2,"bad histogram");
        int w=std::stoi(p[0]),n=std::stoi(p[1]);
        need(w>0&&w<q&&n>0&&h.emplace(w,n).second,"bad histogram weight/count");
    }return h;
}
// Independent recompute: grouped generating functions, using binomial
// coefficients for repeated weights, rather than the instrument's coin loop.
Z recompute(const std::map<int,int>&h,int q) {
    std::vector<Z> d(q);d[0]=1;
    for(auto [w,n]:h) {
        std::vector<Z> next(q);
        for(int k=0;k*w<q;++k) {
            Z multiplicity;mpz_bin_uiui(multiplicity.get_mpz_t(),n+k-1,k);
            for(int i=0;i+k*w<q;++i)next[i+k*w]+=d[i]*multiplicity;
        }
        d=next;
    }
    Z total=0;for(const Z&v:d)total+=v;return total;
}
using Key=std::tuple<int,int,int,int,int,std::string,std::string,std::string>;
using Group=std::tuple<int,int,std::string,std::string,std::string>;
struct Stats {int fewer=0,equal=0,more=0;void add(const Z&a,const Z&b){if(b<a)++fewer;else if(b==a)++equal;else ++more;}};
int main(int argc,char**argv) {
 try {
    if(argc==2&&std::string(argv[1])=="--unit") {
        need(recompute({{2,1},{3,1}},6)==5,"unit1");need(recompute({{2,2}},6)==6,"unit2");
        std::cout<<"grouped_recompute_units=PASS\n";return 0;
    }
    need(argc==4,"use PANEL REPLAY OUTPUT_DIR");std::string a=read(argv[1]);need(a==read(argv[2]),"replay differs");
    std::set<Key> expected;
    for(int l:{16,32,64})for(int t=-l+1;t<l;++t)if(t)for(int z:{-1,1,3})for(int q:{16,24,32,48})
      for(std::string m:{"newton","smith"}) {
        int i=__builtin_ctz(unsigned(std::abs(t)));
        for(std::string k:{"original","entry","scalar"})expected.emplace(l,t,i,z,q,m,k,"same");
        if(q<=24)for(std::string k:{"entry","scalar"})expected.emplace(l,t,i,z,q,m,k,"proxy");
      }
    need(expected.size()==20928,"prospective size");
    std::istringstream in(a);std::string line;std::getline(in,line);
    need(line=="L,t,level,zeta,q,method,kernel,budget,used_q,original_tuples,candidate_tuples,original_weights,candidate_weights","header");
    int rows=0;std::map<Group,Stats> groups;std::map<std::string,Stats> totals;
    while(std::getline(in,line)) {
        auto c=split(line,',');need(c.size()==13,"column count");
        int l=std::stoi(c[0]),t=std::stoi(c[1]),i=std::stoi(c[2]),z=std::stoi(c[3]),q=std::stoi(c[4]),uq=std::stoi(c[8]);
        need(expected.erase({l,t,i,z,q,c[5],c[6],c[7]})==1,"unexpected cell");
        int want=c[7]=="same"?q:c[6]=="entry"?(4*q+2)/3:2*q;need(uq==want,"precision proxy");
        Z original(c[9]),candidate(c[10]);need(original>0&&candidate>0,"nonpositive");
        need(recompute(hist(c[11],q),q)==original,"original recount");
        need(recompute(hist(c[12],uq),uq)==candidate,"candidate recount");
        if(c[6]=="original")need(original==candidate&&c[11]==c[12],"embedded A/A");
        groups[{l,i,c[5],c[6],c[7]}].add(original,candidate);
        totals[c[5]+"_"+c[6]+"_"+c[7]].add(original,candidate);++rows;
    }
    need(expected.empty()&&rows==20928,"incomplete census");
    std::filesystem::create_directories(argv[3]);
    std::ofstream csv(std::string(argv[3])+"/per-level.csv");csv<<"L,level,method,kernel,budget,fewer,equal,more\n";
    for(auto&[key,s]:groups){auto[l,i,m,k,b]=key;csv<<l<<','<<i<<','<<m<<','<<k<<','<<b<<','<<s.fewer<<','<<s.equal<<','<<s.more<<'\n';}
    std::ofstream out(std::string(argv[3])+"/summary.json");
    out<<"{\"rows\":"<<rows<<",\"tuple_recomputations\":"<<2*rows<<",\"replay_byte_identical\":true,\"groups\":{";
    bool first=true;for(auto&[key,s]:totals){if(!first)out<<',';first=false;out<<'"'<<key<<"\":{\"fewer\":"<<s.fewer<<",\"equal\":"<<s.equal<<",\"more\":"<<s.more<<'}';}
    out<<"}}\n";std::cout<<"PASS rows="<<rows<<" recounts="<<2*rows<<" byte_replay=equal\n";
 }catch(const std::exception&e){std::cerr<<"FAIL "<<e.what()<<'\n';return 1;}
}
