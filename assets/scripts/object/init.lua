local _M = {}

-- dumps

function _M.dumps(data)
    if type(data) == 'string' then
        return string.format("%q", data)
    elseif type(data) == 'table' then
        local s = "{"
        for k, v in pairs(data) do
            s = s .. tostring(k) .. "=" .. _M.dumps(v) .. ","
        end
        return s .. "}"
    else
        return tostring(data)
    end
end

-- dump

function _M.dump(data)
    print(_M.dumps(data))
end

-- clone

function _M.clone(data)
    if type(data) == 'table' then
        local new_data = {}
        for k, v in pairs(data) do
            new_data[k] = _M.clone(v)
        end
        setmetatable(new_data, getmetatable(data))
        return new_data
    else
        return data
    end
end

-- set_prototype_of

function _M.set_prototype_of(table, prototype)
    local mt = {
        __index = function (index) return prototype[index] end,
    }
    setmetatable(table, mt)
end

-- seal

local seal_mt = {}

function seal_mt.__index(table, index)
    print("Warning: sealed (index)", index)
end

function seal_mt.__newindex(table, index, value)
    print("Warning: sealed (newindex)", index, value)
end

function _M.seal(table)
    setmetatable(table, seal_mt)
end

-- freeze

function _M.freeze(table)
    local newtable = {}
    for k, v in pairs(table) do
        newtable[k] = v
        table[k] = nil
    end
    local freeze_mt = {
        __index = function (table, index)
            return newtable[index]
        end,
        __newindex = function (table, index, value)
            print("Warning: frozen (newindex)", index, value)
        end,
    }
    setmetatable(newtable, getmetatable(table))
    setmetatable(table, freeze_mt)
end

return _M