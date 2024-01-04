/*
    phoneNumberGenerator
*/

/*
connectingBiblical:
biblicalBibliotOfOutput,BiblicalFormat (creation,Greeting)
*/
use std::io;
use std::io::Write;
//LList of forbidden combinations of numbers (constant (not formatted)) List of prohibited combinations of dials (constant (not format))
const BLOK: [&str; 10] = [
    "0000", "1111", "2222", "3333", "4444", "5555", "6666", "7777", "8888", "9999",
];

// Основний цикл main
fn main() {
    // Вивід слів в термінал
    println!("Введіть номер оператора 2 значне:");

    /*
        Cтворення змінної з водом значень з клавіатури
    */

    // Створення пустого листа
    let mut kib_kod: String = String::new();

    // Від значень з клавіатури в мутабельну зміну kib_kod(ЗМІНА НЕ ОПРЕДІЛЕНА)
    io::stdin().read_line(&mut kib_kod).expect("Помилка");
    /*
    Задаємо нову не мутабельные зміну kod з точно визначеним класом String(силка)
    прицьому видаляємо пробіли з початку і кінця листа за допомогою trim()
    */
    let kod: &str = kib_kod.trim();

    // Задаємо довжину номера
    // let mut kib_length: String = String::new();
    // io::stdin().read_line(&mut kib_length).expect("Помилка");

    // let s: &str = kib_length.as_str().trim();
    // let kil: f32 = s.parse::<f32>().unwrap(); //IN STR, OUT I32

    let mut length: f32 = 0.0000001;
    // Створюємо лічильник кількості згенерованих чисел
    let mut quantity: i32 = 0;

    // Створення файла (nomer.txt)
    println!("Створення файла(nomer.txt) з номерами");
    let mut file: std::fs::File = std::fs::File::create("nomer.txt").expect("Помилка");

    /*
    Цикл з створенням номерів
    */

    while length < 1.0 {
        // Створення номерів за допомогою додавання 1
        length += 0.0000001;

        // Форматуємо цей формат в строку за допомогою (format("{:.x}" - наскільки округлити значення)
        let format: String = format!("{:.7}", length);
        // Видаляємо перших два символи 0.
        let arr: &str = &format[2..];

        // Створюємо остаточний номер
        let z: String = format!("{}{}", &kod, &arr);
        // Перевіряємо чи значення не підпадають під блок
        if !z.contains(&BLOK[0])
            && !z.contains(&BLOK[1])
            && !z.contains(&BLOK[2])
            && !z.contains(&BLOK[3])
            && !z.contains(&BLOK[4])
            && !z.contains(&BLOK[5])
            && !z.contains(&BLOK[6])
            && !z.contains(&BLOK[7])
            && !z.contains(&BLOK[8])
            && !z.contains(&BLOK[9])
        {
            /*
            Водимо значення які пулучилися в фаєл (nomer.txt)
            */
            file.write_all(z.as_bytes()).expect("Проблема");

            // Для перехода на другу сторінку
            file.write_all("\n".as_bytes()).expect("Проблема");

            quantity += 1;
        }
    }

    // Виводемо скільки ми створили номерів
    println!("Було створено {}", &quantity);

    // Виводемо інформацію про те що програма закінчила роботу
    println!("Файл створений для закінчення роботи нажміть любу клавіші");

    // Затичка для того щоб зразу не обривати роботу програми
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Проблема");
}