obs_list = observation.tolist()
print("Observation as list: {}".format(obs_list))

obs_tuple = tuple(obs_list)
print("Observation as tuple: {}".format(obs_list))

obs_ctype = ffi.new("double[4]", obs_tuple)
libtyro.print_array(obs_ctype)