#pragma once
// Native engine copied byte-for-byte from completed shifted-six-v2 through its common helpers.
#define main frozen_norm_entrypoint
#include "norms.cpp"
#undef main
#include "sections_arithmetic.hpp"
#include "shifted-six-basis.hpp"
#include <cstdint>
#include <limits>

namespace shifted {
using AP=audit::Point;
using Sparse=norm_test::Sparse;
using Packed=std::array<unsigned char,32>;
using Word=std::array<int,6>;
static_assert(sizeof(Packed)==32,"packed triple size");
struct Witness {
    Word labels{};
    int sign=1;
    bool operator==(const Witness& other)const{return labels==other.labels&&sign==other.sign;}
};
using Found=std::map<Sparse,Witness>;
struct Table {std::vector<Packed> rows;std::string digest;};
struct Counts {
    std::uint64_t opposite_pairs=0,identity_pairs=0;
    size_t groups=0,largest=0,index_payload=0;
};
struct TripHash {
    norm_test::Digest hash;
    void add(const std::array<int,3>& labels,const std::string& point) {
        std::array<unsigned char,6> bytes{};
        for(size_t i=0;i<3;++i){bytes[2*i]=static_cast<unsigned char>(labels[i]&255);bytes[2*i+1]=static_cast<unsigned char>((labels[i]>>8)&255);}
        const auto n=static_cast<unsigned char>(point.size());
        require(EVP_DigestUpdate(hash.c.get(),bytes.data(),bytes.size())==1&&
                EVP_DigestUpdate(hash.c.get(),&n,1)==1&&
                EVP_DigestUpdate(hash.c.get(),point.data(),point.size())==1,"shifted triple hash frame");
    }
};
int label(const Packed& row,size_t i){return int(row[25+2*i])+(int(row[26+2*i])<<8);}
bool same_key(const Packed& a,const Packed& b){return std::equal(a.begin(),a.begin()+25,b.begin());}
bool less_key(const Packed& a,const Packed& b){return std::lexicographical_compare(a.begin(),a.begin()+25,b.begin(),b.begin()+25);}
size_t expected_count(size_t n){return n*(n+1)*(n+2)/6;}
void complete(const Table& table,size_t symbols){require(table.rows.size()==expected_count(symbols),"all unordered shifted triples present");}
AP symbol(int id,const std::vector<AP>& base) {
    if(id==0)return {};
    require(id>0&&size_t(id)<=2*base.size(),"signed label bounds");
    const auto& p=base.at(size_t((id-1)/2));return id%2?p:audit::negative(p);
}
std::string encoding(const Packed& record) {
    if(record[0]==0){require(record[31]==0,"identity orientation");return std::string(1,'\0');}
    require(record[0]==2&&(record[31]==1||record[31]==2),"valid canonical orientation");
    std::string out(25,'\0');out[0]=record[31]==1?2:3;
    for(size_t i=1;i<25;++i)out[i]=char(record[i]);
    return out;
}
Table enumerate(const std::vector<AP>& base,const AP& rho) {
    require(base.size()<=225&&!rho.zero,"bounded nonzero shifted target");
    std::vector<AP> symbols(1),shifted_symbols;
    for(const auto& p:base){require(!p.zero&&audit::on_curve(p),"valid signed source column");symbols.push_back(p);symbols.push_back(audit::negative(p));}
    audit::SSL ssl;std::vector<audit::SSLPoint> points,third;
    auto minus_rho=ssl.make(audit::negative(rho));
    for(const auto& p:symbols) {
        points.push_back(ssl.make(p));
        shifted_symbols.push_back(audit::plus(p,audit::negative(rho)));
        auto q=ssl.make({});
        require(EC_POINT_add(ssl.group.get(),q.get(),points.back().get(),minus_rho.get(),ssl.context.get())==1&&
                ssl.encode(q.get())==audit::key(shifted_symbols.back()),"separate shifted third symbols");
        third.push_back(std::move(q));
    }
    auto pair=ssl.make({}),sum=ssl.make({});Table table;
    table.rows.reserve(expected_count(symbols.size()));TripHash first,second;
    for(size_t i=0;i<symbols.size();++i) {
        for(size_t j=i;j<symbols.size();++j) {
            const auto ab=audit::plus(symbols[i],symbols[j]);
            require(EC_POINT_add(ssl.group.get(),pair.get(),points[i].get(),points[j].get(),ssl.context.get())==1,
                    "named shifted pair");
            for(size_t k=j;k<symbols.size();++k) {
                require(EC_POINT_add(ssl.group.get(),sum.get(),pair.get(),third[k].get(),ssl.context.get())==1,
                        "named shifted triple");
                const auto encoded=ssl.encode(sum.get()),checked=audit::key(audit::plus(ab,shifted_symbols[k]));
                require(encoded==checked,"every shifted triple agrees between native engines");
                const std::array<int,3> labels{int(i),int(j),int(k)};first.add(labels,encoded);second.add(labels,checked);
                Packed row{};
                if(encoded.size()==25) {
                    require(encoded[0]==2||encoded[0]==3,"compressed shifted orientation");row[0]=2;
                    for(size_t t=1;t<25;++t)row[t]=static_cast<unsigned char>(encoded[t]);
                    row[31]=encoded[0]==2?1:2;
                } else require(encoded==std::string(1,'\0'),"shifted identity encoding");
                for(size_t t=0;t<3;++t){row[25+2*t]=static_cast<unsigned char>(labels[t]&255);row[26+2*t]=static_cast<unsigned char>((labels[t]>>8)&255);}
                table.rows.push_back(row);
            }
        }
        if(i%32==0)std::cerr<<"shifted triple first="<<i<<" records="<<table.rows.size()<<'\n';
    }
    complete(table,symbols.size());table.digest=first.hash.finish();
    require(table.digest==second.hash.finish(),"complete shifted stream hashes agree");return table;
}
Sparse coefficients(const Word& word,const Sparse& reference) {
    Sparse row;
    for(int id:word)if(id)row[(id-1)/2]+=id%2?1:-1;
    for(auto [j,c]:reference)row[j]-=c;
    for(auto at=row.begin();at!=row.end();)if(at->second==0)at=row.erase(at);else ++at;
    return row;
}
void keep(Found& out,Word word,const Sparse& reference) {
    std::sort(word.begin(),word.end());auto row=coefficients(word,reference);Witness witness{word,1};
    if(!row.empty()&&row.begin()->second<0){row=norm_test::opposite(row);witness.sign=-1;}
    auto [at,inserted]=out.emplace(row,witness);
    if(!inserted&&word<at->second.labels)at->second=witness;
    require(out.size()<=4096,"distinct shifted row cap exceeded; completeness rejected");
}
void pair_words(Found& out,const Packed& a,const Packed& b,const Sparse& reference) {
    require(same_key(a,b)&&(a[0]==0||(a[31]+b[31]==3)),"opposite shifted sums or two identities");
    Word word{};for(size_t t=0;t<3;++t){word[t]=label(a,t);word[t+3]=label(b,t);}keep(out,word,reference);
}
Found grouped(Table& table,const Sparse& reference,Counts& counts) {
    std::sort(table.rows.begin(),table.rows.end());Found found;
    for(size_t start=0;start<table.rows.size();) {
        size_t end=start+1;while(end<table.rows.size()&&same_key(table.rows[start],table.rows[end]))++end;
        ++counts.groups;counts.largest=std::max(counts.largest,end-start);
        if(table.rows[start][0]==0) {
            for(size_t a=start;a<end;++a)for(size_t b=a;b<end;++b){pair_words(found,table.rows[a],table.rows[b],reference);++counts.identity_pairs;}
        } else {
            for(size_t a=start;a<end;++a)if(table.rows[a][31]==1)
                for(size_t b=start;b<end;++b)if(table.rows[b][31]==2){pair_words(found,table.rows[a],table.rows[b],reference);++counts.opposite_pairs;}
        }
        start=end;
    }
    return found;
}
Found merged(const Table& table,const Sparse& reference,Counts& counts) {
    require(table.rows.size()<=std::numeric_limits<std::uint32_t>::max(),"bounded index width");
    std::array<size_t,3> sizes{};for(const auto& row:table.rows){require(row[31]<=2,"bounded orientation");++sizes[row[31]];}
    std::array<std::vector<std::uint32_t>,3> lists;
    for(size_t i=0;i<3;++i)lists[i].reserve(sizes[i]);
    for(size_t i=0;i<table.rows.size();++i)lists[table.rows[i][31]].push_back(std::uint32_t(i));
    counts.index_payload=table.rows.size()*sizeof(std::uint32_t);Found found;
    for(size_t a=0;a<lists[0].size();++a)for(size_t b=a;b<lists[0].size();++b) {
        pair_words(found,table.rows[lists[0][a]],table.rows[lists[0][b]],reference);++counts.identity_pairs;
    }
    const auto& plus=lists[1];const auto& minus=lists[2];size_t a=0,b=0;
    while(a<plus.size()&&b<minus.size()) {
        const auto& p=table.rows[plus[a]];const auto& n=table.rows[minus[b]];
        if(less_key(p,n)){++a;continue;}if(less_key(n,p)){++b;continue;}
        size_t ae=a+1,be=b+1;
        while(ae<plus.size()&&same_key(p,table.rows[plus[ae]]))++ae;
        while(be<minus.size()&&same_key(n,table.rows[minus[be]]))++be;
        for(size_t i=a;i<ae;++i)for(size_t j=b;j<be;++j) {
            pair_words(found,table.rows[plus[i]],table.rows[minus[j]],reference);++counts.opposite_pairs;
        }
        a=ae;b=be;
    }
    return found;
}
bool named_sum(const SCRow& row,const std::vector<AP>& base,const AP& target,const audit::SSL& ssl) {
    auto sum=ssl.make({}),term=ssl.make({});
    for(size_t i=0;i<row.size();++i)if(row[i]!=0) {
        SCBig c=sc_reduce(row[i]);if(c>sc_n/2)c-=sc_n;
        SCBig magnitude=c;if(magnitude<0)magnitude=-magnitude;
        auto p=ssl.make(base.at(i));auto scalar=audit::bn(magnitude);
        require(EC_POINT_mul(ssl.group.get(),term.get(),nullptr,p.get(),scalar.get(),ssl.context.get())==1,"named row coefficient");
        if(c<0)require(EC_POINT_invert(ssl.group.get(),term.get(),ssl.context.get())==1,"named row sign");
        require(EC_POINT_add(ssl.group.get(),sum.get(),sum.get(),term.get(),ssl.context.get())==1,"named row sum");
    }
    return ssl.encode(sum.get())==audit::key(target);
}
void verify(const Found& found,const Sparse& reference,const std::vector<AP>& base,const AP& target) {
    audit::SSL ssl;
    for(const auto& [row,witness]:found) {
        auto reconstructed=coefficients(witness.labels,reference);
        if(witness.sign<0)reconstructed=norm_test::opposite(reconstructed);
        require((witness.sign==1||witness.sign==-1)&&row==reconstructed,"retained shifted coefficients and sign");
        AP sum;SCRow word(base.size());for(int id:witness.labels)if(id){sum=audit::plus(sum,symbol(id,base));word[size_t((id-1)/2)]+=id%2?1:-1;}
        require(sum==target&&named_sum(word,base,target,ssl),"retained word target in both group engines");
        const auto dense=norm_test::dense(row,base.size());
        require(norm_test::row_sum(dense,base).zero&&named_sum(dense,base,{},ssl),"retained coefficient row in both group engines");
    }
}
void validate_record(const Packed& row,const std::vector<AP>& base,const AP& rho) {
    AP sum;
    for(size_t i=0;i<3;++i)sum=audit::plus(sum,symbol(label(row,i),base));
    sum=audit::plus(sum,audit::negative(rho));
    require(encoding(row)==audit::key(sum),"record labels and shifted orientation match");
}
void controls() {
    audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());audit::BN x(BN_new()),y(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,x.get(),y.get(),ssl.context.get())==1,"public fixture generator");
    auto integer=[](const audit::BN& n){char* text=BN_bn2dec(n.get());require(text,"fixture integer");SCBig out(text);OPENSSL_free(text);return out;};
    AP g{false,integer(x),integer(y)};std::vector<AP> base{g,audit::plus(g,g)};
    const AP rho=norm_test::multiply(g,3),target=norm_test::multiply(g,6);const Sparse reference{{0,6}};
    auto table=enumerate(base,rho);Counts a,b;auto first=grouped(table,reference,a),second=merged(table,reference,b);
    require(first==second&&a.identity_pairs>0&&a.opposite_pairs>0&&a.identity_pairs==b.identity_pairs&&a.opposite_pairs==b.opposite_pairs,
            "nonempty control joins and branch counts agree");
    Found brute;Word word{};
    std::function<void(size_t,int,AP)> visit=[&](size_t slot,int minimum,AP sum) {
        if(slot==word.size()){if(sum==target)keep(brute,word,reference);return;}
        for(int id=minimum;id<5;++id){word[slot]=id;visit(slot+1,id,audit::plus(sum,symbol(id,base)));}
    };
    visit(0,0,{});require(first==brute,"both joins equal independent brute unordered six-word candidates and witnesses");
    require(first.count(Sparse{})&&first.count(Sparse{{0,6},{1,-3}}),"zero and nonzero fixture rows recovered");
    verify(first,reference,base,target);SCMat rows;for(const auto& [row,w]:first){(void)w;rows.push_back(norm_test::dense(row,2));}
    require(sc_rank(rows)==1,"nonzero fixture rank one");
    auto bad=norm_test::dense(Sparse{{0,6},{1,-3}},2);++bad[0];
    norm_test::rejects_norm([&]{require(norm_test::row_sum(bad,base).zero&&named_sum(bad,base,{},ssl),"changed fixture row");});
    auto missing=first;missing.erase(Sparse{{0,6},{1,-3}});
    norm_test::rejects_norm([&]{require(missing==brute,"missing known nonzero fixture row");});
    bool mutated=false;for(auto row:table.rows)if(row[0]){row[31]=row[31]==1?2:1;norm_test::rejects_norm([&]{validate_record(row,base,rho);});mutated=true;break;}
    require(mutated,"orientation mutation actually exercised");
    for(const auto& row:table.rows)validate_record(row,base,rho);
    table.rows.pop_back();norm_test::rejects_norm([&]{complete(table,5);});
}
AP gmp_point(const ::Point& p) {
    if(p.zero)return {};
    return {false,SCBig(decimal(p.x.p.at(0))),SCBig(decimal(p.y.p.at(0)))};
}
void emit_point(const AP& p) {
    std::cout<<"{\"infinity\":"<<(p.zero?"true":"false");
    if(!p.zero)std::cout<<",\"x\":\""<<p.x<<"\",\"y\":\""<<p.y<<'"';
    std::cout<<'}';
}
}
