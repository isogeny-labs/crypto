// Exhaustive finite-module replay of archived observations, not curve point counting.
#include "json.hpp"
#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <set>
using V=std::array<int,6>;
using M=std::array<V,6>;
using Basis=std::vector<V>;
int m3(int x){return (x%3+3)%3;}
int dot(V a,V b){int n=0;for(int i=0;i<6;++i)n+=a[i]*b[i];return n%3;}
V act(V v,M a){V r{};for(int j=0;j<6;++j)for(int i=0;i<6;++i)r[j]=(r[j]+v[i]*a[i][j])%3;return r;}
M mul(M a,M b){M c{};for(int i=0;i<6;++i)c[i]=act(a[i],b);return c;}
M ident(){M a{};for(int i=0;i<6;++i)a[i][i]=1;return a;}
int rank(Basis a){int r=0;for(int c=0;c<6;++c){int k=r;while(k<int(a.size())&&a[k][c]==0)++k;if(k==int(a.size()))continue;
    std::swap(a[k],a[r]);int z=a[r][c];for(int j=0;j<6;++j)a[r][j]=a[r][j]*z%3;
    for(int i=0;i<int(a.size());++i){if(i!=r){z=a[i][c];for(int j=0;j<6;++j)a[i][j]=m3(a[i][j]-z*a[r][j]);}}
    ++r;}return r;}
bool contains(const Basis& b,V v){for(auto r:b){int c=0;while(c<6&&r[c]==0)++c;require(c<6,"zero basis row");int z=v[c];for(int j=0;j<6;++j)v[j]=m3(v[j]-z*r[j]);}return v==V{};}
bool stable(const Basis& b,M a){for(auto v:b)if(!contains(b,act(v,a)))return false;return true;}
bool isotropic(const Basis& b,M j){for(auto v:b)for(auto w:b)if(dot(act(v,j),w))return false;return true;}
std::vector<Basis> grass(int k){std::vector<Basis> out;std::vector<int> piv;
    std::function<void(int)> choose=[&](int start){if(int(piv.size())<k){for(int c=start;c<6;++c){piv.push_back(c);choose(c+1);piv.pop_back();}return;}
        Basis b(k);std::vector<std::pair<int,int>> slots;for(int i=0;i<k;++i){b[i][piv[i]]=1;for(int c=piv[i]+1;c<6;++c)if(std::find(piv.begin(),piv.end(),c)==piv.end())slots.emplace_back(i,c);}
        std::function<void(size_t)> fill=[&](size_t n){if(n==slots.size()){out.push_back(b);return;}auto [i,c]=slots[n];for(int x=0;x<3;++x){b[i][c]=x;fill(n+1);}};fill(0);
    };choose(0);return out;}
