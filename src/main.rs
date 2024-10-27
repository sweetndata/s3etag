use std::env;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use md5::Context;
use rayon::prelude::*;
use hex;

fn factor_of_1mb(filesize: u64, num_parts: u32) -> u64 {
    let x = filesize / num_parts as u64;
    let y = x % 1048576;
    x + 1048576 - y
}

fn calc_etag(inputfile: &str, partsize: u64) -> io::Result<String> {
    let file = File::open(inputfile)?;
    let filesize = file.metadata()?.len();
    let num_chunks = (filesize + partsize - 1) / partsize;

    // Create a vector to store the MD5 digests
    let md5_digests: Vec<_> = (0..num_chunks).into_par_iter().map(|i| {
        let mut buffer = vec![0; partsize as usize];
        let mut hasher = Context::new();
        let mut file = File::open(inputfile).unwrap(); // Open a new file handle for each thread
        file.seek(SeekFrom::Start(i * partsize)).unwrap();
        let bytes_read = file.read(&mut buffer).unwrap();
        hasher.consume(&buffer[..bytes_read]);
        hasher.compute().to_vec()
    }).collect();

    // Combine all the MD5 digests into a final hash
    let mut final_hasher = Context::new();
    for digest in &md5_digests {
        final_hasher.consume(digest);
    }
    Ok(format!("{}-{}", hex::encode(final_hasher.compute().to_vec()), md5_digests.len()))
}

fn possible_partsizes(filesize: u64, num_parts: u32) -> impl Fn(&u64) -> bool {
    move |&partsize| partsize < filesize && (filesize as f64 / partsize as f64) <= num_parts as f64
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <inputfile> <etag>", args[0]);
        std::process::exit(1);
    }

    let inputfile = &args[1];
    let etag = &args[2];
    let filesize = std::fs::metadata(inputfile)?.len();
    let num_parts: u32 = etag.split('-').nth(1).unwrap().parse().unwrap();

    let partsizes = vec![
        8388608, // aws_cli/boto3
        15728640, // s3cmd
        factor_of_1mb(filesize, num_parts), // Used by many clients to upload large files
    ];

    partsizes.into_par_iter()
        .filter(possible_partsizes(filesize, num_parts))
        .for_each(|partsize| {
            if etag == &calc_etag(inputfile, partsize).unwrap() {
                println!("Local file matches");
                std::process::exit(0);
            }
        });

    println!("Couldn't validate etag");
    std::process::exit(1);
}
