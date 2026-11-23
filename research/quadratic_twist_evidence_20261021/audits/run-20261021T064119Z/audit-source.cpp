#include <openssl/evp.h>
#include <algorithm>
#include <cctype>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace fs = std::filesystem;
void require(bool ok, const std::string& message) {
    if (!ok) throw std::runtime_error(message);
}
std::string read(const fs::path& path) {
    std::ifstream file(path, std::ios::binary);
    require(bool(file), "cannot open " + path.string());
    return {std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()};
}
std::string digest(const std::string& bytes) {
    unsigned char result[EVP_MAX_MD_SIZE]; unsigned size = 0;
    require(EVP_Digest(bytes.data(), bytes.size(), result, &size, EVP_sha256(), nullptr) == 1,
            "SHA-256 failed");
    std::ostringstream out;
    for (unsigned i = 0; i < size; ++i)
        out << std::hex << std::setw(2) << std::setfill('0') << unsigned(result[i]);
    return out.str();
}

// A strict JSON reader. Numbers retain their original decimal spelling.
struct Json {
    enum Kind { Null, Bool, Number, String, Array, Object } kind = Null;
    std::string text;
    std::vector<Json> items;
    std::map<std::string, Json> fields;
    const Json& at(const std::string& key) const {
        require(kind == Object && fields.count(key), "missing JSON field " + key);
        return fields.at(key);
    }
    long long integer() const {
        require(kind == Number && text.find_first_of(".eE") == std::string::npos, "integer required");
        return std::stoll(text);
    }
    const std::string& string() const { require(kind == String, "string required"); return text; }
    bool boolean() const { require(kind == Bool, "boolean required"); return text == "true"; }
};
struct Reader {
    const std::string& s; size_t position = 0;
    void white() { while (position < s.size() && std::isspace(static_cast<unsigned char>(s[position]))) ++position; }
    char take() { require(position < s.size(), "unexpected JSON end"); return s[position++]; }
    bool accept(char c) { white(); if (position < s.size() && s[position] == c) { ++position; return true; } return false; }
    std::string string() {
        require(take() == '"', "JSON string expected"); std::string out;
        while (true) {
            const unsigned char c = take();
            if (c == '"') return out;
            require(c >= 32, "control character in JSON string");
            if (c != '\\') { out += char(c); continue; }
            char e = take();
            switch (e) {
                case '"': case '\\': case '/': out += e; break;
                case 'b': out += '\b'; break; case 'f': out += '\f'; break;
                case 'n': out += '\n'; break; case 'r': out += '\r'; break; case 't': out += '\t'; break;
                case 'u': {
                    unsigned code = 0;
                    for (int i = 0; i < 4; ++i) {
                        const char h = take();
                        require(std::isxdigit(static_cast<unsigned char>(h)), "bad Unicode escape");
                        code = 16 * code + unsigned(h <= '9' ? h - '0' : std::tolower(h) - 'a' + 10);
                    }
                    require(code < 0xd800 || code > 0xdfff, "surrogate escape unsupported in this fixture");
                    if (code < 128) out += char(code);
                    else if (code < 2048) { out += char(0xc0 | (code >> 6)); out += char(0x80 | (code & 63)); }
                    else { out += char(0xe0 | (code >> 12)); out += char(0x80 | ((code >> 6) & 63)); out += char(0x80 | (code & 63)); }
                    break;
                }
                default: throw std::runtime_error("invalid JSON escape");
            }
        }
    }
    Json value() {
        white(); require(position < s.size(), "missing JSON value"); Json out;
        if (s[position] == '"') { out.kind = Json::String; out.text = string(); return out; }
        if (accept('{')) {
            out.kind = Json::Object;
            if (accept('}')) return out;
            do { white(); std::string key = string(); require(accept(':'), "missing colon");
                 require(!out.fields.count(key), "duplicate JSON key"); out.fields[key] = value();
            } while (accept(','));
            require(accept('}'), "missing object end"); return out;
        }
        if (accept('[')) {
            out.kind = Json::Array;
            if (accept(']')) return out;
            do { out.items.push_back(value()); } while (accept(','));
            require(accept(']'), "missing array end"); return out;
        }
        for (const auto& token : {std::string("null"), std::string("true"), std::string("false")}) {
            if (s.compare(position, token.size(), token) == 0) {
                position += token.size(); out.kind = token == "null" ? Json::Null : Json::Bool;
                out.text = token; return out;
            }
        }
        out.kind = Json::Number; size_t start = position;
        accept('-'); require(position < s.size() && std::isdigit(static_cast<unsigned char>(s[position])), "invalid number");
        if (s[position] == '0') ++position;
        else while (position < s.size() && std::isdigit(static_cast<unsigned char>(s[position]))) ++position;
        if (position < s.size() && s[position] == '.') {
            ++position; size_t digits = position;
            while (position < s.size() && std::isdigit(static_cast<unsigned char>(s[position]))) ++position;
            require(position > digits, "missing fraction digits");
        }
        if (position < s.size() && (s[position] == 'e' || s[position] == 'E')) {
            ++position; if (position < s.size() && (s[position] == '+' || s[position] == '-')) ++position;
            size_t digits = position;
            while (position < s.size() && std::isdigit(static_cast<unsigned char>(s[position]))) ++position;
            require(position > digits, "missing exponent digits");
        }
        out.text = s.substr(start, position - start); return out;
    }
};
Json parse(const std::string& text) {
    Reader reader{text}; Json value = reader.value(); reader.white();
    require(reader.position == text.size(), "trailing JSON data"); return value;
}

