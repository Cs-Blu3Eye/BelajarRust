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

fn main(){
    belajar_print();
    belajar_variabel();
}
