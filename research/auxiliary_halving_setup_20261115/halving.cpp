#include "halving.hpp"
int main(int argc,char**argv) {
    try {
        if(argc==2&&std::string(argv[1])=="--selftest"){
            auxiliary::selftest(auxiliary::Tower{},std::cout);
            const reference::Curve E{19,0,2};std::size_t rejected=0;
            auto reject=[&](reference::Curve curve,reference::Point point){
                try{halving::preimages(curve,point);}
                catch(const std::exception&){++rejected;return;}
                throw std::runtime_error("invalid halving input accepted");
            };
            reject(E,{});reject(E,{0,0,false});reject(E,{23,3,false});
            reject({19,0,0},{0,0,false});reject({9,0,2},{4,3,false});
            halving::check(rejected==5,"negative control census");
            std::cout<<"PASS original defaults and5 halving admission controls\n";return 0;
        }
        if(argc==3&&std::string(argv[1])=="--census"){
            const auto root=std::filesystem::absolute(argv[2]);
            halving::check(std::filesystem::create_directory(root),"reused census directory");
            std::ofstream out(root/"preimages.tsv");
            out<<"p\ta\tb\tpx\tpy\thalves\n";std::size_t cases=0;
            // Outer public control census. Not reachable from the setup candidate.
            for(int p:{5,7,11,13,17,19})for(int a=0;a<p;++a)for(int b=0;b<p;++b){
                const reference::Curve E{p,a,b};if(!halving::nonsingular(E))continue;
                for(const auto&P:E.points()){
                    const auto halves=halving::preimages(E,P);
                    out<<p<<'\t'<<a<<'\t'<<b<<'\t'<<P.x<<'\t'<<P.y<<'\t'<<halving::points_text(halves)<<'\n';++cases;
                }
            }
            out.close();halving::check(bool(out),"census write");
            std::cout<<"PASS native complete preimage cases="<<cases<<'\n';return 0;
        }
        if(argc==6&&std::string(argv[1])=="--setup"){
            const int p=reference::read_int(argv[2]);
            const auto seed=canonical::unsigned_decimal(argv[3]);
            halving::check(std::string(argv[4])=="unfiltered","unknown setup mode");
            const auto root=std::filesystem::absolute(argv[5]);
            halving::check(std::filesystem::create_directory(root),"reused setup directory");
            std::ofstream events(root/"events.txt");
            halving::Sampler sampler(p,seed,false,events);const auto setup=sampler.generate();
            halving::receipt(setup,root);
            std::cout<<"PASS generated auxiliary setup p="<<p<<" trials="<<setup.trials<<'\n';return 0;
        }
        throw std::invalid_argument("usage: halving --selftest | --census NEW_ROOT | --setup P SEED unfiltered NEW_ROOT");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
