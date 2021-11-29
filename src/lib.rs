use std::ffi::CString;
use std::marker::{PhantomData, PhantomPinned};

pub struct Wrapped {
    called: u16,
}

#[repr(C)]
pub struct BigData {
    _data: Wrapped,
    _marker:
        PhantomData<(*mut u8, PhantomPinned)>,
}

#[no_mangle]
pub extern "C" fn hello_rust() -> *mut i8 {
    CString::new("Hello, world!").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn make_big_data() -> *mut BigData {
     Box::into_raw(Box::new(BigData {
        _data: Wrapped{ called: 0 },
        _marker: PhantomData,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn update_big_data(data: *mut BigData) {
    (*data)._data.called += 1;
}

#[no_mangle]
pub unsafe extern "C" fn how_much_data(data: *mut BigData) -> u16 {
    // uncomment to see an error from Ruby
    // panic!("ded");
    (*data)._data.called
}

#[no_mangle]
pub unsafe extern "C" fn free_big_data(data: *mut BigData) {
    Box::from_raw(data);
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
