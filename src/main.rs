// ==== Импорт библиотек ====
use ask_input::{int_input, str_input};
use rand::Rng;

fn main() {
    // ==== Создания переменных ====
    let mut count = 0;
    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(1..=100);

    // ==== Цикл игры ====
    loop {
        // Ввод пользователя
        println!("Введите число от 1 до 100: ");
        let guess = int_input();
        count += 1;

        // Проверка
        if guess == secret {
            println!("{}", "=".repeat(50));
            println!("Ты угадал за {} попытки!", count);
            println!("{}", "=".repeat(50));
            break;
        } else if guess < secret {
            println!("{}", "=".repeat(50));
            println!("Число больше {}!", guess);
            println!("{}", "=".repeat(50));
        } else {
            println!("{}", "=".repeat(50));
            println!("Число меньше {}!", guess);
            println!("{}", "=".repeat(50));
        }
    }

    println!("Нажми Enter для выхода...");
    str_input();
}