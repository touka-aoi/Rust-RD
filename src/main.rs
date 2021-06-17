fn main() {
    //あいうえお
    println!("test{}", 1 / 2);
    //変数代入
    let hoge; //よく見たらダイナミックだ...
    hoge = 10;
    println!("{}", hoge);
    //型注釈
    let hoge2: i32;
    //生文字リテラル
    println!(r"\\\\\\\\");
    //ダブルクォーテーションリテラル
    println!(r#""""""""""""""""""""""""a"bbb"#);
    //リテラルをつける
    let spam = 1_f32; //1のfloat32型

    println!("{}", spam);
}
