use std::io;

fn main() {
    // Input nama siswa
    println!("Masukan Nama siswa :");
    let mut nama = String::new();
    io::stdin()
        .read_line(&mut nama)
        .expect("gagal membaca input");

    // Input nilai matematika
    println!("Masukan nilai matematika :");
    let mut matematika = String::new();
    io::stdin()
        .read_line(&mut matematika)
        .expect("Gagal membaca input");
    let matematika: f64 = matematika.trim().parse().expect("Input bukan angka");

    // Input nilai fisika
    println!("Masukan nilai fisika");
    let mut fisika = String::new();
    io::stdin()
        .read_line(&mut fisika)
        .expect("Gagal membaca input");
    let fisika: f64 = fisika.trim().parse().expect("Input buka angka");
}
