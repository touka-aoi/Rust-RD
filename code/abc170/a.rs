use proconio::input;

fn main() {
    //方針
    //偶数でYが4X以下かつ2X以上ならいける
    input! {
        x: i32,
        y: i32,
    }
    if y % 2 == 0 && 2 * x <= y && y <= 4 * x {
        println!("Yes");
    } else {
        println!("No");
    }
}
