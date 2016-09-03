from cffi import FFI
ffibuilder = FFI()

ffibuilder.set_source("_libtyro", 
    # """ """,
    """
        typedef void** Tyro;
        
        Tyro new_tyro(void);
        void set_encoder_ranges(Tyro, double[], double[], int);
        void push_vec_frame(Tyro, void*, int, long[2]);
        void cycle(Tyro);
        void print_array_f64(double[], int);
        void print_array(double[], int, long[2]);
        float add_reward(Tyro, float);
        float get_reward(Tyro);
        int add_100(Tyro, int);
        void drop_tyro(Tyro);
    """,
    libraries=['tyro'])
    # include_dirs=['/usr/'])

ffibuilder.cdef("""
    typedef void** Tyro;

    Tyro new_tyro(void);
    void set_encoder_ranges(Tyro, double[], double[], int);
    void push_vec_frame(Tyro, void*, int, long[2]);
    void cycle(Tyro);    
    void print_array_f64(double[], int);
    void print_array(double[], int, long[2]);
    float add_reward(Tyro, float);
    float get_reward(Tyro);
    int add_100(Tyro, int);
    void drop_tyro(Tyro);
""")

if __name__ == "__main__":
    ffibuilder.compile(verbose = True)

