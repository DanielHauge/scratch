use std::time::Instant;

use hdf5_metno::{File, filters::BloscShuffle};

fn main() {
    // Open or create a file
    let file = File::create("example.h5").unwrap();
    let same_u32_5000_times = vec![42u32; 1000000];
    let length = same_u32_5000_times.len();
    let compression_level = 5;

    // Create a dataset builder
    let now = Instant::now();
    let lzf_dataset = file
        .new_dataset::<u32>()
        .shape((length,))
        .lzf()
        .create("lzf_compressed")
        .unwrap();
    lzf_dataset.write(&same_u32_5000_times).unwrap();
    let lzf_duration = now.elapsed();

    let now = Instant::now();
    let zlib = file
        .new_dataset::<u32>()
        .shape((length,))
        .blosc_zlib(compression_level, BloscShuffle::Byte)
        .create("blosc_zlib")
        .unwrap();
    zlib.write(&same_u32_5000_times).unwrap();
    let zlib_duration = now.elapsed();

    let now = Instant::now();
    let blosc = file
        .new_dataset::<u32>()
        .shape((length,))
        .blosc_lz4(compression_level, BloscShuffle::Byte)
        .create("blosc")
        .unwrap();
    blosc.write(&same_u32_5000_times).unwrap();
    let blosc_duration = now.elapsed();

    let now = Instant::now();
    let zstd = file
        .new_dataset::<u32>()
        .shape((length,))
        .blosc_zstd(compression_level, BloscShuffle::Byte)
        .create("blosc_zstd")
        .unwrap();
    zstd.write(&same_u32_5000_times).unwrap();
    let zstd_duration = now.elapsed();

    let now = Instant::now();
    let snapp = file
        .new_dataset::<u32>()
        .shape((length,))
        .blosc_snappy(compression_level, BloscShuffle::Byte)
        .create("snappy")
        .unwrap();
    snapp.write(&same_u32_5000_times).unwrap();
    let snapp_duration = now.elapsed();

    println!("LZF compression took: {:?}", lzf_duration);
    println!("Zlib compression took: {:?}", zlib_duration);
    println!("Blosc compression took: {:?}", blosc_duration);
    println!("Zstd compression took: {:?}", zstd_duration);
    println!("Snappy compression took: {:?}", snapp_duration);
}
