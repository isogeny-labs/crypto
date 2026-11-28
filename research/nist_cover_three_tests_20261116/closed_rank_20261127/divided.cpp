#define main frozen_divided_norm_entrypoint
#include "norms.cpp"
#undef main
#include <numeric>

namespace divided_test {
using namespace norm_test;
struct Witness {
    int denominator=1;
    Pair labels{};
    Sparse expression;
};
struct Candidate {
    Witness first,second;
    bool identity=false;
    std::string x;
    int divisor=1,sign=1;
};
struct Family {
    int denominator=1;
    size_t pairs=0,divisions=0,identity=0,equal=0,unequal=0;
    std::string digest;
};
struct Search {
    std::map<Sparse,Candidate> candidates;
    std::vector<Family> families;
    size_t distinct_x=0;
};

void division_check(const Point& divided,const Point& original,int m,const audit::SSL& ssl) {
    require(multiply(divided,SCBig(m))==original,"GMP divided point multiplication back");
    auto p=ssl.make(divided),result=ssl.make({});
    auto scalar=bn(Big(m));
    require(EC_POINT_mul(ssl.group.get(),result.get(),nullptr,p.get(),scalar.get(),ssl.context.get())==1,
            "named-curve divided point multiplication");
    require(ssl.encode(result.get())==key(original),"named-curve divided point multiplication back");
}

bool ssl_row_zero(const Sparse& row,const std::vector<Point>& base,const audit::SSL& ssl) {
    auto sum=ssl.make({}),term=ssl.make({});
    for(auto [column,coefficient]:row) {
        auto p=ssl.make(base.at(size_t(column)));
        auto scalar=bn(Big(std::abs(coefficient)));
        require(EC_POINT_mul(ssl.group.get(),term.get(),nullptr,p.get(),scalar.get(),ssl.context.get())==1,
                "named-curve candidate multiplication");
        if(coefficient<0)require(EC_POINT_invert(ssl.group.get(),term.get(),ssl.context.get())==1,
                                 "named-curve candidate negation");
        require(EC_POINT_add(ssl.group.get(),sum.get(),sum.get(),term.get(),ssl.context.get())==1,
                "named-curve candidate sum");
    }
    return EC_POINT_is_at_infinity(ssl.group.get(),sum.get())==1;
}

Sparse primitive(Sparse row,int& divisor,int& sign) {
    for(auto i=row.begin();i!=row.end();) {
        if(i->second==0)i=row.erase(i);else ++i;
    }
    divisor=0;
    for(auto [column,coefficient]:row) {
        (void)column;
        require(coefficient>=-128&&coefficient<=128,"bounded divided coefficient");
        divisor=std::gcd(divisor,std::abs(coefficient));
    }
    sign=1;
    if(row.empty()){divisor=1;return row;}
    require(divisor>0&&Big(divisor)<sc_n,"normalizer is a nonzero unit modulo prime order");
    if(row.begin()->second<0)sign=-1;
    for(auto& [column,coefficient]:row){(void)column;coefficient=coefficient/divisor*sign;}
    return row;
}

void retain(Search& out,const Witness& first,const Witness& second,bool identity,const std::string& x) {
    Sparse row=first.expression;
    if(!identity)for(auto [column,coefficient]:second.expression)row[column]-=coefficient;
    Candidate witness{first,second,identity,x,1,1};
    row=primitive(std::move(row),witness.divisor,witness.sign);
    if(!row.empty())out.candidates.emplace(std::move(row),std::move(witness));
    require(out.candidates.size()<=4096,"distinct divided rows cap; incomplete if exhausted");
}

void complete(const std::vector<Record>& pairs,size_t width) {
    const size_t alphabet=2*width+1;
    require(pairs.size()==alphabet*(alphabet+1)/2,"complete divided signed pair domain");
    size_t offset=0;
    for(size_t i=0;i<alphabet;++i)for(size_t j=i;j<alphabet;++j) {
        require(pairs.at(offset).labels==Pair{int(i),int(j)},"complete ordered divided pair labels");
        ++offset;
    }
}

Search search(const std::vector<Point>& base,const Point& kappa,const Sparse& reference,
              const std::string& expected_first_digest) {
    audit::SSL ssl;
    Search out;
    std::map<std::string,Witness> labels;
    for(int m:{1,2,3}) {
        Big inverse;
        require(mpz_invert(inverse.get_mpz_t(),Big(m).get_mpz_t(),sc_n.get_mpz_t())!=0,
                "denominator invertible modulo order");
        Family family;
        family.denominator=m;
        std::vector<Point> divided;
        for(const auto& p:base) {
            auto q=multiply(p,inverse);
            division_check(q,p,m,ssl);
            divided.push_back(std::move(q));
            ++family.divisions;
        }
        auto target=multiply(kappa,inverse);
        division_check(target,kappa,m,ssl);
        ++family.divisions;
        auto stream=enumerate(divided,target);
        complete(stream.pairs,base.size());
        family.pairs=stream.pairs.size();
        family.digest=stream.digest;
        if(m==1&&!expected_first_digest.empty())require(stream.digest==expected_first_digest,
                                                      "unchanged original aggregate stream");
        for(const auto& record:stream.pairs) {
            auto row=row_for(Quad{record.labels[0],record.labels[1],0,0},reference);
            const bool identity=record.complement==std::string(1,'\0');
            require(identity||record.complement.size()==25,"complete compressed label width");
            const int orientation=identity||record.complement[0]==2?1:-1;
            require(identity||record.complement[0]==2||record.complement[0]==3,"compressed label parity");
            for(auto& [column,coefficient]:row){(void)column;coefficient*=-(6/m)*orientation;}
            Witness witness{m,record.labels,std::move(row)};
            if(identity) {
                ++family.identity;
                retain(out,witness,{},true,{});
                continue;
            }
            auto x=record.complement.substr(1);
            auto [position,inserted]=labels.emplace(x,witness);
            if(inserted)continue;
            if(position->second.expression==witness.expression){++family.equal;continue;}
            ++family.unequal;
            retain(out,witness,position->second,false,x);
        }
        out.families.push_back(std::move(family));
    }
    out.distinct_x=labels.size();
    for(const auto& [row,witness]:out.candidates) {
        (void)witness;
        require(row_sum(dense(row,base.size()),base).zero,"GMP primitive candidate group sum");
        require(ssl_row_zero(row,base,ssl),"named-curve primitive candidate group sum");
    }
    return out;
}

void emit_sparse(const Sparse& row) {
    std::cout<<'[';
    bool comma=false;
    for(auto [column,coefficient]:row) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<'['<<column<<','<<coefficient<<']';
    }
    std::cout<<']';
}
void emit_witness(const Witness& witness) {
    std::cout<<"{\"denominator\":"<<witness.denominator<<",\"symbols\":["
             <<witness.labels[0]<<','<<witness.labels[1]<<"],\"six_times_label_expression\":";
    emit_sparse(witness.expression);
    std::cout<<'}';
}

