// Independent OpenSSL census and sampler replay. No candidate source included.
#define AUXILIARY_REPLAY_NO_MAIN
#include "../projective_aux_norm_20261115/replay.cpp"
#include <random>
#include <memory>
#include <optional>
namespace audit {
void require(bool condition,const std::string&message){if(!condition)throw std::runtime_error(message);}
int reduce(long long value,int p){value%=p;return static_cast<int>(value<0?value+p:value);}
std::string point_text(Point P){return std::to_string(P.first)+":"+std::to_string(P.second);}
std::string points_text(const std::vector<Point>&points){
    if(points.empty())return "-";
    std::string out;for(Point P:points){if(!out.empty())out+=';';out+=point_text(P);}return out;
}
std::string roots_text(const std::set<int>&roots){
    std::string out;for(int x:roots){if(!out.empty())out+=',';out+=std::to_string(x);}return out;
}
struct Curve {
    AuxiliaryGroup group;
    std::vector<Point> points;
    std::map<Point,std::vector<Point>> preimages;
    std::map<int,std::vector<int>> ordinates;
    Curve(int p,int a,int b):group(a,b,p),points(group.all()){
        require(EC_GROUP_check_discriminant(group.group,group.ctx)==1,"OpenSSL discriminant");
        for(Point Q:points){
            ordinates[Q.first].push_back(Q.second);
            const auto point=group.point(Q);auto twice=group.fresh();
            require(EC_POINT_dbl(group.group,twice.get(),point.get(),group.ctx)==1,"OpenSSL doubling");
            if(EC_POINT_is_at_infinity(group.group,twice.get())!=1)
                preimages[group.coordinates(twice.get())].push_back(Q);
        }
    }
    ECPoint multiply(Point P,unsigned scalar){
        const auto point=group.point(P);auto out=group.fresh();auto n=BN_new();BN_set_word(n,scalar);
        const int ok=EC_POINT_mul(group.group,out.get(),nullptr,point.get(),n,group.ctx);BN_free(n);
        require(ok==1,"OpenSSL scalar multiplication");return out;
    }
    void exact_order8(Point T){
        const auto four=multiply(T,4),eight=multiply(T,8);
        require(EC_POINT_is_at_infinity(group.group,four.get())==0
            &&EC_POINT_is_at_infinity(group.group,eight.get())==1,"OpenSSL exact order8");
    }
    bool trace_good(Point T,Point G){
        if(G==Point{-1,-1})return false;
        const auto two=multiply(G,2);
        for(unsigned i=0;i<8;++i){
            const auto H=multiply(T,i);if(group.equal(two.get(),H.get()))return false;
        }
        return true;
    }
};
std::string read(const std::filesystem::path&path){
    std::ifstream in(path,std::ios::binary);require(bool(in),"missing replay input "+path.string());
    return {std::istreambuf_iterator<char>(in),{}};
}
class Random {
    std::mt19937_64 engine;
    std::ostream&events;
public:
    Random(std::uint64_t seed,std::ostream&stream):engine(seed),events(stream){}
    std::uint64_t sample(std::uint64_t size,const char*domain){
        // Protocol sampling law, independent of any group/preimage computation.
        const auto threshold=(std::uint64_t(0)-size)%size;
        for(;;){
            const auto raw=engine();const bool accepted=raw>=threshold;const auto value=accepted?raw%size:0;
            events<<"D "<<domain<<' '<<size<<' '<<raw<<' '<<accepted<<' '<<value<<'\n';
            if(accepted)return value;
        }
    }
};
struct Setup {int p,a,b;Point T,G;std::set<int> roots;std::size_t trials;};
std::optional<Setup> simulate(int p,std::uint64_t seed,bool filter,std::ostream&events){
    events<<"HALVING-SETUP-1 "<<p<<' '<<seed<<' '<<filter<<'\n';Random random(seed,events);
    std::vector<Point> main_points;
    if(filter){AuxiliaryGroup main(0,2);main_points=main.all();}
    for(std::size_t trial=0;trial<4096;++trial){
        const int a=static_cast<int>(random.sample(p,"A")),x=static_cast<int>(random.sample(p,"X"));
        const int b=reduce(-1LL*x*x*x-1LL*a*x,p);
        const bool good=reduce(4LL*a*a*a+27LL*b*b,p)!=0;
        events<<"C "<<trial<<' '<<a<<' '<<x<<' '<<b<<' '<<good<<'\n';
        if(!good)continue;
        Curve curve(p,a,b);Point T{x,0};bool dead=false;
        for(unsigned stage=0;stage<2;++stage){
            const auto found=curve.preimages.find(T);
            const auto halves=found==curve.preimages.end()?std::vector<Point>{}:found->second;
            events<<"H "<<stage<<' '<<T.first<<' '<<T.second<<' '<<halves.size()<<' '<<points_text(halves)<<'\n';
            if(halves.empty()){dead=true;break;}
            const auto chosen=random.sample(halves.size(),stage==0?"H0":"H1");T=halves.at(chosen);
            events<<"K "<<stage<<' '<<chosen<<' '<<T.first<<' '<<T.second<<'\n';
        }
        if(dead)continue;
        curve.exact_order8(T);events<<"T "<<T.first<<' '<<T.second<<'\n';
        for(std::size_t slot=0;slot<64;++slot){
            Point G{-1,-1};bool accepted=false;
            for(std::size_t proposal=0;proposal<128;++proposal){
                const auto u=random.sample(2*p+1,"G");
                if(u==static_cast<std::uint64_t>(2*p)){
                    events<<"P "<<slot<<' '<<proposal<<' '<<u<<" i -1 -1\n";accepted=true;break;
                }
                const int gx=static_cast<int>(u/2),sign=static_cast<int>(u%2);
                const auto found=curve.ordinates.find(gx);
                if(found==curve.ordinates.end()){
                    events<<"P "<<slot<<' '<<proposal<<' '<<u<<" n "<<gx<<" -1\n";continue;
                }
                if(found->second.size()==1&&sign==1){
                    require(found->second[0]==0,"OpenSSL duplicate zero sign");
                    events<<"P "<<slot<<' '<<proposal<<' '<<u<<" z "<<gx<<" 0\n";continue;
                }
                G={gx,found->second.at(sign)};
                events<<"P "<<slot<<' '<<proposal<<' '<<u<<" p "<<G.first<<' '<<G.second<<'\n';accepted=true;break;
            }
            if(!accepted)return std::nullopt;
            const bool trace=curve.trace_good(T,G);
            events<<"R "<<slot<<' '<<G.first<<' '<<G.second<<' '<<trace<<'\n';
            if(!trace)continue;
            const auto roots=curve.group.fibre(T,G);
            require(roots.size()==8,"OpenSSL eight distinct coset abscissas");
            if(filter){
                std::size_t count=0;for(Point P:main_points)if(P.second<=9&&roots.count(P.first))++count;
                events<<"B "<<slot<<' '<<count<<' '<<(count==3)<<'\n';if(count!=3)continue;
            }
            events<<"F "<<a<<' '<<b<<' '<<T.first<<' '<<T.second<<' '<<G.first<<' '<<G.second<<' '<<roots_text(roots)<<'\n';
            return Setup{p,a,b,T,G,roots,trial+1};
        }
    }
    events<<"U curve_cap\n";return std::nullopt;
}
std::optional<Setup> setup_replay(const std::filesystem::path&root,int status){
    const auto actual=read(root/"events.txt");std::istringstream header(actual);
    std::string magic;int p=0,filter=0;std::uint64_t seed=0;
    require(bool(header>>magic>>p>>seed>>filter)&&magic=="HALVING-SETUP-1"&&seed==1234
            &&(filter==0||filter==1),"setup header/seed binding");
    const std::set<int> fields{19,23,31,43,59,83,127,251};
    require(fields.count(p)&&(!filter||p==19),"frozen setup field binding");
    std::ostringstream expected;const auto setup=simulate(p,seed,filter!=0,expected);
    if(status==0){
        require(bool(setup)&&actual==expected.str(),"complete OpenSSL setup transcript mismatch");
        const auto rows=table(root/"setup.tsv","p\ta\tb\ttx\tty\tgx\tgy\tterminal\tfibre\tcurve_trials");
        require(rows.size()==1&&rows[0].size()==10,"setup receipt dimensions");const auto&r=rows[0];
        require(number(r[0])==setup->p&&number(r[1])==setup->a&&number(r[2])==setup->b
            &&number(r[3])==setup->T.first&&number(r[4])==setup->T.second
            &&number(r[5])==setup->G.first&&number(r[6])==setup->G.second
            &&number(r[7])<p&&r[8]==roots_text(setup->roots)
            &&static_cast<std::size_t>(number(r[9]))==setup->trials,"setup receipt binding");
        std::cout<<"PASS OpenSSL complete generated setup p="<<p<<" trials="<<setup->trials<<'\n';
        return setup;
    }
    require(status==2||status==124||status==137,"unexpected unknown status");
    require(expected.str().compare(0,actual.size(),actual)==0,"invalid failed setup transcript prefix");
    std::cout<<"PASS OpenSSL setup prefix UNKNOWN p="<<p<<'\n';return std::nullopt;
}
void census_replay(const std::filesystem::path&path){
    std::ostringstream expected;expected<<"p\ta\tb\tpx\tpy\thalves\n";std::size_t cases=0;
    for(int p:{5,7,11,13,17,19})for(int a=0;a<p;++a)for(int b=0;b<p;++b){
        if(reduce(4LL*a*a*a+27LL*b*b,p)==0)continue;
        Curve curve(p,a,b);
        for(Point P:curve.points){
            const auto found=curve.preimages.find(P);
            const auto halves=found==curve.preimages.end()?std::vector<Point>{}:found->second;
            expected<<p<<'\t'<<a<<'\t'<<b<<'\t'<<P.first<<'\t'<<P.second<<'\t'<<points_text(halves)<<'\n';++cases;
        }
    }
    require(read(path)==expected.str(),"complete OpenSSL preimage census mismatch");
    std::cout<<"PASS OpenSSL complete preimage cases="<<cases<<'\n';
}
} // namespace audit
int main(int argc,char**argv){
    try{
        if(argc==3&&std::string(argv[1])=="--census"){audit::census_replay(argv[2]);return 0;}
        if(argc==4&&std::string(argv[1])=="--setup"){
            audit::setup_replay(argv[2],number(argv[3]));return 0;
        }
        if(argc==4&&std::string(argv[1])=="--cold"){
            const auto setup=audit::setup_replay(argv[2],0);audit::require(bool(setup)&&setup->p==19,"cold setup binding");
            AuxiliaryGroup main(0,2);std::vector<Point> base;
            for(Point P:main.all())if(P.second<=9&&setup->roots.count(P.first))base.push_back(P);
            audit::require(base.size()==3,"generated cold base dimension");
            cold_replay_rows(argv[2],argv[3],main,base,19,13,std::set<Point>{{18,1}},
                [&](int x){return setup->roots.count(x)!=0;});
            return 0;
        }
        throw std::invalid_argument("usage: replay --census FILE | --setup ROOT STATUS | --cold ROOT SUMMARY");
    }catch(const std::exception&e){std::cerr<<"REJECT: "<<e.what()<<'\n';return 2;}
}