std::vector<V> elements(const Basis& b){std::vector<V> out(1);for(auto v:b){auto old=out;for(int s=1;s<3;++s)for(auto w:old){for(int c=0;c<6;++c)w[c]=(w[c]+s*v[c])%3;out.push_back(w);}}return out;}
void print_basis(const Basis& b){std::cout<<'[';bool first=true;for(auto v:b){if(!first)std::cout<<',';first=false;std::cout<<'[';for(int i=0;i<6;++i)std::cout<<(i?",":"")<<v[i];std::cout<<']';}std::cout<<']';}
void block(M& a,const Json& j,int n,int offset){require(j.arr.size()==size_t(n),"matrix row count");for(int i=0;i<n;++i){require(j.arr[i].arr.size()==size_t(n),"matrix column count");for(int k=0;k<n;++k){int x=j.arr[i].arr[k].integer();require(x>=0&&x<3,"noncanonical F3 entry");a[offset+i][offset+k]=x;}}}
int main(int argc,char**argv){try{require(argc==3,"usage: module_audit previous-study/data output-kernels.jsonl");std::string dir=argv[1];std::ofstream kernels(argv[2]);require(bool(kernels),"kernel output");
    auto* old=std::cout.rdbuf();std::array<std::vector<Basis>,3> spaces{grass(1),grass(2),grass(3)};
    require(spaces[0].size()==364&&spaces[1].size()==11011&&spaces[2].size()==33880,"Grassmann count");
    std::set<std::string> keys;int rows=0,nonzero=0,p192=0;long long total=0;std::array<int,6> families{192,224,239,256,384,521};
    for(int family:families){auto file=read_json(dir+"/modules-"+std::to_string(family)+".json");for(const auto& row:file.at("modules").arr){
        int candidate=row.at("candidate").integer(),d=row.at("d").integer();auto key=std::to_string(family)+"/"+std::to_string(candidate)+"/"+std::to_string(d);require(keys.insert(key).second,"duplicate module");
        M a{},j{};block(a,row.at("E_Frobenius"),2,0);block(a,row.at("Prym_Frobenius"),4,2);block(j,row.at("E_Weil"),2,0);block(j,row.at("Prym_Weil"),4,2);
        require(rank(Basis(j.begin(),j.end()))==6,"degenerate Weil form "+key);int multiplier=0;
        for(int i=0;i<6;++i){require(j[i][i]==0,"nonalternating diagonal");for(int k=0;k<6;++k)require(m3(j[i][k]+j[k][i])==0,"nonalternating form");}
        auto aj=mul(a,j);M ajat{};for(int i=0;i<6;++i)for(int k=0;k<6;++k)ajat[i][k]=dot(aj[i],a[k]);
        for(int z=1;z<3;++z){bool ok=true;for(int i=0;i<6;++i)for(int k=0;k<6;++k)ok&=ajat[i][k]==z*j[i][k]%3;if(ok)multiplier=z;}
        require(multiplier!=0,"Frobenius/Weil multiplier mismatch "+key);
        M power=ident();int order=0;do{power=mul(power,a);++order;require(order<=10000,"Frobenius order limit");}while(power!=ident());require(order==row.at("full_torsion_field_degree").integer(),"full field degree mismatch "+key);
        std::array<int,3> counts{};std::vector<Basis> max;
        for(int k=1;k<=3;++k)for(auto& b:spaces[k-1])if(stable(b,a)&&isotropic(b,j)){++counts[k-1];if(k==3)max.push_back(b);}
        require(row.at("kernel_counts").arr.size()==3,"kernel count shape");for(int k=0;k<3;++k)require(counts[k]==row.at("kernel_counts").arr[k].integer(),"kernel count mismatch "+key);
        ++rows;nonzero+=!max.empty();total+=max.size();
        if(family==192&&candidate==139&&d==7){p192=int(max.size());int id=0;for(auto& b:max){auto es=elements(b);require(es.size()==27,"kernel cardinality");std::set<V> unique(es.begin(),es.end());require(unique.size()==27,"kernel rank");int ei=0,bi=0,fix=0;
            for(auto v:es){require(unique.count(act(v,a)),"pointwise stability");bool inE=true,inB=true;for(int c=2;c<6;++c)inE&=v[c]==0;for(int c=0;c<2;++c)inB&=v[c]==0;ei+=inE;bi+=inB;fix+=act(v,a)==v;for(auto w:es)require(dot(act(v,j),w)==0,"pointwise isotropy");}
            int field=0;M ap=ident();do{ap=mul(ap,a);++field;}while(!std::all_of(b.begin(),b.end(),[&](V v){return act(v,ap)==v;}));
            require(ei==1&&bi==3,"unexpected mixed intersections");
            std::cout.rdbuf(kernels.rdbuf());std::cout<<"{\"kernel\":"<<++id<<",\"rref_basis\":";print_basis(b);std::cout<<",\"point_field_degree\":"<<field<<",\"rational_elements\":"<<fix<<",\"E_intersection_size\":"<<ei<<",\"Prym_intersection_size\":"<<bi<<"}\n";std::cout.rdbuf(old);
        }}
        std::cout<<"{\"family\":"<<family<<",\"candidate\":"<<candidate<<",\"d\":"<<d<<",\"counts\":["<<counts[0]<<','<<counts[1]<<','<<counts[2]<<"],\"frobenius_order\":"<<order<<",\"multiplier\":"<<multiplier<<"}\n";
    }}require(rows==306,"missing module rows");require(p192==8,"P192 d7 not found");std::cerr<<"PASS modules="<<rows<<" with_maximal_kernel="<<nonzero<<" maximal_kernels="<<total<<" p192_d7="<<p192<<"\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
