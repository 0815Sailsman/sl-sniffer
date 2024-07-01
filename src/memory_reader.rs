use std::process::exit;
use proc_mem::{Module, Process, Signature};

pub trait MemoryReader {
    fn read_i32(&self, address: usize) -> Option<i32>;
    fn read_f32(&self, address: usize) -> Option<f32>;
    fn read_usize(&self, address: usize) -> Option<usize>;
    fn scan_for_pattern(&self, pattern: String) -> Option<usize>;
    fn base_address(&self) -> usize;
}

pub struct WindowsMemoryReader {
    process: Process,
    module: Module,
    pub(crate) base_address: usize
}

impl WindowsMemoryReader {
    pub(crate) fn new() -> WindowsMemoryReader{
        let process = Process::with_name("DarkSoulsRemastered.exe").unwrap_or_else(|e| {
            eprintln!("Error opening process: {:?}", e);
            exit(1);
        });
        println!("Found process: {} {}", process.process_id, process.process_name);

        let module = process.module("DarkSoulsRemastered.exe").unwrap_or_else(|e| {
            eprintln!("Error opening module: {:?}", e);
            exit(2);
        });

        let base_address = process.process_base_address;
        return WindowsMemoryReader {
            process,
            module,
            base_address
        }
    }
}

impl MemoryReader for WindowsMemoryReader {
    fn read_i32(&self, address: usize) -> Option<i32> {
        return match self.process.read_mem::<i32>(address) {
            Ok(v) => Some(v),
            Err(e) => None
        };
    }

    fn read_f32(&self, address: usize) -> Option<f32> {
        return match self.process.read_mem::<f32>(address) {
            Ok(v) => Some(v),
            Err(e) => None
        };
    }

    fn read_usize(&self, address: usize) -> Option<usize> {
        return match self.process.read_mem::<usize>(address) {
            Ok(v) => Some(v),
            Err(e) => None
        };
    }

    fn scan_for_pattern(&self, pattern: String) -> Option<usize> {
        let signature = Signature {
            name: String::from("some_pattern_on_windows"),
            pattern,
            offsets: vec![],
            extra: 0,
            relative: true,
            rip_relative: false,
            rip_offset: 0,
        };

        return match self.module.find_signature(&signature) {
            Ok(v) => Some(v),
            Err(e) => None
        }
    }

    fn base_address(&self) -> usize {
        return self.base_address;
    }
}