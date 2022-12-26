
// fn main() {
//     // println!("Hello, world!");
//     // let nama = "Aji Tirto Prayogo";
//     // println!("{}",nama);


//     let mut  angka= 1;
//     let pesan = "pesan dari @ajitirto";
//     println!(" No - {0} - {1}", angka, pesan);

//     angka= 2;
//     let pesan = "pesan dari @ajiwoke";
//     println!(" No - {} - {}", angka, pesan);

//     angka= 3;
//     let pesan = 24;
//     println!(" No - {1} - {0}", angka, pesan);

//     // metode tyoe "interface" 
//     let var1 = 12;
//     let var2 = 12;
//     // metode type manifest => menulis secara eskplisit
//     let message: i8 = 24; 
//     println!(
//         "{}-{}-{}",
//         var1,
//         var2,
//         message
//     );


// }

// A.4.7
// fn main() {
//     let(var1, var2) = (24, "hello");
//     println!("var1: {0}", var1);
//     println!("var2: {0}",var2);

//     let(var3, var4):(i8,i8) = (31,24);
//     println!("var3: {0}", var3);
//     println!("var4: {0}",var4);

//     let data1  = 28_i8;
//     println!("data1 {0}", data1);

//     let x = 5;
//     println!{"x {}", x};

//     let x = x +1;
//     println!("x+1: {}", x);
// }


// A.5
// fn main() {
//     // sign integer
//     let number1:i8 = 12;
//     let number2:i8 = 22;
//     let number3:i32 = 148; // 128 => i8 128 > i32

//     println!("{}-{}-{}", number1,number2,number3);

//     // unsign integer => tidak menampung value negatif
//     let number4:i32 = 12;
//     let number5:i8 = 22;
//     let number6:i64 = 148; // 128 => i8 128 > i32

//     println!("{}-{}-{}", number4,number5,number6);


//     // floating point 

//     let angka1:f32 = 3.14;
//     let angka2:f64 = 3.1412312312;

//     println!("{}|{:.5}",angka1,angka2);

//     // Bool
//     let b1= true;
//     let b2= false;

//     println!("{}|{}",b1,b2);

//     // char
//     let c1='n';
//     let c2 = '-';
//     let c3 = '2';

//     println!("{}|{}|{}", c1,c2,c3);

//     // porinter scalar

//     let pointer: &i32 = &24;
//     println!("{}", pointer);

// }


// A.6

// fn main() {
//     let var1: &str = "hello";
//     println!("{}", var1);

//     let var2 =
//         "Halo \
//         aji \
//         tirto \
//         ";
//     println!("{}", var2);


//     let var3 ="baris satu
//     baris dua 
//     baris tiga";
//     println!("{}", var3);
    
//     let var4 = r#"
//     {
//         "name": "ajitiro",
//         "gender": "male"
//     }"#; // raw sring tidak meng-enscape semua karakter
//     println!("{}",var4);

// }


// A.7

// satic keyword
// static NUMBER: i32 = 18;

// fn main() {
//     const LABEL: &str = "nilai pi adalah : ";
//     const PI :f32 = 22.0/7.0;
//     println!("{} {}", LABEL, PI); 

//     println!("{}", NUMBER);

// }

// A.8

// fn main() {

//     let number1 = 12;
//     let number2 = 24;
//     let perbandingan = number1  == number2;
//     println!("hasil_perbandingan = {perbandingan}"); 

//     // operator negasi 

//     let (var1, var2) = (12,-12);

//     let satu = var1 == var2;

//     println!("{satu}");
// }

// A.9

//fn main() {
    // // if else 
    // let n = 14;
    // if  n < 13 {
    //     println!("n dibawah 13")
    // }else {
    //     println!("n diatas  13")
    // }


    
    // // Nested if 
    // let x = 80;

    // if 50  <  x {
    //     println!("Anda Lulus!");
    //     if x == 80 {
    //         println!("nilai adan sudah cukup");
    //     }
    // } 

    // // Return from if 

    // let number = 5;
    // let result : bool;

    // if number == 5 {
    //     result = true;
    // } else {
    //     result = false;
    // }

    // println!("result adalah {result}");

    // let number  = 3 ;
    // let result : &str = if number == 3 {
    //     "angka adalah 3 "
    // } else if number < 3 {
    //     "angka di bawah 3"
    // } else {
    //     "angka di atas3"
    // };

    // println!("result : {result}");

    

//}

// fn main () {

//     let mut i = 0;
//     let max = 5;

//     while i < max {
//         let mut j  = 0;
//         let max_inner = i;
//         while j <= max_inner {
//             print!("* ");
//             j +=1;
//         }
//     }

    
// }

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut i = 0+1;
    let max = 5;

    while i <= max {
        println!("nilai {i}");
        i+=1;

        sleep(Duration::from_secs(1));        
    }
}