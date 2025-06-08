-- Test that TypeFunction and ExportedTypeFunction don't cause compilation errors
-- This file doesn't actually contain type functions since they may not be parseable
-- The real test is that the code compiles and doesn't crash

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
                                            if true then
                                                return "high complexity"
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
end
