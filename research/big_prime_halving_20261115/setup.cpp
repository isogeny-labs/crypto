#include "big.hpp"
int main(int argc,char**argv){
    using namespace big;
    try{
        if(argc==2&&std::string(argv[1])=="--selftest"){
            unsigned rejected=0;
            auto reject=[&](auto action){try{action();}catch(const std::exception&){++rejected;return;}need(false,"invalid input accepted");};
            reject([]{parse("019");});reject([]{parse("-7");});reject([]{Field f(Z(9));});reject([]{Field f(Z(3));});
            const Field f(Z(19));const Curve e{&f,0,2};
            reject([&]{preimages(e,{});});reject([&]{preimages(e,{0,0,false});});
            reject([&]{preimages(e,{23,3,false});});reject([&]{preimages(Curve{&f,0,0},{0,0,false});});
            reject([&]{std::ostringstream out;setup(f,2,1234,out);});
            reject([&]{std::ostringstream out;setup(f,13,1234,out);});
            need(rejected==10,"admission control count");
            const auto good=preimages(e,{4,3,false});need(!good.halves.empty(),"positive preimage control");
            std::cout<<"PASS ten rejected admissions and positive finite preimages\n";return 0;
        }
        if(argc==2&&std::string(argv[1])=="--fixtures"){
            for(unsigned p:{19,23,31,43,59,83,127,251})std::cout<<p<<" 3 1234\n";
            std::vector<Z> primes;
            for(unsigned bits:{61,127,255,256,521}){Z p=1;mpz_mul_2exp(p.get_mpz_t(),p.get_mpz_t(),bits);
                if(bits==255)p-=19;else if(bits==256){Z two32=1;mpz_mul_2exp(two32.get_mpz_t(),two32.get_mpz_t(),32);p-=two32+977;}else p-=1;
                primes.push_back(p);
            }
            for(const Z&p:primes)for(unsigned t:{3,4})std::cout<<p<<' '<<t<<" 1234\n";
            return 0;
        }
        if(argc==3&&std::string(argv[1])=="--census"){
            need(!std::filesystem::exists(argv[2]),"reused census output");
            std::ofstream out(argv[2]);need(bool(out),"census output open");
            out<<"BIG-PREIMAGES-1\n";unsigned cases=0;
            for(unsigned p:{5,7,11,13,17,19}){
                Field f{Z(p)};
                for(unsigned a=0;a<p;++a)for(unsigned b=0;b<p;++b){Curve e{&f,Z(a),Z(b)};if(!e.good())continue;
                    for(unsigned x=0;x<p;++x)for(const Z&y:f.sqrt(e.rhs(Z(x)))){
                        const Point q{Z(x),y,false};const auto h=preimages(e,q);
                        out<<p<<' '<<a<<' '<<b<<' '<<pt(q)<<' '<<h.xs.size()<<' '<<zs(h.xs)<<' '<<h.halves.size()<<' '<<pts(h.halves)<<'\n';++cases;
                    }
                }
            }
            out.close();need(bool(out)&&cases==14754,"census count/write");std::cout<<"PASS census cases="<<cases<<'\n';return 0;
        }
        if(argc==6&&std::string(argv[1])=="--setup"){
            const Field f(parse(argv[2]));const Z tz=parse(argv[3]),seedz=parse(argv[4]);
            need(tz>=3&&tz<=12&&seedz<=std::numeric_limits<std::uint64_t>::max(),"setup argument admission");
            const auto root=std::filesystem::absolute(argv[5]);need(std::filesystem::create_directory(root),"reused setup directory");
            std::ofstream events(root/"events.txt");events.setf(std::ios::unitbuf);
            const auto s=setup(f,static_cast<unsigned>(tz.get_ui()),static_cast<std::uint64_t>(seedz.get_ui()),events);
            std::ofstream receipt(root/"setup.tsv");
            receipt<<"p\ta\tb\ttx\tty\tgx\tgy\tterminal\tfibre\tcurve_trials\n";
            receipt<<f.p<<'\t'<<s.e.a<<'\t'<<s.e.b<<'\t'<<s.t.x<<'\t'<<s.t.y<<'\t'<<s.g.x<<'\t'<<s.g.y<<'\t'
                   <<s.terminal<<'\t'<<zs(s.fibre)<<'\t'<<s.trials<<'\n';
            receipt.close();need(bool(receipt),"setup receipt write");
            std::cout<<"PASS generated setup bits="<<mpz_sizeinbase(f.p.get_mpz_t(),2)<<" t="<<tz<<" trials="<<s.trials<<'\n';return 0;
        }
        throw std::runtime_error("usage: setup --selftest | --fixtures | --census NEW_FILE | --setup P T SEED NEW_ROOT");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
