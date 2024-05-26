use proc_mem::{Module, Process};
use crate::attribute::Attribute;
use crate::bonfire::{Bonfire, BonfireState};
use crate::item::Item;
use crate::pointer::Pointer;
use crate::pointer_node::PointerNode;

pub struct DarkSoulsRemastered {
    pub(crate) process: Process,
    pub(crate) module: Module,
    pub(crate) player_game_data: Option<Pointer>,
    pub(crate) player_position: Option<Pointer>,
    pub(crate) player_ins: Option<Pointer>,
    pub(crate) bonfire_db: Option<Pointer>
}

impl DarkSoulsRemastered {
    pub fn read_player_position(&self) -> Vec<f32> {
        match &self.player_position {
            None => panic!("Position pointer uninitialized"),
            Some(pointer) => {
                return vec![
                    pointer.read_float(0x10, &self.process),
                    pointer.read_float(0x14, &self.process),
                    pointer.read_float(0x18, &self.process),
                ];
            }
        }
    }

    pub fn read_player_health(&self) -> i32 {
        match &self.player_ins {
            None => panic!("player_ins pointer uninitialized"),
            Some(pointer) => {
                return pointer.read_i32(0x3e8, &self.process).expect("Reading player health failed!");
            }
        }
    }

    pub fn read_player_attribute(&self, attribute: Attribute) -> i32 {
        match &self.player_game_data {
            None => panic!("player_game_data pointer uninitialized"),
            Some(pointer) => {
                return pointer.read_i32(0x8 + attribute as usize, &self.process).expect("Reading player attribute failed!");
            }
        }
    }

    // todo contact author of soulmemory for datasheets and more
    // todo refactor and make more idiomatic, this is c# style
    pub fn read_inventory(&self) -> Vec<Item> {
        match &self.player_game_data {
            None => vec![],
            Some(pointer) => {
                //Path: GameDataMan->hostPlayerGameData->equipGameData.equipInventoryData.equipInventoryDataSub
                let equip_inventory_data_sub_offset = 0x3b0;

                let item_list2_len:usize = pointer.read_i32(equip_inventory_data_sub_offset, &self.process).expect("Reading item_list_len failed") as usize; // how many items
                let item_list2_starts_at = pointer.read_i32(equip_inventory_data_sub_offset + 40, &self.process).expect("Reading item_list_starts_at failed"); // where does it start?

                const ITEM_IN_MEMORY_BYTES:usize = 0x1c;
                let mut bytes_buffer: Vec<u8> = vec![0u8;item_list2_len * ITEM_IN_MEMORY_BYTES];
                // Removed adding the module base address from here, since it seems to already be included in the address returned by the previous pointer read
                self.process.read_bytes(item_list2_starts_at as usize, bytes_buffer.as_mut_ptr(), item_list2_len);

                return Item::reconstruct_inventory_from_bytes(bytes_buffer, item_list2_len);
            }
        }
    }

    /// This actually may throw ReadProcMem Errors, when you try to get the state of a bonfire
    /// that you did not yet discover! This means it's state is BonfireState::Unknown. I can't
    /// easily suppress the error though, so just ignore it for now, it works fine like this.
    pub fn get_bonfire_state(&self, bonfire: Bonfire) -> BonfireState {
        return match &self.bonfire_db {
            None => BonfireState::Unknown,
            Some(pointer) => {
                let mut element = pointer.create_pointer_from_address(0x28, &self.process);
                element = element.create_pointer_from_address(0, &self.process);
                let mut net_bonfire_db_item = element.create_pointer_from_address(0x10, &self.process);

                let mut index = 0;
                //For loop purely to have a max amount of iterations
                while index < 100 {
                    let bonfire_id = match net_bonfire_db_item.read_i32(0x8, &self.process) {
                        Ok(id) => {id}
                        Err(error) => {
                            eprintln!("Reading bonfire id failed: {:#?}", error);
                            return BonfireState::Unknown // just return unknown when we fail
                        }
                    };
                    if bonfire_id == (bonfire.clone() as i32) {
                        let bonfire_state = match net_bonfire_db_item.read_i32(0xc, &self.process) {
                            Ok(state) => {state}
                            Err(error) => {
                                eprintln!("Reading bonfire state failed: {:#?}", error);
                                return BonfireState::Unknown // just return unknown when we fail
                            }
                        };
                        return BonfireState::from_value(bonfire_state);
                    }
                    element = element.create_pointer_from_address(0, &self.process);
                    net_bonfire_db_item = element.create_pointer_from_address(0x10, &self.process);
                    index += 1;
                }
                return BonfireState::Unknown;
            }
        }
    }

    pub fn resolve_pointers(&mut self) {
        self.player_game_data = Some(Pointer {
            base_address: PointerNode {
                name: String::from("game_data_man"),
                pattern: String::from("48 8b 05 ? ? ? ? 48 8b 50 10 48 89 54 24 60"),
                address_offset: 3,
                instruction_size: 7,
            }.resolve_to_address_for(&self.process, &self.module),
            offsets: vec![0, 0x10],
        });

        self.player_position = Some(Pointer {
            base_address: PointerNode {
                name: String::from("player_pos"),
                pattern: String::from("48 8b 0d ? ? ? ? 0f 28 f1 48 85 c9 74 ? 48 89 7c"),
                address_offset: 3,
                instruction_size: 7,
            }.resolve_to_address_for(&self.process, &self.module),
            offsets: vec![0, 0x68, 0x68, 0x28],
        });

        self.player_ins = Some(Pointer {
            base_address: PointerNode {
                name: String::from("player_ins"),
                pattern: String::from("48 8b 0d ? ? ? ? 0f 28 f1 48 85 c9 74 ? 48 89 7c"),
                address_offset: 3,
                instruction_size: 7,
            }.resolve_to_address_for(&self.process, &self.module),
            offsets: vec![0, 0x68],
        });

        self.bonfire_db = Some(Pointer{
            base_address: PointerNode {
                name: String::from("bonfire_db"),
                pattern: String::from("48 83 3d ? ? ? ? 00 48 8b f1"),
                address_offset: 3,
                instruction_size: 8,
            }.resolve_to_address_for(&self.process, &self.module),
            offsets: vec![0, 0xb68],
        });
    }
}
