use proconio::input;

fn main() {
    input! {
        sx: f32,
        sy: f32,
        gx: f32,
        gy: f32,

    }
    let ans: f32;
    ans = sx + (((gx - sx) / (gy + sy)) * sy);
    println!("{}", ans);
}
