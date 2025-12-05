local json = require"@omniplex-rust/json"

local ctx = ...
print("Sample layer loaded", ctx, json.stringify(ctx.event), json.stringify(ctx.layer.config))

return {}