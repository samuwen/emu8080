mod flags;
mod memory;
mod opcode;
mod pointers;
mod registers;
mod stack;

use flags::Flags;
use memory::Memory;
use opcode::Opcode;
use pointers::Pointer;
use registers::Register;
use stack::Stack;

#[derive(Clone, Copy, Default)]
pub struct Cpu {
    a: Register,
    b: Register,
    c: Register,
    d: Register,
    e: Register,
    h: Register,
    l: Register,
    pc: Pointer,
    sp: Pointer,
    memory: Memory,
    stack: Stack,
    flags: Flags,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_rom_into_memory(&mut self, start_addr: usize, rom: &[u8; 0x7FF]) {
        for b in 0..rom.len() {
            self.memory.ram[start_addr + b] = rom[b];
        }
    }

    pub fn execute_opcode(mut self) {
        let code = self.memory.ram[usize::from(self.pc)];
        let op = Opcode::new(code);
        match op.code {
            0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0xCB | 0xD9 | 0xDD | 0xED
            | 0xFD => {}
            0x01 | 0x11 | 0x21 | 0x31 => self.lxi_operation(op.code),
            0x02 | 0x12 => self.stax_operation(op.code),
            0x03 | 0x13 | 0x23 | 0x33 => self.inx_operation(op.code),
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => self.inr_operation(op.code),
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => self.dcr_operation(op.code),
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => self.mvi_operation(op.code),
            0x07 => self.rlc(op.code),
            0x09 | 0x19 | 0x29 | 0x39 => self.dad_operation(op.code),
            0x0F => self.rrc(op.code),
            // // 0x09 => debug!("{:x} DAD   B", self.registers.pc),
            // // 0x0A => debug!("{:x} LDAX  B", self.registers.pc),
            // // 0x0B => debug!("{:x} DCX   B", self.registers.pc),
            // // 0x12 => debug!("{:x} STAX  D", self.registers.pc),
            // // 0x17 => debug!("{:x} RAL", self.registers.pc),
            // // 0x19 => debug!("{:x} DAD   D", self.registers.pc),
            // // 0x1A => debug!("{:x} LDAX  D", self.registers.pc),
            // // 0x1B => debug!("{:x} DCX   D", self.registers.pc),
            // // 0x1F => debug!("{:x} RAR", self.registers.pc),
            // // 0x22 => {
            // //     debug!(
            // //         "{:x} SHLD     {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0x27 => debug!("{:x} DAA", self.registers.pc),
            // // 0x29 => debug!("{:x} DAD   H", self.registers.pc),
            // // 0x2A => {
            // //     debug!(
            // //         "{:x} LHLD     {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0x2B => debug!("{:x} DCX   H", self.registers.pc),
            // // 0x2F => debug!("{:x} CMA", self.registers.pc),
            // // 0x32 => {
            // //     debug!(
            // //         "{:x} STA   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0x37 => debug!("{:x} STC", self.registers.pc),
            // // 0x39 => debug!("{:x} DAD  SP", self.registers.pc),
            // // 0x3A => {
            // //     debug!(
            // //         "{:x} LDA      {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0x3B => debug!("{:x} DCX  SP", self.registers.pc),
            // // 0x3F => debug!("{:x} CMC", self.registers.pc),
            // // 0x40 => debug!("{:x} MOV  B,B", self.registers.pc),
            // // 0x41 => debug!("{:x} MOV  B,C", self.registers.pc),
            // // 0x42 => debug!("{:x} MOV  B,D", self.registers.pc),
            // // 0x43 => debug!("{:x} MOV  B,E", self.registers.pc),
            // // 0x44 => debug!("{:x} MOV  B,H", self.registers.pc),
            // // 0x45 => debug!("{:x} MOV  B,L", self.registers.pc),
            // // 0x46 => debug!("{:x} MOV  B,M", self.registers.pc),
            // // 0x47 => debug!("{:x} MOV  B,A", self.registers.pc),
            // // 0x48 => debug!("{:x} MOV  C,B", self.registers.pc),
            // // 0x49 => debug!("{:x} MOV  C,C", self.registers.pc),
            // // 0x4A => debug!("{:x} MOV  C,D", self.registers.pc),
            // // 0x4B => debug!("{:x} MOV  C,E", self.registers.pc),
            // // 0x4C => debug!("{:x} MOV  C,H", self.registers.pc),
            // // 0x4D => debug!("{:x} MOV  C,L", self.registers.pc),
            // // 0x4E => debug!("{:x} MOV  C,M", self.registers.pc),
            // // 0x4F => debug!("{:x} MOV  C,A", self.registers.pc),
            // // 0x50 => debug!("{:x} MOV  D,B", self.registers.pc),
            // // 0x51 => debug!("{:x} MOV  D,C", self.registers.pc),
            // // 0x52 => debug!("{:x} MOV  D,D", self.registers.pc),
            // // 0x53 => debug!("{:x} MOV  D,E", self.registers.pc),
            // // 0x54 => debug!("{:x} MOV  D,H", self.registers.pc),
            // // 0x55 => debug!("{:x} MOV  D,L", self.registers.pc),
            // // 0x56 => debug!("{:x} MOV  D,M", self.registers.pc),
            // // 0x57 => debug!("{:x} MOV  D,A", self.registers.pc),
            // // 0x58 => debug!("{:x} MOV  E,B", self.registers.pc),
            // // 0x59 => debug!("{:x} MOV  E,C", self.registers.pc),
            // // 0x5A => debug!("{:x} MOV  E,D", self.registers.pc),
            // // 0x5B => debug!("{:x} MOV  E,E", self.registers.pc),
            // // 0x5C => debug!("{:x} MOV  E,H", self.registers.pc),
            // // 0x5D => debug!("{:x} MOV  E,L", self.registers.pc),
            // // 0x5E => debug!("{:x} MOV  E,M", self.registers.pc),
            // // 0x5F => debug!("{:x} MOV  E,A", self.registers.pc),
            // // 0x60 => debug!("{:x} MOV  H,B", self.registers.pc),
            // // 0x61 => debug!("{:x} MOV  H,C", self.registers.pc),
            // // 0x62 => debug!("{:x} MOV  H,D", self.registers.pc),
            // // 0x63 => debug!("{:x} MOV  H,E", self.registers.pc),
            // // 0x64 => debug!("{:x} MOV  H,H", self.registers.pc),
            // // 0x65 => debug!("{:x} MOV  H,L", self.registers.pc),
            // // 0x66 => debug!("{:x} MOV  H,M", self.registers.pc),
            // // 0x67 => debug!("{:x} MOV  H,A", self.registers.pc),
            // // 0x68 => debug!("{:x} MOV  L,B", self.registers.pc),
            // // 0x69 => debug!("{:x} MOV  L,C", self.registers.pc),
            // // 0x6A => debug!("{:x} MOV  L,D", self.registers.pc),
            // // 0x6B => debug!("{:x} MOV  L,E", self.registers.pc),
            // // 0x6C => debug!("{:x} MOV  L,H", self.registers.pc),
            // // 0x6D => debug!("{:x} MOV  L,L", self.registers.pc),
            // // 0x6E => debug!("{:x} MOV  L,M", self.registers.pc),
            // // 0x6F => debug!("{:x} MOV  L,A", self.registers.pc),
            // // 0x70 => debug!("{:x} MOV  M,B", self.registers.pc),
            // // 0x71 => debug!("{:x} MOV  M,C", self.registers.pc),
            // // 0x72 => debug!("{:x} MOV  M,D", self.registers.pc),
            // // 0x73 => debug!("{:x} MOV  M,E", self.registers.pc),
            // // 0x74 => debug!("{:x} MOV  M,H", self.registers.pc),
            // // 0x75 => debug!("{:x} MOV  M,L", self.registers.pc),
            // // 0x76 => debug!("{:x} HLT", self.registers.pc),
            // // 0x77 => debug!("{:x} MOV M,A", self.registers.pc),
            // // 0x78 => debug!("{:x} MOV A,B", self.registers.pc),
            // // 0x79 => debug!("{:x} MOV A,C", self.registers.pc),
            // // 0x7A => debug!("{:x} MOV A,D", self.registers.pc),
            // // 0x7B => debug!("{:x} MOV A,E", self.registers.pc),
            // // 0x7C => debug!("{:x} MOV A,H", self.registers.pc),
            // // 0x7D => debug!("{:x} MOV A,L", self.registers.pc),
            // // 0x7E => debug!("{:x} MOV A,M", self.registers.pc),
            // // 0x7F => debug!("{:x} MOV A,A", self.registers.pc),
            0x80...0x87 => self.add_operation(op.code),
            0x88...0x8F => self.adc_operation(op.code),
            0x90...0x97 => self.sub_operation(op.code),
            0x98...0x9F => self.sbb_operation(op.code),
            0xA0...0xA7 => self.ana_operation(op.code),
            0xA8...0xAF => self.xra_operation(op.code),
            0xB0...0xB7 => self.ora_operation(op.code),
            0xB8...0xBF => self.cmp_operation(op.code),
            // // 0xC0 => debug!("{:x} RNZ", self.registers.pc),
            // // 0xC1 => debug!("{:x} POP   B", self.registers.pc),
            // // 0xC2 => {
            // //     debug!(
            // //         "{:x} JNZ   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // 0xC3 => {
            //     op.operation_name = String::from("JMP");
            //     self.registers.pc = ((op.assc_bytes.0 as u16) << 8 as u16) + op.assc_bytes.1 as u16;
            // }
            // // 0xC4 => {
            // //     debug!(
            // //         "{:x} CNZ   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // 0xC5 => {
            //     op.operation_name = String::from("PUSH");
            // }
            // // 0xC6 => {
            // //     debug!("{:x} ADI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xC7 => debug!("{:x} RST", self.registers.pc),
            // // 0xC8 => debug!("{:x} RZ", self.registers.pc),
            // // 0xC9 => debug!("{:x} RET", self.registers.pc),
            // // 0xCA => debug!("{:x} JZ", self.registers.pc),
            // // 0xCC => {
            // //     debug!(
            // //         "{:x} CZ    {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xCD => {
            // //     debug!(
            // //         "{:x} CALL  {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xCE => {
            // //     debug!("{:x} ACI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xCF => debug!("{:x} RST   1", self.registers.pc),
            // // 0xD0 => debug!("{:x} RNC", self.registers.pc),
            // // 0xD1 => debug!("{:x} POP   D", self.registers.pc),
            // // 0xD2 => {
            // //     debug!(
            // //         "{:x} JNC   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xD3 => {
            // //     debug!("{:x} OUT  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xD4 => {
            // //     debug!(
            // //         "{:x} CNC   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // 0xD5 => {
            //     op.operation_name = String::from("PUSH");
            // }
            // // 0xD6 => {
            // //     debug!("{:x} SUI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xD7 => debug!("{:x} RST   2", self.registers.pc),
            // // 0xD8 => debug!("{:x} RC", self.registers.pc),
            // // 0xDA => {
            // //     debug!(
            // //         "{:x} JC    {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xDB => {
            // //     debug!("{:x} IN    D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xDC => {
            // //     debug!(
            // //         "{:x} CC    {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xDE => {
            // //     debug!("{:x} SBI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xDF => debug!("{:x} RST   3", self.registers.pc),
            // // 0xE0 => debug!("{:x} RPO", self.registers.pc),
            // // 0xE1 => debug!("{:x} POP   H", self.registers.pc),
            // // 0xE2 => {
            // //     debug!(
            // //         "{:x} JPO   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xE3 => debug!("{:x} XTHL", self.registers.pc),
            // // 0xE4 => {
            // //     debug!(
            // //         "{:x} CPO   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // 0xE5 => {
            //     op.operation_name = String::from("PUSH");
            // }
            // // 0xE6 => {
            // //     debug!("{:x} ANI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xE7 => debug!("{:x} RST   4", self.registers.pc),
            // // 0xE8 => debug!("{:x} RPE", self.registers.pc),
            // // 0xE9 => debug!("{:x} PCHL", self.registers.pc),
            // // 0xEA => {
            // //     debug!(
            // //         "{:x} JPE   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xEB => debug!("{:x} XCHG", self.registers.pc),
            // // 0xEC => {
            // //     debug!(
            // //         "{:x} CPE   {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xEE => {
            // //     debug!("{:x} XRE   D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xEF => debug!("{:x} RST   5", self.registers.pc),
            // // 0xF0 => debug!("{:x} RP", self.registers.pc),
            // // 0xF1 => debug!("{:x} POP PSW", self.registers.pc),
            // // 0xF2 => {
            // //     debug!(
            // //         "{:x} JP    {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xF3 => debug!("{:x} DI", self.registers.pc),
            // // 0xF4 => {
            // //     debug!(
            // //         "{:x} CP    {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // 0xF5 => {
            //     op.operation_name = String::from("PUSH PSW");
            // }
            // // 0xF6 => {
            // //     debug!("{:x} ORI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xF7 => debug!("{:x} RST   6", self.registers.pc),
            // // 0xF8 => debug!("{:x} RM", self.registers.pc),
            // // 0xF9 => debug!("{:x} SPHL", self.registers.pc),
            // // 0xFA => {
            // //     debug!(
            // //         "{:x} JM    {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xFB => debug!("{:x} EI", self.registers.pc),
            // // 0xFC => {
            // //     debug!(
            // //         "{:x} CM    {:x}  {:x}",
            // //         self.registers.pc,
            // //         self.extra_byte(2),
            // //         self.extra_byte(1)
            // //     );
            // //     self.registers.pc += 2
            // // }
            // // 0xFE => {
            // //     debug!("{:x} CPI  D8,  {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
            // // 0xFF => debug!("{:x} RST   7", self.registers.pc),
            _ => panic!("Invalid opcode"),
        }
        self.pc += 1;
    }

