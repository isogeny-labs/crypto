// Column-indexed polynomial-calculus producer. Independent verifier unchanged.
#include "../norm_pc_closure_20261115/common.hpp"
#include <unistd.h>
namespace indexed {
using namespace mca;
using Sparse=std::vector<std::pair<std::size_t,U>>;
struct Stats {std::size_t rows=0,pivots=0,steps=0,entries=0,nodes=0;U products=0,updates=0;};
struct Field {
    U p;Stats&stats;std::vector<U> table;
    Field(U prime,Stats&counts):p(prime),stats(counts){
        if(p<=257){table.resize(p*p);for(U a=0;a<p;++a)for(U b=0;b<p;++b){table[a*p+b]=a*b%p;++stats.products;}}
    }
    U mul(U a,U b){++stats.products;return table.empty()?a*b%p:table[a*p+b];}
    U add(U a,U b)const{const auto sum=a+b;return sum>=p?sum-p:sum;}
    U sub(U a,U b)const{return a>=b?a-b:a+p-b;}
    U pow(U a,U e){U out=1;for(;e;e>>=1,a=mul(a,a))if(e&1U)out=mul(out,a);return out;}
};
struct Index {
    std::size_t n,columns=1,lower=0,fixed=0;
    unsigned degree;
    std::vector<Monomial> monomials;
    std::map<Monomial,std::size_t> lookup;
    std::vector<unsigned> degrees;
    std::vector<int> single;
    std::vector<std::size_t> variables;
    std::vector<std::vector<std::size_t>> products;
    void enumerate(Monomial&m,std::size_t at,unsigned remaining){
        if(at==n){monomials.push_back(m);return;}
        for(unsigned e=0;e<=remaining;++e){m[at]=e;enumerate(m,at+1,remaining-e);}m[at]=0;
    }
    Index(const System&s,unsigned d):n(s.names.size()),degree(d){
        for(unsigned k=1;k<=d;++k)columns=columns*(n+k)/k;
        require(columns<=entry_cap/8,"index column admission cap");
        Monomial m(n);enumerate(m,0,d);require(monomials.size()==columns,"index enumeration count");
        std::sort(monomials.begin(),monomials.end(),[](const auto&a,const auto&b){return quotient::greater(a,b);});
        single.assign(columns,-1);variables.resize(n);
        for(std::size_t i=0;i<columns;++i){lookup.emplace(monomials[i],i);degrees.push_back(canonical::degree(monomials[i]));
            if(degrees.back()==1)for(std::size_t j=0;j<n;++j)if(monomials[i][j])single[i]=static_cast<int>(j);}
        while(lower<columns&&degrees[lower]==d)++lower;
        std::size_t input_entries=0;for(const auto&f:s.equations)input_entries+=f.terms.size();
        fixed=8*columns+n*(columns-lower)+(s.p<=257?s.p*s.p:0)+input_entries+3*n;
        require(fixed<=entry_cap,"index/work entry cap");
        products.assign(n,std::vector<std::size_t>(columns-lower));
        for(std::size_t j=0;j<n;++j){Monomial var(n);var[j]=1;variables[j]=lookup.at(var);
            for(std::size_t i=lower;i<columns;++i){auto next=monomials[i];++next[j];products[j][i-lower]=lookup.at(next);}}
        require(degrees.back()==0,"constant column order");
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
struct Node {char kind;std::size_t first,second;Sparse links;};
class Engine {
    const System&s;unsigned degree;Stats&stats;Index index;Field field;
    std::vector<Node> nodes;std::vector<Sparse> values;
    std::vector<int> pivots;
    std::vector<U> dense;
    std::vector<std::size_t> queue,originals,completion;
    bool linear_checked=false;
    std::size_t append(char kind,std::size_t first,std::size_t second,Sparse links,Sparse value={}){
        require(nodes.size()<pc::node_cap,"proof node cap");
        const auto count=3+links.size()+value.size();require(stats.entries+count<=entry_cap,"stored entry cap");
        stats.entries+=count;nodes.push_back({kind,first,second,std::move(links)});values.push_back(std::move(value));
        stats.nodes=nodes.size();return nodes.size()-1;
    }
    void check_completion(){
        if(pivots.back()>=0){completion={static_cast<std::size_t>(pivots.back())};return;}
        if(linear_checked)return;
        std::vector<std::size_t> ids;
        for(auto col:index.variables){if(pivots[col]<0)return;ids.push_back(pivots[col]);}
        linear_checked=true;std::vector<U> point(index.n,0);
        for(std::size_t var=index.n;var-->0;){const auto&row=values.at(ids[var]);U sum=0;
            require(!row.empty()&&row.front().first==index.variables[var]&&row.front().second==1,"linear pivot invariant");
            for(std::size_t k=1;k<row.size();++k){const auto col=row[k].first;require(index.degrees[col]<=1,"linear tail degree");
                const int j=index.single[col];require(j<0||static_cast<std::size_t>(j)>var,"triangular linear tail");
                sum=field.add(sum,field.mul(row[k].second,j<0?1:point[j]));}
            point[var]=sum?s.p-sum:0;
        }
        for(const auto id:originals){U sum=0;
            for(const auto&term:values.at(id)){U value=term.second;const auto&m=index.monomials[term.first];
                for(std::size_t j=0;j<index.n;++j)if(m[j])value=field.mul(value,field.pow(point[j],m[j]));
                sum=field.add(sum,value);}
            if(sum!=0)return;
        }
        completion=std::move(ids);
    }
    void absorb(std::size_t start,const Sparse&source){
        require(stats.rows<row_cap,"attempted row cap");++stats.rows;std::fill(dense.begin(),dense.end(),0);
        for(const auto&term:source)dense.at(term.first)=term.second;
        Sparse links{{start,1}};std::size_t at=0;
        while(at<index.columns){
            while(at<index.columns&&dense[at]==0)++at;
            if(at==index.columns)return;
            if(pivots[at]<0){const U inverse=field.pow(dense[at],s.p-2);Sparse row;
                for(std::size_t j=at;j<index.columns;++j)if(dense[j])row.emplace_back(j,field.mul(dense[j],inverse));
                for(auto&link:links)link.second=field.mul(link.second,inverse);
                const auto id=append('L',0,0,std::move(links),std::move(row));pivots[at]=static_cast<int>(id);++stats.pivots;
                if(index.degrees[at]<degree)queue.push_back(id);
                check_completion();
                if(stats.pivots%100==0)std::cout<<"PROGRESS rows="<<stats.rows<<" pivots="<<stats.pivots
                    <<" reductions="<<stats.steps<<" entries="<<stats.entries<<" updates="<<stats.updates<<std::endl;
                return;
            }
            require(++stats.steps<=step_cap,"row-reduction step cap");const auto id=static_cast<std::size_t>(pivots[at]);const U factor=dense[at];
            for(const auto&term:values.at(id)){dense[term.first]=field.sub(dense[term.first],field.mul(factor,term.second));++stats.updates;}
            require(dense[at]==0,"leading cancellation invariant");links.emplace_back(id,s.p-factor);++at;
            require(stats.entries+links.size()<=entry_cap,"active proof entry cap");
        }
    }
public:
    Engine(const System&input,unsigned d,Stats&counts):s(input),degree(d),stats(counts),index(input,d),field(input.p,counts),
        pivots(index.columns,-1),dense(index.columns,0){stats.entries=index.fixed;}
    void run(const std::filesystem::path&basis,const std::filesystem::path&certificate){
        require(s.equations.size()<=row_cap,"source row cap");
        for(std::size_t eq=0;eq<s.equations.size();++eq)originals.push_back(append('I',eq,0,{},index.encode(s.equations[eq])));
        std::cout<<"INDEX columns="<<index.columns<<" index_work_entries="<<index.fixed<<std::endl;
        for(const auto id:originals){if(!completion.empty())break;absorb(id,values.at(id));}
        for(std::size_t at=0;at<queue.size()&&completion.empty();++at){const auto source=queue[at];
            for(std::size_t var=0;var<index.n&&completion.empty();++var){Sparse product;product.reserve(values[source].size());
                for(const auto&term:values[source]){
                    require(term.first>=index.lower,"product degree invariant");
                    const auto col=index.products[var][term.first-index.lower];
                    require(product.empty()||product.back().first<col,"monomial multiplication order");product.emplace_back(col,term.second);
                }
                const auto start=append('M',source,var,{});absorb(start,product);
            }
        }
        std::vector<std::size_t> selected=completion;
        if(selected.empty()){
            for(std::size_t col=index.columns;col-->0;)if(pivots[col]>=0){bool minimal=true;
                for(const auto id:selected)if(quotient::divides(index.monomials[values[id].front().first],index.monomials[col])){minimal=false;break;}
                if(minimal){selected.push_back(pivots[col]);require(selected.size()<=basis_cap,"candidate basis count cap");}}
            std::sort(selected.begin(),selected.end(),[&](auto a,auto b){return index.monomials[values[a].front().first]<index.monomials[values[b].front().first];});
        }
        for(auto id:originals){if(values[id].empty())continue;bool present=false;
            for(auto other:selected)if(values[id]==values[other]){present=true;break;}
            if(!present)selected.push_back(id);
        }
        require(selected.size()<=basis_cap,"candidate basis count cap");
        const auto final_work=3*nodes.size()+2*selected.size();
        require(stats.entries+final_work<=entry_cap,"proof remap entry cap");
        std::vector<bool> used(nodes.size(),false);for(auto id:selected)used[id]=true;
        for(std::size_t i=nodes.size();i-->0;)if(used[i]){
            if(nodes[i].kind=='M')used.at(nodes[i].first)=true;
            for(const auto&link:nodes[i].links)used.at(link.first)=true;
        }
        std::vector<std::size_t> remap(nodes.size());std::size_t count=0;
        for(std::size_t i=0;i<nodes.size();++i)if(used[i])remap[i]=count++;
        std::ostringstream proof;proof<<"PC-DAG-CERT-1 "<<s.p<<' '<<index.n<<' '<<degree<<' '<<count<<' '<<selected.size()<<'\n';
        for(std::size_t i=0;i<nodes.size();++i)if(used[i]){const auto&node=nodes[i];proof<<node.kind;
            if(node.kind=='I')proof<<' '<<node.first;
            else if(node.kind=='M')proof<<' '<<remap.at(node.first)<<' '<<node.second;
            else{proof<<' '<<node.links.size();for(const auto&link:node.links)proof<<' '<<remap.at(link.first)<<' '<<link.second;}
            proof<<'\n';require(proof.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");
        }
        proof<<'F';std::size_t export_entries=0;
        for(auto id:selected){proof<<' '<<remap[id];export_entries+=values[id].size();}proof<<'\n';
        require(proof.tellp()<=static_cast<std::streamoff>(cert_cap),"certificate byte cap");
        require(stats.entries+final_work+export_entries<=entry_cap,"basis export entry cap");
        std::vector<Polynomial> gb;for(auto id:selected)gb.push_back(index.decode(values[id],s.p));
        write_basis(basis,s,gb);std::ofstream out(certificate);require(bool(out),"certificate output open");out<<proof.str();out.close();require(bool(out),"certificate output write");
        std::cout<<"CANDIDATE INDEXED degree="<<degree<<" columns="<<index.columns<<" rows="<<stats.rows<<" pivots="<<stats.pivots
            <<" reductions="<<stats.steps<<" nodes="<<stats.nodes<<" proof_nodes="<<count<<" proof_bytes="<<proof.str().size()
            <<" product_requests="<<stats.products<<" row_updates="<<stats.updates<<std::endl;
    }
};
} // namespace indexed
int main(int argc,char**argv){
    indexed::Stats stats;
    try{
        std::filesystem::path input,output;unsigned degree=6;
        if(const char*env=std::getenv("PC_DEGREE_CAP")){const auto d=canonical::unsigned_decimal(env);indexed::require(d>=1&&d<=6,"degree cap");degree=d;}
        for(int i=1;i<argc;++i){const std::string arg=argv[i];indexed::require(i+1<argc,"missing argument value");const std::string value=argv[++i];
            if(arg=="-f")input=value;else if(arg=="-o")output=value;
            else if(arg=="--degree"){const auto d=canonical::unsigned_decimal(value);indexed::require(d>=1&&d<=6,"degree cap");degree=d;}
            else if(arg=="-t")indexed::require(value=="1","one thread required");else if(arg=="-g")indexed::require(value=="2","full basis output required");
            else throw std::invalid_argument("unknown producer argument");
        }
        indexed::require(!input.empty()&&!output.empty(),"input/output required");const auto cert=std::filesystem::path(output.string()+".cert");
        indexed::require(!std::filesystem::exists(output)&&!std::filesystem::exists(cert),"refusing reused output");
        const auto system=canonical::read_system(input);indexed::require(system.names.size()<=20,"variable cap");
        const char*verifier=std::getenv("PC_VERIFIER");indexed::require(verifier&&std::filesystem::is_regular_file(verifier),"missing separate verifier");
        indexed::Engine(system,degree,stats).run(output,cert);
        execl(verifier,verifier,input.c_str(),output.c_str(),cert.c_str(),nullptr);throw std::runtime_error("verifier exec failed");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<" rows="<<stats.rows<<" pivots="<<stats.pivots
        <<" reductions="<<stats.steps<<" entries="<<stats.entries<<" nodes="<<stats.nodes<<" row_updates="<<stats.updates<<'\n';return 2;}
}
