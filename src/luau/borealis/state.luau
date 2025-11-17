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

--- Information about a cache server
export type CacheServerInfo = {
    bots_role: string,
    system_bots_role: string,
    logs_channel: string,
    staff_role: string,
    web_moderator_role: string,
    name: string,
}

export type CacheServerManager = {
    --- Returns information about a cache server
    get: (guildid: string) -> CacheServerInfo?,
    --- Returns the cache server id for a bot given its bot id
    lookupBot: (botid: string) -> string?,
    --- Removes a bot from its cache server
    removeBot: (botid: string) -> (),
    --- Deletes a cache server
    delete: (guildid: string) -> (),
}

export type Borealis = {
    bot: shared.Bot,
    sharedlayer: shared.SharedLayer,
    config: BorealisConfig,
    --- Returns information about a cache server
    cacheserver: CacheServerManager
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