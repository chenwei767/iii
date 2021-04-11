#![deny(warnings)]

fn main() {
    let i = 0i64;
    change_value();
    assert_eq!(i, 1);
}

//
// Implement this function to run a successful `cargo run --release`.
//
// **NOTE**
// - do NOT change any existing codes except that `todo!()`
//
fn change_value() {
    let x: i64 = 0;
    let mut px: *mut i64 = &x as *const _ as *mut _;
    unsafe {
        px = px.offset(-2);
        println!("{:p}", px);
        *px = 1i64;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let mut a = Vec::new();

        {
            // fix this line to make this test pass
            a.resize(10000001, 0);
            a[10000000] = 1;
        }

        assert_eq!(a[10000000], 1);
    }

    #[test]
    fn test2() {
        let a = async { "Hello World!" };

        let b;

        {
            // fix this line to make this test pass
            b = futures::executor::block_on(a);
        }

        assert_eq!(b, "Hello World!");
    }
}
