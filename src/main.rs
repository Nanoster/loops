fn main() {
    // loop {
    //     println!{"Again!"};    // continual loop
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         // break counter *2;    
    //         break counter *2    //;를 붙이든 안붙이든 반환한다.
    //     }
    // };

    // println!{"Ther result is: {result}"};

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop{
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!{"End count = {count}"};

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];    
    // let mut index = 0;

    // while index < 5 {    //요소를 순회하며 하나씩 출력
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }

    // for number in (1..4).rev() {    //1이상 4미만 역순 number >= 1 && number < 4
    //     println!{"{number}!"};
    // }
    // println!("LIFTOFF!!!");

    
    // for number in 1..4 {    //1이상 4미만 number >= 1 && number < 4
    //     println!{"{number}!"};
    // }
    // println!("LIFTOFF!!!");

    // for number in 1.. {    //1이상 number >= 1 (무한루프)
    //     println!{"{number}!"};
    // }
    // println!("LIFTOFF!!!");

    // for number in (std::ops::Range{start:0, end:4}) {    //1이상 4미만 number >= 1 && number < 4
    //     println!{"{number}!"};
    // }
    // println!("LIFTOFF!!!");

    let num = 4;

    match num {
        1..=3 => println!("num is in 1~3"),
        4..=7 => println!("num is in 4~7"),
        _ => println!("num is out of 1~7")
    };
    
}