long long mod(long long x, long long p) { x %= p; return x < 0 ? x + p : x; }
long long power(long long x, long long n, long long p) {
    long long result = 1;
    for (; n; n /= 2, x = x * x % p) if (n % 2) result = result * x % p;
    return result;
}
struct F { long long a = 0, b = 0; bool operator==(F x) const { return a == x.a && b == x.b; } };
struct Field {
    long long p, d;
    F add(F x, F y) const { return {mod(x.a + y.a,p), mod(x.b + y.b,p)}; }
    F neg(F x) const { return {mod(-x.a,p),mod(-x.b,p)}; }
    F sub(F x,F y) const { return add(x,neg(y)); }
    F mul(F x,F y) const { return {mod(x.a*y.a+d*x.b*y.b,p),mod(x.a*y.b+x.b*y.a,p)}; }
    F inv(F x) const {
        auto norm = mod(x.a*x.a-d*x.b*x.b,p); require(norm != 0,"division by zero");
        auto ni = power(norm,p-2,p); return {mod(x.a*ni,p),mod(-x.b*ni,p)};
    }
    F div(F x,F y) const { return mul(x,inv(y)); }
    F pow(F x,long long n) const { F answer{1,0}; for (; n; n /= 2,x=mul(x,x)) if (n%2) answer=mul(answer,x); return answer; }
};
F element(std::string text) {
    text.erase(std::remove_if(text.begin(),text.end(),[](unsigned char c){return std::isspace(c);}),text.end());
    if (text.back() != 'u') return {std::stoll(text),0};
    text.pop_back(); auto separator = text.find('+'); require(separator != std::string::npos,"field encoding");
    return {std::stoll(text.substr(0,separator)),std::stoll(text.substr(separator+1))};
}
struct Affine { F x,y; bool operator==(const Affine& P) const { return x==P.x && y==P.y; } };
using Point = std::optional<Affine>;
Point point(std::string text) {
    if (text == "O") return {};
    require(text.front()=='(' && text.back()==')',"point encoding");
    text = text.substr(1,text.size()-2); auto comma = text.find(','); require(comma!=std::string::npos,"point comma");
    return Affine{element(text.substr(0,comma)),element(text.substr(comma+1))};
}
struct Curve {
    Field f; F a,b;
    bool contains(Point P) const { return !P || f.mul(P->y,P->y)==f.add(f.add(f.pow(P->x,3),f.mul(a,P->x)),b); }
    Point neg(Point P) const { return P ? Point(Affine{P->x,f.neg(P->y)}) : Point{}; }
    F slope(Affine P,Affine Q) const {
        if (P==Q) return f.div(f.add(f.mul({3,0},f.mul(P.x,P.x)),a),f.mul({2,0},P.y));
        return f.div(f.sub(Q.y,P.y),f.sub(Q.x,P.x));
    }
    Point add(Point P,Point Q) const {
        if (!P) return Q;
        if (!Q) return P;
        if (P->x==Q->x && f.add(P->y,Q->y)==F{}) return {};
        auto m=slope(*P,*Q); auto x=f.sub(f.sub(f.mul(m,m),P->x),Q->x);
        return Affine{x,f.sub(f.mul(m,f.sub(P->x,x)),P->y)};
    }
    Point scalar(long long n,Point P) const {
        Point out;
        for (;n;n/=2,P=add(P,P)) if (n%2) out=add(out,P);
        return out;
    }
    F line(Point A,Point B,Point X) const {
        require(A && B && X,"Miller affine input");
        if (A->x==B->x && f.add(A->y,B->y)==F{}) return f.sub(X->x,A->x);
        auto numerator=f.sub(f.sub(X->y,A->y),f.mul(slope(*A,*B),f.sub(X->x,A->x)));
        auto sum=add(A,B); return sum ? f.div(numerator,f.sub(X->x,sum->x)) : numerator;
    }
    F miller(Point P,Point X,long long n) const {
        F value{1,0}; Point V=P; long long bit=1;
        while (bit<=n/2) bit*=2;
        for (bit/=2;bit;bit/=2) {
            value=f.mul(f.mul(value,value),line(V,V,X)); V=add(V,V);
            if (n&bit) { value=f.mul(value,line(V,P,X)); V=add(V,P); }
        }
        require(!V,"Miller endpoint"); return value;
    }
    F weil(Point P,Point Q,long long n) const {
        if (!P || !Q) return {1,0};
        auto value=f.div(miller(P,Q,n),miller(Q,P,n)); return n%2 ? f.neg(value) : value;
    }
};
size_t rank(std::vector<std::vector<long long>> matrix,long long p) {
    size_t row=0;
    for (size_t column=0;column<matrix.front().size() && row<matrix.size();++column) {
        size_t pivot=row; while (pivot<matrix.size() && mod(matrix[pivot][column],p)==0) ++pivot;
        if (pivot==matrix.size()) continue;
        std::swap(matrix[pivot],matrix[row]); auto inv=power(mod(matrix[row][column],p),p-2,p);
        for (auto& x:matrix[row]) x=mod(x*inv,p);
        for (size_t other=0;other<matrix.size();++other) if (other!=row) {
            auto c=mod(matrix[other][column],p);
            for (size_t j=column;j<matrix[other].size();++j) matrix[other][j]=mod(matrix[other][j]-c*matrix[row][j],p);
        }
        ++row;
    }
    return row;
}
long long count_points(long long p,long long a,long long b,long long c) {
    long long count=1; auto inverse=power(c,p-2,p);
    for (long long x=0;x<p;++x) {
        auto rhs=mod((x*x%p*x+a*x+b)*inverse,p);
        if (rhs==0) ++count;
        else if (power(rhs,(p-1)/2,p)==1) count+=2;
    }
    return count;
}

