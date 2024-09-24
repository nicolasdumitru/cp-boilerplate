#include <cstddef>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <istream>
#include <ostream>

using usize = std::size_t;
using i8 = std::int8_t;  // cin treats this as char
using u8 = std::uint8_t; // cin treats this as char
using i16 = std::int16_t;
using u16 = std::uint16_t;
using i32 = std::int32_t;
using u32 = std::uint32_t;
using i64 = std::int64_t;
using u64 = std::uint64_t;

using std::istream;
using std::ostream;

int main() {
    std::ios::sync_with_stdio(false);

    std::ifstream fin("file.in");
    std::ofstream fout("file.out");

    istream &in = std::cin;
    ostream &out = std::cout;

    fin.close();
    fout.close();

    return 0;
}
