--!strict
local json = require"@omniplex-rust/json"
local bot = require"@omniplex-common/bot"

type SampleLayerConfig = {
    foo: string,
}

type SampleLayer = {
    config: SampleLayerConfig,
}

type SampleLayerEvent = {
    type: "Startup",
    data: {}
} | {
    type: "TestEvent",
    data: string,
}

local function SampleLayer(ctx: bot.Context<SampleLayer, SampleLayerEvent>) 
    print("Sample layer loaded", ctx, json.stringify(ctx.event), json.stringify(ctx.layer.config))
end

return SampleLayer