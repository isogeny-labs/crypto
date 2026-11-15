// Standard bounded Buchberger scheduling; PC-DAG checker is unchanged.
#include "../norm_pc_closure_20261115/common.hpp"
#include <unistd.h>
namespace critical {
using namespace mca;
using Sparse=std::vector<std::pair<std::size_t,U>>;
struct Stats {
    std::size_t rows=0,steps=0,entries=0,nodes=0,basis=0,pairs=0,skipped=0,chains=0,deferred=0;
    U products=0,updates=0;
};
struct Field {
    U p;Stats&stats;std::vector<U> table;
    Field(U prime,Stats&counts):p(prime),stats(counts){
        if(p<=257){table.resize(p*p);for(U a=0;a<p;++a)for(U b=0;b<p;++b){table[a*p+b]=a*b%p;++stats.products;}}
    }
    U mul(U a,U b){++stats.products;return table.empty()?a*b%p:table[a*p+b];}
    U add(U a,U b)const{const auto sum=a+b;return sum>=p?sum-p:sum;}
    U sub(U a,U b)const{return a>=b?a-b:a+p-b;}
    U power(U a,U e){U out=1;for(;e;e>>=1,a=mul(a,a))if(e&1U)out=mul(out,a);return out;}
};
struct Index {
    std::size_t n,columns=1,lower=0,fixed=0;unsigned degree;
    std::vector<Monomial> monomials;std::vector<unsigned> degrees;
    std::map<Monomial,std::size_t> lookup;
    std::vector<std::vector<std::size_t>> products;
    void enumerate(Monomial&m,std::size_t at,unsigned remaining){
        if(at==n){monomials.push_back(m);return;}
        for(unsigned e=0;e<=remaining;++e){m[at]=e;enumerate(m,at+1,remaining-e);}m[at]=0;
    }
    Index(const System&s,unsigned d):n(s.names.size()),degree(d){
        for(unsigned k=1;k<=d;++k)columns=columns*(n+k)/k;
        require(columns<=entry_cap/(2*n+12),"monomial index admission cap");
        Monomial m(n);enumerate(m,0,d);require(monomials.size()==columns,"monomial count");
        std::sort(monomials.begin(),monomials.end(),[](const auto&a,const auto&b){return quotient::greater(a,b);});
        for(std::size_t i=0;i<columns;++i){lookup.emplace(monomials[i],i);degrees.push_back(canonical::degree(monomials[i]));}
        while(lower<columns&&degrees[lower]==d)++lower;
        std::size_t inputs=0;for(const auto&f:s.equations)inputs+=2*f.terms.size();
        fixed=(2*n+12)*columns+n*(columns-lower)+(s.p<=257?s.p*s.p:0)+inputs+8*n;
        require(fixed<=entry_cap,"index/work entry cap");
        products.assign(n,std::vector<std::size_t>(columns-lower));
        for(std::size_t j=0;j<n;++j)for(std::size_t i=lower;i<columns;++i){
            auto next=monomials[i];++next[j];products[j][i-lower]=lookup.at(next);
        }
    }
    Sparse encode(const Polynomial&f)const{
        require(f.largest_degree()<=degree,"input degree cap");Sparse out;
        for(const auto&term:f.terms)out.emplace_back(lookup.at(term.first),term.second);
        std::sort(out.begin(),out.end());return out;
    }
    Polynomial decode(const Sparse&row,U p)const{
        Polynomial out(n,p);for(const auto&term:row)out.insert(monomials.at(term.first),term.second);return out;
    }
};
struct Node {char kind;std::size_t first,second;Sparse links,value;};
using Pair=std::tuple<unsigned,std::size_t,std::size_t,std::size_t>;
class Engine {
    const System&s;Stats&stats;Index index;Field field;
    std::vector<Node> nodes;std::vector<std::size_t> basis,reducers,originals;
    std::vector<U> dense;
    std::set<Pair> pairs;
    std::set<std::pair<std::size_t,std::size_t>> represented;
    std::map<std::pair<std::size_t,std::size_t>,std::size_t> cache;
    void reserve_slots(std::size_t count){require(stats.entries+count<=entry_cap,"entry cap");stats.entries+=count;}
    static std::pair<std::size_t,std::size_t> key(std::size_t a,std::size_t b){return std::minmax(a,b);}
    void mark_represented(std::size_t a,std::size_t b){
        const auto pair=key(a,b);if(represented.count(pair))return;
        reserve_slots(7);represented.insert(pair);
    }
    bool chain_represented(std::size_t a,std::size_t b,std::size_t lcm)const{
        for(const auto h:basis){
            if(h==a||h==b||!quotient::divides(index.monomials[nodes[h].value.front().first],index.monomials[lcm]))continue;
            if(represented.count(key(a,h))&&represented.count(key(b,h)))return true;
        }
        return false;
    }
    std::size_t append(char kind,std::size_t first,std::size_t second,Sparse links,Sparse value){
        require(nodes.size()<pc::node_cap,"proof node cap");reserve_slots(6+2*links.size()+2*value.size());
        nodes.push_back({kind,first,second,std::move(links),std::move(value)});stats.nodes=nodes.size();return nodes.size()-1;
    }
    std::size_t variable_product(std::size_t source,std::size_t variable){
        const auto key=std::make_pair(source,variable);const auto found=cache.find(key);if(found!=cache.end())return found->second;
        Sparse result;result.reserve(nodes[source].value.size());
        for(const auto&term:nodes[source].value){
            require(term.first>=index.lower,"product degree cap");const auto col=index.products[variable][term.first-index.lower];
            require(result.empty()||result.back().first<col,"multiplication order");result.emplace_back(col,term.second);
        }
        const auto id=append('M',source,variable,{},std::move(result));reserve_slots(7);cache.emplace(key,id);return id;
    }
    std::size_t multiply(std::size_t source,const Monomial&monomial){
        for(std::size_t j=0;j<index.n;++j)for(unsigned e=0;e<monomial[j];++e)source=variable_product(source,j);
        return source;
    }
    std::size_t reduction(std::size_t source){
        std::fill(dense.begin(),dense.end(),0);for(const auto&term:nodes[source].value)dense[term.first]=term.second;
        std::map<std::size_t,U> links{{source,1}};
        for(std::size_t at=0;at<index.columns;){
            if(dense[at]==0){++at;continue;}
            require(++stats.steps<=step_cap,"reduction step cap");bool changed=false;
            for(const auto reducer:reducers){
                const auto lead=nodes[reducer].value.front().first;
                if(!quotient::divides(index.monomials[lead],index.monomials[at]))continue;
                Monomial multiplier(index.n);for(std::size_t j=0;j<index.n;++j)multiplier[j]=index.monomials[at][j]-index.monomials[lead][j];
                const auto product=multiply(reducer,multiplier);const U factor=dense[at];
                require(!nodes[product].value.empty()&&nodes[product].value.front()==std::make_pair(at,U(1)),"monic reducer");
                for(const auto&term:nodes[product].value){dense[term.first]=field.sub(dense[term.first],field.mul(factor,term.second));++stats.updates;}
                auto&coefficient=links[product];coefficient=field.sub(coefficient,factor);if(coefficient==0)links.erase(product);
                require(stats.entries+2*links.size()<=entry_cap,"active proof cap");
                require(dense[at]==0,"leading cancellation");changed=true;break;
            }
            if(!changed)++at;
        }
        std::size_t first=0;while(first<index.columns&&dense[first]==0)++first;
        if(first==index.columns)return nodes.size();
        const U inverse=field.power(dense[first],s.p-2);Sparse row,proof;
        for(std::size_t col=first;col<index.columns;++col)if(dense[col])row.emplace_back(col,field.mul(dense[col],inverse));
        for(const auto&link:links)if(link.second)proof.emplace_back(link.first,field.mul(link.second,inverse));
        require(!proof.empty(),"nonzero remainder has empty provenance");return append('L',0,0,std::move(proof),std::move(row));
    }
    void install(std::size_t node){
        require(basis.size()<row_cap,"working basis count cap");reserve_slots(2);
        const auto&lm=index.monomials[nodes[node].value.front().first];
        for(const auto other:basis){
            const auto&om=index.monomials[nodes[other].value.front().first];bool coprime=true;Monomial lcm(index.n);
            for(std::size_t j=0;j<index.n;++j){if(lm[j]&&om[j])coprime=false;lcm[j]=std::max(lm[j],om[j]);}
            if(coprime){++stats.skipped;mark_represented(other,node);continue;}
            const auto degree=canonical::degree(lcm);if(degree>index.degree){++stats.deferred;continue;}
            reserve_slots(9);pairs.emplace(degree,index.lookup.at(lcm),other,node);
        }
        basis.push_back(node);reducers.push_back(node);
        std::sort(reducers.begin(),reducers.end(),[&](auto a,auto b){
            const auto ca=nodes[a].value.front().first,cb=nodes[b].value.front().first;
            return std::make_tuple(index.degrees[ca],ca,a)<std::make_tuple(index.degrees[cb],cb,b);
        });stats.basis=basis.size();
        std::cout<<"PROGRESS basis="<<basis.size()<<" pairs_pending="<<pairs.size()<<" reductions="<<stats.steps
                 <<" entries="<<stats.entries<<" nodes="<<nodes.size()<<std::endl;
    }
    bool unit()const{return !basis.empty()&&index.degrees[nodes[basis.back()].value.front().first]==0;}
    void export_candidate(const std::filesystem::path&output){
        std::vector<std::size_t> selected;
        for(const auto id:basis){bool minimal=true;const auto col=nodes[id].value.front().first;
            for(const auto other:basis){if(other==id)continue;const auto othercol=nodes[other].value.front().first;
                if(quotient::divides(index.monomials[othercol],index.monomials[col])&&(othercol!=col||other<id)){minimal=false;break;}}
            if(minimal)selected.push_back(id);
        }
        require(!selected.empty()&&selected.size()<=basis_cap,"exported basis count cap");reserve_slots(3*nodes.size()+2*selected.size());
        std::vector<bool> used(nodes.size(),false);for(auto id:selected)used[id]=true;
        for(std::size_t i=nodes.size();i-->0;)if(used[i]){
            if(nodes[i].kind=='M')used[nodes[i].first]=true;
            for(const auto&link:nodes[i].links)used[link.first]=true;
        }
        std::vector<std::size_t> remap(nodes.size());std::size_t count=0;
        for(std::size_t i=0;i<nodes.size();++i)if(used[i])remap[i]=count++;
        std::ostringstream proof;proof<<"PC-DAG-CERT-1 "<<s.p<<' '<<index.n<<' '<<index.degree<<' '<<count<<' '<<selected.size()<<'\n';
        for(std::size_t i=0;i<nodes.size();++i)if(used[i]){const auto&node=nodes[i];proof<<node.kind;
            if(node.kind=='I')proof<<' '<<node.first;
            else if(node.kind=='M')proof<<' '<<remap[node.first]<<' '<<node.second;
            else{proof<<' '<<node.links.size();for(const auto&link:node.links)proof<<' '<<remap[link.first]<<' '<<link.second;}
            proof<<'\n';require(proof.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");
        }
        proof<<'F';std::vector<Polynomial> gb;
        for(auto id:selected){proof<<' '<<remap[id];reserve_slots(2*nodes[id].value.size());gb.push_back(index.decode(nodes[id].value,s.p));}proof<<'\n';
        require(proof.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");write_basis(output,s,gb);
        std::ofstream certificate(output.string()+".cert");require(bool(certificate),"certificate open");certificate<<proof.str();certificate.close();require(bool(certificate),"certificate write");
        std::cout<<"CANDIDATE CRITICAL degree="<<index.degree<<" columns="<<index.columns<<" rows="<<stats.rows
                 <<" basis="<<stats.basis<<" exported="<<gb.size()<<" reductions="<<stats.steps<<" entries="<<stats.entries
                 <<" nodes="<<stats.nodes<<" proof_nodes="<<count<<" proof_bytes="<<proof.str().size()<<" pairs="<<stats.pairs
                 <<" skipped="<<stats.skipped<<" chains="<<stats.chains<<" deferred="<<stats.deferred<<" products="<<stats.products<<" updates="<<stats.updates<<std::endl;
    }
public:
    Engine(const System&input,unsigned degree,Stats&counts):s(input),stats(counts),index(input,degree),field(input.p,counts),dense(index.columns,0){stats.entries=index.fixed;}
    void run(const std::filesystem::path&output){
        for(std::size_t i=0;i<s.equations.size();++i)originals.push_back(append('I',i,0,{},index.encode(s.equations[i])));
        for(const auto id:originals){if(unit())break;require(++stats.rows<=row_cap,"row cap");const auto result=reduction(id);if(result<nodes.size())install(result);}
        while(!pairs.empty()&&!unit()){
            const auto pair=*pairs.begin();pairs.erase(pairs.begin());stats.entries-=9;++stats.pairs;
            const auto lcm=std::get<1>(pair),a=std::get<2>(pair),b=std::get<3>(pair);
            if(chain_represented(a,b,lcm)){++stats.chains;mark_represented(a,b);continue;}
            require(++stats.rows<=row_cap,"row cap");Monomial ma(index.n),mb(index.n);
            for(std::size_t j=0;j<index.n;++j){ma[j]=index.monomials[lcm][j]-index.monomials[nodes[a].value.front().first][j];mb[j]=index.monomials[lcm][j]-index.monomials[nodes[b].value.front().first][j];}
            const auto left=multiply(a,ma),right=multiply(b,mb);std::map<std::size_t,U> difference;
            for(const auto&term:nodes[left].value)difference[term.first]=term.second;
            for(const auto&term:nodes[right].value){auto&value=difference[term.first];value=field.sub(value,term.second);}
            Sparse value;for(const auto&term:difference)if(term.second)value.push_back(term);
            if(value.empty()){mark_represented(a,b);continue;}
            require(stats.entries+4*difference.size()<=entry_cap,"active S-row cap");
            const auto source=append('L',0,0,{{left,1},{right,s.p-1}},std::move(value));
            const auto result=reduction(source);if(result<nodes.size())install(result);
            mark_represented(a,b);
        }
        export_candidate(output);
    }
};
} // namespace critical
int main(int argc,char**argv){
    critical::Stats stats;
    try{
        std::filesystem::path input,output;unsigned degree=6;
        if(const char*env=std::getenv("PC_DEGREE_CAP")){const auto d=canonical::unsigned_decimal(env);critical::require(d>=1&&d<=6,"degree cap");degree=d;}
        for(int i=1;i<argc;++i){const std::string arg=argv[i];critical::require(i+1<argc,"missing argument value");const std::string value=argv[++i];
            if(arg=="-f")input=value;else if(arg=="-o")output=value;
            else if(arg=="--degree"){const auto d=canonical::unsigned_decimal(value);critical::require(d>=1&&d<=6,"degree cap");degree=d;}
            else if(arg=="-t")critical::require(value=="1","one thread required");else if(arg=="-g")critical::require(value=="2","full basis required");
            else throw std::invalid_argument("unknown producer argument");
        }
        critical::require(!input.empty()&&!output.empty(),"input/output required");
        critical::require(!std::filesystem::exists(output)&&!std::filesystem::exists(output.string()+".cert"),"refusing reused output");
        const auto system=canonical::read_system(input);critical::require(!system.names.empty()&&system.names.size()<=20,"variable cap");
        const char*verifier=std::getenv("PC_VERIFIER");critical::require(verifier&&std::filesystem::is_regular_file(verifier),"missing separate verifier");
        critical::Engine(system,degree,stats).run(output);
        execl(verifier,verifier,input.c_str(),output.c_str(),(output.string()+".cert").c_str(),nullptr);throw std::runtime_error("verifier exec failed");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<" rows="<<stats.rows<<" basis="<<stats.basis
        <<" reductions="<<stats.steps<<" entries="<<stats.entries<<" nodes="<<stats.nodes<<" pairs="<<stats.pairs
        <<" skipped="<<stats.skipped<<" chains="<<stats.chains<<" deferred="<<stats.deferred<<'\n';return 2;}
}