    #[inline]
    fn set_flags(&mut self, result: &u8, reg_value: u8, overflow: bool) {
        let aux_vals: (bool, bool) = self.get_b3_vals(&reg_value);
        self.flags.p = self.sets_parity_flag(&result);
        self.flags.z = self.sets_zero_flag(&result);
        self.flags.s = self.sets_sign_flag(&result);
        self.flags.ac = self.sets_aux_carry_flag(aux_vals.0, aux_vals.1, &result);
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
    fn get_b3_vals(mut self, register: &u8) -> (bool, bool) {
        let a_val = self.a.into();
        let b3_1 = self.is_b3_set(&a_val);
        let b3_2 = self.is_b3_set(&register);
        (b3_1, b3_2)
    }

    #[inline]
    fn is_b3_set(&mut self, val: &u8) -> bool {
        (*val & 0x8) == 0x8
    }

    fn adc_operation(&mut self, code: u8) {
        let carry_value = if self.flags.cy { 1 } else { 0 };
        self.a = self.update_register_with_overflow(
            self.get_register(code),
            self.get_register_value(code) + carry_value,
            false,
            &overflowing_add_u8,
        );
    }

    fn add_operation(&mut self, code: u8) {
        self.a = self.update_register_with_overflow(
            self.get_register(code),
            self.get_register_value(code),
            false,
            &overflowing_add_u8,
        );
    }

    fn sbb_operation(&mut self, code: u8) {
        let carry_value = if self.flags.cy { 1 } else { 0 };
        self.a = self.update_register_with_overflow(
            self.get_register(code),
            self.get_register_value(code) + carry_value,
            true,
            &overflowing_sub_u8,
        );
    }

    fn sub_operation(&mut self, code: u8) {
        self.a = self.update_register_with_overflow(
            self.get_register(code),
            self.get_register_value(code),
            false,
            &overflowing_sub_u8,
        );
    }

    fn ana_operation(&mut self, code: u8) {
        self.a = self.update_register_no_overflow(
            self.get_register(code),
            self.get_register_value(code),
            &logical_and,
        );
    }

    fn xra_operation(&mut self, code: u8) {
        self.a = self.update_register_no_overflow(
            self.get_register(code),
            self.get_register_value(code),
            &logical_xor,
        );
    }

    fn ora_operation(&mut self, code: u8) {
        self.a = self.update_register_no_overflow(
            self.get_register(code),
            self.get_register_value(code),
            &logical_or,
        );
    }

    fn cmp_operation(&mut self, code: u8) {
        let reg_value = self.get_register_value(code);
        let acc_value: u8 = self.a.into();
        let (result, overflow) = acc_value.overflowing_sub(reg_value);
        self.set_flags(&result, reg_value, !overflow);
    }

    fn lxi_operation(&mut self, code: u8) {
        let val1 = self.memory.ram[usize::from(self.pc + 1)].into();
        let val2 = self.memory.ram[usize::from(self.pc + 2)].into();
        match code {
            0x01 => {
                self.c = val1;
                self.b = val2;
            }
            0x11 => {
                self.e = val1;
                self.d = val2;
            }
            0x21 => {
                self.l = val1;
                self.h = val2;
            }
            0x31 => {
                let v: u16 = u16::from(val2) | (u16::from(val1) >> 8);
                self.sp = v.into();
            }
            _ => panic!("Bug exists in opcode routing operation."),
        }
        self.pc += 2;
    }

    fn stax_operation(&mut self, code: u8) {
        let mem_ref = match code {
            0x02 => self.get_reg_pair_value(self.b, self.c),
            0x12 => self.get_reg_pair_value(self.d, self.e),
            _ => panic!("Bug exists in opcode routing operation."),
        };
        self.memory.ram[mem_ref as usize] = self.a.into();
    }

    fn inx_operation(&mut self, code: u8) {
        match code {
            0x03 => {
                let (x, y) = self.update_register_pair(self.b, self.c, 1, &wrapping_add);
                self.b = x;
                self.c = y;
            }
            0x13 => {
                let (x, y) = self.update_register_pair(self.d, self.e, 1, &wrapping_add);
                self.d = x;
                self.e = y;
            }
            0x23 => {
                let (x, y) = self.update_register_pair(self.h, self.l, 1, &wrapping_add);
                self.h = x;
                self.l = y;
            }
            0x33 => {
                let val: u16 = self.sp.into();
                let result = val.wrapping_add(1);
                self.sp = Pointer::from(result);
            }
            _ => panic!("Bug exists in opcode routing"),
        }
    }

    fn inr_operation(&mut self, code: u8) {
        match code {
            0x04 => {
                self.b = self.update_register_with_overflow(self.b, 1, true, &overflowing_add_u8)
            }
            0x0C => {
                self.c = self.update_register_with_overflow(self.c, 1, true, &overflowing_add_u8)
            }
            0x14 => {
                self.d = self.update_register_with_overflow(self.d, 1, true, &overflowing_add_u8)
            }
            0x1C => {
                self.e = self.update_register_with_overflow(self.e, 1, true, &overflowing_add_u8)
            }
            0x24 => {
                self.h = self.update_register_with_overflow(self.h, 1, true, &overflowing_add_u8)
            }
            0x2C => {
                self.l = self.update_register_with_overflow(self.l, 1, true, &overflowing_add_u8)
            }
            0x34 => {
                let mem_ref = self.get_reg_pair_value(self.l, self.h);
                let value = self.memory.ram[mem_ref as usize];
                let (result, overflow) = value.overflowing_add(1);
                self.set_flags(&result, value, overflow);
                self.l = (result & 0xFF).into();
                self.h = (result & 0x00FF).into();
            }
            0x3C => {
                self.a = self.update_register_with_overflow(self.a, 1, true, &overflowing_add_u8)
            }
            _ => panic!("Bug exists in opcode routing"),
        }
    }

    fn dcr_operation(&mut self, code: u8) {
        match code {
            0x05 => {
                self.b = self.update_register_with_overflow(self.b, 1, true, &overflowing_sub_u8)
            }
            0x0D => {
                self.c = self.update_register_with_overflow(self.c, 1, true, &overflowing_sub_u8)
            }
            0x15 => {
                self.d = self.update_register_with_overflow(self.d, 1, true, &overflowing_sub_u8)
            }
            0x1D => {
                self.e = self.update_register_with_overflow(self.e, 1, true, &overflowing_sub_u8)
            }
            0x25 => {
                self.h = self.update_register_with_overflow(self.h, 1, true, &overflowing_sub_u8)
            }
            0x2D => {
                self.l = self.update_register_with_overflow(self.l, 1, true, &overflowing_sub_u8)
            }
            0x35 => {
                let mem_ref = self.get_reg_pair_value(self.l, self.h);
                let value = self.memory.ram[mem_ref as usize];
                let (result, overflow) = value.overflowing_sub(1);
                self.set_flags(&result, value, !overflow);
                self.l = (result & 0xFF).into();
                self.h = (result & 0x00FF).into();
            }
            0x3D => {
                self.a = self.update_register_with_overflow(self.a, 1, true, &overflowing_sub_u8)
            }
            _ => panic!("Bug exists in opcode routing"),
        }
    }

    fn mvi_operation(&mut self, code: u8) {
        match code {
            0x05 => self.b = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x0E => self.c = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x15 => self.d = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x1E => self.e = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x25 => self.h = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x2E => self.l = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x35 => {
                let mem_ref = self.get_reg_pair_value(self.h, self.l);
                self.memory.ram[mem_ref as usize] = self.memory.ram[(usize::from(self.pc) + 1)];
            }
            0x3E => self.a = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            _ => panic!("Bug exists in opcode routing"),
        }
    }

    fn rlc(&mut self) {
        let reg_value: u8 = self.a.into();
        let carry_flag = ((&reg_value & 0x80) >> 7) == 1;
        let temp: u8 = &reg_value << 1;
        if carry_flag {
            self.flags.cy = true;
            self.a = (temp | 0x1).into();
        } else {
            self.flags.cy = false;
            self.a = (temp | 0).into();
        }
    }

    fn rrc(&mut self) {
        let reg_value: u8 = self.a.into();
        let carry_flag = (&reg_value & 0x1) == 1;
        let temp: u8 = &reg_value >> 1;
        if carry_flag {
            self.flags.cy = true;
            self.a = (temp | 0x80).into();
        } else {
            self.flags.cy = false;
            self.a = (temp | 0).into();
        }
    }

    fn dad_operation(&mut self, code: u8) {
        match code {
            0x09 => {
                let hl_reg = self.get_reg_pair_value(self.h, self.l);
                let (x, y) = self.update_register_pair_overflow(
                    self.b,
                    self.c,
                    hl_reg,
                    &overflowing_add_u16,
                );
                self.h = x;
                self.l = y;
            }
            0x19 => {
                let hl_reg = self.get_reg_pair_value(self.h, self.l);
                let (x, y) = self.update_register_pair_overflow(
                    self.d,
                    self.e,
                    hl_reg,
                    &overflowing_add_u16,
                );
                self.h = x;
                self.l = y;
            }
            0x29 => {
                // This is just doubled. :)
                let hl_reg: u32 = self.get_reg_pair_value(self.h, self.l) as u32;
                let result = hl_reg << 1;
                let (x, y) = Cpu::return_split_registers((&result & 0xFFFF) as u16);
                self.h = x;
                self.l = y;
                let overflow = (&result & 0xFF0000) >> 16;
                self.flags.cy = if overflow == 0 { true } else { false };
            }
            0x39 => {
                let hl_reg = self.get_reg_pair_value(self.h, self.l);
                let (x, y) = self.update_register_pair_overflow(
                    (self.sp & 0xFF).into(),
                    (self.sp & 0x00FF).into(),
                    hl_reg,
                    &overflowing_add_u16,
                );
                self.h = x;
                self.l = y;
            }
            _ => panic!("Bug in opcode router"),
        }
    }

    fn update_register_with_overflow(
        &mut self,
        reg: Register,
        op: u8,
        invert: bool,
        f: &Fn(u8, u8) -> (u8, bool),
    ) -> Register {
        let val: u8 = reg.into();
        let (result, overflow) = f(val, op);
        let over = if invert { !overflow } else { overflow };
        self.set_flags(&result, reg.into(), over);
        result.into()
    }

    fn update_register_no_overflow(
        &mut self,
        reg: Register,
        op: u8,
        f: &Fn(u8, u8) -> u8,
    ) -> Register {
        let val: u8 = reg.into();
        let result = f(val, op);
        self.set_flags(&result, reg.into(), false);
        result.into()
    }

    fn update_register_pair(
        &mut self,
        msb: Register,
        lsb: Register,
        op: u16,
        f: &Fn(u16, u16) -> u16,
    ) -> (Register, Register) {
        let concat_val = self.get_reg_pair_value(msb, lsb);
        let result = f(concat_val, op);
        Cpu::return_split_registers(result)
    }

    fn update_register_pair_overflow(
        &mut self,
        msb: Register,
        lsb: Register,
        op: u16,
        f: &Fn(u16, u16) -> (u16, bool),
    ) -> (Register, Register) {
        let reg_val = self.get_reg_pair_value(self.b, self.c);
        let (result, overflow) = f(op, reg_val);
        self.flags.cy = if overflow { true } else { false };
        Cpu::return_split_registers(result)
    }

    fn return_split_registers(val: u16) -> (Register, Register) {
        ((val & 0xFF).into(), (val & 0x00FF).into())
    }

    fn get_register_value(self, code: u8) -> u8 {
        self.get_register(code).into()
    }

    fn get_register(self, code: u8) -> Register {
        match code % 8 {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => {
                let mem_ref = self.get_reg_pair_value(self.h, self.l);
                Register::from(self.memory.ram[mem_ref as usize])
            }
            7 => self.a,
            _ => panic!("Input not valid"),
        }
    }

    fn get_reg_pair_value(self, msb: Register, lsb: Register) -> u16 {
        (u16::from(msb) << 8) | u16::from(lsb)
    }
}

fn overflowing_add_u8(val: u8, operand: u8) -> (u8, bool) {
    val.overflowing_add(operand)
}

fn overflowing_add_u16(val: u16, operand: u16) -> (u16, bool) {
    val.overflowing_add(operand)
}

fn wrapping_add(val: u16, operand: u16) -> u16 {
    val.wrapping_add(operand)
}

// Carries are inverted in subtraction
fn overflowing_sub_u8(val: u8, operand: u8) -> (u8, bool) {
    let (result, overflow) = val.overflowing_sub(operand);
    let overflow = if overflow { false } else { true };
    (result, overflow)
}

fn overflowing_sub_u16(val: u16, operand: u16) -> (u16, bool) {
    val.overflowing_sub(operand)
}

fn logical_and(val: u8, operand: u8) -> u8 {
    val & operand
}

fn logical_or(val: u8, operand: u8) -> u8 {
    val | operand
}

fn logical_xor(val: u8, operand: u8) -> u8 {
    val ^ operand
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_new_and_init() {
        let cpu = Cpu::new();
        let addr1 = get_random_number(0xFFFF) as usize;
        let addr2 = get_random_number(0xFFFF) as usize;
        let addr3 = get_random_number(0xFFFF) as usize;

        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.memory.ram[addr1], 0);
        assert_eq!(cpu.memory.ram[addr2], 0);
        assert_eq!(cpu.memory.ram[addr3], 0);
    }

