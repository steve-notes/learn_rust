/*

- Type Alias atau Type Aliasing dapat digunakan untuk memberikan nama baru kepada type yang sudah ada ( existing type )
- keywork yang digunakan untuk alias adalah "type"
- Cara penulisan type alias adalah UpperCamelcase, atau diawali dengan huruf besar
- Misalkan seperti yang kita ketahui ada type data i32 untuk bilangan bulat, kita dapat menggunakan type alias untuk memberikan alias atau nama baru kepada tipe tersebut
- ex:  type BilanganBulat = i32
- contoh diatas berarti BilanganBulat adalah alias dari tipe data i32
*/

// Example :

type Age = u8;
type IdentityNumber = String;


struct IdentityCard {
    id : IdentityNumber,
    name : String,
    age : Age,
}

pub fn identity() {
    let identity: IdentityCard = IdentityCard {
        id: String::from("3243234234234"),
        name: String::from("sempak"),
        age: 30,
    };

    println!("id : {}",identity.id);
    println!("name : {}",identity.name);
    println!("id : {}",identity.age);
}