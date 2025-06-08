-- Type functions should not contribute to cyclomatic complexity
-- since they are compile-time constructs

type function simple_type_function()
    return {}
end

export type function exported_type_function()
    return {}
end

-- Regular function should still contribute to complexity (this should warn)
local function regular_function()
    if true then
        if true then
            if true then
                if true then
                    if true then
                        if true then
                            if true then
                                if true then
                                    if true then
                                        if true then
                                            print("too complex")
                                        end
                                    end
                                end
                            end
                        end
                    end
                end
            end
        end
    end
end
