/*
Trait adalah definisi fungsionalitas yang dapat digunakan untuk tipe data lain
Trait dapat digunakan untuk mengimplementasikan fungsionalitas yang sama untuk beberapa tipe data
simplenya trait adalah method yang dapat di share untuk digunakan di type data lain baik itu struct maupun enum
Gaya Penulisan nama Trait sama seperti struct yaitu UpperCamelCase
untuk mengimplementasi trait adalah seperti berikut :
    impl NamaTrait for NamaType {
    // isi method
    }
*/

// Contoh Pembuatan dan penggunaan Trait pada struct Person


pub struct Person {
   pub first_name: String,
   pub last_name: String,
   pub age: u8,
   pub middle_name: String,
}

// impl Person {
//     pub fn say_hello(&self, name: &str){
//         println!("Hello {}, nama saya {}", name, self.first_name)
//     }
// }


pub trait HelloCuy {
    fn hello_cuy(&self) -> String;
    fn hello_bray(&self, name: &str) -> String;
}

impl HelloCuy for Person {
    fn hello_cuy(&self) -> String {
        format!("Hello cuy nama gw {}", self.first_name)
    }
    
    fn hello_bray(&self, name: &str) -> String {
        format!("Hello bray {} nama gw {}", self.first_name,name )
    }
}


pub fn test_trait(){
    let person = Person {
        first_name: String::from("Dadang"),
        middle_name: String::from("Cempak"),
        last_name: String::from("cangcimen"),
        age: 20,
    };

    let cuy = person.hello_cuy();
    let bray = person.hello_bray("komeng");

    println!("{}", cuy);
    println!("{}", bray);
}