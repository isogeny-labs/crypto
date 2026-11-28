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
int run(int argc,char** argv) {
    require(argc==6,"usage: shifted-six QUARTIC BACKMAP BASELINE TANGENTS PRIOR_SIX");
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),tangents=read_json(argv[4]),prior=read_json(argv[5]);
    require(prior.at("domain_columns").integer()==225&&prior.at("closed_rank").integer()==6&&
            prior.at("distinct_nonzero_spanning_rows").integer()==0&&prior.at("controls_pass").scalar=="true","bound completed zero-target six result");
    ss_private(baseline,tangents);const auto curve=read_bp(model.at("terms"),true);std::array<BP,4> map;size_t position=0;
    for(const auto* name:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[position++]=read_bp(back.at(name));
    require(global_identity(curve,map),"reference actual map global identity");
    std::vector<AP> base,all_base;std::set<std::string> keys;
    for(const auto& q:tangents.at("projected_base").arr) {
        auto p=audit::read_point(q);require(!p.zero&&audit::on_curve(p)&&p.y<=audit::p-p.y&&keys.insert(p.x.get_str()).second,"distinct canonical tangent columns");all_base.push_back(p);
    }
    require(all_base.size()==429&&baseline.at("projected_base").arr.size()==225,"frozen column dimensions");
    for(size_t i=0;i<225;++i){const auto p=audit::read_point(baseline.at("projected_base").arr[i]);require(p==all_base[i],"exact original prefix");base.push_back(p);}
    Sparse reference;AP rho;::Point rho_bn;
    for(size_t i=0;i<3;++i) {
        const size_t id=223+i;const auto& q=baseline.at("pool").at("points").arr.at(id);
        const auto x=coefficient(q.at("x")),y=coefficient(q.at("y"));require(fs_eval(curve,x,y)==F(0),"reference point lies on actual curve");
        const auto value=fs_image(curve,map,x,y).value;require(value==point(baseline.at("images").arr.at(id).at("point")),"fresh map equals saved original image");
        const AP actual=gmp_point(value);require(actual==audit::negative(base[222+i]),"fixed parent reference labels -222,-223,-224");
        reference[int(222+i)]-=2;rho=audit::plus(rho,actual);rho_bn=add(rho_bn,value);
    }
    require(gmp_point(rho_bn)==rho,"reference trace separate arithmetic equality");
    const AP target=audit::plus(rho,rho);require(!target.zero,"shifted six target zero: reuse historical result instead of rerun");
    audit::SSL ssl;require(named_sum(norm_test::dense(reference,225),base,target,ssl),"reference coefficient target named check");
    AP kappa;
    for(const auto& id:baseline.at("pool").at("split_fibers").arr.at(0).at("point_indices").arr)
        kappa=audit::plus(kappa,audit::read_point(baseline.at("images").arr.at(size_t(id.integer())).at("point")));
    SCMat old(6,SCRow(225));
    require(baseline.at("rows").arr.size()==6,"exact original six rows");
    for(size_t i=0;i<6;++i) {
        for(const auto& pair:baseline.at("rows").arr[i].arr)old[i].at(size_t(pair.arr[0].integer()))+=SCBig(pair.arr[1].scalar);
        require(norm_test::row_sum(old[i],base).zero&&named_sum(old[i],base,{},ssl),"original baseline row both engines");
    }
    require(sc_rank(old)==6,"frozen original six-dimensional span");auto full=sc_read_matrix(tangents.at("matrix"));
    require(full.size()==108&&sc_rank(full)==108,"full tangent rank108");SCMat auxiliary;
    for(const auto& row:full){require(row.size()==429&&norm_test::row_sum(row,all_base).zero&&named_sum(row,all_base,{},ssl),"full tangent rows both engines");auxiliary.emplace_back(row.begin()+225,row.end());}
    require(sc_rank(auxiliary)==102,"original auxiliary rank102");MIBasis original_basis(old),full_basis(full);
    controls();auto table=enumerate(base,rho);Counts grouped_counts,merged_counts;
    const auto found=grouped(table,reference,grouped_counts),checked=merged(table,reference,merged_counts);
    require(found==checked&&grouped_counts.opposite_pairs==merged_counts.opposite_pairs&&grouped_counts.identity_pairs==merged_counts.identity_pairs,
            "complete shifted joins and exact witness sets agree");
    Word formal{444,444,446,446,448,448};Found required;keep(required,formal,reference);
    require(required.size()==1&&required.begin()->first.empty()&&found.count(Sparse{}),"doubled-reference formal zero word recovered");
    verify(found,reference,base,target);size_t independent=0;
    for(const auto& [s,w]:found) {
        (void)w;auto row=norm_test::dense(s,225);const auto dependent=original_basis.representation(row).first;auto extended=row;extended.resize(429);
        require(full_basis.representation(extended).first==dependent,"old-column and auxiliary-eliminated candidate dependence agrees");
        if(!dependent){original_basis.append(row,"shifted-candidate:"+std::to_string(independent));full_basis.append(extended,"shifted-candidate:"+std::to_string(independent));++independent;}
    }
    const size_t closed=sc_rank(original_basis.source),frank=sc_rank(full_basis.source);SCMat final_aux;
    for(const auto& row:full_basis.source)final_aux.emplace_back(row.begin()+225,row.end());
    const auto arank=sc_rank(final_aux);require(arank==102&&frank-arank==closed&&closed==6+independent,"separate actual closure check");
    std::cout<<"{\"scope\":\"reference-shifted-degree-six-necessary-image-bound\",\"old_columns\":225,\"signed_alphabet\":451,\"reference_curve_indices\":[223,224,225],\"reference\":";
    sc_emit_row(std::cout,norm_test::dense(reference,225));std::cout<<",\"rho\":";emit_point(rho);std::cout<<",\"target\":";emit_point(target);
    std::cout<<",\"target_is_zero\":false,\"target_is_kappa\":"<<(target==kappa?"true":"false")<<",\"target_is_twice_kappa\":"<<(target==audit::plus(kappa,kappa)?"true":"false")
             <<",\"unordered_triples\":"<<table.rows.size()<<",\"packed_record_bytes\":32,\"table_payload_bytes\":"<<table.rows.size()*sizeof(Packed)
             <<",\"join_index_payload_bytes\":"<<merged_counts.index_payload<<",\"triple_stream_sha256\":\""<<table.digest
             <<"\",\"independent_triple_streams_match\":true,\"independent_join_sets_match\":true,\"sum_groups\":"<<grouped_counts.groups
             <<",\"largest_sum_group\":"<<grouped_counts.largest<<",\"opposite_orientation_pairs\":"<<grouped_counts.opposite_pairs
             <<",\"identity_pairs\":"<<grouped_counts.identity_pairs<<",\"distinct_rows_including_zero\":"<<found.size()
             <<",\"independent_added_rows\":"<<independent<<",\"matrix_rows\":"<<full_basis.source.size()<<",\"matrix_columns\":429,\"full_rank\":"<<frank
             <<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<closed<<",\"candidate_gain\":"<<closed-6
             <<",\"controls_pass\":true,\"fixture_rank\":1,\"fixture_rows_in_actual_matrix\":0,\"formal_reference_recovered\":true,\"positive_gain_established\":false,\"rows\":[";
    bool comma=false;for(const auto& [row,w]:found) {
        if(comma)std::cout<<',';
        comma=true;std::cout<<"{\"labels\":[";
        for(size_t i=0;i<6;++i)std::cout<<(i?",":"")<<w.labels[i];
        std::cout<<"],\"overall_sign\":"<<w.sign<<",\"row\":";sc_emit_row(std::cout,norm_test::dense(row,225));std::cout<<'}';
    }
    std::cout<<"],\"standing\":\"producer-unverified;necessary image condition only;independent candidates need actual RR/KM and separate replay\"}\n";
    return 0;
}
}
int main(int argc,char** argv){try{return shifted::run(argc,argv);}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