    #[test]
    fn test_load_rom_into_memory() {
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
        let result = cpu.sets_parity_flag(&28);

        assert_eq!(result, true);
    }

    #[test]
    fn test_sets_parity_flag_if_odd() {
        let mut cpu = Cpu::new();
        let result = cpu.sets_parity_flag(&27);

        assert_eq!(result, false);
    }

    #[test]
    fn test_sets_zero_flag_if_zero() {
        let mut cpu = Cpu::new();
        let result = cpu.sets_zero_flag(&0);

        assert_eq!(result, true);
    }

    #[test]
    fn test_sets_zero_flag_if_non_zero() {
        let mut cpu = Cpu::new();
        let result = cpu.sets_zero_flag(&190);

        assert_eq!(result, false);
    }

    #[test]
    fn test_sets_sign_flag_if_last_bit_set() {
        let mut cpu = Cpu::new();
        let result = cpu.sets_sign_flag(&0x85);

        assert_eq!(result, true);
    }

    #[test]
    fn test_sets_sign_flag_if_last_bit_unset() {
        let mut cpu = Cpu::new();
        let result = cpu.sets_sign_flag(&0x14);

        assert_eq!(result, false);
    }

    #[test]
    fn test_sets_aux_carry_flag_if_carry_out_of_bit_3() {
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
        let v1: u8 = get_random_number(0xFF) as u8;
        let v2: u8 = get_random_number(0xFF) as u8;
        cpu.a = v1.into();
        let (b1, b2) = cpu.get_b3_vals(&v2);
        let (result, overflow) = v1.overflowing_add(v2);
        cpu.set_flags(&result, v2, overflow);
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
        let mut cpu = Cpu::new();
        let val = get_random_number(0xFF) as u8;
        cpu.b = val.into();
        cpu.add_operation(0x80);

        assert_eq!(cpu.a, val);
    }

