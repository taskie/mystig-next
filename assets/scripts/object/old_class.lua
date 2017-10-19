local all_instances = {}
local all_instances_mt = {__mode = "kv"}
setmetatable(all_instances, all_instances_mt)

function get_all_instances()
    return all_instances
end

local function create_instance(class, target)
    local t = target or {}
    if rawget(class, "__index") == class then
        setmetatable(t, class)
    else
        local mt = {__index = class}
        setmetatable(t, mt)
    end

    all_instances[t] = true
    return t
end

local function create_class(classname, super_class, namespace, class_vars, on_class_reload)
    assert(type(class_vars) == "table")

    namespace = namespace or _G
    local class = rawget(namespace, classname)
    local is_reload = false

    if class == nil then
        class = class_vars
        namespace[classname] = class
    else
        is_reload = true
        if on_class_reload ~= nil then
            on_class_reload(class, super_class, class_vars)
        end
    end

    if super_class ~= nil then
        class.super = super_class
        if rawget(super_class, "__index") == super_class then
            setmetatable(class, super_class)
        else
            local mt = {__index = super_class}
            setmetatable(class, mt)
        end
    end

    local mt = getmetatable(class) or {}
    mt["__call"] = function(self, ...)
        local t = create_instance(self, {})
        t:new(...)
        return t
    end
    setmetatable(class, mt)

    class.classname = classname
    class.__index = class
end

function class(classname, namespace, on_class_reload)
    return function(class_vars)
        create_class(classname, nil, namespace, class_vars, on_class_reload)
    end
end

function subclass(classname, super_class, namespace, on_class_reload)
    return function(class_vars)
        create_class(classname, super_class, namespace, class_vars, on_class_reload)
    end
end
