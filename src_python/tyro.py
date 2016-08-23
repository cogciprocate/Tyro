import numpy as np
from _libtyro import ffi, lib

# Declare type "constants" (for ffi):
TYPEID_FLOAT32 = 0 # float
TYPEID_FLOAT64 = 1 # double
TYPEID_INT32   = 2 # int
TYPEID_INT64   = 3 # long


class Tyro(object):
    _ptr = None

    def __enter__(self):
        self._ptr = ffi.new('Tyro', lib.new_tyro())
        return self

    def push_observation(self, obs):
        obs_ptr = ffi.cast("double*", obs.ctypes.data)
        assert len(obs.shape) == obs.ndim
        assert len(obs.shape) == 1
        dims = np.array([obs.shape[0], 1])
        dims_ptr = ffi.cast("long*", dims.ctypes.data)
        type_id = TYPEID_FLOAT64        
        lib.push_vec_frame(self._ptr[0], obs_ptr, type_id, dims_ptr)

    def cycle(self):
        lib.cycle(self._ptr[0])

    def add_100(self, addend):
        return lib.add_100(self._ptr[0], addend)

    def print_observation(self, obs):
        obs_ptr = ffi.cast("double*", obs.ctypes.data)
        assert len(obs.shape) == obs.ndim
        assert len(obs.shape) == 1
        dims = np.array([obs.shape[0], 1])
        dims_ptr = ffi.cast("long*", dims.ctypes.data)
        type_id = TYPEID_FLOAT64        
        lib.print_array(obs_ptr, type_id, dims_ptr)

    def add_reward(self, reward):
        lib.add_reward(self._ptr[0], reward)

    def get_reward(self):
        return lib.get_reward(self._ptr[0])

    def __exit__(self, exc_type, exc_value, traceback):
        # Return threads and drop:
        lib.drop_tyro(self._ptr[0])