int run(int argc,char** argv) {
    require(argc==4,"usage: divided BASELINE TANGENTS OLD_NORM");
    const auto baseline=read_json(argv[1]),tangents=read_json(argv[2]),norm=read_json(argv[3]);
    std::vector<Point> base;
    for(const auto& p:baseline.at("projected_base").arr) {
        auto q=read_point(p);
        require(!q.zero&&on_curve(q)&&q.y<=::audit::p-q.y,"canonical actual original point");
        base.push_back(std::move(q));
    }
    require(base.size()==225&&tangents.at("projected_base").arr.size()==429,"frozen widths");
    for(size_t i=0;i<base.size();++i)require(base[i]==read_point(tangents.at("projected_base").arr[i]),
                                         "old column prefix unchanged");
    Sparse reference;
    Point kappa;
    for(const auto& i:baseline.at("pool").at("split_fibers").arr[0].at("point_indices").arr) {
        const auto& image=baseline.at("images").arr.at(size_t(i.integer()));
        kappa=plus(kappa,read_point(image.at("point")));
        const int column=image.at("projected_index").integer();
        if(column>=0)reference[column]+=image.at("sign").integer();
    }
    require(!kappa.zero&&row_sum(dense(reference,base.size()),base)==kappa,"exact old trace expression");
    SCMat original(6,SCRow(base.size()));
    require(baseline.at("rows").arr.size()==6,"frozen six baseline rows");
    for(size_t i=0;i<6;++i)for(const auto& entry:baseline.at("rows").arr[i].arr)
        original[i].at(size_t(entry.arr[0].integer()))+=SCBig(entry.arr[1].scalar);
    for(const auto& row:original)require(row_sum(row,base).zero,"old actual elliptic baseline row");
    require(sc_rank(original)==6,"old baseline rank6");
    auto augmented=sc_read_matrix(tangents.at("matrix"));
    require(sc_rank(augmented)==108,"original augmented rank108");
    for(const auto& row:augmented)require(row.size()==429,"original augmented width429");

    // The same production search runs against an independent, nonempty fixture.
    audit::SSL ssl;
    const auto generator=EC_GROUP_get0_generator(ssl.group.get());
    BN gx(BN_new()),gy(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,gx.get(),gy.get(),ssl.context.get())==1,
            "public generator fixture");
    auto integer=[](const BN& n){char* text=BN_bn2dec(n.get());require(text,"generator decimal");
                               Big result(text);OPENSSL_free(text);return result;};
    Point g{false,integer(gx),integer(gy)};
    std::vector<Point> fixture{g,plus(g,g)};
    auto control=search(fixture,multiply(g,4),{{0,4}},{});
    require(control.candidates.count(Sparse{{0,2},{1,-1}})==1,"nonempty primitive fixture relation");
    SCMat control_rows;
    size_t control_identity=0;
    for(const auto& family:control.families)control_identity+=family.identity;
    for(const auto& [row,witness]:control.candidates){(void)witness;control_rows.push_back(dense(row,2));}
    require(sc_rank(control_rows)==1&&control_identity>0,"fixture rank1 and identity branches exercised");
    Big inverse2;
    require(mpz_invert(inverse2.get_mpz_t(),Big(2).get_mpz_t(),sc_n.get_mpz_t())!=0,"fixture half");
    rejects_norm([&]{division_check(plus(multiply(g,inverse2),g),g,2,ssl);});
    Sparse corrupted{{0,3},{1,-1}};
    require(!row_sum(dense(corrupted,2),fixture).zero&&!ssl_row_zero(corrupted,fixture,ssl),
            "changed primitive coefficient rejected by both implementations");
    auto incomplete=enumerate(fixture,multiply(g,4));
    incomplete.pairs.pop_back();
    rejects_norm([&]{complete(incomplete.pairs,fixture.size());});

    auto found=search(base,kappa,reference,norm.at("pair_stream_sha256").scalar);
    SCMat combined=original;
    for(const auto& [row,witness]:found.candidates) {
        (void)witness;
        combined.push_back(dense(row,225));
        augmented.push_back(dense(row,429));
    }
    const size_t rank=sc_rank(combined),full=sc_rank(augmented);
    SCMat auxiliary;
    for(const auto& row:augmented)auxiliary.emplace_back(row.begin()+225,row.end());
    const size_t arank=sc_rank(auxiliary);
    require(arank==102&&full>=arank&&full-arank==rank&&rank>=6,"old and augmented closure agree");
    std::cout<<"{\"scope\":\"divided-old-pair-aggregate-alias-candidates\",\"old_columns\":225,"
             <<"\"denominators\":[1,2,3],\"common_label_multiple\":6,\"families\":[";
    bool comma=false;
    for(const auto& family:found.families) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<"{\"denominator\":"<<family.denominator<<",\"pairs\":"<<family.pairs
                 <<",\"checked_divisions\":"<<family.divisions<<",\"pair_stream_sha256\":\""
                 <<family.digest<<"\",\"identity_labels\":"<<family.identity
                 <<",\"equal_expression_aliases\":"<<family.equal
                 <<",\"unequal_expression_aliases\":"<<family.unequal<<'}';
    }
    std::cout<<"],\"distinct_nonidentity_x\":"<<found.distinct_x
             <<",\"distinct_nonzero_candidate_rows\":"<<found.candidates.size()
             <<",\"original_rank\":6,\"candidate_combined_rank\":"<<rank
             <<",\"full_rank\":"<<full<<",\"auxiliary_rank\":"<<arank
             <<",\"closed_rank\":"<<rank<<",\"candidate_gain\":"<<rank-6
             <<",\"controls_pass\":true,\"control_rank\":1,\"control_identity_labels\":"<<control_identity
             <<",\"positive_gain_established\":false,\"rows\":[";
    comma=false;
    for(const auto& [row,witness]:found.candidates) {
        if(comma)std::cout<<',';
        comma=true;
        Big x;
        if(!witness.x.empty())mpz_import(x.get_mpz_t(),witness.x.size(),1,1,1,0,witness.x.data());
        std::cout<<"{\"identity\":"<<(witness.identity?"true":"false")<<",\"shared_x\":";
        if(witness.identity)std::cout<<"null";else std::cout<<'"'<<x.get_str()<<'"';
        std::cout<<",\"normalizer_gcd\":"<<witness.divisor<<",\"normalizer_sign\":"<<witness.sign
                 <<",\"first\":";
        emit_witness(witness.first);
        std::cout<<",\"second\":";
        if(witness.identity)std::cout<<"null";else emit_witness(witness.second);
        std::cout<<",\"row\":";
        emit_sparse(row);
        std::cout<<'}';
    }
    std::cout<<"],\"standing\":\"producer-unverified; elliptic candidates require actual cover/KM realization and separate replay\"}\n";
    return 0;
}
}
int main(int argc,char** argv) {
    try{return divided_test::run(argc,argv);}
    catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}
}
