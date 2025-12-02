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

return {
}