local policy = {
    "Bạo lực và khích nộ",
    "Cá nhân và tổ chức nguy hiểm",
    "Cấu kết gây hại và cổ xúy tội ác",
	"Hàng hóa và dịch vụ bị hạn chế",
	"Gian lận và lừa đảo",
    "Tự tử, tự gây thương tích và chứng rối loạn ăn uống",
    "Ảnh khỏa thân, lạm dụng và bóc lột tình dục trẻ em",
    "Bóc lột tình dục người lớn",
    "Bắt nạt và quấy rối",
    "Bóc lột con người",
    "Vi phạm quyền riêng tư",
    "Ngôn từ gây thù ghét",
    "Nội dung bạo lực và phản cảm",
    "Ảnh khỏa thân người lớn và hoạt động tình dục",
    "Hành vi gạ gẫm tình dục người lớn và ngôn ngữ khiêu dâm",
    "Danh tính thực và tính toàn vẹn của tài khoản",
    "Spam",
    "An ninh mạng",
    "Hành vi gian dối",
    "Thông tin sai lệch",
    "Tưởng nhớ",
    "Quyền sở hữu trí tuệ"
}
-- get csv file
local csv_files = {}
local listcsv = io.open("list.txt","r")
if listcsv ~= nil then
    for i in listcsv:lines() do
        table.insert(csv_files,i)
    end
    listcsv:close()
end

for i=1,#csv_files do
    local file = io.open(csv_files[i], "r+")
    print(csv_files[i])
    --print(string.gsub(teststr,",(%d+)\n", function (x) return ","..policy[tonumber(x)] end))
    if file ~= nil then
        local content = file:read("a")
        content = string.gsub(content,",(%d+)\n", function (y)
                print(y)
                return ",\""..policy[tonumber(y)].."\"\n"
            end)
        file:seek("set")
        file:write(content)
    end
end
