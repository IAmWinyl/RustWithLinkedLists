use std::cell::Cell;

fn main() {
    // fn opaque_read(val: &i32) {
    //     println!("{}", val);
    // }

    unsafe {
        let mut data = Cell::new(10);
        let mref1 = &mut data;
        let ptr2 = mref1 as *mut Cell<i32>;
        let sref3 = &*mref1;

        sref3.set(sref3.get() + 3);
        (*ptr2).set((*ptr2).get()+2);
        mref1.set(mref1.get() + 1);

        println!("{}", data.get())
    }
}
