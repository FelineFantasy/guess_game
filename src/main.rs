use ask_input::input;
use rand::Rng;

fn main() {
    let mut count = 0;
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Угадай число от 1 до 100!");
    println!("{}", "=".repeat(50));

    loop {
        print!("Введите число: ");
        
        let guess: i32 = match input() {
            Ok(num) => num,
            Err(_) => {
                println!("Ошибка! Введите целое число.");
                continue;
            }
        };
        
        count += 1;

        if guess == secret {
            println!("{}", "=".repeat(50));
            println!("Ты угадал за {} попыток!", count);
            println!("{}", "=".repeat(50));
            break;
        } else if guess < secret {
            println!("Больше {}!", guess);
        } else {
            println!("Меньше {}!", guess);
        }
        println!("{}", "=".repeat(50));
    }

    println!("Нажми Enter для выхода...");
    input::<String>().unwrap();
}
