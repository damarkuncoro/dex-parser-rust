# Software Architecture Document (SAD) - DEX Parser Rust

## 1. Introduction
Dokumen ini menjelaskan arsitektur tingkat tinggi dari `dex-parser-rust`, sebuah engine parser file Android DEX (Dalvik Executable) yang fokus pada performa tinggi, modularitas, dan keamanan memori menggunakan bahasa Rust.

## 2. Architectural Goals
- **High Performance**: Memanfaatkan pemrosesan paralel untuk menangani file DEX besar.
- **Memory Safety**: Menjamin tidak ada *memory leak* atau *buffer overflow* saat memproses binary yang tidak terpercaya.
- **Extensibility**: Memungkinkan penambahan fitur (format output baru, resolusi simbol baru) tanpa merombak kode inti.
- **Library-First**: Didesain sebagai library (Crate) yang bisa diintegrasikan ke project lain.

## 3. High-Level System Architecture

Project ini dibagi menjadi tiga layer utama:

### A. Presentation Layer (CLI & Display)
- **`cli.rs`**: Menangani argumen baris perintah menggunakan `clap`.
- **`display/`**: Mengimplementasikan **Strategy Pattern** melalui `DexPrinter` trait untuk menghasilkan output Text atau JSON.

### B. Core Logic Layer (Parsers & Instructions)
- **`parsers/`**: Mengelola pipeline parsing (Header -> Metadata -> Classes).
- **`instructions/`**: Berisi engine disassembler dan tabel opcode Dalvik.
- **`context.rs`**: Bertindak sebagai **DI Container** yang menyimpan state parsing dan menyediakan resolusi simbol.

### C. Data Access Layer (Models & Utils)
- **`models/`**: Definisi struktur data DEX yang strongly-typed dan mendukung serialisasi (Serde).
- **`utils/`**: Fungsi pembantu binari (LEB128, Checksum calculation).

## 4. Key Design Patterns

### 1. Dependency Injection (DI) dengan Traits
Untuk menghindari ketergantungan keras (*hard coupling*), parser instruksi tidak mengakses data secara langsung melainkan melalui `DexResolver` trait.
- **Manfaat**: Memudahkan unit testing dengan Mock Objects dan membuat komponen bersifat reusable.

### 2. Strategy Pattern untuk Output
Antarmuka output didefinisikan melalui trait `DexPrinter`.
- **Manfaat**: Menambahkan format output baru (misal: XML atau HTML) cukup dengan membuat implementasi trait baru.

### 3. Parallel Iteration (Rayon)
Parsing kelas dilakukan secara paralel menggunakan *Work-stealing* algorithm dari library Rayon.
- **Manfaat**: Skalabilitas performa linear terhadap jumlah core CPU.

## 5. Parsing Pipeline (Data Flow)

1. **Initialization**: Membaca file ke buffer memori.
2. **Stage 1 (Header)**: Memvalidasi magic number dan menentukan *endianness*.
3. **Stage 2 (Metadata)**: Parsing tabel string, tipe, prototipe, field, dan method secara sekuensial (karena saling bergantung).
4. **Stage 3 (Parallel Class Processing)**: 
   - Memetakan definisi kelas.
   - Melakukan disassembly bytecode instruksi per method secara paralel.
   - Resolusi simbol (cross-reference) menggunakan `DexContext`.
5. **Output**: Mentransformasikan objek `Dex` menjadi format yang diinginkan melalui `DexPrinter`.

## 6. CI/CD Pipeline
Project ini menggunakan GitHub Actions untuk:
- **CI**: Melakukan pengecekan kualitas kode (linting), formatting, dan unit testing otomatis.
- **CD**: Membangun binary otomatis untuk Linux, macOS, dan Windows setiap kali ada pembuatan tag versi baru.

## 7. Future Considerations
- Implementasi analisis aliran data (Data Flow Analysis).
- Dukungan untuk instruksi ART (Android Runtime) yang dioptimalkan (Vdex/Oat).
- Penambahan fitur deteksi pola obfuscation.
