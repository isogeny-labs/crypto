// Fixed-degree rational preimages and bounded generated auxiliary setup.
#pragma once
#define AUXILIARY_COLD_NO_MAIN
#include "../projective_aux_norm_20261115/cold.cpp"

namespace halving {
using reference::Curve;
using reference::Point;
using auxiliary::Tower;
using auxiliary::check;
using Pair=std::pair<int,int>;
constexpr std::size_t curve_cap=4096,coset_cap=64,proposal_cap=128;

bool nonsingular(const Curve&E) {
    return E.residue(4LL*E.a*E.a*E.a+27LL*E.b*E.b)!=0;
}
void admit(const Curve&E) {
    check(E.p>=5&&E.p<=257&&reference::isprime(E.p),"bounded prime required");
    check(E.a>=0&&E.a<E.p&&E.b>=0&&E.b<E.p&&nonsingular(E),"canonical nonsingular curve required");
}
std::string point_text(Point P) {
    return P.infinity?"-":std::to_string(P.x)+":"+std::to_string(P.y);
}
std::string points_text(const std::vector<Point>&points) {
    if(points.empty())return "-";
    std::string out;
    for(const auto&P:points){if(!out.empty())out+=';';out+=point_text(P);}
    return out;
}
std::string roots_text(const std::set<int>&roots) {
    std::string out;
    for(int x:roots){if(!out.empty())out+=',';out+=std::to_string(x);}
    return out;
}
// The candidate receives only E,P. The census driver below is not called here.
std::vector<Point> preimages(const Curve&E,Point P) {
    admit(E);
    check(!P.infinity&&P.x>=0&&P.x<E.p&&P.y>=0&&P.y<E.p&&E.contains(P),
          "canonical finite on-curve target required");
    const coeff::Poly f{
        E.residue(1LL*E.a*E.a-4LL*E.b*P.x),
        E.residue(-8LL*E.b-4LL*E.a*P.x),
        E.residue(-2LL*E.a),E.residue(-4LL*P.x),1};
    std::set<Pair> unique;
    for(int x:algebraic::roots(f,E.p)){
        const int rhs=E.residue(1LL*x*x*x+1LL*E.a*x+E.b);
        for(int y:algebraic::roots(coeff::Poly{E.residue(-rhs),0,1},E.p)){
            const Point H{x,y,false};
            check(E.contains(H),"halving lift is off curve");
            if(reference::equal(E.sum(H,H),P))unique.emplace(x,y);
        }
    }
    check(unique.size()<=4,"multiplication-by-two fibre degree");
    std::vector<Point> out;
    for(const auto&v:unique)out.push_back({v.first,v.second,false});
    return out;
}
class Random {
    std::mt19937_64 engine;
    std::ostream&events;
public:
    Random(std::uint64_t seed,std::ostream&stream):engine(seed),events(stream){}
    std::uint64_t bounded(std::uint64_t size,const char*domain) {
        check(size>0,"empty sampling domain");
        const auto threshold=(std::uint64_t(0)-size)%size;
        for(;;){
            const auto raw=engine();const bool accepted=raw>=threshold;
            const auto value=accepted?raw%size:0;
            events<<"D "<<domain<<' '<<size<<' '<<raw<<' '<<accepted<<' '<<value<<'\n';
            if(accepted)return value;
        }
    }
};
std::vector<Point> main_base(const Tower&tower) {
    check(tower.initial.p==19,"main-base control field");
    const Curve E{19,0,2};std::vector<Point> base;
    for(int x:tower.fibre()){
        const int rhs=E.residue(x*x*x+2);
        for(int y:algebraic::roots(coeff::Poly{E.residue(-rhs),0,1},19))
            if(y<=9)base.push_back({x,y,false});
    }
    return base;
}
struct Setup {Tower tower;std::size_t trials;};
class Sampler {
    int p;bool filter;
    std::ostream&events;
    Random random;
    Point sample_point(const Curve&E,std::size_t slot) {
        for(std::size_t proposal=0;proposal<proposal_cap;++proposal){
            const auto u=random.bounded(2*p+1,"G");
            if(u==static_cast<std::uint64_t>(2*p)){
                events<<"P "<<slot<<' '<<proposal<<' '<<u<<" i -1 -1\n";return {};
            }
            const int x=static_cast<int>(u/2),sign=static_cast<int>(u%2);
            const int rhs=E.residue(1LL*x*x*x+1LL*E.a*x+E.b);
            const auto ys=algebraic::roots(coeff::Poly{E.residue(-rhs),0,1},p);
            if(ys.empty()){events<<"P "<<slot<<' '<<proposal<<' '<<u<<" n "<<x<<" -1\n";continue;}
            const std::vector<int> ordered(ys.begin(),ys.end());
            if(ordered.size()==1&&sign==1){
                check(ordered[0]==0,"unexpected repeated quadratic root");
                events<<"P "<<slot<<' '<<proposal<<' '<<u<<" z "<<x<<" 0\n";continue;
            }
            const int y=ordered.at(sign);
            events<<"P "<<slot<<' '<<proposal<<' '<<u<<" p "<<x<<' '<<y<<'\n';
            return {x,y,false};
        }
        throw std::runtime_error("point proposal cap");
    }
public:
    Sampler(int prime,std::uint64_t seed,bool cold_filter,std::ostream&stream)
        :p(prime),filter(cold_filter),events(stream),random(seed,stream) {
        check(p>=5&&p<=257&&reference::isprime(p)&&(!filter||p==19),"sampler field admission");
        events<<"HALVING-SETUP-1 "<<p<<' '<<seed<<' '<<filter<<'\n';
    }
    Setup generate() {
        for(std::size_t trial=0;trial<curve_cap;++trial){
            const int a=static_cast<int>(random.bounded(p,"A"));
            const int x=static_cast<int>(random.bounded(p,"X"));
            Curve E{p,a,0};E.b=E.residue(-1LL*x*x*x-1LL*a*x);
            const bool good=nonsingular(E);
            events<<"C "<<trial<<' '<<a<<' '<<x<<' '<<E.b<<' '<<good<<'\n';
            if(!good)continue;
            Point T{x,0,false};bool dead=false;
            for(unsigned stage=0;stage<2;++stage){
                const auto halves=preimages(E,T);
                events<<"H "<<stage<<' '<<T.x<<' '<<T.y<<' '<<halves.size()<<' '<<points_text(halves)<<'\n';
                if(halves.empty()){dead=true;break;}
                const auto selected=random.bounded(halves.size(),stage==0?"H0":"H1");
                T=halves.at(selected);
                events<<"K "<<stage<<' '<<selected<<' '<<T.x<<' '<<T.y<<'\n';
            }
            if(dead)continue;
            check(algebraic::multiply(E,T,8).infinity&&!algebraic::multiply(E,T,4).infinity,
                  "generated exact order8");
            events<<"T "<<T.x<<' '<<T.y<<'\n';
            for(std::size_t slot=0;slot<coset_cap;++slot){
                const Point G=sample_point(E,slot);
                std::optional<Tower> tower;
                try{tower.emplace(E,T,G);}
                catch(const std::runtime_error&e){
                    if(std::string(e.what())!="auxiliary terminal/trace admission")throw;
                }
                events<<"R "<<slot<<' '<<(G.infinity?-1:G.x)<<' '<<(G.infinity?-1:G.y)<<' '<<bool(tower)<<'\n';
                if(!tower)continue;
                const auto fibre=tower->fibre();
                if(filter){
                    const auto base=main_base(*tower);
                    events<<"B "<<slot<<' '<<base.size()<<' '<<(base.size()==3)<<'\n';
                    if(base.size()!=3)continue;
                }
                events<<"F "<<E.a<<' '<<E.b<<' '<<T.x<<' '<<T.y<<' '<<G.x<<' '<<G.y<<' '<<roots_text(fibre)<<'\n';
                events.flush();check(bool(events),"setup event write");
                return {*tower,trial+1};
            }
        }
        events<<"U curve_cap\n";events.flush();
        throw std::runtime_error("curve trial cap");
    }
};
void receipt(const Setup&setup,const std::filesystem::path&root) {
    const auto&t=setup.tower;
    std::ofstream out(root/"setup.tsv");
    out<<"p\ta\tb\ttx\tty\tgx\tgy\tterminal\tfibre\tcurve_trials\n";
    out<<t.initial.p<<'\t'<<t.initial.a<<'\t'<<t.initial.b<<'\t'<<t.generator.x<<'\t'<<t.generator.y<<'\t'
       <<t.coset.x<<'\t'<<t.coset.y<<'\t'<<t.terminal<<'\t'<<roots_text(t.fibre())<<'\t'<<setup.trials<<'\n';
    out.close();check(bool(out),"setup receipt write");
}
} // namespace halving
