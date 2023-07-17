mod strctPerson;

use std::io;
//io - i= input o=output, bu veri alma ve cikti verme icin kullanilan bir kutuphanedir
use crate::strctPerson::Person;

#[derive(Debug)]
struct Str(i32,String,f64);//tuples

struct Triangle{
    cat1:f64,
    cat2:f64
}

fn main() {
    /*let (eksi,arti,carpi) = math(5,3    );
    println!("{eksi},{arti},{carpi}");*/

}

fn personstrct() {
    let age: i8 = 24;
    let balance: f64 = 900.54;
    let person1 = Person {
        name: "Bega".to_string(),
        surname: "Haciyew".to_string(),
        age,
        balance,
    };
    let person2 = Person {
        name: "Begench".to_string(),
        surname: "Hacc".to_string(),
        ..person1
    };
    //tuples
    let object = Str(5,"Txt".to_string(),4512.21);
    println!("{object:#?}");
}

fn insturtion() {
    let mult = mul(4, 4);
    println!("{mult}");
}

fn math(a: i32, b: i32) -> (i32, i32, i32) {
    (a - b, a + b, a * b)
}

fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}

fn info(name: String, age: i32, wallat: f64) {
    println!("Name:{}\nAge:{}\nWallat:${}", name, age, wallat);
}

fn sum(a: i32, b: i32) {//kurs functions
    println!("{}", a + b);
}

fn kortej() {
    //kortej ve array arasindaki fark array icinde sadece bir tur veri saklanabilir
    //kortej icinde ise farkli turlerden veriler saklanabilir.
    /*let tuple = (12, 34.6, String::from("LOL"));
    println!("{:?}",tuple);*/
    let people = (String::from("Bega"), 11, 12);
    //destruturizatsiya
    /*let (name, grande) = people;
    println!("Name is:{name},\nGrade is:{grande}");*/
    /*let name = people.0;
    let grande1 = people.1;
    let grande2 = people.2;
    let (name,grande1,grande2)=people;
    println!("{},{},{}",name,grande1,grande2);*/
}

fn constant() {
    const NUM: i8 = 3;
    //constlar da fonksiyona esit olamazlar ama let ile bunu yapabiliriz.
}

fn massivi() {
    /*let arr = [2;50];
    print!("{:?}",arr);
    let array = [5,5,4,4,5,6,7,8];
    for i in array.iter(){
        println!("{}",i);
    }
    let array = [5,5,4,4,5,6,7,8];
    println!("{}",array.len());
    for i in 0..array.len(){
        println!("{}",array[i]);
    }
    let array = [5, 5, 4, 4, 5, 6, 7, 8];
    for i in array.iter() {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }*/

    let array = [1, 8, 2, 2, 4, 5, 6, 7, 7, 8, 9, 9];
    let mut i = 0;
    while i < array.len() {
        let mut j = i + 1;
        while j < array.len() {
            if array[i] == array[j] {
                print!("{}", array[i]);
            }
            j += 1;
        }
        i += 1;
    }
}

fn krutayaproga() {
    //ax^2+bx+c=0
    //D=b^2-4*(a*c)
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    loop {
        println!("Solution of quadratic equation");

        println!("Enter a:");
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {}
            Err(e) => println!("Error occurred in variable input - {}", e)
        }
        println!("Enter b:");
        match io::stdin().read_line(&mut b_str) {
            Ok(_) => {}
            Err(e) => println!("Error occurred in variable input - {}", e)
        }
        println!("Enter c:");
        match io::stdin().read_line(&mut c_str) {
            Ok(_) => {}
            Err(e) => println!("Error occurred in variable input - {}", e)
        }
        let a: f64 = a_str.trim().parse().unwrap();
        let b: f64 = b_str.trim().parse().unwrap();
        let c: f64 = c_str.trim().parse().unwrap();

        let d: f64 = (b * b) - 4.0 * (a * c);

        if d > 0.0 {
            let x1 = ((-b) + d.sqrt()) / (2.0 * a);
            let x2 = ((-b) - d.sqrt()) / (2.0 * a);
            println!("Has been dissolved:\nX1:{x1}\nX2:{x2}\nD:{d}");
        }
        if d == 0.0 {
            let x = (-b) / (2.0 * a);

            println!("Has been dissolved:\nX:{x}");
        }
        if d < 0.0 {
            println!("No root!!!");
        }
    }
}

fn girdi_alma() {
    let mut name: String = String::new();
    println!("Enter your name:");
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Welcome {}", name);
        }
        Err(e) => {
            println!("Error program - {e}");
        }
    }
}

fn dersmatch1() {
    let mut num = 10;
    match num {//bu operator aslinda if else operetorlerinin yerine kullanilmaktadir
        11 => println!("Num is 10"),
        23 => {
            println!("Num is 23");
            println!("Num is matched!!");
        }
        10..=50 => {
            println!("number is under 10 and 50");
        }
        _ => {//bu satir default olarak eger kullanici olmayan bir secenegi secerse
            println!("You have selected an unavailable option");
        }
    }
}

fn dersmatch2() {
    let is_old: bool = true;
    let mut ready_num: String = String::new();//bos bir satir olusturmak icin
    match is_old {
        false => {
            ready_num = String::from("You cannot enter");
        }
        true => {
            ready_num = String::from("You can enter");
        }
    }
    println!("{ready_num}");
}

fn fordonguler() {
    for i in 0..101 {
        if i % 2 == 0 {
            println!("{i}")
        }
    }
}

fn donguwhile() {
    let mut num = 0;
    while num <= 100 {
        if num % 11 == 0 {
            println!("{num}");
        }
        num += 1;
    }
}

fn donguloop() {
    let mut num = 0;
    loop {//bu dongu turu sonsuza dek giden turden

        println!("{num}");
        num += 1;
        if num == 100 {
            break;
        }
    }
}

fn find_bigger() {
    let numbers = vec![10, 12, 47, 6, 5, 45, 4, 5, 4, 4, 45, 5, 555, 565, 54, 6, 4, 564, 4654, 646, 564, 654, 4, 56, 4654, 1231, 1121];

    let mut max_number = numbers[0];

    for &number in &numbers {
        if number > max_number {
            max_number = number;
        }
    }

    println!("En büyük sayı: {}", max_number);
}
