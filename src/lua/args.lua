function args_check(arg, expected_type, arg_index, allow_nil)
	if arg == nil then
		if allow_nil then
			return
		else
			local callsite = debug.getinfo(2, "nSl")
			local err = string.format("%s:%d: inside of function %s", callsite.short_src, callsite.currentline, callsite.name)
			local arg_name = debug.getlocal(2, arg_index)
			error(string.format("%s:\n\tArgument %q cannot be nil", err, arg_name))
		end
	elseif expected_type and type(arg) ~= expected_type then
		local arg_name, arg_value
		if arg_index ~= nil then
			arg_name, arg_value = debug.getlocal(2, arg_index)
		end

		arg_name = arg_name or "unknown_name"
		arg_value = arg_value or "unknown_value"
		arg_index = arg_index or 0

		local callsite = debug.getinfo(2, "nSl")
		local error_context = string.format("Invalid argument %s:%d:\n" ..
		"function %q called with invalid argument #%d %q.\n" ..
		"Expected type of %q, but got %q with value %q",
		callsite.short_src, callsite.linedefined,
		callsite.name, tostring(arg_index), tostring(arg_name),
		tostring(expected_type), type(arg), tostring(arg_value)
		)

		local callsite_context = string.format(
		"\n\tSource: %s:%d, from callsite %s",
		callsite.short_src, callsite.currentline, callsite.name
		)
		error(error_context .. callsite_context)
	end
end
