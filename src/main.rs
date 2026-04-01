use std::io;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    loop {
        println!("数字を入力してください（'q' で終了）:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        if trimmed == "q" {
            break;
        }

        match trimmed.parse::<i32>() {
            Ok(num) => {
                println!("Memory Add: {}", num);
                numbers.push(num);
            }
            Err(_) => println!("数字か 'q' を入れてね！"),
        }
    }

    let sum: i32 = numbers.iter().sum();

    println!("--- Result ---");
    println!("入力された数: {:?}", numbers);
    println!("合計値: {}", sum);
    println!("プログラムを終了します。メモリは自動で解放されました。");
}
