fn main() {
    fn opaque_read(val: &i32) {
        println!("{}", val);
    }

    unsafe {
        let mut data = 10;
        let mref1 = &mut data;
        let ptr2 = mref1 as *mut i32;
        let sref3 = &*mref1;

        *ptr2 += 2;
        opaque_read(sref3); // Read in the wrong order?
        *mref1 += 1;

        opaque_read(&data);
    }
}
