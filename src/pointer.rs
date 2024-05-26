use proc_mem::{Process, ProcMemError};

// an actual pointer, that can do all the heavy lifting once it has been constructed properly
//  don't necessarily move the process into here, just implement reading to take the process
pub struct Pointer {
    pub(crate) base_address: usize,
    pub(crate) offsets: Vec<usize>,
}

impl Pointer {
    pub fn read_float(&self, offset: usize, process: &Process) -> f32 {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);

        let read_float = process.read_mem::<f32>(
            self.resolve_offsets(
                offsets_copy,
                &process,
            )).unwrap();
        return read_float;
    }

    pub fn read_i32(&self, offset: usize, process: &Process) -> Result<i32, ProcMemError> {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);

        return process.read_mem::<i32>(
            self.resolve_offsets(
                offsets_copy,
                &process,
            ));
    }

    // Start at base, walk offset, read as new base, repeat
    pub fn resolve_offsets(&self, offsets: Vec<usize>, process: &Process) -> usize {
        let mut ptr = self.base_address;
        for (index, offset) in offsets.iter().enumerate() {
            let address = ptr + offset;
            if index + 1 < offsets.len() {
                ptr = process.read_mem::<usize>(address).unwrap();
                if ptr == 0 {
                    return 0;
                }
            } else {
                ptr = address;
            }
        }
        return ptr;
    }

    // Creates a new pointer with the address of the old pointer as base address
    pub fn create_pointer_from_address(&self, offset: usize, process: &Process) -> Pointer {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);
        offsets_copy.push(0);

        let new_base_address = self.resolve_offsets(offsets_copy, process);
        return Pointer{
            base_address: new_base_address,
            offsets: vec![],
        };
    }
}
