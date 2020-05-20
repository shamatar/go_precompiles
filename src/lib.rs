mod api;

pub fn perform_operation(address: u8, bytes: &[u8]) -> Result<(), ()> {
    let raw_operation_value: std::os::raw::c_char = unsafe { std::mem::transmute(address) };

    let input = bytes.as_ptr() as *const std::os::raw::c_char;
    let input_len = bytes.len() as u32;

    let is_error = unsafe { self::api::run_precompile(
        raw_operation_value,
        input, 
        input_len, 
    ) };
    if is_error != 0 {
        return Err(());
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::perform_operation;

    #[test]
    fn test_valid_vector_0() {
        let input = vec![0u8; 128];
        let _ = perform_operation(1u8, &input).unwrap();
    }
}