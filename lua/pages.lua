local type = "fb"
local idpage_fb = {
    Beatvn = "FP - Beatvn",
    ["Chuyện của Hà Nội"] = "FP - Chuyện của Hà Nội",
    ["Sài Gòn Nghenn"] = "FP - Sài Gòn nghenn",
    ["Cao Thủ"] = "FP - Cao Thủ",
    ["Kiến Không Ngủ"] = "FP - Kiến không ngủ",
    ["Beatnow"] = "FP - BEAT NOW",
    ["Showbeat"] = "FP - ShowBeat",
    ["Tám chuyện Showbiz"] = "FP - Tám Chuyện Showbiz",
    ["What The Duck"] = "FP - What the Duck",
    ["Chuyển Động Showbiz"] = "FP - Chuyển động Showbiz",
    ["Inside The Box"] = "FP - Inside the Box"
}
local idpage_tiktok = {
    ["Kiến Không Ngủ"] = "TT - KIẾN KHÔNG NGỦ",
    ["Showbeat"] = "TT - SHOWBEAT",
    ["Beat Viral World"] = "TT - Beatvn Viral World",
    ["Hello Vn"] = "TT - HelloVietNam",
    ["Beatvn"] = "TT - BEATVN"
}
if arg[1] == "tiktok" or arg[1] == "fb" then
    local file = io.open("testpage.csv","r+")
    local fileline = {}
    if file then
        for lines in file:lines() do
            fileline[#fileline + 1] = lines
        end
        for k,v in pairs(fileline) do
            if arg[1] == "fb" then
                if idpage_fb[v] ~= nil then
                    fileline[k] = idpage_fb[v]
                end
            elseif arg[1] == "tiktok" then
                if idpage_tiktok[v] ~= nil then
                    fileline[k] = idpage_tiktok[v]
                end
            end
        end
        print(fileline[5])
        file = io.open("tesstpage.txt","w+")
        for _,v in ipairs(fileline) do
            file:write(v,"\n")
        end
    end
else print("bad argument")
end
