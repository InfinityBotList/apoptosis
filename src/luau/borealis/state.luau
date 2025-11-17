local shared = require"../common/bot"

export type Borealis = {
    bot: shared.Bot,
    sharedlayer: shared.SharedLayer,
}

local borealis: Borealis

local function SetState(newstate: Borealis)
    borealis = newstate
end

local function GetState(): Borealis
    if borealis then
        return borealis
    else
        error("internal error: borealis state not set")
    end
end

return {
    SetState = SetState,
    GetState = GetState,
}