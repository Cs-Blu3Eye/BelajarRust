use core::num;


mod main2;
use crate::main2::*;

fn belajar_print() {
    // Print dengan 1 line
    println!("Hello, world!");
    
    // Print tanpa menekankan pembuatan line baru
    print!("Hello, world! ");

    // Print yang menambahkan karakter khusus \n untuk line baru
    print!("Fuck You!\n")
}

fn belajar_variabel(){
    // Variabel sama seeperti python (tidak memerlukan deklarasi type data)
    let nama="Fahmi";
    let umur=18;

    // Disini variabel yang dideklarasikan dimasukkan sesuai urutannya diletakkan
    println!("Nama saya {} Umur saya {}",nama, umur)
}

fn belajar_type_data(){
    let mynum=5;
    let double=5.21;
    let my_letter='I';
    let my_text= "Like";
    let my_bool= true;

    print!("Hello {} {} a number {} and {} this type {} \n",my_letter,my_text,mynum,double,my_bool)
}

fn main(){
    // belajar_print();
    // belajar_variabel();
    // belajar_type_data();
    // belajar_opration();
    // belajar_if_else();

    belajar_match();
}
