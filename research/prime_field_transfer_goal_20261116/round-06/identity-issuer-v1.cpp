// Fixed integer coefficient identities only; never linked into a recognizer.
#include "../round-04/identity_support.hpp"
std::ofstream fresh(const std::filesystem::path& p) {
    if(std::filesystem::exists(p))throw std::runtime_error("refusing output replacement: "+p.string());
    return std::ofstream(p);
}
int main(int argc,char** argv)try {
    if(argc!=3)throw std::runtime_error("usage: issuer OUTDIR LEAN_SOURCE");
    Expr a=var("a"),b=var("b");
    Expr pa=add({a,mul({num(106),b})}),pb=add({a,mul({num(92),b})});
    Expr da=add({mul({num(164),a}),mul({num(36),b})});
    Expr db=add({mul({num(128),a}),mul({num(71),b})});
    std::vector<Statement> rows={
      {"left-row-zero","u7 split-prime decoder left row zero","SplitPrime199V1.left_row_zero",{"a","b"},1,
        sub(add({mul({num(164),pa}),mul({num(36),pb})}),a),
        mul({num(199),add({a,mul({num(104),b})})})},
      {"left-row-one","u7 split-prime decoder left row one","SplitPrime199V1.left_row_one",{"a","b"},1,
        sub(add({mul({num(128),pa}),mul({num(71),pb})}),b),
        mul({num(199),add({a,mul({num(101),b})})})},
      {"right-row-zero","u7 split-prime decoder right row zero","SplitPrime199V1.right_row_zero",{"a","b"},1,
        sub(add({da,mul({num(106),db})}),a),
        mul({num(199),add({mul({num(69),a}),mul({num(38),b})})})},
      {"right-row-one","u7 split-prime decoder right row one","SplitPrime199V1.right_row_one",{"a","b"},1,
        sub(add({da,mul({num(92),db})}),b),
        mul({num(199),add({mul({num(60),a}),mul({num(33),b})})})}
    };
    std::ifstream proof(argv[2],std::ios::binary);
    if(!proof)throw std::runtime_error("missing proof source");
    std::string proofBytes((std::istreambuf_iterator<char>(proof)),{});
    std::filesystem::create_directories(argv[1]);
    for(const auto& s:rows) {
        if(std::max(s.lhs.degree(),s.rhs.degree())>s.degree)throw std::runtime_error("degree bound");
        std::string id="IDC1h"+hex(digest(s.json())).substr(0,16),values,ignored;
        if(!evaluate(s,id,20261116,values))throw std::runtime_error("identity refused");
        Statement bad=s;bad.rhs=add({bad.rhs,num(1)});
        if(evaluate(bad,id,20261116,ignored))throw std::runtime_error("mutation accepted");
        auto out=fresh(std::filesystem::path(argv[1])/(s.file+".json"));
        out<<"{\"schema\":\"identity.certificate/v1\",\"id\":"<<quote(id)<<",\"statement\":"<<s.json()
          <<",\"modulus\":"<<modulus<<",\"points\":32,\"seed\":20261116,\"value_digest\":"<<quote(hex(digest(values)))
          <<",\"independence\":\"Native producer issuance; exact integer Lean pointer, not independent Rust replay.\""
          <<",\"proof\":{\"kind\":\"lean4\",\"path\":"<<quote(argv[2])<<",\"sha256\":"<<quote(hex(digest(proofBytes)))<<"}}\n";
        std::cout<<s.file<<' '<<id<<" accepted32; changed-rhs rejected\n";
    }
    return 0;
}catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
