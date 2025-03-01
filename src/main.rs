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

    // Input nilai kimia
    let mut kimia = String::new();
    io::stdin()
        .read_line(&mut kimia)
        .expect("Gagal membaca input");
    let kimia: f64 = kimia.trim().parse().expect("Input bukan angka");

    // save data lewat tuple
    let nilai_siswa = (matematika, fisika, kimia);

    // Menghitung niai rata-rata
    let rata_rata = (nilai_siswa.0 + nilai_siswa.1 + nilai_siswa.2) / 3.0;
}