int main(int argc,char** argv) {
    try {
        require(argc==3,"usage: audit EXTRACTED_SNAPSHOT OUTPUT_RECEIPT.json");
        const fs::path root=argv[1]; size_t hashes=0;
        std::istringstream manifest(read(root/"SHA256SUMS")); std::string line;
        while (std::getline(manifest,line)) {
            require(line.size()>66 && line.substr(64,2)=="  ","invalid checksum record");
            const auto name=line.substr(66); fs::path path=name;
            require(!path.is_absolute(),"absolute manifest path");
            for (const auto& part:path) require(part!="..","manifest traversal");
            require(digest(read(root/path))==line.substr(0,64),"checksum mismatch: "+name); ++hashes;
        }
        require(hashes>=10,"incomplete checksum manifest");
        const auto results=parse(read(root/"results/results.json"));
        const auto meta=parse(read(root/"results/run-metadata.json"));
        require(meta.at("experiment_exit_code").integer()==0 && meta.at("runner_exit_code").integer()==0,"historical run failed");
        require(meta.at("sources_unchanged_during_run").boolean(),"historical input changed");
        require(meta.at("scientific_results_match_reference").boolean(),"historical reference mismatch");
        require(results.at("source_sha256").string()==digest(read(root/"experiments.py")),"historical source binding");
        size_t certificates=0, scalar_checks=0, bilinear_checks=0;
        for (const auto& item:results.at("pairings").items) {
            const auto p=item.at("p").integer(), d=item.at("d").integer(), r=item.at("r").integer();
            require(p>3 && r>2 && power(d,(p-1)/2,p)==p-1,"invalid pairing fixture");
            const Field f{p,d}; const Curve curve{f,{item.at("a").integer(),0},{item.at("b").integer(),0}};
            auto P=point(item.at("P").string()), S=point(item.at("S").string()), Q=point(item.at("Q").string());
            require(curve.contains(P) && curve.contains(S) && curve.contains(Q),"off-curve fixture");
            require(P && S && !curve.scalar(r,P) && !curve.scalar(r,S),"torsion check");
            require(P->x.b==0 && P->y.b==0 && S->x.b==0 && S->y.a==0,"Frobenius eigenspace check");
            const auto g=element(item.at("pairing_generator").string()), target=element(item.at("pairing_target").string());
            require(!(g==F{1,0}) && f.pow(g,r)==F{1,0},"pairing order");
            require(curve.weil(P,S,r)==g && curve.weil(Q,S,r)==target,"native pairing replay");
            const auto& ic=item.at("index_calculus"); auto recovered=ic.at("recovered_log").integer();
            require(curve.scalar(recovered,P)==Q && f.pow(g,recovered)==target,"public scalar verification");
            std::vector<long long> logs;
            for (const auto& value:ic.at("projected_factor_base_logs").items) logs.push_back(value.integer());
            std::vector<std::vector<long long>> matrix;
            for (const auto& entry:ic.at("independent_relations").items) {
                std::vector<long long> row; long long lhs=0;
                for (const auto& x:entry.at("coefficients").items) row.push_back(x.integer());
                require(row.size()==logs.size(),"matrix dimension");
                for (size_t j=0;j<row.size();++j) lhs=mod(lhs+row[j]*logs[j],r);
                require(lhs==entry.at("rhs").integer(),"relation equation"); matrix.push_back(row);
            }
            require(matrix.size()==logs.size() && rank(matrix,r)==logs.size(),"relation rank");
            const auto h=ic.at("projection_exponent").integer();
            require(ic.at("factor_base").items.size()==logs.size(),"factor-base dimension");
            for (size_t j=0;j<logs.size();++j) {
                const auto& name=ic.at("factor_base").items[j].string();
                require(name.rfind("u+",0)==0,"factor-base encoding");
                require(f.pow({std::stoll(name.substr(2)),1},h)==f.pow(g,logs[j]),"factor-base log");
            }
            for (long long a=0;a<r;++a) {
                require(curve.weil(curve.scalar(a,P),S,r)==f.pow(g,a),"scalar pairing replay"); ++scalar_checks;
                for (long long b=0;b<r;++b) {
                    require(curve.weil(curve.scalar(a,P),curve.scalar(b,S),r)==f.pow(g,a*b),"bilinearity"); ++bilinear_checks;
                }
            }
            ++certificates;
        }
        const auto& smooth=results.at("no_transfer").at("smooth_twist");
        const auto p=smooth.at("p").integer(), a=smooth.at("a").integer(), b=smooth.at("b").integer();
        require(count_points(p,a,b,1)==1019 && count_points(p,a,b,smooth.at("d").integer())==1001,"weak-twist point count");
        const auto& anomalous=results.at("no_transfer").at("anomalous_twist");
        auto ap=anomalous.at("p").integer(), aa=anomalous.at("a").integer(), ab=anomalous.at("b").integer();
        long long ad=2; while (power(ad,(ap-1)/2,ap)!=ap-1) ++ad;
        require(count_points(ap,aa,ab,1)==13 && count_points(ap,aa,ab,ad)==11,"anomalous-twist point count");
        std::ofstream receipt(argv[2]); require(bool(receipt),"cannot write receipt");
        receipt << "{\n  \"schema\": \"quadratic-twist-historical-audit/v1\",\n  \"status\": \"passed\",\n"
                << "  \"snapshot_manifest_files_verified\": " << hashes << ",\n"
                << "  \"saved_ic_certificates_verified\": " << certificates << ",\n"
                << "  \"native_pairing_scalar_checks\": " << scalar_checks << ",\n"
                << "  \"native_bilinearity_checks\": " << bilinear_checks << ",\n"
                << "  \"native_point_count_examples_verified\": 2,\n"
                << "  \"full_942_model_census_rerun\": false,\n  \"native_relation_collection_rerun\": false,\n"
                << "  \"formal_lean_proof_checked\": false,\n  \"performance_claim\": null\n}\n";
        std::cout << "PASS: " << hashes << " historical checksum records\nPASS: " << certificates
                  << " independent native IC certificate checks\nPASS: " << scalar_checks
                  << " native scalar-pairing checks; " << bilinear_checks << " bilinearity checks\n"
                  << "PASS: smooth/anomalous twist point counts\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "FAIL: " << error.what() << '\n'; return 1;
    }
}
