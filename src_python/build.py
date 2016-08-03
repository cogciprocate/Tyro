from cffi import FFI
ffibuilder = FFI()

ffibuilder.set_source("_libtyro", 
    # """ """,
    """
        typedef void** Tyro;

        void hello(int a);
        Tyro new_tyro(void);
        int add_100(Tyro, int);
        float add_reward(Tyro, float);
        float get_reward(Tyro);
        void drop_tyro(Tyro);
    """,
    libraries=['tyro'])
    # include_dirs=['/usr/'])

ffibuilder.cdef("""
    typedef void** Tyro;

    void hello(int a);
    Tyro new_tyro(void);
    int add_100(Tyro, int);
    float add_reward(Tyro, float);
    float get_reward(Tyro);
    void drop_tyro(Tyro);
""")

if __name__ == "__main__":
    ffibuilder.compile(verbose = True)

