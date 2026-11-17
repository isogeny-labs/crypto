// One uniquely determined graph embedding of the cached depth5 point set.
#include "retained_prefix.inc"
int main(int argc,char** argv) {
    try {
        require(argc==3,"usage: footprint_lift CACHE FRESH_OUT");
        std::filesystem::path out=argv[2]; require(!std::filesystem::exists(out),"fresh output");
        std::istringstream in(read(argv[1])); size_t n=0,Np=0,k=0; std::string tag;
        in>>tag>>n; require(tag=="VARS" && n==6,"cached variable count"); Vars vars;
        for(size_t i=0;i<n;++i) {std::string v;in>>v;vars.push_back(v);}
        in>>tag>>Np; require(tag=="POINTS" && Np==42,"cached point count"); Assignments points;
        for(size_t i=0;i<Np;++i) {
            std::map<std::string,Z> p;
            for(const auto& v:vars) {std::string c;in>>c;Z z(c);require(z>=0 && z<P,"cached coordinate");p[v]=z;}
            points.push_back(std::move(p));
        }
        in>>tag>>k; require(tag=="FEATURES" && k==4,"cached top footprint size");
        std::vector<Monomial> features;
        for(size_t i=0;i<k;++i) {
            Monomial mon; size_t terms=0; in>>terms;
            for(size_t j=0;j<terms;++j) {std::string v;unsigned e;in>>v>>e;require(e>0 && std::find(vars.begin(),vars.end(),v)!=vars.end(),"feature variable");mon[v]=e;}
            require(md(mon)==4,"only cached degree4 features"); features.push_back(mon);
        }
        in>>tag;require(tag=="END" && !(in>>tag),"cache terminator");
        std::vector<std::string> defs;
        for(size_t j=0;j<features.size();++j) {
            const std::string name="g"+num(j);vars.push_back(name);
            for(auto& p:points) p[name]=mt(features[j]).eval(p);
            defs.push_back(object({{"name",jquote(name)},{"unique_graph_polynomial",mt(features[j]).json()}}));
        }
        const auto before=counters; Basis basis=buchberger_moeller(vars,points);
        unsigned D=0;for(const auto& m:basis.standard) D=std::max(D,md(m)+1);
        const std::string cert=check_basis(basis,vars,points,D);
        bool rejected=false;try {check_basis(basis,vars,points,D+1);}catch(const std::exception&){rejected=true;}
        require(rejected,"wrong degree control accepted");
        write(out/"certificate.json",cert);
        write(out/"manifest.json",object({{"factor_base",jquote("FB1h8bc0ab2ed2af")},
          {"parent_point_cache_sha256",jquote(hash(read(argv[1])))},{"signed_points",num(points.size())},
          {"coordinates",num(vars.size())},{"added_coordinates",num(features.size())},
          {"features",array(defs)},{"ideal_module_regularity",num(D)},
          {"certificate_sha256",jquote(hash(cert+"\n"))},{"corrupt_degrees_rejected","1"},
          {"modular_reduction_API_calls",num(counters.reductions-before.reductions)},
          {"inversion_API_calls",num(counters.inversions-before.inversions)},
          {"counter_scope",jquote("construction and verification GMP API calls, not solve equivalents")},
          {"full_relation_regularity","null"},{"goal_achieved","false"}}));
        std::cout<<"Graph membership D="<<D<<"; complete-system goal remains open.\n";
    }catch(const std::exception& e){std::cerr<<"footprint_lift: "<<e.what()<<'\n';return 1;}
}
