/*
- untuk membuat module kita dapat menggukan keyword "mod"
- jika membuat module di file terpisah, kita tidak perlu lagi menggunakan 'mod', dikarenakan nama file modul yang kita buat akan otomatis menjadi nama module
  sehingga kita tidak perlu lagi menambahkan 'mod' kecuali jika kita ingin membuat sub-module, 
- untuk membuat module agar dapat diakses harus mengubah status dari private ke public
- untuk mengubah dari private ke public tambahkan keywork "pub"
- keyword "pub" dapat ditambahkan didepan atau diawal Type, Function atau Method
- Lihat contoh dibawah berikut, pada Type Struct kita tambahkan pub didepannya, berikut juga setiap awal nama field kita tambahkan pub
   agar filed pada struct tidak lagi menjadi private
- pada method, kita tambahkan juga pub sebelum keyword fn untuk menuliskan function
- untuk memanggil module dari luar,atau mengakses module dari luar kita menggunakan tanda :: ( titik dua, dua kali) lalu di ikutin dengan nama Type atau Functionnya
*/

mod model {
pub struct User {

   pub first_name: String,
   pub last_name: String,
   pub username: String,
   pub email: String,
   pub age: u8,
}

impl User {
    pub fn hello(&self, name: &str) {
        println!("Hello {}", name);
        println!("Your Full Name is {} {}", self.first_name, self.last_name);
        println!("Your Age is {} ", self.age);
    }
}
}

pub fn test_module() {

    let user  = model::User {
        first_name : String::from("sempak"),
        last_name : String::from("kendor"),
        username : String::from("sempaks"),
        email : String::from("sempaks@kendor"),
        age : 30,
    };

    user.hello("sempak")

}


/*
USE KEYWORD
- USE keyword dapat digunakan untuk mengambil member dari sebuah module
  dengan menggunakan use kita dapat masuk ke dalam scope module, dan membawa member dalam sccope module tersebut keluar
  sehingga kita dapat menggunakan member dari module tersebut, tanpa harus berulang kali memanggil nama modul tersebut
- jika saat menggunakan use untuk memanggil member atau type pada sebuah module
  terdapat dua member atau type dalam satu scope yang memiliki nama yang sama, kita dapat menggunakan "as" keyword, untuk
  memberikan nama baru atau alias untuk member / type tersebut
- Untuk menggunakan `use` keyword, jika kita ingin mengambil seluruh member dari module, kita dapat menggunakan tanda `*` (bintang) : use nama_module::*
- Jika kita ingin mengambil beberapa module saja, kita dapat menggunakan tanda {} ( kurung kurawal) : use modul_name::{A,B,C}
*/
// Example :
mod first {
    pub fn hello() {
        println!("Hello from first module");
    }
}

mod second {
    pub fn hello() {
        println!("Hello from second module");
    }
}

// menggunakan 'use' keyword mengambil function dari sccope module 
use first::hello;
use second::hello as hello_second;

// kita dapat menggunakan function yang terdapat pada scope module tersebut, tanpa harus memanggil nama module nya lagi.
pub fn test_use_keyword() {
    hello();
    hello_second();
}