    #[test]
    fn test_opcode_00_nop() {
        let cpu = Cpu::new();
        cpu.execute_opcode();

        assert_eq!(cpu.pc, 0x1);
    }

    #[test]
    fn test_opcode_01_lxi_bc() {
        let mut cpu = Cpu::new();
        let val1 = get_random_number(0xFF) as u8;
        let val2 = get_random_number(0xFF) as u8;
        cpu.memory.ram[0x0] = 0x01;
        cpu.memory.ram[0x1] = val1;
        cpu.memory.ram[0x2] = val2;
        cpu.execute_opcode();

        assert_eq!(cpu.b, val2);
        assert_eq!(cpu.c, val1);
    }

    #[test]
    fn test_opcode_85_add_l() {
        let mut cpu = Cpu::new();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0x85;
        cpu.l = val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, val);
    }

    #[test]
    fn test_opcode_88_adc_b_with_carry() {
        let mut cpu = Cpu::new();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0x88;
        cpu.b = val.into();
        cpu.flags.cy = true;
        cpu.execute_opcode();

        assert_eq!(cpu.a, val + 1);
    }

    #[test]
    fn test_opcode_8b_adc_e_without_carry() {
        let mut cpu = Cpu::new();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0x8B;
        cpu.e = val.into();
        cpu.flags.cy = false;
        cpu.execute_opcode();

        assert_eq!(cpu.a, val);
    }

    #[test]
    fn test_opcode_92_sub_d_without_borrow() {
        let mut cpu = Cpu::new();
        let val = get_random_number(0xFF) as u8;
        let base = 0xFF;
        let pc = get_random_number(0xFFFF);
        let result = base - val;
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0x92;
        cpu.a = base.into();
        cpu.d = val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, result);
    }

    #[test]
    fn test_opcode_96_sub_m_with_borrow() {
        let mut cpu = Cpu::new();
        let val1 = get_random_number(0xFF) as u8;
        let val2 = get_random_number(0xFF) as u8;
        let val = get_random_number(0xFF) as u8;
        let base: u8 = 0x0;
        let pc = get_random_number(0xFFFF);
        let result = base.wrapping_sub(val);
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0x96;
        cpu.memory.ram[((val1 as u16) << 8 | val2 as u16) as usize] = val;
        cpu.a = base.into();
        cpu.h = val1.into();
        cpu.l = val2.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, result);
    }

    #[test]
    fn test_opcode_9d_sbb_l_without_borrow() {
        let mut cpu = Cpu::new();
        let val = get_random_number(0xFF) as u8;
        let base = 0xFF;
        let pc = get_random_number(0xFFFF);
        let result = base - val;
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0x95;
        cpu.a = base.into();
        cpu.l = val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, result);
    }

    #[test]
    fn test_opcode_9f_sbb_a_with_borrow() {
        let mut cpu = Cpu::new();
        let val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0x97;
        cpu.a = val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, 0);
    }

    #[test]
    fn test_opcode_a3_ana_e() {
        let mut cpu = Cpu::new();
        let reg_val = get_random_number(0xFF) as u8;
        let acc_val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        let result = reg_val & acc_val;
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0xA3;
        cpu.e = reg_val.into();
        cpu.a = acc_val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, result);
    }

    #[test]
    fn test_opcode_aa_xra_d() {
        let mut cpu = Cpu::new();
        let reg_val = get_random_number(0xFF) as u8;
        let acc_val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        let result = reg_val ^ acc_val;
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0xAA;
        cpu.d = reg_val.into();
        cpu.a = acc_val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, result);
    }

    #[test]
    fn test_opcode_b5_ora_l() {
        let mut cpu = Cpu::new();
        let reg_val = get_random_number(0xFF) as u8;
        let acc_val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        let result = reg_val | acc_val;
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0xB5;
        cpu.l = reg_val.into();
        cpu.a = acc_val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, result);
    }

    #[test]
    fn test_opcode_b9_cmp_c_not_equal() {
        let mut cpu = Cpu::new();
        let reg_val = get_random_number(0xFF) as u8;
        let acc_val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0xB9;
        cpu.c = reg_val.into();
        cpu.a = acc_val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, acc_val);
        assert_eq!(cpu.flags.z, false);
    }

    #[test]
    fn test_opcode_b9_cmp_c_equal() {
        let mut cpu = Cpu::new();
        let reg_val = get_random_number(0xFF) as u8;
        let pc = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.memory.ram[pc as usize] = 0xB9;
        cpu.c = reg_val.into();
        cpu.a = reg_val.into();
        cpu.execute_opcode();

        assert_eq!(cpu.a, reg_val);
        assert_eq!(cpu.flags.z, true);
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
