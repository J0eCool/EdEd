extern {
    fn print(n: i32);
}

#[export_name = "prog"]
pub fn prog(x: i32) {
    unsafe {
        print(1);
        print(2);
        print(x);
    }
}

fn main() {
    unsafe {
        print(3);
    }
}
