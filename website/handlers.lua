Handlers = {
	["^/$"] = "HomePage",
}

function HomePage(req)
	local file = assert(io.open("pages/index.html", "r"))
	local content = file:read("a")
	file:close()
	return string.format(content, req.visitor_count)
end
