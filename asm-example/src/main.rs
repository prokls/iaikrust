#![feature(asm)]
#![feature(llvm_asm)]

// 2020-09-18 I just discovered that they deprecated asm!()
//   and introduced a new syntax. The old asm!() was renamed to
//   llvm_asm!() and I migrated two examples just now.
//   See https://github.com/rust-lang/rfcs/pull/2873

/// Runs the CPUID instruction to determine whether the
/// RDTSC instruction in supported.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn has_rdtsc_support() -> bool {
    // Step 1: ask for generic information and print it to stdout
    {
        let ebx: u32;
        let ecx: u32;
        let edx: u32;
        let mut manu_id = [0u8; 12];

        // invoke CPUID
        unsafe {
            asm!(
                // “assembly template” following
                "cpuid",
                // “output operands” following
                out("ebx") ebx,
                out("ecx") ecx,
                out("edx") edx,
                // “input operands” following
                //   eax is set to 0
                in("eax") 0,
                // “options”: comma-separated list of strings ∈ {"pure", "nomem", "nostack"}
                options(nomem, nostack)
            );
        }

        // assemble manufacturer ID ASCII string
        for i in 0..4 {
            // yes, order [0, 8, 4] is intentional! See CPUID docu.
            manu_id[i + 0] = (ebx >> 8 * i) as u8;
            manu_id[i + 8] = (ecx >> 8 * i) as u8;
            manu_id[i + 4] = (edx >> 8 * i) as u8;
        }

        // assemble manu_id into ASCII string
        let manufacturer_id = (0..12).map(|i| char::from(manu_id[i])).collect::<String>();

        // print manufacturer ID to stdout
        dbg!(manufacturer_id);
    }

    // Step 2: ask for rdtsc support
    let edx: u32;
    {
        unsafe {
            llvm_asm!(
                "mov $0, %eax\n\
                 cpuid\n"
                : "={edx}"(edx)
                : "0"(0x80000001u64)
                : "eax", "edx"
                : "volatile"
            );
        }
    }

    (edx >> 27) & 1 > 0
}

/// Returns the clflush cache line size in bytes.
/// On Linux, you can call `getconf LEVEL1_DCACHE_LINESIZE`.
/// Here we tried to evaluate it programmatically. But it was
/// unreliable and thus we use the CPUID instruction now.
fn get_clflush_cache_line_size() -> u16 {
    let ebx: u32;

    unsafe {
        llvm_asm!(
            "mov $0, %eax\n\
                cpuid\n"
            : "={ebx}"(ebx)
            : "0"(1)
            : "eax", "ebx"
            : "volatile"
        );
    }

    // bits 8..15 contain clflush's cacheline size in bytes if taken times 8
    ((ebx >> 8) as u16 & 0xF) * 8
}

/// Wrapper for the rdtscp instruction
fn rdtscp() -> (u64, u32) {
    let eax: u32;
    let ecx: u32;
    let edx: u32;
    {
        unsafe {
            asm!(
                "rdtscp",
                out("eax") eax,
                out("ecx") ecx,
                out("edx") edx
            );
        }
    }

    // The EDX register is loaded with the high-order 32 bits of the IA32_TSC MSR;
    // the EAX register is loaded with the low-order 32 bits of the IA32_TSC MSR;
    // and the ECX register is loaded with the low-order 32-bits of IA32_TSC_AUX MSR
    // via https://www.felixcloutier.com/x86/rdtscp
    let counter: u64 = (edx as u64) << 32 | eax as u64;
    (counter, ecx)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("has rdtsc support? {}", has_rdtsc_support());
    println!("your clflush cache line size is {}", get_clflush_cache_line_size());
    let (counter, ecx) = rdtscp();
    println!("rdtsc returned {} and {}", counter, ecx);
    Ok(())
}
