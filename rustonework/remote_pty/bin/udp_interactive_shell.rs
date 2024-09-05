#[derive(Debug)]
struct MyInt(i32);

impl From<MyInt> for i32 {
    fn from(value: MyInt) -> i32 {
        value.0
    }
}

impl Into<MyInt> for i32 {
    fn into(self) -> MyInt {
        MyInt(self)
    }
}

fn main() {
    let a = MyInt(10);
    let b: i32 = a.into();

    let aa = MyInt(100);
    let c = i32::from(aa);
    println!("{}, {}", b, c);

    let ct: MyInt = c.into();
    println!("{:?}", ct);
}
