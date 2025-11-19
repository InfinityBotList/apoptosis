local shared = require"../common/bot"

export type BorealisConfig = {
    needed_bots: {{
        id: string,
        name: string,
        invite: string,
    }},
    main_server: string,
    pinned_servers: {string},
}

export type Borealis = {
    bot: shared.Bot,
    sharedlayer: shared.SharedLayer,
    config: BorealisConfig,
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