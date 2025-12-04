type TimeoutThread = {
    thread: thread,
    isDone: boolean,
    isCancelled: boolean
}

type IntervalThread = {
    intervalWaiter: thread,
    isWaiting: boolean,
    isCancelled: boolean
}


local timeouts: { [number]: TimeoutThread } = {}
local intervals: { [number]: IntervalThread } = {}

--- Implementation of setTimeout from Javascript.
---
--- Functions like JavaScript's setTimeout.
---
--- @param callback The function to call after the delay.
--- @param delay The delay in milliseconds. If a string is passed, it will be converted to a number.
--- @param ... Additional arguments to pass to the callback function.
--- @return tid The timeout ID, which can be used to cancel the timeout with `clearTimeout`.
local function setTimeout(callback: (any) -> (), delay: number?, ...): number 
    local args = {...}

    if typeof(delay) == "string" then
        -- Convert string to number
        delay = tonumber(delay)
    end

    -- Default delay is 0
    delay = (delay or 0)

    local tid = #timeouts + 1

    timeouts[tid] = {
        isCancelled = false,
        isDone = false
    } :: TimeoutThread;

    timeouts[tid].thread = task.delay((delay :: number)/1000, function() 
        timeouts[tid].isDone = true -- Mark as done
        if(not timeouts[tid] or timeouts[tid].isCancelled) then
            return
        end

        callback(args)
    end);

    return tid
end

--- Implementation of clearTimeout from Javascript.
---
--- Functions like JavaScript's clearTimeout.
---
--- @param tid The timeout ID to cancel.
--- @return nil
local function clearTimeout(tid: number)
    local th = timeouts[tid]

    if not th then
        return
    end

    timeouts[tid].isCancelled = true

    if not timeouts[tid].isDone then
        task.cancel(th.thread)
    end
end

--- Implementation of setInterval from Javascript
---
--- Functions like JavaScript's setInterval.
---
--- @param callback The function to call at each interval.
--- @param delay The delay in milliseconds. If a string is passed, it will be converted to a number.
--- @param ... Additional arguments to pass to the callback function.
--- @return tid The interval ID, which can be used to cancel the interval with `clear
local function setInterval(callback: (any) -> (), delay: number?, ...): number    
    local args = {...}
    if(typeof(delay) == "string") then
        -- Convert string to number
        delay = tonumber(delay)
    end

    -- Default delay is 500 milliseconds
    delay = delay or 500

    local tid = #intervals + 1
    intervals[tid] = {
        isWaiting = false,
        isCancelled = false
    } :: IntervalThread; -- Ensure its set initially
    intervals[tid].intervalWaiter = task.spawn(function()
        while true do
            if(not intervals[tid] or intervals[tid].isCancelled) then
                return
            end

            intervals[tid].isWaiting = true
            task.wait((delay :: number)/1000);
            intervals[tid].isWaiting = false

            if(not intervals[tid] or intervals[tid].isCancelled) then
                return
            end

            callback(args)
        end
    end);

    return tid
end

--- Implementation of clearInterval from Javascript.
---
--- Functions like JavaScript's clearInterval.
---
--- @param tid The interval ID to cancel.
--- @return nil
local function clearInterval(tid: number)
    local th = intervals[tid]

    if not th then
        return
    end

    intervals[tid].isCancelled = true

    -- If isWaiting, remove the intervalWaiter from waiting queue
    if intervals[tid].isWaiting then
        task.cancel(intervals[tid].intervalWaiter)
    end
end

-- Test
if(_G.TEST ~= nil) then
    setTimeout(function()
        print("Task")
    end, 1000)

    print("Hello, world A")

    setTimeout(function()
        print("Task2")
    end, 2000)

    print("Hello, world B")

    setTimeout(function()
        print("Task3")
    end, 500)

    local idx = 0
    local tid;
    tid = setInterval(function()
        idx += 1
        print(idx)

        if(idx >= 5) then
            clearInterval(tid :: number)
        end
    end, 1000)

    while(idx < 5) do
        task.wait(0.1)
    end

    print("Hello, world C")

    local sidABC = setInterval(function()
        print("This should not print")
    end, 10000)

    setTimeout(function()
        print("Clearing interval for sidABC")
        clearInterval(sidABC)
    end, 100)

    print("Hello, world D")

    local tidABC = setTimeout(function()
        print("This should also not print")
    end, 10000)

    setTimeout(function()
        print("Clearing timeout for tidABC")
        clearTimeout(tidABC)
    end, 1000)
end

return {
    setTimeout = setTimeout,
    clearTimeout = clearTimeout,
    setInterval = setInterval,
    clearInterval = clearInterval,
}