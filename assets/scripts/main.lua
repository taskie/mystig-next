package.path = "./assets/scripts/?.lua;./assets/scripts/?/init.lua;" .. package.path
mystig = require("mystig")
object = require("object")

function update(t)
    return mystig.update(t)
end

function draw(t)
    object.dump(t)
    result = {
        eye = { 1, 0, 0 },
        target = {
            2 * math.sin(t.frame / 360 * 11),
            2 * math.sin(t.frame / 360 * 13),
            2 * math.sin(t.frame / 360 * 17),
        },
        perspective = {
            aspect = 640 / 480,
            fovy = 3.141592 / 2,
            znear = 0,
            zfar = 1000,
        },
        isometry = {
            look_at = "rh",
        },
    }
    object.dump(result)
    return result
end
