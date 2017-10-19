package.path = "./assets/scripts/?.lua;./assets/scripts/?/init.lua;" .. package.path
mystig = require("mystig")
object = require("object")

function update()
    mystig.update()
end

function draw()
    mystig.draw()
end