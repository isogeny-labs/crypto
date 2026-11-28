#pragma once
#include <numeric>
#include <tuple>

namespace closed_residual {
using AP=shifted::AP;
using Sparse=norm_test::Sparse;
using Pair=audit::Pair;
struct Query {int m=0,label=0;AP value;std::string key;};
struct Witness {
    int m=0;Pair anchors{};int image=0,divisor=1,sign=1;
    bool operator==(const Witness& w)const{return std::tie(m,anchors,image,divisor,sign)==std::tie(w.m,w.anchors,w.image,w.divisor,w.sign);}
    bool operator<(const Witness& w)const{return std::tie(m,anchors,image,divisor,sign)<std::tie(w.m,w.anchors,w.image,w.divisor,w.sign);}
};
using Slots=std::array<int,4>;
struct Result {
    std::map<Sparse,Witness> rows;
    std::set<Slots> slots;
    std::array<size_t,2> matches{},identity_matches{};
};
struct Queries {std::vector<Query> values;std::string digest;};
Sparse raw_row(const Witness& w,const Sparse& reference) {
    Sparse row=norm_test::row_for(audit::Quad{w.anchors[0],w.anchors[1],0,0},reference);
    if(w.image)row[(w.image-1)/2]+=w.m*(w.image%2?1:-1);
    for(auto i=row.begin();i!=row.end();)if(i->second==0)i=row.erase(i);else ++i;
    return row;
}
Sparse primitive(Sparse row,int& divisor,int& sign) {
    divisor=0;sign=1;
    for(auto [j,c]:row){(void)j;require(c>=-128&&c<=128,"bounded residual coefficient");divisor=std::gcd(divisor,std::abs(c));}
    if(row.empty()){divisor=1;return row;}
    require(divisor>0&&SCBig(divisor)<sc_n,"residual gcd is a unit modulo subgroup order");
    if(row.begin()->second<0)sign=-1;
    for(auto& [j,c]:row){(void)j;c=c/divisor*sign;}
    return row;
}
void retain(Result& result,const Pair& pair,const Query& query,const Sparse& reference) {
    require(query.m==5||query.m==6,"declared residual degree");
    const Slots slot{query.m,pair[0],pair[1],query.label};
    require(result.slots.insert(slot).second,"no duplicated raw residual match");
    require(result.slots.size()<=4096,"raw residual match cap exceeded; incomplete");
    const auto family=size_t(query.m-5);++result.matches[family];if(query.label==0)++result.identity_matches[family];
    Witness w{query.m,pair,query.label,1,1};auto row=primitive(raw_row(w,reference),w.divisor,w.sign);
    auto [at,inserted]=result.rows.emplace(row,w);if(!inserted&&w<at->second)at->second=w;
    require(result.rows.size()<=4096,"distinct residual row cap exceeded; incomplete");
}
void complete_pairs(const std::vector<norm_test::Record>& pairs,size_t columns) {
    const size_t alphabet=2*columns+1;require(pairs.size()==alphabet*(alphabet+1)/2,"complete residual pair domain");
    size_t offset=0;for(size_t i=0;i<alphabet;++i)for(size_t j=i;j<alphabet;++j)
        require(pairs.at(offset++).labels==Pair{int(i),int(j)},"exact residual ordered pair labels");
}
void validate_query(const Query& query,const std::vector<AP>& base,const audit::SSL& ssl) {
    require((query.m==5||query.m==6)&&query.label>=0&&size_t(query.label)<=2*base.size(),"residual query bounds");
    const AP p=shifted::symbol(query.label,base);
    require(query.value==norm_test::multiply(p,SCBig(query.m))&&query.key==audit::key(query.value),"GMP residual multiple query");
    auto point=ssl.make(p),value=ssl.make({});auto m=audit::bn(SCBig(query.m));
    require(EC_POINT_mul(ssl.group.get(),value.get(),nullptr,point.get(),m.get(),ssl.context.get())==1&&
            ssl.encode(value.get())==query.key,"named residual multiple query");
}
void complete_queries(const std::vector<Query>& queries,size_t columns) {
    const size_t alphabet=2*columns+1;require(queries.size()==2*alphabet,"complete residual multiple domain");
    size_t offset=0;
    for(int m:{5,6})for(size_t label=0;label<alphabet;++label) {
        const auto& query=queries.at(offset++);require(query.m==m&&query.label==int(label),"exact residual ordered query labels");
    }
}
Queries multiple_queries(const std::vector<AP>& base) {
    audit::SSL ssl;Queries result;norm_test::Digest first,second;
    for(int m:{5,6}) {
        std::set<std::string> family_keys;
        for(size_t label=0;label<=2*base.size();++label) {
            const auto p=shifted::symbol(int(label),base);const auto q=norm_test::multiply(p,SCBig(m));
            auto named_p=ssl.make(p),named_q=ssl.make({});auto scalar=audit::bn(SCBig(m));
            require(EC_POINT_mul(ssl.group.get(),named_q.get(),nullptr,named_p.get(),scalar.get(),ssl.context.get())==1,"named query precomputation");
            const auto named=ssl.encode(named_q.get()),gmp=audit::key(q);require(named==gmp,"every independent residual multiple agrees");
            require(family_keys.insert(gmp).second,"distinct signed queries within each invertible multiplier");
            first.add(norm_test::Record{{m,int(label)},named,named});second.add(norm_test::Record{{m,int(label)},gmp,gmp});
            result.values.push_back(Query{m,int(label),q,gmp});
        }
    }
    complete_queries(result.values,base.size());result.digest=first.finish();require(result.digest==second.finish(),"independent query stream hashes agree");
    for(const auto& query:result.values)validate_query(query,base,ssl);
    return result;
}
Result lookup(const std::vector<norm_test::Record>& pairs,const std::vector<Query>& queries,const Sparse& reference) {
    std::map<std::string,std::vector<size_t>> index;
    for(size_t i=0;i<queries.size();++i)index[queries[i].key].push_back(i);
    Result result;
    for(const auto& pair:pairs) {
        const auto found=index.find(pair.complement);if(found==index.end())continue;
        for(size_t i:found->second)retain(result,pair.labels,queries[i],reference);
    }
    return result;
}
Result sorted_merge(const std::vector<norm_test::Record>& pairs,const std::vector<Query>& queries,const Sparse& reference) {
    std::vector<size_t> a(pairs.size()),b(queries.size());std::iota(a.begin(),a.end(),0);std::iota(b.begin(),b.end(),0);
    std::sort(a.begin(),a.end(),[&](size_t i,size_t j){return std::tie(pairs[i].complement,pairs[i].labels)<std::tie(pairs[j].complement,pairs[j].labels);});
    std::sort(b.begin(),b.end(),[&](size_t i,size_t j){return std::tie(queries[i].key,queries[i].m,queries[i].label)<std::tie(queries[j].key,queries[j].m,queries[j].label);});
    Result result;size_t i=0,j=0;
    while(i<a.size()&&j<b.size()) {
        const auto& left=pairs[a[i]].complement;const auto& right=queries[b[j]].key;
        if(left<right){++i;continue;}if(right<left){++j;continue;}
        size_t ie=i+1,je=j+1;while(ie<a.size()&&pairs[a[ie]].complement==left)++ie;while(je<b.size()&&queries[b[je]].key==right)++je;
        for(size_t x=i;x<ie;++x)for(size_t y=j;y<je;++y)retain(result,pairs[a[x]].labels,queries[b[y]],reference);
        i=ie;j=je;
    }
    return result;
}
void agree(const Result& a,const Result& b) {
    require(a.rows==b.rows&&a.slots==b.slots&&a.matches==b.matches&&a.identity_matches==b.identity_matches,
            "independent residual joins agree on every raw slot,row and minimal witness");
}
void verify(const Result& result,const Sparse& reference,const std::vector<AP>& base,const AP& kappa) {
    audit::SSL ssl;
    for(const auto& slot:result.slots) {
        const AP a=shifted::symbol(slot[1],base),b=shifted::symbol(slot[2],base),p=shifted::symbol(slot[3],base);
        require(audit::plus(audit::plus(a,b),norm_test::multiply(p,SCBig(slot[0])))==kappa,"raw residual match GMP target");
        Witness w{slot[0],Pair{slot[1],slot[2]},slot[3],1,1};const auto raw=norm_test::dense(raw_row(w,reference),base.size());
        require(norm_test::row_sum(raw,base).zero&&shifted::named_sum(raw,base,{},ssl),"raw residual match both group engines");
    }
    for(const auto& [row,w]:result.rows) {
        int divisor=1,sign=1;const auto reconstructed=primitive(raw_row(w,reference),divisor,sign);
        require(row==reconstructed&&divisor==w.divisor&&sign==w.sign,"residual primitive row reconstruction");
        require(result.slots.count(Slots{w.m,w.anchors[0],w.anchors[1],w.image}),"retained residual witness present");
        const auto dense=norm_test::dense(row,base.size());require(norm_test::row_sum(dense,base).zero&&shifted::named_sum(dense,base,{},ssl),"primitive residual row both group engines");
    }
}
struct ControlCounts {std::array<size_t,2> matches{},identity_matches{};size_t rows=0;};
ControlCounts control(bool identity) {
    audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());audit::BN x(BN_new()),y(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,x.get(),y.get(),ssl.context.get())==1,"closed residual public generator");
    auto integer=[](const audit::BN& n){char* text=BN_bn2dec(n.get());require(text,"closed residual fixture integer");SCBig value(text);OPENSSL_free(text);return value;};
    AP g{false,integer(x),integer(y)};std::vector<AP> base{g,audit::plus(g,g)};const AP kappa=norm_test::multiply(g,SCBig(identity?2:12));
    const Sparse reference=identity?Sparse{{1,1}}:Sparse{{0,6},{1,3}};
    require(norm_test::row_sum(norm_test::dense(reference,2),base)==kappa&&shifted::named_sum(norm_test::dense(reference,2),base,kappa,ssl),"fixture reference checked before search");
    auto pairs=norm_test::enumerate(base,kappa);complete_pairs(pairs.pairs,base.size());auto queries=multiple_queries(base);
    const auto first=lookup(pairs.pairs,queries.values,reference),second=sorted_merge(pairs.pairs,queries.values,reference);agree(first,second);
    Result brute;
    for(int m:{5,6})for(int a=0;a<5;++a)for(int b=a;b<5;++b)for(int p=0;p<5;++p) {
        const AP q=norm_test::multiply(shifted::symbol(p,base),SCBig(m));
        if(audit::plus(audit::plus(shifted::symbol(a,base),shifted::symbol(b,base)),q)==kappa)
            retain(brute,Pair{a,b},Query{m,p,q,audit::key(q)},reference);
    }
    agree(first,brute);verify(first,reference,base,kappa);
    require(first.rows.count(Sparse{{0,2},{1,-1}})&&first.matches[0]>0&&first.matches[1]>0,"nonempty m5 and m6 known-answer residual rows");
    if(identity)require(first.identity_matches[0]>0&&first.identity_matches[1]>0&&first.rows.count(Sparse{}),"identity-query branch and zero row exercised in both degrees");
    SCMat rows;for(const auto& [row,w]:first.rows){(void)w;rows.push_back(norm_test::dense(row,2));}require(sc_rank(rows)==1,"closed residual fixture rank one");
    auto bad=SCRow{2,-1};++bad[0];require(!norm_test::row_sum(bad,base).zero&&!shifted::named_sum(bad,base,{},ssl),"changed fixture coefficient rejected");
    auto missing=first;missing.rows.erase(Sparse{{0,2},{1,-1}});norm_test::rejects_norm([&]{agree(missing,brute);});
    auto wrong=queries.values[1];wrong.value=audit::plus(wrong.value,g);norm_test::rejects_norm([&]{validate_query(wrong,base,ssl);});
    queries.values.pop_back();norm_test::rejects_norm([&]{complete_queries(queries.values,2);});
    pairs.pairs.pop_back();norm_test::rejects_norm([&]{complete_pairs(pairs.pairs,2);});
    return ControlCounts{first.matches,first.identity_matches,first.rows.size()};
}
void emit_witness(const Witness& w) {
    std::cout<<"{\"m\":"<<w.m<<",\"anchors\":["<<w.anchors[0]<<','<<w.anchors[1]<<"],\"image_label\":"<<w.image<<",\"gcd\":"<<w.divisor<<",\"overall_sign\":"<<w.sign<<'}';
}
}
