use rs_meminfo::MemoryInfo;

fn main() {
    let minfo: MemoryInfo = MemoryInfo::new();

    let total_mib: u64 = minfo.total_mib();
    let used_mib: u64 = minfo.used_mib();
    let free_mib: u64 = minfo.free_mib();
    let available_mib: u64 = minfo.available_mib();

    let available_ratio: u64 = (100 * available_mib) / total_mib;

    println!("Total Memory:     {total_mib} MiB");
    println!("Used Memory:      {used_mib} MiB");
    println!("Free Memory:      {free_mib} MiB");
    println!("Available Memory: {available_mib} MiB");
    println!("Available Ratio:  {available_ratio} %");
}
