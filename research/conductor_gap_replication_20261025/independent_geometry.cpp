// Exact independent fixture and rational 2-isogeny graph checks; no crypto_lib.
// Compile only after the measured campaign has ended.
#include <array>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>
using U=uint64_t;
using I=__int128_t;
U mod(I a,U p){I r=a%I(p);return U(r<0?r+p:r);}
U mul(U a,U b,U p){return U((__uint128_t)a*b%p);}
U power(U a,U k,U p){U r=1;for(;k;k>>=1,a=mul(a,a,p)){if(k&1)r=mul(r,a,p);}return r;}
void need(bool ok,const char*why){if(!ok)throw std::runtime_error(why);}
bool prime(U n){
    if(n<2)return false;
    for(U q:{2,3,5,7,11,13,17,19,23,29,31,37})if(n%q==0)return n==q;
    U d=n-1;int s=0;while(!(d&1)){d>>=1;++s;}
    for(U a:{2,325,9375,28178,450775,9780504,1795265022}){
        if(a%n==0)continue;
        U x=power(a%n,d,n);if(x==1||x==n-1)continue;
        bool good=false;
        for(int j=1;j<s;++j){x=mul(x,x,n);if(x==n-1){good=true;break;}}
        if(!good)return false;
    }
    return true;
}
std::vector<std::string> split(const std::string&line){
    std::stringstream s(line);std::string word;std::vector<std::string>v;
    while(std::getline(s,word,','))v.push_back(word);
    return v;
}
U number(const std::string&s){return std::stoull(s);}
struct Fixture{U d,p;int64_t t;U order,r,ell,f,kd;};
struct Vertex{int ci,vi,id,level,parent;U p,a,b,j;std::vector<int>nbrs;};
using Key=std::tuple<int,int,int>;
std::array<U,4> phi(U y,U p){
    U y2=mul(y,y,p),y3=mul(y2,y,p);
    return {mod(I(y3)-162000*I(y2)+8748000000LL*I(y)-157464000000000LL,p),
            mod(1488*I(y2)+40773375LL*I(y)+8748000000LL,p),
            mod(-I(y2)+1488*I(y)-162000,p),1};
}
U eval(const std::array<U,4>&c,U x,U p){U r=0;for(int i=3;i>=0;--i)r=(mul(r,x,p)+c[i])%p;return r;}
U cm_j(U d,U p){
    const std::map<U,I>values{{11,-I(32768)},{19,-I(884736)},{43,-I(884736000)},{67,-I(147197952000LL)}};
    return mod(values.at(d),p);
}
int main(int argc,char**argv){try{
    need(argc==3,"fixtures.csv vertices.csv required");
    std::map<std::pair<std::string,int>,Fixture> fixtures;
    std::ifstream in(argv[1]);need(bool(in),"fixture CSV unreadable");
    std::string line;std::getline(in,line);
    U classes=0,kernel_degrees=0;
    std::set<std::pair<U,int64_t>>classes_seen;
    while(std::getline(in,line)){
        auto v=split(line);need(v.size()==11,"fixture column count");
        Fixture c{number(v[2]),number(v[4]),std::stoll(v[5]),number(v[6]),number(v[7]),number(v[8]),number(v[9]),number(v[10])};
        need(prime(c.p)&&prime(c.r)&&prime(c.ell),"parameter primality");
        need(I(c.t)*c.t+I(c.d)*c.f*c.f==4*I(c.p),"Frobenius discriminant identity");
        need(I(c.order)==I(c.p)+1-c.t&&c.order%c.r==0,"Hasse order and subgroup relation");
        need(I(c.r)*c.r>16*I(c.p),"prime subgroup must exceed Hasse interval width");
        need(classes_seen.insert({c.p,c.t}).second,"classes must be distinct");
        U x=mul(mod(I(c.t),c.ell),(c.ell+1)/2,c.ell),z=1,k=0;
        do{z=mul(z,x,c.ell);++k;}while(z!=1&&z!=c.ell-1&&k<c.ell);
        need(k==c.kd,"kernel-abscissa field degree");++kernel_degrees;
        if(v[0]=="A"){
            need(c.f==16*c.ell&&I(c.ell)*c.ell*c.ell*c.ell>I(c.p),"deep A conductor and large gap");
        }else{
            need(v[0]=="B"&&c.f==c.ell&&c.t!=1&&c.r!=c.p,"non-anomalous prime conductor B");
            need(8*k>=c.ell-1,"B hard-kernel degree admission");
            U q=(c.d+1)/4;
            need((I(c.t)-I(c.ell)+2)%(2*q)==0,"B canonical twist-kernel admission");
        }
        fixtures[{v[0],std::stoi(v[1])}]=c;++classes;
    }
    need(classes==12,"planned fixture count");
    std::map<Key,Vertex> graph;
    std::map<int,std::set<U>> js;
    in.close();in.open(argv[2]);need(bool(in),"vertex CSV unreadable");std::getline(in,line);
    while(std::getline(in,line)){
        auto v=split(line);need(v.size()==12,"vertex column count");
        Vertex c{std::stoi(v[0]),std::stoi(v[1]),std::stoi(v[2]),std::stoi(v[3]),std::stoi(v[8]),
                 number(v[4]),number(v[5]),number(v[6]),number(v[7]),{}};
        for(int i=9;i<12;++i)if(std::stoi(v[i])>=0)c.nbrs.push_back(std::stoi(v[i]));
        const auto&f=fixtures.at({"A",c.ci});need(c.p==f.p,"graph field");
        U a3=mul(mul(c.a,c.a,c.p),c.a,c.p),b2=mul(c.b,c.b,c.p);
        U den=mod(4*I(a3)+27*I(b2),c.p);need(den!=0,"singular vertex");
        need(mul(mul(1728,mod(4*I(a3),c.p),c.p),power(den,c.p-2,c.p),c.p)==c.j,"j-invariant");
        need(js[c.ci].insert(c.j).second,"four volcanoes must have disjoint j sets");
        need(graph.emplace(Key{c.ci,c.vi,c.id},c).second,"duplicate vertex");
    }
    need(graph.size()==1472,"planned graph coverage");
    U edges=0,irreducible_floor_quadratics=0;
    std::map<std::pair<int,int>,std::array<int,5>>counts;
    for(const auto&[key,c]:graph){
        need(c.level>=0&&c.level<=4,"graph level bound");++counts[{c.ci,c.vi}][c.level];
        need(c.nbrs.size()==(c.level==4?1:3),"complete rational neighbor count");
        if(c.level==0){
            need(c.parent==-1,"crater parent");U expected=cm_j(fixtures.at({"A",c.ci}).d,c.p);
            need((c.vi==0)==(c.j==expected),"maximal-order CM crater identity");
        }
        auto polynomial=phi(c.j,c.p);int parents=0;
        for(int ni:c.nbrs){
            const auto&n=graph.at(Key{c.ci,c.vi,ni});
            need(std::abs(n.level-c.level)==1,"volcano must be stratified without horizontal edges");
            need(eval(polynomial,n.j,c.p)==0,"Phi_2 edge identity");
            bool reverse=false;for(int ri:n.nbrs)if(ri==c.id)reverse=true;need(reverse,"symmetric adjacency");
            if(n.level<c.level){need(c.parent==ni,"parent index");++parents;}
            if(c.id<ni)++edges;
        }
        need(parents==(c.level==0?0:1),"unique ascent");
        if(c.level==4){
            U root=graph.at(Key{c.ci,c.vi,c.nbrs[0]}).j;
            U b=(polynomial[2]+root)%c.p;
            U cc=(polynomial[1]+mul(root,b,c.p))%c.p;
            U disc=mod(I(mul(b,b,c.p))-4*I(cc),c.p);
            need(power(disc,(c.p-1)/2,c.p)==c.p-1,"floor has no extra rational 2-neighbor");
            ++irreducible_floor_quadratics;
        }
    }
    for(const auto&[k,c]:counts)need(c==std::array<int,5>{1,3,6,12,24},"balanced full-volcano census");
    need(counts.size()==32&&edges==1440&&irreducible_floor_quadratics==768,"graph totals");
    std::cout<<"{\"schema\":\"conductor-gap.independent-geometry/v1\",\"host_independence\":\"caller provenance required\",\"classes\":"<<classes
             <<",\"exact_kernel_degrees\":"<<kernel_degrees<<",\"volcanoes\":32,\"vertices\":"<<graph.size()
             <<",\"verified_Phi2_edges\":"<<edges<<",\"irreducible_floor_quadratics\":"<<irreducible_floor_quadratics
             <<",\"disjoint_j_sets\":true,\"verdict\":\"all exact checks passed; independent C++ implementation\"}\n";
    return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<"\n";return 1;}}
