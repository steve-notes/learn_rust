/*
CRATES
- Menggunakan keyword `crate` 
- Digunakan ketika kita ingin melakukan `use` pada suatu file module, dari file module yang lainnya, bukan dari file `main.rs`
- sebagai contoh pada module atau file sebelumnya `print.rs` kita buat sebuah function baru bernama `test_crate` 
- untuk memanggil module crates.rs ini tetap dilakukan di `main.rs`
  lalu kita gunakan dan panggil menggukanan `use` dari file atau module `crates.rs` ini ke file print.rs untuk menggunakan function test_crate()
*/

// 
// pub fn test_crate(){
//     println!("Hello from module crate!")
// }

use crate::print::test_crate;

pub fn use_crate() {
    println!("Hello from crates.rs files ");

    test_crate()
}


/* 
SUPER
- `super` digunakan untuk mengakses module yang ada diatasnya ( parent module ) 
- `super` adalah alternatif lain, karena kita juga bisa melakukan hal itu dengan menggunakan `crate`
*/
// example 


pub mod satu {
    pub mod dua {
        pub fn say_hello() {
            // crate::crates::use_crate();
            super::super::use_crate();
        }
    }
}