#include "common.hpp"
#include <unistd.h>
namespace pc {
struct Stats {std::size_t rows=0,pivots=0,steps=0,entries=0,nodes=0;};
class Producer {
    const System&s;unsigned degree;Stats&stats;
    std::vector<Node> nodes;
    std::map<Monomial,std::size_t> pivots;
    std::vector<std::size_t> queue,originals;
    std::vector<std::size_t> completion_basis;
    bool linear_checked=false;
    void check_completion(){
        const auto unit=pivots.find(Monomial(s.names.size(),0));
        if(unit!=pivots.end()){completion_basis={unit->second};return;}
        if(linear_checked)return;
        std::vector<std::size_t> ids;std::vector<Polynomial> linear;
        for(std::size_t var=0;var<s.names.size();++var){
            Monomial mon(s.names.size(),0);mon[var]=1;const auto found=pivots.find(mon);
            if(found==pivots.end())return;
            ids.push_back(found->second);linear.push_back(nodes.at(found->second).poly);
        }
        linear_checked=true;
        std::size_t monomials=1,linear_entries=0;
        for(unsigned k=1;k<=degree;++k)monomials=monomials*(s.names.size()+k)/k;
        for(const auto&f:linear)linear_entries+=f.terms.size();
        require(stats.entries+3*monomials+linear_entries<=entry_cap,"completion scratch-entry cap");
        for(const auto&f:s.equations)if(!normal_form(f,linear,stats.steps).terms.empty())return;
        completion_basis=std::move(ids);
    }
    std::size_t append(Node node){
        require(nodes.size()<node_cap,"proof node cap");check_degree(node.poly,degree);
        const auto entries=node.poly.terms.size()+node.links.size();
        require(stats.entries+entries<=entry_cap,"live-entry cap");stats.entries+=entries;
        nodes.push_back(std::move(node));stats.nodes=nodes.size();return nodes.size()-1;
    }
    void absorb(std::size_t start){
        require(++stats.rows<=row_cap,"attempted row cap");
        auto row=nodes.at(start).poly;
        std::vector<std::pair<std::size_t,U>> links{{start,1}};
        while(!row.terms.empty()){
            require(stats.entries+row.terms.size()+links.size()<=entry_cap,"live-entry cap");
            const auto lm=quotient::lead(row);const auto found=pivots.find(lm);
            if(found==pivots.end()){
                const auto inverse=quotient::power(row.terms.at(lm),s.p-2,s.p);
                row=row.times(Polynomial::constant(row.n,s.p,inverse));
                for(auto&link:links)link.second=link.second*inverse%s.p;
                const auto id=append({'L',0,0,std::move(links),std::move(row)});
                pivots.emplace(lm,id);++stats.pivots;
                if(nodes[id].poly.largest_degree()<degree)queue.push_back(id);
                check_completion();
                return;
            }
            require(++stats.steps<=step_cap,"row-reduction step cap");
            const auto c=row.terms.at(lm);const auto id=found->second;
            row=row.plus(nodes.at(id).poly.times(Polynomial::constant(row.n,s.p,c)),true);
            links.emplace_back(id,s.p-c);
        }
    }
public:
    Producer(const System&input,unsigned d,Stats&counts):s(input),degree(d),stats(counts){}
    void run(const std::filesystem::path&basis,const std::filesystem::path&certificate){
        for(std::size_t eq=0;eq<s.equations.size();++eq){
            const auto id=append({'I',eq,0,{},s.equations[eq]});originals.push_back(id);
            if(completion_basis.empty())absorb(id);
        }
        for(std::size_t at=0;at<queue.size()&&completion_basis.empty();++at){const auto source=queue[at];
            for(std::size_t var=0;var<s.names.size()&&completion_basis.empty();++var){
                const auto product=variable_multiple(nodes.at(source).poly,var);
                const auto id=append({'M',source,var,{},product});absorb(id);
            }
        }
        std::vector<std::size_t> selected=completion_basis;
        if(selected.empty())for(const auto&entry:pivots){bool minimal=true;
            for(const auto&other:pivots)if(entry.first!=other.first&&quotient::divides(other.first,entry.first)){minimal=false;break;}
            if(minimal)selected.push_back(entry.second);
        }
        for(const auto id:originals){const auto&f=nodes[id].poly;if(f.terms.empty())continue;bool present=false;
            for(const auto chosen:selected)if(nodes[chosen].poly.terms==f.terms){present=true;break;}
            if(!present)selected.push_back(id);
        }
        require(selected.size()<=basis_cap,"candidate basis count cap");
        std::vector<bool> used(nodes.size(),false);for(const auto id:selected)used.at(id)=true;
        for(std::size_t i=nodes.size();i-->0;)if(used[i]){
            const auto&node=nodes[i];if(node.kind=='M')used.at(node.first)=true;
            for(const auto&link:node.links)used.at(link.first)=true;
        }
        std::vector<std::size_t> remap(nodes.size());std::size_t count=0;
        for(std::size_t i=0;i<nodes.size();++i)if(used[i])remap[i]=count++;
        std::ostringstream proof;proof<<"PC-DAG-CERT-1 "<<s.p<<' '<<s.names.size()<<' '<<degree<<' '<<count<<' '<<selected.size()<<'\n';
        for(std::size_t i=0;i<nodes.size();++i)if(used[i]){const auto&node=nodes[i];proof<<node.kind;
            if(node.kind=='I')proof<<' '<<node.first;
            else if(node.kind=='M')proof<<' '<<remap.at(node.first)<<' '<<node.second;
            else{proof<<' '<<node.links.size();for(const auto&link:node.links)proof<<' '<<remap.at(link.first)<<' '<<link.second;}
            proof<<'\n';require(proof.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");
        }
        std::vector<Polynomial> gb;proof<<'F';for(const auto id:selected){proof<<' '<<remap[id];gb.push_back(nodes[id].poly);}proof<<'\n';
        require(proof.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");
        write_basis(basis,s,gb);std::ofstream out(certificate);require(bool(out),"certificate output open");out<<proof.str();out.close();require(bool(out),"certificate output write");
        std::cout<<"CANDIDATE degree="<<degree<<" rows="<<stats.rows<<" pivots="<<stats.pivots<<" reductions="<<stats.steps
                 <<" nodes="<<nodes.size()<<" proof_nodes="<<count<<" proof_bytes="<<proof.str().size()<<std::endl;
    }
};
} // namespace pc
int main(int argc,char**argv){
    pc::Stats stats;
    try{
        std::filesystem::path input,output;unsigned degree=6;
        if(const char*env=std::getenv("PC_DEGREE_CAP")){const auto d=canonical::unsigned_decimal(env);pc::require(d>=1&&d<=6,"degree cap");degree=d;}
        for(int i=1;i<argc;++i){const std::string arg=argv[i];pc::require(i+1<argc,"missing argument value");const std::string value=argv[++i];
            if(arg=="-f")input=value;else if(arg=="-o")output=value;
            else if(arg=="--degree"){const auto d=canonical::unsigned_decimal(value);pc::require(d>=1&&d<=6,"degree cap");degree=d;}
            else if(arg=="-t")pc::require(value=="1","one thread required");else if(arg=="-g")pc::require(value=="2","full basis output required");
            else throw std::invalid_argument("unknown producer argument");
        }
        pc::require(!input.empty()&&!output.empty(),"input/output required");const auto certificate=std::filesystem::path(output.string()+".cert");
        pc::require(!std::filesystem::exists(output)&&!std::filesystem::exists(certificate),"refusing reused output");
        const auto s=canonical::read_system(input);pc::require(s.names.size()<=20,"variable cap");
        const char*verifier=std::getenv("PC_VERIFIER");pc::require(verifier&&std::filesystem::is_regular_file(verifier),"missing separate verifier");
        pc::Producer(s,degree,stats).run(output,certificate);
        execl(verifier,verifier,input.c_str(),output.c_str(),certificate.c_str(),nullptr);throw std::runtime_error("verifier exec failed");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<" rows="<<stats.rows<<" pivots="<<stats.pivots
        <<" reductions="<<stats.steps<<" nodes="<<stats.nodes<<'\n';return 2;}
}
