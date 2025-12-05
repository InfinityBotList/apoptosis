local function stringify<T>(value: T): string
    error("Implemented in rust")
end

local function parse<T>(text: string): T
    error("Implemented in rust")
end

local function stringifypretty<T>(value: T): string
    error("Implemented in rust")
end

return {
    stringify = stringify,
    parse = parse,
    stringifypretty = stringifypretty,
}