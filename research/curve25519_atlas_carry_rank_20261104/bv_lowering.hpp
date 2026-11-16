// Bounded small-prime integer constraints lowered to signed bit-vectors.
// The caller supplies a width covering every intermediate, not only residues.
#pragma once
#include <cctype>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <vector>
class BoundedBV {
    unsigned width;
    static void check(bool ok,const char* why) {if(!ok)throw std::runtime_error(why);}
    std::string numeral(std::int64_t value)const {
        const auto modulus=std::int64_t{1}<<width;
        value%=modulus;if(value<0)value+=modulus;
        return "(_ bv"+std::to_string(value)+" "+std::to_string(width)+")";
    }
    std::string parse(const std::string& s,std::size_t& i)const {
        auto space=[&](){while(i<s.size() && std::isspace(static_cast<unsigned char>(s[i])))++i;};
        space();check(i<s.size(),"empty SMT expression");
        if(s[i]!='(') {
            auto start=i;while(i<s.size() && s[i]!=')' && !std::isspace(static_cast<unsigned char>(s[i])))++i;
            auto token=s.substr(start,i-start);check(!token.empty(),"empty SMT token");
            if(std::isdigit(static_cast<unsigned char>(token[0])))return numeral(std::stoll(token));
            return token;
        }
        ++i;space();auto start=i;
        while(i<s.size() && s[i]!=')' && !std::isspace(static_cast<unsigned char>(s[i])))++i;
        std::string op=s.substr(start,i-start);std::vector<std::string> args;
        for(;;) {space();check(i<s.size(),"unclosed SMT expression");if(s[i]==')'){++i;break;}args.push_back(parse(s,i));}
        if(op=="mod") {
            check(args.size()==2,"mod arity");
            return "(let ((__rem (bvsrem "+args[0]+" "+args[1]+"))) (ite (bvslt __rem "+numeral(0)+") (bvadd __rem "+args[1]+") __rem))";
        }
        if(op=="+" || op=="-" || op=="*") {
            check(!args.empty(),"empty arithmetic expression");
            if(op=="-" && args.size()==1)return "(bvneg "+args[0]+")";
            std::string out=args[0],bvop=op=="+"?"bvadd":op=="-"?"bvsub":"bvmul";
            for(std::size_t j=1;j<args.size();++j)out="("+bvop+" "+out+" "+args[j]+")";
            return out;
        }
        if(op=="<=") {check(args.size()==2,"comparison arity");op="bvsle";}
        check(op=="bvsle" || op=="=" || op=="and" || op=="or" || op=="not" || op=="=>" || op=="ite","unsupported SMT operator");
        std::string out="("+op;for(const auto& a:args)out+=" "+a;return out+")";
    }
public:
    explicit BoundedBV(unsigned bits):width(bits){check(bits>=20 && bits<=40,"unsupported bounded width");}
    std::string sort()const{return "(_ BitVec "+std::to_string(width)+")";}
    std::string lower(const std::string& expression)const {
        std::size_t i=0;auto result=parse(expression,i);
        while(i<expression.size() && std::isspace(static_cast<unsigned char>(expression[i])))++i;
        check(i==expression.size(),"trailing SMT expression");return result;
    }
};
