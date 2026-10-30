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

