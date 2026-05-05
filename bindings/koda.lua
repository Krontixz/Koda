local ffi = require("ffi")
local koda = ffi.load("./target/release/libkoda.so")

ffi.cdef[[
    char* koda_parse_to_json(const char* input);
    void koda_free_string(char* p);
]]

local Koda = {}
function Koda.parse(input)
    local c_str = koda.koda_parse_to_json(input)
    local result = ffi.string(c_str)
    koda.koda_free_string(c_str)
    return result
end

return Koda
