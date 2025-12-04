--!strict
local concurrency = require"./concurrency"

--- Task Manager is a construct for managing recurring tasks
export type TaskManager = {
    --- Registers a recurring task with the given name and period (in seconds)
    ---
    --- If the task errors, it will be caught and logged, and the task will continue running
    Register: (self: TaskManager, name: string, per: number, task: () -> ()) -> (),

    --- Cancels a recurring task with the given name
    Cancel: (self: TaskManager, name: string) -> (),
}

local function PerSecond(n: number): number 
    return n
end

local function PerMinute(n: number): number
    return n * 60
end

--- Returns a Task Manager which can be used to register and cancel recurring tasks
local function TaskManager(): TaskManager
    local self = {}
    local tasks: {[string]: {
        per: number,
        tid: number,
    }} = {}

    self.Register = function(self: TaskManager, name: string, per: number, task: () -> ())
        if tasks[name] then
            error(`Task with name {name} is already registered`)
        end

        local tid = concurrency.setInterval(function()
            local ok, err = xpcall(function() return task(), nil end, debug.traceback :: any) 
            if not ok then
                warn(`Error in task {name}: {err}`)
            end
        end, per)

        tasks[name] = {
            per = per,
            tid = tid,
        }
    end

    self.Cancel = function(self: TaskManager, name: string)
        local taskInfo = tasks[name]
        if not taskInfo then
            error(`Task with name {name} is not registered`)
        end

        concurrency.clearInterval(taskInfo.tid)
        tasks[name] = nil
    end

    return self
end

return {
    TaskManager = TaskManager,
    PerSecond = PerSecond,
    PerMinute = PerMinute,
}