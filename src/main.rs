use proconio::input;

fn main() {
    input! {
        x: i32;
    }
    let r = x % 10;
    //チェックしてあげる
    assert!(0 <= r && r < 10);
    println!("あまりは {}", r);
}
