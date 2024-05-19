use proc_mem::Process;

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

    pub fn read_i32(&self, offset: usize, process: &Process) -> i32 {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);

        let read_i32 = process.read_mem::<i32>(
            self.resolve_offsets(
                offsets_copy,
                &process,
            )).unwrap();
        return read_i32;
    }

    // Start at base, walk offset, read as new base, repeat
    fn resolve_offsets(&self, offsets: Vec<usize>, process: &Process) -> usize {
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

}
