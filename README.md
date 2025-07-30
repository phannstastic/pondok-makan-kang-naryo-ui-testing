***UI Testing Pondok Makan Kang Naryo***
Repositori ini berisi kumpulan skrip pengujian UI otomatis untuk aplikasi web Pondok Makan Kang Naryo. Proyek ini dibuat untuk memastikan kualitas dan fungsionalitas antarmuka (UI) berjalan sesuai harapan untuk berbagai peran pengguna.

**🧪 Skenario Pengujian**
Pengujian dibagi berdasarkan tiga peran utama pengguna:

**Admin**
* Login berhasil dan gagal.
* Menambah, mengubah, dan menghapus item menu.
* Logout.

**Pelanggan (Customer)**
* Memulai pesanan dengan nama.
* Menambah dan mengurangi jumlah item di keranjang.
* Melakukan checkout (simulasi).

**Pengunjung (Visitor)**
* Navigasi halaman utama.
* Melihat halaman galeri.
* Memastikan link navigasi berfungsi.

**🛠️ Tools & Teknologi**
* Framework Utama: Katalon Studio
* Bahasa Skrip: Groovy
* Library Inti: Selenium WebDriver
* Version Control: Git & GitHub

**🚀 Memulai Proyek**
Berikut adalah cara untuk mengunduh dan menjalankan proyek ini di komputer Anda.

*Prasyarat*
* Katalon Studio versi terbaru telah terpasang.
* Git telah terpasang.

*Instalasi*
Clone repositori ini ke lokal Anda:
```bash
git clone https://github.com/phannstatic/pondok-makan-kang-naryo-ui-testing.git
```

Buka Proyek di Katalon Studio:
1. Buka aplikasi Katalon Studio.
2. Pilih File > Open Project.
3. Arahkan ke folder tempat Anda melakukan clone repositori, lalu klik Open.

**▶️ Cara Menjalankan Tes**
Ada dua cara utama untuk menjalankan pengujian:
1. Menjalankan Peran Spesifik (Test Suite)
Jika Anda hanya ingin menguji skenario untuk satu peran (misalnya, Admin):
  * Di panel Tests Explorer, buka folder Test Suites.
  * Klik dua kali pada Test Suite yang diinginkan, misalnya Test_Suite_Admin.
  * Klik tombol Run (ikon panah hijau) di toolbar atas dan pilih browser.

2. Menjalankan Semua Tes (Test Suite Collection)
Untuk menjalankan seluruh rangkaian tes regresi dari semua peran:
  * Di panel Tests Explorer, buka folder Test Suites.
  * Klik dua kali pada Test Suite Collection Regression_Test_Full_App.
  * Klik tombol Execute di toolbar atas.
