#include "common.hpp"
#include <unistd.h>
namespace mca {
using Weights=std::map<std::size_t,U>;
struct Pivot{Polynomial poly;Weights weights;};
struct Stats{std::size_t rows=0,steps=0,entries=0,pivots=0;};
void scale(Weights&w,U factor,U p){for(auto&i:w)i.second=i.second*factor%p;}
void subtract(Weights&a,const Weights&b,U factor,U p){
    for(const auto&i:b){const U value=(a[i.first]+p-factor*i.second%p)%p;
        if(value)a[i.first]=value;else a.erase(i.first);}
}
class Producer{
    const System&s;unsigned degree;Stats&stats;std::vector<RowDefinition> rows;
    std::map<Monomial,Pivot> pivots;
    std::vector<std::size_t> original_ids;
    void emit(std::size_t eq,const Monomial&m){
        require(rows.size()<row_cap,"input-multiple count cap");const auto id=rows.size();rows.push_back({eq,m});++stats.rows;
        if(canonical::degree(m)==0)original_ids.at(eq)=id;
        Pivot row{multiple(s.equations[eq],m),{{id,1}}};
        while(!row.poly.terms.empty()){
            require(stats.entries+row.poly.terms.size()+row.weights.size()<=entry_cap,"live-entry cap");
            const auto lm=quotient::lead(row.poly);const auto found=pivots.find(lm);
            if(found==pivots.end()){
                const U inv=quotient::power(row.poly.terms.at(lm),s.p-2,s.p);
                row.poly=row.poly.times(Polynomial::constant(s.names.size(),s.p,inv));scale(row.weights,inv,s.p);
                stats.entries+=row.poly.terms.size()+row.weights.size();pivots.emplace(lm,std::move(row));++stats.pivots;return;
            }
            require(++stats.steps<=step_cap,"row-reduction step cap");const U c=row.poly.terms.at(lm);
            row.poly=row.poly.plus(found->second.poly.times(Polynomial::constant(s.names.size(),s.p,c)),true);
            subtract(row.weights,found->second.weights,c,s.p);
        }
    }
    void enumerate(std::size_t eq,Monomial&m,std::size_t at,unsigned remaining){
        if(at==m.size()){emit(eq,m);return;}
        for(unsigned e=0;e<=remaining;++e){m[at]=e;enumerate(eq,m,at+1,remaining-e);}m[at]=0;
    }
public:
    Producer(const System&input,unsigned d,Stats&counts):s(input),degree(d),stats(counts),original_ids(input.equations.size(),row_cap){}
    void run(const std::filesystem::path&basis,const std::filesystem::path&certificate){
        for(std::size_t i=0;i<s.equations.size();++i){const auto&f=s.equations[i];if(f.terms.empty())continue;
            require(f.largest_degree()<=degree,"input degree exceeds cap");Monomial m(s.names.size());enumerate(i,m,0,degree-f.largest_degree());}
        std::vector<const Pivot*> selected;
        for(const auto&entry:pivots){bool minimal=true;for(const auto&other:pivots){
            if(entry.first!=other.first&&quotient::divides(other.first,entry.first)){minimal=false;break;}}
            if(minimal)selected.push_back(&entry.second);}
        std::vector<Pivot> originals;originals.reserve(s.equations.size());
        for(std::size_t i=0;i<s.equations.size();++i){const auto&f=s.equations[i];if(f.terms.empty())continue;
            bool present=false;for(const auto*row:selected)if(row->poly.terms==f.terms){present=true;break;}
            if(present)continue;
            require(original_ids.at(i)<rows.size(),"missing original source row");
            originals.push_back({f,{{original_ids[i],1}}});selected.push_back(&originals.back());}
        require(selected.size()<=basis_cap,"candidate basis count cap");
        std::map<std::size_t,std::size_t> used;
        for(const auto*row:selected)for(const auto&w:row->weights)used.emplace(w.first,0);
        std::size_t compact=0;for(auto&entry:used)entry.second=compact++;
        std::ostringstream cert;cert<<"MCAULAY-CERT-1 "<<s.p<<' '<<s.names.size()<<' '<<degree<<' '<<used.size()<<' '<<selected.size()<<'\n';
        for(const auto&entry:used){const auto&r=rows.at(entry.first);cert<<r.equation;for(auto e:r.multiplier)cert<<' '<<e;cert<<'\n';require(cert.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");}
        std::vector<Polynomial> gb;
        for(const auto*row:selected){gb.push_back(row->poly);cert<<row->weights.size();
            for(const auto&w:row->weights)cert<<' '<<used.at(w.first)<<' '<<w.second;
            cert<<'\n';
            require(cert.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");}
        write_basis(basis,s,gb);std::ofstream output(certificate);require(bool(output),"certificate output open");output<<cert.str();output.close();require(bool(output),"certificate output write");
    }
};
} // namespace mca
int main(int argc,char**argv){
    mca::Stats stats;
    try{
        std::filesystem::path input,output;unsigned degree=6;
        if(const char*env=std::getenv("MACAULAY_DEGREE_CAP")){
            const auto parsed=canonical::unsigned_decimal(env);mca::require(parsed>=1&&parsed<=6,"degree cap");degree=static_cast<unsigned>(parsed);}
        for(int i=1;i<argc;++i){const std::string arg=argv[i];mca::require(i+1<argc,"missing argument value");const std::string value=argv[++i];
            if(arg=="-f")input=value;else if(arg=="-o")output=value;else if(arg=="--degree"){
                const auto parsed=canonical::unsigned_decimal(value);mca::require(parsed>=1&&parsed<=6,"degree cap");degree=static_cast<unsigned>(parsed);}
            else if(arg=="-t")mca::require(value=="1","one thread required");else if(arg=="-g")mca::require(value=="2","full basis output required");
            else throw std::invalid_argument("unknown producer argument");}
        mca::require(!input.empty()&&!output.empty()&&degree>=1&&degree<=6,"input/output or degree cap");
        const auto cert=std::filesystem::path(output.string()+".cert");mca::require(!std::filesystem::exists(output)&&!std::filesystem::exists(cert),"refusing reused output");
        const auto system=canonical::read_system(input);mca::require(system.names.size()<=20,"variable cap");
        const char*verifier=std::getenv("MACAULAY_VERIFIER");mca::require(verifier&&std::filesystem::is_regular_file(verifier),"missing separate verifier");
        mca::Producer(system,degree,stats).run(output,cert);
        std::cout<<"CANDIDATE degree="<<degree<<" rows="<<stats.rows<<" pivots="<<stats.pivots<<" reductions="<<stats.steps<<std::endl;
        execl(verifier,verifier,input.c_str(),output.c_str(),cert.c_str(),nullptr);throw std::runtime_error("verifier exec failed");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<" rows="<<stats.rows<<" pivots="<<stats.pivots<<" reductions="<<stats.steps<<'\n';return 2;}
}
