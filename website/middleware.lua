Middleware = {
	["^/$"] = "CheckUniqueVisitor",
}

CheckForIp = "SELECT * FROM visitors WHERE ip_addr = '%s'"
AddIp = "INSERT INTO visitors (ip_addr) VALUES ('%s')"
GetVisitorCount = "SELECT * FROM state WHERE name = 'u_vist'"
AddVisitorCount = "UPDATE state SET value = value + 1"

function CheckUniqueVisitor(req)
	local driver = require("luasql.sqlite3")
	local env = assert(driver.sqlite3())
	local conn = assert(env:connect("koalatree.db"))
	local check_cmd = string.format(CheckForIp, req.their_addr)
	local cur = assert(conn:execute(check_cmd))
	local row = cur:fetch({}, "a")
	local _ = cur:close()
	if row == nil then
		local add_cmd = string.format(AddIp, req.their_addr)
		local _ = assert(conn:execute(add_cmd))
		local _ = assert(conn:execute(AddVisitorCount))
		local cur2 = assert(conn:execute(GetVisitorCount))
		local row2 = cur2:fetch({}, "a")
		req.visitor_count = row2.value
		local _ = cur2:close()
	else
		local cur2 = assert(conn:execute(GetVisitorCount))
		local row2 = cur2:fetch({}, "a")
		req.visitor_count = row2.value
		cur2:close()
		local _ = cur2:close()
	end
	local _ = conn:close()
	local _ = env:close()

	return req
end
