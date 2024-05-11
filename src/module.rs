/*
- untuk membuat module kita dapat menggukan keyword "mod"
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
