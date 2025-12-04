local shared = require"../common/bot"

local MAX_PER_CACHE_SERVER = 40

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

return {
    MAX_PER_CACHE_SERVER = MAX_PER_CACHE_SERVER,
}