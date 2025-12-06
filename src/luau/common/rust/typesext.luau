--- A multi option is either T (Some(Some(T)), a empty table (Some(None)), or nil (None)
export type MultiOption<T> = T | {} | nil

export type U64Convertibles = U64 | string | number

--- A U64 is a 64 bit unsigned integer. A U64 can be implicitly created from a number or a string
---
--- Note: A U64
export type U64 = typeof(setmetatable({} :: {
    to_ne_bytes: (self: U64) -> { number },
    from_ne_bytes: (self: U64, bytes: { number }) -> U64,
    to_le_bytes: (self: U64) -> { number },
    from_le_bytes: (self: U64, bytes: { number }) -> U64,
    to_be_bytes: (self: U64) -> { number },
    from_be_bytes: (self: U64, bytes: { number }) -> U64,
    to_i64: (self: U64) -> I64,
}, {} :: {
    __add: (self: U64, other: U64Convertibles) -> U64,
    __sub: (self: U64, other: U64Convertibles) -> U64,
    __mul: (self: U64, other: U64Convertibles) -> U64,
    __div: (self: U64, other: U64Convertibles) -> U64, -- This is the same as IDiv right now
    __mod: (self: U64, other: number) -> U64,
    __pow: (self: U64, other: U64Convertibles) -> U64,
    __idiv: (self: U64, other: U64Convertibles) -> U64, -- This is the same as Div right now
    __eq: (self: U64, other: U64Convertibles) -> boolean,
    __lt: (self: U64, other: U64Convertibles) -> boolean,
    __le: (self: U64, other: U64Convertibles) -> boolean,
    __tostring: (self: U64) -> string,
    __type: "U64",
}))

export type I64Convertibles = U64 | I64 | string | number

export type I64 = typeof(setmetatable({} :: {
    to_ne_bytes: (self: I64) -> { number },
    from_ne_bytes: (self: I64, bytes: { number }) -> I64,
    to_le_bytes: (self: I64) -> { number },
    from_le_bytes: (self: I64, bytes: { number }) -> I64,
    to_be_bytes: (self: I64) -> { number },
    from_be_bytes: (self: I64, bytes: { number }) -> I64,
    to_u64: (self: I64) -> U64,
}, {} :: {
    __add: (self: I64, other: I64 | string | number) -> I64,
    __sub: (self: I64, other: I64 | string | number) -> I64,
    __mul: (self: I64, other: I64 | string | number) -> I64,
    __div: (self: I64, other: I64 | string | number) -> I64,
    __mod: (self: I64, other: number) -> I64,
    __pow: (self: I64, other: I64 | string | number) -> I64,
    __idiv: (self: I64, other: I64 | string | number) -> I64,
    __eq: (self: I64, other: I64 | string | number) -> boolean,
    __lt: (self: I64, other: I64 | string | number) -> boolean,
    __le: (self: I64, other: I64 | string | number) -> boolean,
    __tostring: (self: I64) -> string,
    __type: "I64",
}))

--[[
/// Aim is to implement as many functions from https://luau.org/library#bit32-library as possible but for 64 bit unsigned integers
///
/// arshift, band, bnot, bor, bxor, btest, extract, lrotate, lshift, replace, rrotate, rshift, countlz
/// countrz, byteswap
///
/// Implemented:
/// - band
/// - bnot
/// - bor
/// - bxor
/// - btest
/// - extract
/// - lrotate
/// - lshift
/// - replace
/// - rrotate
/// - rshift
/// - countlz
/// - countrz
/// - byteswap
///
/// Not yet implemented due to spec difficulties:
/// - arshift
]]
export type BitU64 = {
    band: (...U64Convertibles) -> U64,
    bnot: (U64Convertibles) -> U64,
    bor: (...U64Convertibles) -> U64,
    bxor: (...U64Convertibles) -> U64,
    btest: (...U64Convertibles) -> boolean,

    --- Untested and might not work yet. f must be less than 64
    extract: (n: U64Convertibles, f: number, w: number?) -> U64, 
    lrotate: (n: U64Convertibles, i: number) -> U64,
    lshift: (n: U64Convertibles, i: number) -> U64,

    --- Untested and might not work yet. f must be less than 64
    replace: (n: U64Convertibles, r: number, f: number, w: number?) -> U64,

    rrotate: (n: U64Convertibles, i: number) -> U64,
    rshift: (n: U64Convertibles, i: number) -> U64,
    countlz: (n: U64Convertibles) -> number,
    countrz: (n: U64Convertibles) -> number,
    byteswap: (n: U64Convertibles) -> U64,
}

local bitu64 = {} :: BitU64

--- Creates a U64 from a number or string
---
--- If nil is passed, 0 is used as default
local function U64(value: U64Convertibles | nil): U64
    error("Implemented internally in Rust!")
end

--- Creates a I64 from a number or string
---
--- If nil is passed, 0 is used as default
local function I64(value: I64 | string | number | nil): I64
    error("Implemented internally in Rust!")
end

--- Generates a random string of length `length`
local function randstring(length: number): string
    --- A crypto-secure one is implemented in the Rust
    local chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    local result = ""
    for i = 1, length do
        local index = math.random(1, #chars)
        result = result .. string.sub(chars, index, index)
    end
    return result
end

return {
    U64 = U64,
    I64 = I64,
    bitu64 = bitu64,
    randstring = randstring,
}