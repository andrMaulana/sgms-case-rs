fn main() {
    // Input nama siswa
    println!("Masukan Nama siswa :");
    let mut nama = String::new();
    io::stdin()
        .read_line(&mut nama)
        .expect("gagal membaca input");
}
