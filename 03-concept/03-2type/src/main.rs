fn main() {

    // 複数の型が推定される場合は、型注釈が必要
    let guess: u32 = "42".parse().expect("Not a number!");

    /*  スカラー型4つ
            整数
            浮動小数点数
            論理値
            文字
    */

    // 整数
    // 指定しない場合i32だった。
    // _tenはi32
    let ten_num = 98_222;

    // 浮動小数点数
    // 指定しない場合はf64
    let float_num = 2.0;

    // 論理値
    let bool_type = true;

    // 文字型
    let test_str = 'a';

    //tuple
    let tup: (i32,f64,u8) =(500,3.0,1);

    let tupp = (500,6.4,1);
    let (x ,y, z) = tupp;
    println!("number{}",y)

    // 配列
    // 長さが決まってる時のみ使う。
    let arr = [1,2,3,4,5];
    let arrs :[i32; 5] = [1,2,3,4,5];
    // アクセス
    let first = arr[0];

}
