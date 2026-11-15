// Replays local polynomial identities; never calls producer elimination.
#include "common.hpp"
int main(int argc,char**argv){
    try{
        pc::require(argc==4,"usage: verify INPUT BASIS CERTIFICATE");const auto s=canonical::read_system(argv[1]);
        pc::require(std::filesystem::file_size(argv[3])<=pc::cert_cap,"certificate byte cap");
        const auto basis=quotient::read_basis(argv[2],s);std::ifstream proof(argv[3]);
        std::string magic;canonical::U p,n,d,count,nb;proof>>magic>>p>>n>>d>>count>>nb;
        pc::require(bool(proof)&&magic=="PC-DAG-CERT-1"&&p==s.p&&n==s.names.size()&&n<=20&&d>=1&&d<=6
                    &&count<=pc::node_cap&&nb<=pc::basis_cap&&nb==basis.size(),"certificate header");
        std::vector<canonical::Polynomial> nodes;std::size_t entries=0;
        for(std::size_t i=0;i<count;++i){char kind;proof>>kind;pc::require(bool(proof),"missing node");
            auto f=canonical::Polynomial::constant(n,p,0);
            if(kind=='I'){canonical::U eq;proof>>eq;pc::require(bool(proof)&&eq<s.equations.size(),"invalid source equation");f=s.equations[eq];}
            else if(kind=='M'){canonical::U ref,var;proof>>ref>>var;
                pc::require(bool(proof)&&ref<i&&var<n,"invalid backward reference or variable");f=pc::variable_multiple(nodes[ref],var);
            }else if(kind=='L'){canonical::U size;proof>>size;pc::require(bool(proof)&&size>0&&size<=i,"invalid combination size");
                std::set<std::size_t> seen;entries+=size;pc::require(entries<=pc::entry_cap,"proof entry cap");
                for(std::size_t j=0;j<size;++j){canonical::U ref,c;proof>>ref>>c;
                    pc::require(bool(proof)&&ref<i&&c>0&&c<p&&seen.insert(ref).second,"invalid combination reference or coefficient");
                    f=f.plus(nodes[ref].times(canonical::Polynomial::constant(n,p,c)));
                    pc::require(entries+f.terms.size()<=pc::entry_cap,"proof entry cap");
                }
            }else throw std::runtime_error("invalid node kind");
            pc::check_degree(f,d);entries+=f.terms.size();pc::require(entries<=pc::entry_cap,"proof entry cap");nodes.push_back(std::move(f));
        }
        char final;proof>>final;pc::require(bool(proof)&&final=='F',"missing final basis references");
        for(std::size_t i=0;i<basis.size();++i){canonical::U ref;proof>>ref;
            pc::require(bool(proof)&&ref<count&&nodes[ref].terms==basis[i].terms,"final polynomial identity mismatch");}
        std::string extra;pc::require(!(proof>>extra),"trailing certificate content");std::size_t steps=0;
        for(std::size_t i=0;i<basis.size();++i)for(std::size_t j=i+1;j<basis.size();++j)
            pc::require(pc::normal_form(pc::s_polynomial(basis[i],basis[j]),basis,steps).terms.empty(),"Buchberger completion failed");
        for(const auto&f:s.equations)pc::require(pc::normal_form(f,basis,steps).terms.empty(),"original generator does not reduce to zero");
        std::cout<<"PASS local DAG identities, bounded intermediate degree, Buchberger completion and original inclusion; nodes="<<nodes.size()
                 <<" basis="<<basis.size()<<" normal_form_steps="<<steps<<'\n';return 0;
    }catch(const std::exception&e){std::cerr<<"REJECT/UNKNOWN: "<<e.what()<<'\n';return 2;}
}
