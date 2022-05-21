fn main() {
    println!("hello!world");

    // 命名はスネークケース
    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("value of y is :{}",y);

    let five_x = five();
    println!("x:{}",five_x);
}

fn another_function(value: i32, unit_label:char){
    println!("the measurement is {}{}",value, unit_label);
}

fn five () -> i32 {
    5
}
