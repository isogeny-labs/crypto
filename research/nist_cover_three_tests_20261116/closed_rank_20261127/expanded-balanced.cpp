#define main frozen_expanded_balanced_norm_entrypoint
#include "norms.cpp"
#undef main

namespace expanded_balanced {
using namespace norm_test;
struct Domain {int multiple;size_t pairs;std::string digest;Found found;};
Sparse read_sparse(const Json& input,size_t width) {
    Sparse result;
    for(const auto& term:input.arr) {
        const int column=term.arr.at(0).integer(),value=term.arr.at(1).integer();
        require(column>=0&&size_t(column)<width&&value&&value>=-8&&value<=8&&
                result.emplace(column,value).second,"canonical saved sparse row");
    }
    return result;
}
bool named_zero(const Sparse& row,const std::vector<Point>& base,const audit::SSL& ssl) {
    auto sum=ssl.make({}),term=ssl.make({});
    for(auto [column,value]:row) {
        auto point=ssl.make(base.at(size_t(column)));
        auto scalar=bn(Big(std::abs(value)));
        require(EC_POINT_mul(ssl.group.get(),term.get(),nullptr,point.get(),scalar.get(),ssl.context.get())==1,
                "expanded bound named coefficient");
        if(value<0)require(EC_POINT_invert(ssl.group.get(),term.get(),ssl.context.get())==1,"expanded bound named negation");
        require(EC_POINT_add(ssl.group.get(),sum.get(),sum.get(),term.get(),ssl.context.get())==1,
                "expanded bound named row addition");
    }
    return EC_POINT_is_at_infinity(ssl.group.get(),sum.get())==1;
}
size_t aux_rank(const SCMat& matrix) {
    SCMat auxiliary;
    for(const auto& row:matrix)auxiliary.emplace_back(row.begin()+225,row.end());
    return sc_rank(auxiliary);
}
void complete_pairs(const std::vector<Record>& pairs,size_t width) {
    const size_t alphabet=2*width+1;
    require(pairs.size()==alphabet*(alphabet+1)/2,"complete signed-pair count");
    size_t offset=0;
    for(size_t i=0;i<alphabet;++i)for(size_t j=i;j<alphabet;++j)
        require(pairs.at(offset++).labels==Pair{int(i),int(j)},"complete ordered pair labels");
}
int run(int argc,char** argv) {
    require(argc==4,"usage: expanded-balanced BASELINE TANGENTS EXPANDED_ANCHORS");
    const auto baseline=read_json(argv[1]),tangents=read_json(argv[2]),expanded=read_json(argv[3]);
    require(expanded.at("controls_pass").scalar=="true"&&expanded.at("closed_rank").integer()==6&&
            expanded.at("selected_sections").integer()==128&&expanded.at("expanded_curve_pool").integer()==1434,
            "frozen successful actual expansion");
    std::vector<Point> base,original_base;
    std::set<std::string> labels;
    for(const auto& q:tangents.at("projected_base").arr) {
        const auto p=read_point(q);
        require(!p.zero&&on_curve(p)&&p.y<=::audit::p-p.y&&labels.insert(key(p).substr(1)).second,
                "original canonical point domain");
        original_base.push_back(p);
        if(base.size()<225)base.push_back(p);
    }
    require(original_base.size()==429&&baseline.at("projected_base").arr.size()==225,"fixed original widths");
    for(size_t i=0;i<225;++i)require(base[i]==read_point(baseline.at("projected_base").arr[i]),"old225prefix retained");
    for(const auto& q:expanded.at("selected_images").arr) {
        const auto p=read_point(q);
        require(!p.zero&&on_curve(p)&&p.y<=::audit::p-p.y&&labels.insert(key(p).substr(1)).second,
                "new canonical image outside original429");
        base.push_back(p);
    }
    require(base.size()==481,"fixed expanded481image domain");
    Sparse reference;Point kappa;
    for(const auto& id:baseline.at("pool").at("split_fibers").arr[0].at("point_indices").arr) {
        const auto& image=baseline.at("images").arr.at(size_t(id.integer()));
        kappa=plus(kappa,read_point(image.at("point")));
        const int column=image.at("projected_index").integer();
        if(column>=0)reference[column]+=image.at("sign").integer();
    }
    require(!kappa.zero&&row_sum(dense(reference,base.size()),base)==kappa,"exact reference expression");
    auto original=sc_read_matrix(tangents.at("matrix"));
    auto private_columns=[&](const SCMat& rows) {
        require(rows.size()==108,"original108rows");
        std::vector<int> owners(108);
        for(const auto& row:rows)require(row.size()==429,"original429columns");
        for(size_t column=225;column<429;++column) {
            int owner=-1;
            for(size_t i=0;i<108;++i)if(sc_reduce(rows[i][column])!=0) {
                require(owner<0,"original column remains private");owner=int(i);
            }
            require(owner>=6,"private owner outside original baseline");++owners[size_t(owner)];
        }
        for(size_t i=6;i<108;++i)require(owners[i]==2,"two private witnesses per original added row");
    };
    private_columns(original);
    auto corrupt_private=original;corrupt_private[0][225]=1;
    rejects_norm([&]{private_columns(corrupt_private);});
    require(sc_rank(original)==108,"original full rank108");
    audit::SSL ssl;
    for(const auto& row:original)require(row_sum(row,original_base).zero&&named_zero(sparse(row),original_base,ssl),
                                       "original group rows checked in both engines");
    SCMat core;
    std::set<Sparse> allowed,seeds;
    allowed.insert(Sparse{});
    require(baseline.at("rows").arr.size()==6,"six fixed baseline rows");
    for(size_t i=0;i<6;++i) {
        const auto row=read_sparse(baseline.at("rows").arr[i],225);
        const auto expected=dense(row,429);
        for(size_t j=0;j<429;++j)require(sc_reduce(original[i][j])==sc_reduce(expected[j]),"exact original baseline coefficient binding");
        core.push_back(dense(row,481));allowed.insert(row);allowed.insert(opposite(row));
    }
    for(const auto& record:expanded.at("seed_sections").arr) {
        const auto row=read_sparse(record.at("row"),481);
        require(seeds.insert(row).second&&row_sum(dense(row,481),base).zero&&named_zero(row,base,ssl),
                "distinct actual seed row and both group sums");
        core.push_back(dense(row,481));allowed.insert(row);
    }
    require(seeds.size()==128&&core.size()==134&&sc_rank(core)==134&&aux_rank(core)==128,
            "frozen reduced134/128/6core");
    std::set<Sparse> imported;
    for(const auto& record:expanded.at("norm_rows").arr) {
        const auto row=read_sparse(record.at("row"),481);
        require(allowed.count(row)&&imported.insert(row).second&&row_sum(dense(row,481),base).zero&&named_zero(row,base,ssl),
                "saved kappa norm rows belong to verified seed/baseline span");
    }
    require(imported.size()==135,"all135saved kappa rows imported without enumeration");
    for(const auto& row:seeds)require(imported.count(row),"every seed in imported kappa inventory");

    std::vector<Domain> domains;
    for(int multiple:{0,2}) {
        Sparse ref;
        if(multiple)for(auto [column,value]:reference)if(value)ref[column]=multiple*value;
        auto stream=enumerate(base,multiply(kappa,SCBig(multiple)));
        complete_pairs(stream.pairs,base.size());
        const auto found=grouped(stream.pairs,ref),checked=sorted(stream.pairs,ref);
        require(found==checked,"complete expanded-target grouped/sorted rows and witnesses");
        for(const auto& [row,witness]:found) {
            (void)witness;
            require(row_sum(dense(row,481),base).zero&&named_zero(row,base,ssl),"every new target row in both group engines");
            core.push_back(dense(row,481));
        }
        require(core.size()<=256&&core[0].size()<=1024,"unchanged dense caps;incomplete if exhausted");
        domains.push_back({multiple,stream.pairs.size(),stream.digest,found});
    }
    auto generator=EC_GROUP_get0_generator(ssl.group.get());
    audit::BN gx(BN_new()),gy(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,gx.get(),gy.get(),ssl.context.get())==1,
            "public fixture generator");
    auto integer=[](const audit::BN& n) {
        char* text=BN_bn2dec(n.get());require(text,"fixture decimal");Big out(text);OPENSSL_free(text);return out;
    };
    const Point g{false,integer(gx),integer(gy)};
    const std::vector<Point> fixture{g,plus(g,g)};
    for(int multiple:{0,2}) {
        Sparse ref;if(multiple)ref[0]=4*multiple;
        auto stream=enumerate(fixture,multiply(g,SCBig(4*multiple)));
        complete_pairs(stream.pairs,2);
        const auto a=grouped(stream.pairs,ref),b=sorted(stream.pairs,ref);
        const Sparse expected=multiple?Sparse{{0,-8},{1,4}}:Sparse{{0,-2},{1,1}};
        require(a==b&&a.count(expected)&&row_sum(dense(expected,2),fixture).zero&&named_zero(expected,fixture,ssl),
                "nonempty zero/twice-reference fixture in both engines");
        auto changed=expected;++changed[0];
        require(!row_sum(dense(changed,2),fixture).zero&&!named_zero(changed,fixture,ssl),"changed fixture coefficient rejects");
        auto missing=a;missing.erase(expected);
        rejects_norm([&]{require(missing.count(expected)==1,"missing expected fixture row");});
        stream.pairs.pop_back();rejects_norm([&]{complete_pairs(stream.pairs,2);});
    }
    const size_t full=sc_rank(core),auxiliary=aux_rank(core);
    require(full>=auxiliary+6,"baseline closed rank retained");
    const bool balanced_excluded=domains.at(1).found.empty();
    std::cout<<"{\"scope\":\"expanded-rational-image-balanced-conic-necessary-bound\",\"old_columns\":225,"
             <<"\"new_auxiliary_columns\":256,\"total_image_columns\":481,\"imported_kappa_rows\":135,"
             <<"\"kappa_domain_reenumerated\":false,\"core_rows\":"<<core.size()<<",\"core_columns\":481,"
             <<"\"core_full_rank\":"<<full<<",\"core_auxiliary_rank\":"<<auxiliary
             <<",\"restored_ranks_are_derived\":true,\"full_rank\":"<<full+102<<",\"auxiliary_rank\":"<<auxiliary+102
             <<",\"closed_rank\":"<<full-auxiliary<<",\"candidate_gain\":"<<full-auxiliary-6
             <<",\"balanced_conic_pattern_excluded\":"<<(balanced_excluded?"true":"false")
             <<",\"zero_quartets_only_formal\":"<<((domains[0].found.size()==1&&domains[0].found.count(Sparse{}))?"true":"false")
             <<",\"controls_pass\":true,\"positive_gain_established\":false,\"domains\":[";
    bool comma=false;
    for(const auto& domain:domains) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<"{\"target_multiple\":"<<domain.multiple<<",\"pairs\":"<<domain.pairs
                 <<",\"pair_stream_sha256\":\""<<domain.digest<<"\",\"distinct_rows_including_zero\":"<<domain.found.size()<<",\"rows\":[";
        bool sep=false;
        for(const auto& [row,word]:domain.found) {
            if(sep)std::cout<<',';
            sep=true;
            std::cout<<"{\"symbols\":[";
            for(size_t i=0;i<4;++i)std::cout<<(i?",":"")<<word[i];
            std::cout<<"],\"row\":";sc_emit_row(std::cout,dense(row,481));std::cout<<'}';
        }
        std::cout<<"]}";
    }
    std::cout<<"],\"standing\":\"producer-unverified;necessary image bound only;positive candidates need actual cover/KM and separate replay\"}\n";
    return 0;
}
}
int main(int argc,char** argv) {
    try{return expanded_balanced::run(argc,argv);}
    catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}
}
