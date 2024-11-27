// Import the sector_reader module
mod sector_reader;

// Import necessary standard library modules
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, Write};

// Import anyhow for error handling
use anyhow::{bail, Context, Result};

// Import NTFS-related structures and traits
use ntfs::indexes::NtfsFileNameIndex;
use ntfs::{Ntfs, NtfsFile, NtfsReadSeek};

// Import the SectorReader from the sector_reader module
use sector_reader::SectorReader;

// Import random distributions for generating file names
use rand::distributions::{Alphanumeric, DistString};

// External crate declarations
extern crate ntfs;
extern crate rand;
extern crate anyhow;

// Struct to hold command information
struct CommandInfo<'n, T>
where
    T: Read + Seek,
{
    current_directory: Vec<NtfsFile<'n>>, // Current directory files
    fs: T,                                // Filesystem reader
    ntfs: &'n Ntfs,                       // NTFS instance
}

fn main() -> Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        bail!("Usage: <program> <volume_path> <target_file> <xor_key>\n\
        Example: rustyninja.exe c:\\windows\\system32\\config\\SYSTEM 0x33");
    }

    // Parse the volume path
    let path = &args[1];
    let mut dirs: Vec<&str> = path.split('\\').collect();

    // Open the volume
    let f = File::open([r"\\.\", dirs[0]].concat()).context("Failed to open volume")?;
    let sr = SectorReader::new(f, 4096).context("Failed to create SectorReader")?;
    let mut fs = BufReader::new(sr);

    // Initialize the NTFS filesystem
    let mut ntfs = Ntfs::new(&mut fs).context("Failed to initialize NTFS")?;
    ntfs.read_upcase_table(&mut fs).context("Failed to read upcase table")?;

    // Get the root directory
    let current_directory = vec![ntfs.root_directory(&mut fs).context("Failed to get root directory")?];
    let mut info = CommandInfo { current_directory, fs, ntfs: &ntfs };

    // Navigate to the target file directory
    let target_file = dirs.pop();
    for dir in dirs.iter().skip(1) {
        cd(dir, &mut info).context(format!("Failed to change directory to {}", dir))?;
    }

    // Get the target file and apply XOR if needed
    let key = &args[2];
    get(target_file.unwrap(), &mut info, key.to_owned()).context("Failed to get target file")?;

    Ok(())
}

// Function to change the current directory
fn cd<T>(arg: &str, info: &mut CommandInfo<T>) -> Result<()>
where
    T: Read + Seek,
{
    // Get the directory index
    let index = info.current_directory.last().unwrap().directory_index(&mut info.fs)?;
    let mut finder = index.finder();

    // Find the directory entry
    let maybe_entry = NtfsFileNameIndex::find(&mut finder, info.ntfs, &mut info.fs, arg);
    let entry = maybe_entry.unwrap()?;
    let _file_name = entry.key().expect("key must exist for a found Index Entry")?;

    // Convert the entry to a file and update the current directory
    let file = entry.to_file(info.ntfs, &mut info.fs)?;
    info.current_directory.push(file);

    Ok(())
}

// Function to get the target file and apply XOR if needed
fn get<T>(arg: &str, info: &mut CommandInfo<T>, key: String) -> Result<()>
where
    T: Read + Seek,
{
    // Generate a random output file name
    let out_file = format!("{}.bin", Alphanumeric.sample_string(&mut rand::thread_rng(), 16));

    // Split the file name and data stream name
    let (file_name, data_stream_name) = match arg.find(':') {
        Some(mid) => (&arg[..mid], &arg[mid + 1..]),
        None => (arg, ""),
    };

    // Open the output file for writing
    let mut output_file = OpenOptions::new().write(true).create_new(true).open(&out_file)
        .with_context(|| format!("Failed to open file for writing"))?;

    // Find the target file
    let file = filearg(file_name, info)?;

    // Get the data item and attribute
    let data_item = match file.data(&mut info.fs, data_stream_name) {
        Some(data_item) => data_item,
        None => {
            println!("Missing $DATA");
            return Ok(());
        }
    };
    let data_item = data_item?;
    let data_attribute = data_item.to_attribute()?;
    let mut data_value = data_attribute.value(&mut info.fs)?;

    // Print the size of the data to be saved
    println!("Xoring and saving {} bytes of data to: {}", data_value.len(), out_file);

    // Buffer for reading data
    let mut buf = [0u8; 4096];

    // Apply XOR and write to the output file
    if let Ok(xorkey) = u8::from_str_radix(key.trim_start_matches("0x"), 16) {
        loop {
            let bytes_read = data_value.read(&mut info.fs, &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            for i in 0..bytes_read {
                buf[i] ^= xorkey;
            }
            output_file.write_all(&buf[..bytes_read])?;
        }
    } else {
        println!("Couldn't xor the data.");
    }

    Ok(())
}

// Function to find a file by name
fn filearg<'n, T>(arg: &str, info: &mut CommandInfo<'n, T>) -> Result<NtfsFile<'n>>
where
    T: Read + Seek,
{
    // Get the directory index
    let index = info.current_directory.last().unwrap().directory_index(&mut info.fs)?;
    let mut finder = index.finder();

    // Find the file entry
    if let Some(entry) = NtfsFileNameIndex::find(&mut finder, info.ntfs, &mut info.fs, arg) {
        let entry = entry?;
        let file = entry.to_file(info.ntfs, &mut info.fs)?;
        Ok(file)
    } else {
        bail!("File not found: \"{}\".", arg)
    }
}
