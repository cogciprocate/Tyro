from cffi import FFI

ffi = FFI()

libtyro = ffi.dlopen('../target/debug/libtyro.so')
print('Loaded lib {0}'.format(libtyro))

# Describe the data type and function prototype to cffi.
ffi.cdef('''
    typedef void** Tyro;

    void hello(int a);
    Tyro new_tyro();
    int add_100(Tyro, int);
    void drop_tyro(Tyro);
''')

# Create an array of DataPoint structs and initialize it.
tyro = ffi.new('Tyro', libtyro.new_tyro())

print('Calling add_100 via cffi')
# Interesting variation: passing invalid arguments to add_data will trigger
# a cffi type-checking exception.
# dout = lib.add_data(dps, 4)

libtyro.hello(5)

addend = libtyro.add_100(tyro[0], 55)

libtyro.hello(addend)

libtyro.drop_tyro(tyro[0])