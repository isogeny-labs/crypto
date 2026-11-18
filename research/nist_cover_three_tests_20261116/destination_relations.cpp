#include "point_arithmetic.hpp"
#include <array>
#include <map>

template<class Fn> void rejects(Fn test,const std::string& label){
    bool caught=false;
    try{test();}catch(const std::exception&){caught=true;}
    require(caught,"control accepted: "+label);
}

Z zpow(Z a,int n){Z r(1);for(int i=0;i<n;++i)r=r*a;return r;}
void split_linear_part(Poly<Z> f,std::vector<Z>& roots){
    f=f.monic();if(f.degree()<1)return;
    if(f.degree()==1){roots.push_back(-f.at(0));return;}
    Poly<Z> x{Z(0),Z(1)};
    for(int shift=0;shift<64;++shift){
        auto h=powmod(x+Poly<Z>{Z(shift)},mpz_class((prime-1)/2),f)-Poly<Z>{Z(1)};
        auto[g,s,t]=xgcd(f,h);
        if(g.degree()>0&&g.degree()<f.degree()){split_linear_part(g,roots);split_linear_part(f.exact(g),roots);return;}
    }
    throw std::runtime_error("linear-factor splitting exceeded fixed64-shift bound");
}
struct BasePoint {int x;Z y;};
struct Fiber {int x;std::vector<int> support;};
using Row=std::vector<mpz_class>;
int coefficient_rank(std::vector<Row> a,const mpz_class& r){
    if(a.empty())return 0;
    int rank=0,width=int(a[0].size());
    for(int j=0;j<width&&rank<int(a.size());++j){
        int pick=rank;while(pick<int(a.size())&&a[pick][j]==0)++pick;if(pick==int(a.size()))continue;
        std::swap(a[pick],a[rank]);mpz_class inv;require(mpz_invert(inv.get_mpz_t(),a[rank][j].get_mpz_t(),r.get_mpz_t())!=0,"coefficient inverse");
        for(int k=j;k<width;++k){a[rank][k]=(a[rank][k]*inv)%r;if(a[rank][k]<0)a[rank][k]+=r;}
        for(int i=rank+1;i<int(a.size());++i){mpz_class c=a[i][j];for(int k=j;k<width;++k){a[i][k]=(a[i][k]-c*a[rank][k])%r;if(a[i][k]<0)a[i][k]+=r;}}
        ++rank;
    }
    return rank;
}
int main(int argc,char** argv){try{
    require(argc==2,"usage: destination_relations candidate-model.json");auto model=read_json(argv[1]);prime=mpz_class(model.at("p").scalar);
    require(prime==mpz_class("6277101735386680763835789423207666416083908700390324961279"),"exact P192 field");
    std::map<std::array<int,3>,Z> curve_coefficients;
    for(auto& term:model.at("terms").arr){std::array<int,3> e{};for(int j=0;j<3;++j)e[j]=term.at("exponents").arr.at(j).integer();require(e[0]+e[1]+e[2]==4,"homogeneous quartic degree");require(curve_coefficients.emplace(e,Z(mpz_class(term.at("coefficient").scalar))).second,"unique quartic monomial");}
    auto fiber_polynomial=[&](int x){std::vector<Z> c(5);for(auto& [e,v]:curve_coefficients)c[e[1]]=c[e[1]]+v*zpow(Z(x),e[0]);return Poly<Z>(c);};
    std::vector<BasePoint> base;std::vector<Fiber> split;Poly<Z> y{Z(0),Z(1)};int factorizations=0;
    for(int x=0;x<256;++x){
        auto f=fiber_polynomial(x);require(f.degree()==4,"declared degree4 vertical chart");auto[g,s,t]=xgcd(f,powmod(y,prime,f)-y);std::vector<Z> roots;split_linear_part(g,roots);
        std::sort(roots.begin(),roots.end(),[](const Z&a,const Z&b){return a.x<b.x;});Fiber fiber{x,{}};
        for(auto& root:roots){require(f.eval(root)==Z(0),"destination point equation");fiber.support.push_back(int(base.size()));base.push_back({x,root});}
        require(int(roots.size())==g.degree(),"all rational linear factors");
        if(roots.size()==4){Poly<Z> product{f.c.back()};for(auto& root:roots)product=product*Poly<Z>{-root,Z(1)};require(product==f,"exact full split fiber factorization");split.push_back(fiber);++factorizations;}
    }
    require(base.size()>=85&&split.size()>=2,"fixed point-pool and split-line coverage");
    std::vector<Row> relations;for(size_t i=1;i<split.size();++i){Row row(base.size());for(int j:split[i].support)row[j]+=1;for(int j:split[0].support)row[j]-=1;relations.push_back(row);}
    mpz_class r("6277101735386680763835789423176059013767194773182842284081");int rank=coefficient_rank(relations,r);require(rank==int(relations.size()),"disjoint-fiber coefficient rank");
    auto& first=base[split[0].support[0]];
    rejects([&]{require(fiber_polynomial(first.x).eval(first.y+Z(1))==Z(0),"corrupted destination point");},"corrupted destination point");
    rejects([&]{Row bad=relations[0];bad[split[0].support[0]]+=1;Row expected(base.size());for(int j:split[1].support)expected[j]+=1;for(int j:split[0].support)expected[j]-=1;require(bad==expected,"corrupted principal support row");},"corrupted relation row");
    std::cout<<"{\"scope\":\"P192-kernel1-candidate-destination-vertical-relations\",\"x_window\":[0,255],\"factor_base_points\":"<<base.size()<<",\"fully_split_fibers\":"<<factorizations<<",\"principal_relations\":"<<relations.size()<<",\"coefficient_rank_mod_source_r\":"<<rank<<",\"source_image_bound\":false,\"base_logs_computed\":false,\"DLP_solved\":false,\"rejected_controls\":2,\"standing\":\"producer-unverified\",\"points\":[";
    for(size_t i=0;i<base.size();++i){if(i)std::cout<<',';std::cout<<"{\"x\":\""<<base[i].x<<"\",\"y\":\""<<base[i].y.x<<"\"}";}
    std::cout<<"],\"split_fibers\":[";for(size_t i=0;i<split.size();++i){if(i)std::cout<<',';std::cout<<"{\"x\":"<<split[i].x<<",\"point_indices\":[";for(int j=0;j<4;++j)std::cout<<(j?",":"")<<split[i].support[j];std::cout<<"]}";}
    std::cout<<"],\"relations\":[";for(size_t i=0;i<relations.size();++i){if(i)std::cout<<',';std::cout<<"{\"zero_fiber_x\":"<<split[i+1].x<<",\"pole_fiber_x\":"<<split[0].x<<",\"entries\":[";bool first_entry=true;for(size_t j=0;j<relations[i].size();++j)if(relations[i][j]!=0){if(!first_entry)std::cout<<',';first_entry=false;std::cout<<'['<<j<<','<<relations[i][j]<<']';}std::cout<<"]}";}std::cout<<"]}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
