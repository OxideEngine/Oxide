use std::array::TryFromSliceError;

#[macro_export]
macro_rules! debug_print {
    ($e:expr) => {
        println!("{}: {:?}", stringify!($e), $e)
    };
}

pub fn bytes_to_u32(bytes: &[u8]) -> Result<u32, TryFromSliceError> {
    let bytes = <[u8; 4]>::try_from(bytes)?;
    Ok(u32::from_le_bytes(bytes))
}
