local file = io.open("test.lua", "r")
if file == nil then
    return 0
end
print(file:read("*l"))
file:close()
