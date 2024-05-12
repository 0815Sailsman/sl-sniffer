use std::process::exit;
use proc_mem::Process;

fn main() {
    let proc_name = "DarkSoulsRemastered.exe";

    let dark_souls_remastered = Process::with_name(proc_name).unwrap_or_else(|e| {
        eprintln!("Error opening process: {:?}", e);
        exit(1);
    });

    println!("Found process: {} {}", dark_souls_remastered.process_id, dark_souls_remastered.process_name);
    // resolve_pointers(dark_souls_remastered);
}

/*fn resolve_pointers(process: Process) {
    let baseAddress = process.MainModule.BaseAddress.ToInt64();
    let bytes = process.ReadProcessMemory(baseAddress, process.MainModule.ModuleMemorySize).Unwrap();
    let is64Bit = process.Is64Bit().Unwrap();
}*/

/*
        NodeType = NodeType.RelativeScan,
        Name = "GameMan",
        Pattern = "48 8b 05 ? ? ? ? c6 40 18 00",
        AddressOffset = 3,
        InstructionSize = 7,
    };
.AddPointer(//pointer name_gameMan, // offsets 0)
;
 */