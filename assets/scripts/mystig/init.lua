_M = {}

object = require("object")

function _M.update(t)
    return {

    }
end

function _M.draw(t)
    object.dump(t)
    result = {
        eye = {0, 0, 1},
        target = {
            (t.mouse.x - 320) / 640,
            (t.mouse.y - 240) / 480,
            0.0,
        },
        persepective = {
            aspect = 640 / 480,
            fovy = 3.14 / 2,
            znear = math.sin(t.frame),
            zfar = 1000,
        }
    }
    object.dump(result)
    return result
end

return _M
