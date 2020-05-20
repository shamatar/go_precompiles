extern "C" {
    pub fn run_precompile(
        op: ::std::os::raw::c_char,
        i: *const ::std::os::raw::c_char,
        i_len: u32
    ) -> u32;
}
