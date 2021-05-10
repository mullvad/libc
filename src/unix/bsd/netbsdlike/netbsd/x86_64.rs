use PT_FIRSTMACH;

pub type c_long = i64;
pub type c_ulong = u64;
pub type c_char = i8;
pub type c_greg_t = u64;
pub type __cpu_simple_lock_nv_t = ::c_uchar;

// should be pub(crate), but that requires Rust 1.18.0
cfg_if! {
    if #[cfg(libc_const_size_of)] {
        #[doc(hidden)]
        pub const _ALIGNBYTES: usize = ::mem::size_of::<::c_long>() - 1;
    } else {
        #[doc(hidden)]
        pub const _ALIGNBYTES: usize = 8 - 1;
    }
}

pub const PT_STEP: ::c_int = PT_FIRSTMACH + 0;
pub const PT_GETREGS: ::c_int = PT_FIRSTMACH + 1;
pub const PT_SETREGS: ::c_int = PT_FIRSTMACH + 2;
pub const PT_GETFPREGS: ::c_int = PT_FIRSTMACH + 3;
pub const PT_SETFPREGS: ::c_int = PT_FIRSTMACH + 4;

s_no_extra_traits! {
    #[repr(align(8))]
    pub struct mcontext_t {
        pub __gregs: [c_greg_t; 26],
        pub _mc_tlsbase: c_greg_t,
        pub __fpregs: [::c_char; 512],
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_uint,
        pub uc_link: *mut ::ucontext_t,
        pub uc_sigmask: ::sigset_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: ::mcontext_t,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for mcontext_t {
            fn eq(&self, other: &mcontext_t) -> bool {
                self.__gregs.iter().zip(other.__gregs.iter())
                .all(|(a, b)| a == b) &&
                self._mc_tlsbase == other._mc_tlsbase &&
                self.__fpregs.iter().zip(other.__fpregs.iter())
                .all(|(a, b)| a == b)
            }
        }
        impl Eq for mcontext_t {}
        impl ::fmt::Debug for mcontext_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("mcontext_t")
                    .field("_mc_tlsbase", &self._mc_tlsbase)
            }
        }
    }
}
