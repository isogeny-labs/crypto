#pragma once
#include <array>
#include <string>
struct InputPin { const char* path; const char* sha256; };
const std::string reference_pin="891d63d978d8800e245952d32ddb733d15d6156f6d080aa74d190452f9101cd4";
const std::array<InputPin,3> construction_pins{{
    {"inputs/p192-d7-prym.json","adcdb4bc5d46b541e70da01feb05d10f14ad0686ab69447d6bca1b2dd270525b"},
    {"evidence/p192-d7-kernel3-plane-quartic.json","38df7f8a2635de0d241fce94e3a9ba3060d04373cee48c8d4fd73e96229ea540"},
    {"backmap.json","72f2b79819fdb80adbf6bdd2536bec3f2906a84e629116009a24623fe8bef129"}
}};
