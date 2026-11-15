// Symbolic certificate checker; does not call the producer's elimination.
#include "common.hpp"
int main(int argc,char**argv){
    try{
        mca::require(argc==4,"usage: verify INPUT BASIS CERTIFICATE");const auto s=canonical::read_system(argv[1]);
        mca::require(std::filesystem::file_size(argv[3])<=mca::cert_cap,"certificate byte cap");
        const auto basis=quotient::read_basis(argv[2],s);std::ifstream cert(argv[3]);
        std::string magic;canonical::U p,n,d,nrows,nb;cert>>magic>>p>>n>>d>>nrows>>nb;
        mca::require(bool(cert)&&magic=="MCAULAY-CERT-1"&&p==s.p&&n==s.names.size()&&n<=20&&d>=1&&d<=6&&nrows<=mca::row_cap&&nb<=mca::basis_cap&&nb==basis.size(),"certificate header");
        std::vector<mca::RowDefinition> rows;
        for(std::size_t i=0;i<nrows;++i){canonical::U eq;cert>>eq;mca::require(bool(cert)&&eq<s.equations.size(),"invalid source equation");
            canonical::Monomial mon(n);unsigned degree=0;for(auto&e:mon){canonical::U exponent;cert>>exponent;mca::require(bool(cert)&&exponent<=d,"invalid multiplier exponent");e=exponent;degree+=e;}
            mca::require(degree+s.equations[eq].largest_degree()<=d,"source multiple exceeds degree cap");rows.push_back({static_cast<std::size_t>(eq),std::move(mon)});}
        for(std::size_t i=0;i<basis.size();++i){canonical::U count;cert>>count;mca::require(bool(cert)&&count<=nrows,"certificate weight count");
            auto reconstructed=canonical::Polynomial::constant(n,p,0);std::set<std::size_t> seen;
            for(std::size_t j=0;j<count;++j){canonical::U id,c;cert>>id>>c;
                mca::require(bool(cert)&&id<nrows&&c>0&&c<p&&seen.insert(id).second,"invalid or duplicate weight");const auto&row=rows[id];
                reconstructed=reconstructed.plus(mca::multiple(s.equations[row.equation],row.multiplier,c));}
            mca::require(reconstructed.terms==basis[i].terms,"input-row identity mismatch");}
        std::string extra;mca::require(!(cert>>extra),"trailing certificate content");
        std::size_t steps=0;
        for(std::size_t i=0;i<basis.size();++i)for(std::size_t j=i+1;j<basis.size();++j)
            mca::require(mca::normal_form(mca::s_polynomial(basis[i],basis[j]),basis,steps).terms.empty(),"Buchberger completion failed");
        for(const auto&f:s.equations)mca::require(mca::normal_form(f,basis,steps).terms.empty(),"original generator does not reduce to zero");
        std::cout<<"PASS source identities, original inclusion and Buchberger completion; basis="<<basis.size()<<" normal_form_steps="<<steps<<'\n';return 0;
    }catch(const std::exception&e){std::cerr<<"REJECT/UNKNOWN: "<<e.what()<<'\n';return 2;}
}
