fn main() {
    let number = 4;

    //　if else elese if の使い方
    if number < 5 {
        println!("confition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("4で割り切れる")
    } else if number % 3 == 0 {
        println!("3で割り切れる")
    } else if number % 2 == 0 {
        println!("2で割り切れる")
    } else {
        println!("numberは4,3,2で割り切れません")
    }
    println!("-----------------------");

    // ifは式なので代入可能
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("numは{}",num);
    println!("-----------------------");

    // loop
    let mut count = 0;
    // 'counting_upはループラベル
    'counting_up: loop {
        println!("count={}", count);
        let mut remaining = 10;

        loop {
            println!("remaining={}",remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}",count);
    println!("-----------------------");

    // while文
    let mut while_number = 3;
    while while_number != 0 {
        println!("{}!",while_number);
        while_number -= 1;
    }
    println!("発射!");
    println!("-----------------------");
    println!("whileだと遅く、間違えやすい");
    println!("-----------------------");

    // for
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("arrya value is: {}",a[index]);
        index += 1;
    }
    println!("-----------------------");
    println!("forだと早く、間違えにくい");
    println!("-----------------------");
    for element in a {
        println!("the value is:{}",element)
    }
    println!("-----------------------");
    println!("whileではなく、forを使うことがおおい");
    println!("-----------------------");

    for number in (1..4).rev() {
        println!("{}!",number);
    }
    println!("発射!");
}
