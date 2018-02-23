import cffi
ffi = cffi.FFI()
ffi.cdef('''
         typedef void (*hello)(void);
         hello getProcAddr(void);
         ''')
dll = ffi.dlopen('/home/realitix/git/grind-kernel/target/release/libgrindkernel.so')

proc = dll.getProcAddr()
proc()
