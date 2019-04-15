mod flags;
mod memory;
mod opcode;
mod registers;

use flags::Flags;
use memory::Memory;
use opcode::Opcode;
use registers::Registers;

// struct Stack {
//     pointer: u16,
//     stack: [u8; 16],
// }

pub struct Cpu {
    registers: Registers,
    memory: Memory,
    // stack: Stack,
    flags: Flags,
}

impl Cpu {
    pub fn new_and_init() -> Cpu {
        Cpu {
            registers: Registers {
                pc: 0,
                register: [0u8; 7],
            },
            memory: Memory { ram: [0; 0xFFFF] },
            // stack: Stack {
            //     pointer: 0,
            //     stack: [0; 16],
            // },
            flags: Flags {
                z: false,
                s: false,
                p: false,
                cy: false,
                ac: false,
            },
        }
    }

    pub fn load_rom_into_memory(&mut self, start_addr: usize, rom: &[u8; 0x7FF]) {
        for b in 0..rom.len() {
            self.memory.ram[start_addr + b] = rom[b];
        }
    }

    pub fn execute_opcode(&mut self) {
        let mut op = Opcode::new();
        op.code = self.memory.ram[self.registers.pc as usize];
        op.set_operation_register();
        op.assc_bytes = (
            self.memory.ram[(self.registers.pc + 2) as usize],
            self.memory.ram[(self.registers.pc + 1) as usize],
        );
        self.registers.pc += 1;
        match op.code {
            0x00 => {
                op.operation_name = String::from("NOP");
            }
            0x01 => {
                op.operation_name = String::from("LXI");
                self.registers.set_value("B", op.assc_bytes.0);
                self.registers.set_value("C", op.assc_bytes.1);
                self.registers.pc += 2;
            }
            0x02 => {
                op.operation_name = String::from("STAX B");
            }
            0x03 => {
                op.operation_name = String::from("INX B");
            }
            0x04 => {
                op.operation_name = String::from("INR B");
            }
            0x05 => {
                op.operation_name = String::from("DCR B");
            }
            0x06 => {
                op.operation_name = String::from("MVI B");
            }
            // 0x07 => debug!("{:x} RLC", self.registers.pc),
            // 0x08 => debug!("{:x} NOP", self.registers.pc),
            // 0x09 => debug!("{:x} DAD   B", self.registers.pc),
            // 0x0A => debug!("{:x} LDAX  B", self.registers.pc),
            // 0x0B => debug!("{:x} DCX   B", self.registers.pc),
            // 0x0C => debug!("{:x} INR   C", self.registers.pc),
            // 0x0D => debug!("{:x} DCR   C", self.registers.pc),
            // 0x0E => {
            //     debug!("{:x} MVI   C, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0x0F => debug!("{:x} RRC", self.registers.pc),
            // 0x10 => debug!("{:x} NOP", self.registers.pc),
            // 0x11 => {
            //     debug!(
            //         "{:x} LXI   D, {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0x12 => debug!("{:x} STAX  D", self.registers.pc),
            // 0x13 => debug!("{:x} INX   D", self.registers.pc),
            // 0x14 => debug!("{:x} INR   D", self.registers.pc),
            // 0x15 => debug!("{:x} DCR   D", self.registers.pc),
            // 0x16 => {
            //     debug!("{:x} MVI   D, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0x17 => debug!("{:x} RAL", self.registers.pc),
            // 0x18 => debug!("{:x} NOP", self.registers.pc),
            // 0x19 => debug!("{:x} DAD   D", self.registers.pc),
            // 0x1A => debug!("{:x} LDAX  D", self.registers.pc),
            // 0x1B => debug!("{:x} DCX   D", self.registers.pc),
            // 0x1C => debug!("{:x} INR   E", self.registers.pc),
            // 0x1D => debug!("{:x} DCR   E", self.registers.pc),
            // 0x1E => {
            //     debug!("{:x} MVI   E, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0x1F => debug!("{:x} RAR", self.registers.pc),
            // 0x20 => debug!("{:x} NOP", self.registers.pc),
            // 0x21 => {
            //     debug!(
            //         "{:x} LXI   H, {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0x22 => {
            //     debug!(
            //         "{:x} SHLD     {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0x23 => debug!("{:x} INX   H", self.registers.pc),
            // 0x24 => debug!("{:x} INR   H", self.registers.pc),
            // 0x25 => debug!("{:x} DCR   H", self.registers.pc),
            // 0x26 => {
            //     debug!("{:x} MVI   H, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0x27 => debug!("{:x} DAA", self.registers.pc),
            // 0x28 => debug!("{:x} NOP", self.registers.pc),
            // 0x29 => debug!("{:x} DAD   H", self.registers.pc),
            // 0x2A => {
            //     debug!(
            //         "{:x} LHLD     {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0x2B => debug!("{:x} DCX   H", self.registers.pc),
            // 0x2C => debug!("{:x} INR   L", self.registers.pc),
            // 0x2D => debug!("{:x} DCR   L", self.registers.pc),
            // 0x2E => {
            //     debug!("{:x} MVI   L, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0x2F => debug!("{:x} CMA", self.registers.pc),
            // 0x30 => debug!("{:x} NOP", self.registers.pc),
            // 0x31 => {
            //     debug!(
            //         "{:x} LXI  SP, {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0x32 => {
            //     debug!(
            //         "{:x} STA   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0x33 => debug!("{:x} INX  SP", self.registers.pc),
            // 0x34 => debug!("{:x} INR   M", self.registers.pc),
            // 0x35 => debug!("{:x} DCR   M", self.registers.pc),
            // 0x36 => {
            //     debug!("{:x} MVI   M, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0x37 => debug!("{:x} STC", self.registers.pc),
            // 0x38 => debug!("{:x} NOP", self.registers.pc),
            // 0x39 => debug!("{:x} DAD  SP", self.registers.pc),
            // 0x3A => {
            //     debug!(
            //         "{:x} LDA      {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0x3B => debug!("{:x} DCX  SP", self.registers.pc),
            // 0x3C => debug!("{:x} INR   A", self.registers.pc),
            // 0x3D => debug!("{:x} DCR   A", self.registers.pc),
            // 0x3E => {
            //     debug!("{:x} MVI   A, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0x3F => debug!("{:x} CMC", self.registers.pc),
            // 0x40 => debug!("{:x} MOV  B,B", self.registers.pc),
            // 0x41 => debug!("{:x} MOV  B,C", self.registers.pc),
            // 0x42 => debug!("{:x} MOV  B,D", self.registers.pc),
            // 0x43 => debug!("{:x} MOV  B,E", self.registers.pc),
            // 0x44 => debug!("{:x} MOV  B,H", self.registers.pc),
            // 0x45 => debug!("{:x} MOV  B,L", self.registers.pc),
            // 0x46 => debug!("{:x} MOV  B,M", self.registers.pc),
            // 0x47 => debug!("{:x} MOV  B,A", self.registers.pc),
            // 0x48 => debug!("{:x} MOV  C,B", self.registers.pc),
            // 0x49 => debug!("{:x} MOV  C,C", self.registers.pc),
            // 0x4A => debug!("{:x} MOV  C,D", self.registers.pc),
            // 0x4B => debug!("{:x} MOV  C,E", self.registers.pc),
            // 0x4C => debug!("{:x} MOV  C,H", self.registers.pc),
            // 0x4D => debug!("{:x} MOV  C,L", self.registers.pc),
            // 0x4E => debug!("{:x} MOV  C,M", self.registers.pc),
            // 0x4F => debug!("{:x} MOV  C,A", self.registers.pc),
            // 0x50 => debug!("{:x} MOV  D,B", self.registers.pc),
            // 0x51 => debug!("{:x} MOV  D,C", self.registers.pc),
            // 0x52 => debug!("{:x} MOV  D,D", self.registers.pc),
            // 0x53 => debug!("{:x} MOV  D,E", self.registers.pc),
            // 0x54 => debug!("{:x} MOV  D,H", self.registers.pc),
            // 0x55 => debug!("{:x} MOV  D,L", self.registers.pc),
            // 0x56 => debug!("{:x} MOV  D,M", self.registers.pc),
            // 0x57 => debug!("{:x} MOV  D,A", self.registers.pc),
            // 0x58 => debug!("{:x} MOV  E,B", self.registers.pc),
            // 0x59 => debug!("{:x} MOV  E,C", self.registers.pc),
            // 0x5A => debug!("{:x} MOV  E,D", self.registers.pc),
            // 0x5B => debug!("{:x} MOV  E,E", self.registers.pc),
            // 0x5C => debug!("{:x} MOV  E,H", self.registers.pc),
            // 0x5D => debug!("{:x} MOV  E,L", self.registers.pc),
            // 0x5E => debug!("{:x} MOV  E,M", self.registers.pc),
            // 0x5F => debug!("{:x} MOV  E,A", self.registers.pc),
            // 0x60 => debug!("{:x} MOV  H,B", self.registers.pc),
            // 0x61 => debug!("{:x} MOV  H,C", self.registers.pc),
            // 0x62 => debug!("{:x} MOV  H,D", self.registers.pc),
            // 0x63 => debug!("{:x} MOV  H,E", self.registers.pc),
            // 0x64 => debug!("{:x} MOV  H,H", self.registers.pc),
            // 0x65 => debug!("{:x} MOV  H,L", self.registers.pc),
            // 0x66 => debug!("{:x} MOV  H,M", self.registers.pc),
            // 0x67 => debug!("{:x} MOV  H,A", self.registers.pc),
            // 0x68 => debug!("{:x} MOV  L,B", self.registers.pc),
            // 0x69 => debug!("{:x} MOV  L,C", self.registers.pc),
            // 0x6A => debug!("{:x} MOV  L,D", self.registers.pc),
            // 0x6B => debug!("{:x} MOV  L,E", self.registers.pc),
            // 0x6C => debug!("{:x} MOV  L,H", self.registers.pc),
            // 0x6D => debug!("{:x} MOV  L,L", self.registers.pc),
            // 0x6E => debug!("{:x} MOV  L,M", self.registers.pc),
            // 0x6F => debug!("{:x} MOV  L,A", self.registers.pc),
            // 0x70 => debug!("{:x} MOV  M,B", self.registers.pc),
            // 0x71 => debug!("{:x} MOV  M,C", self.registers.pc),
            // 0x72 => debug!("{:x} MOV  M,D", self.registers.pc),
            // 0x73 => debug!("{:x} MOV  M,E", self.registers.pc),
            // 0x74 => debug!("{:x} MOV  M,H", self.registers.pc),
            // 0x75 => debug!("{:x} MOV  M,L", self.registers.pc),
            // 0x76 => debug!("{:x} HLT", self.registers.pc),
            // 0x77 => debug!("{:x} MOV M,A", self.registers.pc),
            // 0x78 => debug!("{:x} MOV A,B", self.registers.pc),
            // 0x79 => debug!("{:x} MOV A,C", self.registers.pc),
            // 0x7A => debug!("{:x} MOV A,D", self.registers.pc),
            // 0x7B => debug!("{:x} MOV A,E", self.registers.pc),
            // 0x7C => debug!("{:x} MOV A,H", self.registers.pc),
            // 0x7D => debug!("{:x} MOV A,L", self.registers.pc),
            // 0x7E => debug!("{:x} MOV A,M", self.registers.pc),
            // 0x7F => debug!("{:x} MOV A,A", self.registers.pc),
            0x80...0x87 => {
                op.operation_name = String::from("ADD");
                self.add_operation(op.operation_register);
            }
            0x88...0x8F => {
                op.operation_name = String::from("ADC");
                self.adc_operation(op.operation_register);
            }
            0x90...0x97 => {
                op.operation_name = String::from("SUB");;
                self.sub_operation(op.operation_register);
            }
            0x98...0x9F => {
                op.operation_name = String::from("SBB");
                self.sbb_operation(op.operation_register);
            }
            0xA0...0xA7 => {
                op.operation_name = String::from("ANA");
                self.ana_operation(op.operation_register);
            }
            // 0xA1 => debug!("{:x} ANA   C", self.registers.pc),
            // 0xA2 => debug!("{:x} ANA   D", self.registers.pc),
            // 0xA3 => debug!("{:x} ANA   E", self.registers.pc),
            // 0xA4 => debug!("{:x} ANA   H", self.registers.pc),
            // 0xA5 => debug!("{:x} ANA   L", self.registers.pc),
            // 0xA6 => debug!("{:x} ANA   M", self.registers.pc),
            // 0xA7 => debug!("{:x} ANA   A", self.registers.pc),
            // 0xA8 => debug!("{:x} XRA   B", self.registers.pc),
            // 0xA9 => debug!("{:x} XRA   C", self.registers.pc),
            // 0xAA => debug!("{:x} XRA   D", self.registers.pc),
            // 0xAB => debug!("{:x} XRA   E", self.registers.pc),
            // 0xAC => debug!("{:x} XRA   H", self.registers.pc),
            // 0xAD => debug!("{:x} XRA   L", self.registers.pc),
            // 0xAE => debug!("{:x} XRA   M", self.registers.pc),
            // 0xAF => debug!("{:x} XRA   A", self.registers.pc),
            // 0xB0 => debug!("{:x} ORA   B", self.registers.pc),
            // 0xB1 => debug!("{:x} ORA   C", self.registers.pc),
            // 0xB2 => debug!("{:x} ORA   D", self.registers.pc),
            // 0xB3 => debug!("{:x} ORA   E", self.registers.pc),
            // 0xB4 => debug!("{:x} ORA   H", self.registers.pc),
            // 0xB5 => debug!("{:x} ORA   L", self.registers.pc),
            // 0xB6 => debug!("{:x} ORA   M", self.registers.pc),
            // 0xB7 => debug!("{:x} ORA   A", self.registers.pc),
            // 0xB8 => debug!("{:x} CMP   B", self.registers.pc),
            // 0xB9 => debug!("{:x} CMP   C", self.registers.pc),
            // 0xBA => debug!("{:x} CMP   D", self.registers.pc),
            // 0xBB => debug!("{:x} CMP   E", self.registers.pc),
            // 0xBC => debug!("{:x} CMP   H", self.registers.pc),
            // 0xBD => debug!("{:x} CMP   L", self.registers.pc),
            // 0xBE => debug!("{:x} CMP   M", self.registers.pc),
            // 0xBF => debug!("{:x} CMP   A", self.registers.pc),
            // 0xC0 => debug!("{:x} RNZ", self.registers.pc),
            // 0xC1 => debug!("{:x} POP   B", self.registers.pc),
            // 0xC2 => {
            //     debug!(
            //         "{:x} JNZ   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            0xC3 => {
                op.operation_name = String::from("JMP");
                self.registers.pc = ((op.assc_bytes.0 as u16) << 8 as u16) + op.assc_bytes.1 as u16;
            }
            // 0xC4 => {
            //     debug!(
            //         "{:x} CNZ   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            0xC5 => {
                op.operation_name = String::from("PUSH");
            }
            // 0xC6 => {
            //     debug!("{:x} ADI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xC7 => debug!("{:x} RST", self.registers.pc),
            // 0xC8 => debug!("{:x} RZ", self.registers.pc),
            // 0xC9 => debug!("{:x} RET", self.registers.pc),
            // 0xCA => debug!("{:x} JZ", self.registers.pc),
            // 0xCB => debug!("{:x} NOP", self.registers.pc),
            // 0xCC => {
            //     debug!(
            //         "{:x} CZ    {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xCD => {
            //     debug!(
            //         "{:x} CALL  {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xCE => {
            //     debug!("{:x} ACI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xCF => debug!("{:x} RST   1", self.registers.pc),
            // 0xD0 => debug!("{:x} RNC", self.registers.pc),
            // 0xD1 => debug!("{:x} POP   D", self.registers.pc),
            // 0xD2 => {
            //     debug!(
            //         "{:x} JNC   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xD3 => {
            //     debug!("{:x} OUT  D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xD4 => {
            //     debug!(
            //         "{:x} CNC   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            0xD5 => {
                op.operation_name = String::from("PUSH");
            }
            // 0xD6 => {
            //     debug!("{:x} SUI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xD7 => debug!("{:x} RST   2", self.registers.pc),
            // 0xD8 => debug!("{:x} RC", self.registers.pc),
            // 0xD9 => debug!("{:x} NOP", self.registers.pc),
            // 0xDA => {
            //     debug!(
            //         "{:x} JC    {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xDB => {
            //     debug!("{:x} IN    D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xDC => {
            //     debug!(
            //         "{:x} CC    {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xDD => debug!("{:x} NOP", self.registers.pc),
            // 0xDE => {
            //     debug!("{:x} SBI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xDF => debug!("{:x} RST   3", self.registers.pc),
            // 0xE0 => debug!("{:x} RPO", self.registers.pc),
            // 0xE1 => debug!("{:x} POP   H", self.registers.pc),
            // 0xE2 => {
            //     debug!(
            //         "{:x} JPO   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xE3 => debug!("{:x} XTHL", self.registers.pc),
            // 0xE4 => {
            //     debug!(
            //         "{:x} CPO   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            0xE5 => {
                op.operation_name = String::from("PUSH");
            }
            // 0xE6 => {
            //     debug!("{:x} ANI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xE7 => debug!("{:x} RST   4", self.registers.pc),
            // 0xE8 => debug!("{:x} RPE", self.registers.pc),
            // 0xE9 => debug!("{:x} PCHL", self.registers.pc),
            // 0xEA => {
            //     debug!(
            //         "{:x} JPE   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xEB => debug!("{:x} XCHG", self.registers.pc),
            // 0xEC => {
            //     debug!(
            //         "{:x} CPE   {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xED => debug!("{:x} NOP", self.registers.pc),
            // 0xEE => {
            //     debug!("{:x} XRE   D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xEF => debug!("{:x} RST   5", self.registers.pc),
            // 0xF0 => debug!("{:x} RP", self.registers.pc),
            // 0xF1 => debug!("{:x} POP PSW", self.registers.pc),
            // 0xF2 => {
            //     debug!(
            //         "{:x} JP    {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xF3 => debug!("{:x} DI", self.registers.pc),
            // 0xF4 => {
            //     debug!(
            //         "{:x} CP    {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            0xF5 => {
                op.operation_name = String::from("PUSH PSW");
            }
            // 0xF6 => {
            //     debug!("{:x} ORI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xF7 => debug!("{:x} RST   6", self.registers.pc),
            // 0xF8 => debug!("{:x} RM", self.registers.pc),
            // 0xF9 => debug!("{:x} SPHL", self.registers.pc),
            // 0xFA => {
            //     debug!(
            //         "{:x} JM    {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xFB => debug!("{:x} EI", self.registers.pc),
            // 0xFC => {
            //     debug!(
            //         "{:x} CM    {:x}  {:x}",
            //         self.registers.pc,
            //         self.extra_byte(2),
            //         self.extra_byte(1)
            //     );
            //     self.registers.pc += 2
            // }
            // 0xFD => debug!("{:x} NOP", self.registers.pc),
            // 0xFE => {
            //     debug!("{:x} CPI  D8,  {:x}", self.registers.pc, self.extra_byte(1));
            //     self.registers.pc += 1
            // }
            // 0xFF => debug!("{:x} RST   7", self.registers.pc),
            _ => {
                op.operation_name = String::from("NYI");
            }
        }
    }

    #[inline]
    fn set_flags(&mut self, val: &u8, overflow: bool, aux_vals: (bool, bool)) {
        self.flags.p = self.sets_parity_flag(&val);
        self.flags.z = self.sets_zero_flag(&val);
        self.flags.s = self.sets_sign_flag(&val);
        self.flags.ac = self.sets_aux_carry_flag(aux_vals.0, aux_vals.1, &val);
        self.flags.cy = overflow;
    }

    #[inline]
    fn sets_parity_flag(&mut self, val: &u8) -> bool {
        val % 2 == 0
    }

    #[inline]
    fn sets_zero_flag(&mut self, val: &u8) -> bool {
        *val == 0
    }

    #[inline]
    fn sets_sign_flag(&mut self, val: &u8) -> bool {
        (*val & 0x80) == 0x80
    }

    #[inline]
    fn sets_aux_carry_flag(&mut self, v1_b3_set: bool, v2_b3_set: bool, val: &u8) -> bool {
        if !v1_b3_set && !v2_b3_set {
            return false;
        }
        (*val & 0x8) == 0x0
    }

    #[inline]
    fn get_b3_vals(&mut self, register: &u8) -> (bool, bool) {
        let a_val = self.registers.get_value("A");
        let b3_1 = self.is_b3_set(&a_val);
        let b3_2 = self.is_b3_set(&register);
        (b3_1, b3_2)
    }

    #[inline]
    fn is_b3_set(&mut self, val: &u8) -> bool {
        (*val & 0x8) == 0x8
    }

    fn adc_operation(&mut self, register: String) {
        let flag_set = if self.flags.cy { true } else { false };
        self.add_operation(register);
        if flag_set {
            let result = self.registers.get_value("A").wrapping_add(1);
            self.registers.set_value("A", result);
        }
    }

    fn add_operation(&mut self, register: String) {
        let register_value = self.get_register_value(&register);
        let (result, overflow) = self
            .registers
            .get_value("A")
            .overflowing_add(register_value);
        let (b1, b2) = self.get_b3_vals(&register_value);

        self.set_flags(&result, overflow, (b1, b2));
        self.registers.set_value("A", result);
    }

    fn sbb_operation(&mut self, register: String) {
        let register_value = self.get_register_value(&register);
        if self.flags.cy {
            self.registers.set_value(&register, register_value + 1)
        }
        self.sub_operation(register);
    }

    fn sub_operation(&mut self, register: String) {
        let register_value = self.get_register_value(&register);
        let (result, overflow) = self
            .registers
            .get_value("A")
            .overflowing_sub(register_value);
        let (b1, b2) = self.get_b3_vals(&register_value);

        self.set_flags(&result, overflow, (b1, b2));
        self.registers.set_value("A", result);
    }

    fn ana_operation(&mut self, register: String) {
        let result = self.registers.get_value("A") & self.registers.get_value(&register);
        self.set_flags(&result, false, (false, false));
        self.registers.set_value("A", result);
    }

    fn get_register_value(&mut self, register: &String) -> u8 {
        let mem_ref = self.get_memory_reference();
        if register == "M" {
            return self.memory.ram[mem_ref as usize];
        } else {
            return self.registers.get_value(register);
        }
    }

    fn get_memory_reference(&mut self) -> u16 {
        let h = self.registers.get_value("H") as u16;
        h << 8 | self.registers.get_value("L") as u16
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use rand::prelude::*;

    #[test]
    fn test_new_and_init() {
        let cpu = Cpu::new_and_init();
        let addr1 = get_random_number(0xFFFF) as usize;
        let addr2 = get_random_number(0xFFFF) as usize;
        let addr3 = get_random_number(0xFFFF) as usize;

        assert_eq!(cpu.registers.pc, 0);
        assert_eq!(cpu.memory.ram[addr1], 0);
        assert_eq!(cpu.memory.ram[addr2], 0);
        assert_eq!(cpu.memory.ram[addr3], 0);
    }

    #[test]
    fn test_load_rom_into_memory() {
        let mut cpu = Cpu::new_and_init();
        let mut rom: [u8; 0x7FF] = [0; 0x7FF];
        let addr1 = get_random_number(0x7FF) as usize;
        let addr2 = get_random_number(0x7FF) as usize;
        let val1 = get_random_number(0xFF) as u8;
        let val2 = get_random_number(0xFF) as u8;
        rom[addr1] = val1;
        rom[addr2] = val2;
        let start_addr: usize = 0;
        cpu.load_rom_into_memory(start_addr, &rom);

        assert_eq!(cpu.memory.ram[addr1], val1);
        assert_eq!(cpu.memory.ram[addr2], val2);
    }

    #[test]
    fn test_sets_parity_flag_if_even() {
        let mut cpu = Cpu::new_and_init();
        let result = cpu.sets_parity_flag(&28);

        assert_eq!(result, true);
    }

    #[test]
    fn test_sets_parity_flag_if_odd() {
        let mut cpu = Cpu::new_and_init();
        let result = cpu.sets_parity_flag(&27);

        assert_eq!(result, false);
    }

    #[test]
    fn test_sets_zero_flag_if_zero() {
        let mut cpu = Cpu::new_and_init();
        let result = cpu.sets_zero_flag(&0);

        assert_eq!(result, true);
    }

    #[test]
    fn test_sets_zero_flag_if_non_zero() {
        let mut cpu = Cpu::new_and_init();
        let result = cpu.sets_zero_flag(&190);

        assert_eq!(result, false);
    }

    #[test]
    fn test_sets_sign_flag_if_last_bit_set() {
        let mut cpu = Cpu::new_and_init();
        let result = cpu.sets_sign_flag(&0x85);

        assert_eq!(result, true);
    }

    #[test]
    fn test_sets_sign_flag_if_last_bit_unset() {
        let mut cpu = Cpu::new_and_init();
        let result = cpu.sets_sign_flag(&0x14);

        assert_eq!(result, false);
    }

    #[test]
    fn test_sets_aux_carry_flag_if_carry_out_of_bit_3() {
        let mut cpu = Cpu::new_and_init();
        let v1 = 0x2E;
        let v2 = 0x74;
        let b3_1 = cpu.is_b3_set(&v1);
        let b3_2 = cpu.is_b3_set(&v2);
        let result = v1 + v2;
        let result = cpu.sets_aux_carry_flag(b3_1, b3_2, &result);

        assert_eq!(result, true);
    }

    #[test]
    fn test_sets_aux_carry_flag_if_last_word_bit_unset() {
        let mut cpu = Cpu::new_and_init();
        let v1 = 0x1;
        let v2 = 0x2;
        let b3_1 = cpu.is_b3_set(&v1);
        let b3_2 = cpu.is_b3_set(&v2);
        let result = v1 + v2;
        let result = cpu.sets_aux_carry_flag(b3_1, b3_2, &result);

        assert_eq!(result, false);
    }

    #[test]
    fn test_set_flags() {
        let mut cpu = Cpu::new_and_init();
        let v1: u8 = get_random_number(0xFF) as u8;
        let v2: u8 = get_random_number(0xFF) as u8;
        cpu.registers.set_value("A", v1);
        let (b1, b2) = cpu.get_b3_vals(&v2);
        let (result, overflow) = v1.overflowing_add(v2);
        cpu.set_flags(&result, overflow, (b1, b2));
        let p = if result % 2 == 0 { true } else { false };
        let s = if (result & 0x80) >> 7 == 1 {
            true
        } else {
            false
        };
        let z = if result == 0 { true } else { false };
        let cy = overflow;
        let ac = {
            if !b1 && !b2 {
                false
            } else {
                if &result & 0x8 == 0x8 {
                    false
                } else {
                    true
                }
            }
        };

        test_flag_values(&cpu, p, s, z, cy, ac);
    }

    #[test]
    fn test_add_operation() {
        let mut cpu = Cpu::new_and_init();
        let val = get_random_number(0xFF) as u8;
        cpu.registers.set_value("B", val);
        cpu.add_operation(String::from("B"));

        assert_eq!(cpu.registers.get_value("A"), val);
    }

    #[test]
    fn test_opcode_00_nop() {
        let mut cpu = Cpu::new_and_init();
        cpu.execute_opcode();

        assert_eq!(cpu.registers.pc, 0x1);
    }

    #[test]
    fn test_opcode_01_lxi_bc() {
        let mut cpu = Cpu::new_and_init();
        let val1 = get_random_number(0xFF) as u8;
        let val2 = get_random_number(0xFF) as u8;
        cpu.memory.ram[0x0] = 0x01;
        cpu.memory.ram[0x1] = val1;
        cpu.memory.ram[0x2] = val2;
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("B"), val2);
        assert_eq!(cpu.registers.get_value("C"), val1);
    }

    #[test]
    fn test_opcode_85_add_l() {
        let mut cpu = Cpu::new_and_init();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0x85;
        cpu.registers.set_value("L", val);
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), val);
    }

    #[test]
    fn test_opcode_88_adc_b_with_carry() {
        let mut cpu = Cpu::new_and_init();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0x88;
        cpu.registers.set_value("B", val);
        cpu.flags.cy = true;
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), val + 1);
    }

    #[test]
    fn test_opcode_8b_adc_e_without_carry() {
        let mut cpu = Cpu::new_and_init();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0x8B;
        cpu.registers.set_value("E", val);
        cpu.flags.cy = false;
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), val);
    }

    #[test]
    fn test_opcode_92_sub_d_without_borrow() {
        let mut cpu = Cpu::new_and_init();
        let val = get_random_number(0xFF) as u8;
        let base = 0xFF;
        let pc = get_random_number(0xFFFF);
        let result = base - val;
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0x92;
        cpu.registers.set_value("A", base);
        cpu.registers.set_value("D", val);
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), result);
    }

    #[test]
    fn test_opcode_96_sub_m_with_borrow() {
        let mut cpu = Cpu::new_and_init();
        let val1 = get_random_number(0xFF) as u8;
        let val2 = get_random_number(0xFF) as u8;
        let val = get_random_number(0xFF) as u8;
        let base: u8 = 0x0;
        let pc = get_random_number(0xFFFF);
        let result = base.wrapping_sub(val);
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0x96;
        cpu.memory.ram[((val1 as u16) << 8 | val2 as u16) as usize] = val;
        cpu.registers.set_value("A", base);
        cpu.registers.set_value("H", val1);
        cpu.registers.set_value("L", val2);
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), result);
    }

    #[test]
    fn test_opcode_9d_sbb_l_without_borrow() {
        let mut cpu = Cpu::new_and_init();
        let val = get_random_number(0xFF) as u8;
        let base = 0xFF;
        let pc = get_random_number(0xFFFF);
        let result = base - val;
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0x95;
        cpu.registers.set_value("A", base);
        cpu.registers.set_value("L", val);
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), result);
    }

    #[test]
    fn test_opcode_9f_sbb_a_with_borrow() {
        let mut cpu = Cpu::new_and_init();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0x97;
        cpu.registers.set_value("A", val);
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), 0);
    }

    #[test]
    fn test_opcode_a3_ana_e() {
        let mut cpu = Cpu::new_and_init();
        let reg_val = get_random_number(0xFF) as u8;
        let acc_val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        let result = reg_val & acc_val;
        cpu.registers.pc = pc;
        cpu.memory.ram[pc as usize] = 0xA3;
        cpu.registers.set_value("E", reg_val);
        cpu.registers.set_value("A", acc_val);
        cpu.execute_opcode();

        assert_eq!(cpu.registers.get_value("A"), result);
    }

    fn test_flag_values(cpu: &Cpu, p: bool, s: bool, z: bool, cy: bool, ac: bool) {
        assert_eq!(cpu.flags.p, p);
        assert_eq!(cpu.flags.s, s);
        assert_eq!(cpu.flags.z, z);
        assert_eq!(cpu.flags.cy, cy);
        assert_eq!(cpu.flags.ac, ac);
    }

    fn get_random_number(max: u16) -> u16 {
        let mut rand = rand::thread_rng();
        rand.gen_range(0x0, max)
    }
}
