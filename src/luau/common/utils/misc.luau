local function isApprovedState(state: string): boolean
    return state == "active" or state == "certified"
end

return {
    isApprovedState = isApprovedState,
}