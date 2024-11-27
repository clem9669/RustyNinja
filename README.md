# RustyNinja - A Stealthy File Exfiltration Tool

> [!WARNING]
> This tool is a work-in-progress and was built as a **learn-Rust** project. The code is rough and not production-ready. Use it at your own risk, and feel free to contribute or suggest improvements!

RustyNinja is a Rust-based tool for **stealthily exfiltrating locked files** from an NTFS volume, inspired by the [NinjaCopy](https://github.com/PowerShellMafia/PowerSploit/blob/master/Exfiltration/Invoke-NinjaCopy.ps1) PowerShell script. 

If you have **Admin privileges**, RustyNinja allows you to access and copy files that are typically locked by the OS (such as registry hives: `SYSTEM`, `SAM`, `SECURITY`, or `NTDS.dit`). The tool works by opening a read handle to the NTFS volume and parsing the filesystem structure directly, bypassing file locks and restrictions.

> [!TIP]
> Copying sensitive files this way may help bypass certain Endpoint Detection and Response (EDR) solutions, making it useful for red team engagements. Traditional methods like `ntdsutil`, `diskshadow`, `reg save`, or RPC can be quite noisy, while RustyNinja attempts to be more discreet.

### Key Features:
- **Bypass File Locks**: Extract locked files from NTFS volumes, even if they're in use by the system.
- **XOR Encryption**: Optionally XOR the file content in-memory before writing it to disk for detection evasion.
- **Randomized Output Filenames**: Each output file is given a randomly generated name for added stealth.
- **Flexible XOR Key**: If you don't need encryption, simply use `0x00` as the XOR byte to copy the file as-is.

## 🛠️ Compiling

To compile RustyNinja for Windows:

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
This will create a release binary targeting Windows.

## 🚀 Usage

RustyNinja requires two arguments:

    1. Volume Path: Path to the NTFS volume containing the file you want to copy.
    2. XOR Key: A hex value used to XOR the file's content.

Example:

```sh
rustyninja.exe [volume path] [XOR hex value]

Example: 
rustyninja.exe c:\windows\system32\config\SYSTEM 0x33

Output: 
Xoring and saving 20185088 bytes of data to: tySXLjFMRndoxTuq.bin
```
    Note: If you want to skip the XOR operation and copy the file directly, pass 0x00 as the XOR key.

## 🔄 XOR Decoder

If you have XORed the file content, use the following Python script to decode it back:

```sh
python de-xor.py tySXLjFMRndoxTuq.bin 0x33
```

This will create a .out file with the decoded content.

## 🧪 Verifying the SYSTEM Hive

To verify the integrity of the extracted SYSTEM hive, you can use the RegRipper tool:

```sh
regripper -r tySXLjFMRndoxTuq.bin.out -f system
```

This will parse the SYSTEM hive and verify its validity.

## 🙌 Acknowledgements

Big thanks and inspiration from Colin Finck who created the NTFS implementation in Rust.
