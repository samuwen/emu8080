mod flags;
mod memory;
mod opcode;
mod pointers;
mod registers;

use flags::Flags;
use memory::Memory;
use opcode::Opcode;
use pointers::Pointer;
use registers::Register;

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
            0x07 => self.rlc(),
            0x09 | 0x19 | 0x29 | 0x39 => self.dad_operation(op.code),
            0x0F => self.rrc(),
            0x0A | 0x1A => self.ldax_operation(op.code),
            0x0B | 0x1B | 0x2B | 0x3B => self.dcx_operation(op.code),
            0x17 => self.ral(),
            0x1F => self.rar(),
            0x22 => self.shld(),
            0x27 => self.daa(),
            0x2A => self.lhld(),
            0x2F => self.cma(),
            0x32 => self.sta(),
            0x37 => self.stc(),
            0x3A => self.lda(),
            0x3F => self.cmc(),
            0x40...0x47 => self.mov_b_operation(op.code),
            0x48...0x4F => self.mov_c_operation(op.code),
            0x50...0x57 => self.mov_d_operation(op.code),
            0x58...0x5F => self.mov_e_operation(op.code),
            0x60...0x67 => self.mov_h_operation(op.code),
            0x68...0x6F => self.mov_l_operation(op.code),
            0x70...0x75 | 0x77 => self.mov_m_operation(op.code),
            0x76 => self.hlt(),
            0x78...0x7F => self.mov_a_operation(op.code),
            0x80...0x87 => self.add_operation(op.code),
            0x88...0x8F => self.adc_operation(op.code),
            0x90...0x97 => self.sub_operation(op.code),
            0x98...0x9F => self.sbb_operation(op.code),
            0xA0...0xA7 => self.ana_operation(op.code),
            0xA8...0xAF => self.xra_operation(op.code),
            0xB0...0xB7 => self.ora_operation(op.code),
            0xB8...0xBF => self.cmp_operation(op.code),
            0xC0 => self.rnz(),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => self.pop_operation(op.code),
            0xC2 => self.jnz(),
            0xC3 => self.jmp(),
            0xC4 => self.cnz(),
            0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push_operation(op.code),
            0xC6 => self.adi(),
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => self.rst_operation(op.code),
            0xC8 => self.rz(),
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
            // // 0xD0 => debug!("{:x} RNC", self.registers.pc),
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
            // // 0xD6 => {
            // //     debug!("{:x} SUI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
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
            // // 0xE0 => debug!("{:x} RPO", self.registers.pc),
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
            // // 0xE6 => {
            // //     debug!("{:x} ANI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
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
            // // 0xF0 => debug!("{:x} RP", self.registers.pc),
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
            // // 0xF6 => {
            // //     debug!("{:x} ORI  D8, {:x}", self.registers.pc, self.extra_byte(1));
            // //     self.registers.pc += 1
            // // }
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
            _ => panic!("Invalid opcode"),
        }
        self.pc += 1;
    }

    #[inline]
    fn set_flags(&mut self, result: &u8, reg_value: u8, op: u8, overflow: bool) {
        let aux_vals: (bool, bool) = self.get_b3_vals(&reg_value, &op);
        self.flags.p = self.sets_parity_flag(&result);
        self.flags.z = self.sets_zero_flag(&result);
        self.flags.s = self.sets_sign_flag(&result);
        self.flags.ac = self.sets_aux_carry_flag(aux_vals.0, aux_vals.1, &result);
        self.flags.cy = if self.flags.cy { !overflow } else { overflow };
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
    fn get_b3_vals(mut self, register: &u8, op: &u8) -> (bool, bool) {
        let b3_1 = self.is_b3_set(&op);
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
            self.a,
            self.get_reg_value(code) + carry_value,
            &overflowing_add_u8,
        );
    }

    fn add_operation(&mut self, code: u8) {
        self.a = self.update_register_with_overflow(
            self.a,
            self.get_reg_value(code),
            &overflowing_add_u8,
        );
    }

    fn adi(&mut self) {
        let a: u8 = self.a.into();
        let next_byte = self.get_next_byte();
        let (result, overflow) = a.overflowing_add(next_byte);
        self.set_flags(&result, a, next_byte, overflow);
        self.a = result.into();
        self.pc += 2;
    }

    fn sbb_operation(&mut self, code: u8) {
        let carry_value = if self.flags.cy { 1 } else { 0 };
        self.a = self.update_register_with_overflow(
            self.a,
            !(self.get_reg_value(code) + carry_value) + 1,
            &overflowing_add_u8,
        );
    }

    fn sub_operation(&mut self, code: u8) {
        self.a = self.update_register_with_overflow(
            self.a,
            !self.get_reg_value(code) + 1,
            &overflowing_add_u8,
        );
    }

    fn ana_operation(&mut self, code: u8) {
        self.a = self.update_register_no_overflow(self.a, self.get_reg_value(code), &logical_and);
    }

    fn xra_operation(&mut self, code: u8) {
        self.a = self.update_register_no_overflow(self.a, self.get_reg_value(code), &logical_xor);
    }

    fn ora_operation(&mut self, code: u8) {
        self.a = self.update_register_no_overflow(self.a, self.get_reg_value(code), &logical_or);
    }

    fn cmp_operation(&mut self, code: u8) {
        let reg_value = !self.get_reg_value(code) + 1;
        let acc_value: u8 = self.a.into();
        let (result, overflow) = acc_value.overflowing_add(reg_value);
        self.set_flags(&result, reg_value, acc_value, !overflow);
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
                self.sp = result.into();
            }
            _ => panic!("Bug exists in opcode routing"),
        }
    }

    fn inr_operation(&mut self, code: u8) {
        match code {
            0x04 => self.b = self.update_register_with_overflow(self.b, 1, &overflowing_add_u8),
            0x0C => self.c = self.update_register_with_overflow(self.c, 1, &overflowing_add_u8),
            0x14 => self.d = self.update_register_with_overflow(self.d, 1, &overflowing_add_u8),
            0x1C => self.e = self.update_register_with_overflow(self.e, 1, &overflowing_add_u8),
            0x24 => self.h = self.update_register_with_overflow(self.h, 1, &overflowing_add_u8),
            0x2C => self.l = self.update_register_with_overflow(self.l, 1, &overflowing_add_u8),
            0x34 => {
                let mem_ref = self.get_reg_pair_value(self.l, self.h);
                let value = self.memory.ram[mem_ref as usize];
                let (result, overflow) = value.overflowing_add(1);
                self.set_flags(&result, value, 1, overflow);
                self.l = (result & 0xFF).into();
                self.h = (result & 0x00FF).into();
            }
            0x3C => self.a = self.update_register_with_overflow(self.a, 1, &overflowing_add_u8),
            _ => panic!("Bug exists in opcode routing"),
        }
    }

    fn dcr_operation(&mut self, code: u8) {
        match code {
            0x05 => self.b = self.update_register_with_overflow(self.b, 1, &overflowing_sub_u8),
            0x0D => self.c = self.update_register_with_overflow(self.c, 1, &overflowing_sub_u8),
            0x15 => self.d = self.update_register_with_overflow(self.d, 1, &overflowing_sub_u8),
            0x1D => self.e = self.update_register_with_overflow(self.e, 1, &overflowing_sub_u8),
            0x25 => self.h = self.update_register_with_overflow(self.h, 1, &overflowing_sub_u8),
            0x2D => self.l = self.update_register_with_overflow(self.l, 1, &overflowing_sub_u8),
            0x35 => {
                let mem_ref = self.get_reg_pair_value(self.h, self.l);
                let value = self.memory.ram[mem_ref as usize];
                let (result, overflow) = value.overflowing_sub(1);
                self.set_flags(&result, value, 1, !overflow);
                self.memory.ram[mem_ref as usize] = result;
            }
            0x3D => self.a = self.update_register_with_overflow(self.a, 1, &overflowing_sub_u8),
            _ => panic!("Bug exists in opcode routing"),
        }
    }

    fn mvi_operation(&mut self, code: u8) {
        match code {
            0x06 => self.b = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x0E => self.c = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x16 => self.d = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x1E => self.e = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x26 => self.h = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x2E => self.l = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            0x36 => {
                let mem_ref = self.get_reg_pair_value(self.h, self.l);
                self.memory.ram[mem_ref as usize] = self.memory.ram[(usize::from(self.pc) + 1)];
            }
            0x3E => self.a = self.memory.ram[(usize::from(self.pc) + 1)].into(),
            _ => panic!("Bug exists in opcode routing"),
        }
        self.pc += 1
    }

    fn pop_operation(&mut self, code: u8) {
        let (msb, lsb) = self.pop_off_stack();
        match code {
            0xC1 => {
                self.b = msb.into();
                self.c = lsb.into();
            }
            0xD1 => {
                self.d = msb.into();
                self.e = lsb.into();
            }
            0xE1 => {
                self.h = msb.into();
                self.l = lsb.into();
            }
            0xF1 => {
                self.a = msb.into();
                self.flags.s = (lsb & 0x80) >> 7 == 1;
                self.flags.z = (lsb & 0x40) >> 6 == 1;
                self.flags.ac = (lsb & 0x10) >> 4 == 1;
                self.flags.p = (lsb & 0x4) >> 2 == 1;
                self.flags.cy = (lsb & 0x1) == 1;
            }
            _ => panic!("Bug exists in opcode routing"),
        }
        self.sp += 2;
        self.pc += 2;
    }

    fn push_operation(&mut self, code: u8) {
        match code {
            0xC5 => {
                let val = self.get_reg_pair_value(self.b, self.c);
                self.push_to_stack(val);
            }
            0xD5 => {
                let val = self.get_reg_pair_value(self.d, self.e);
                self.push_to_stack(val);
            }
            0xE5 => {
                let val = self.get_reg_pair_value(self.h, self.l);
                self.push_to_stack(val);
            }
            0xF5 => {
                let msb: u8 = self.a.into();
                let mut lsb: u8 = 0;
                lsb += (self.flags.s as u8) << 7;
                lsb += (self.flags.z as u8) << 6;
                lsb += (self.flags.ac as u8) << 4;
                lsb += (self.flags.p as u8) << 2;
                lsb += 1 << 1;
                lsb += self.flags.cy as u8;
                self.push_to_stack(((msb as u16) << 8) | lsb as u16);
            }
            _ => panic!("Bug in opcode routing"),
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

    fn ral(&mut self) {
        let reg_value: u8 = self.a.into();
        let carry_flag = ((&reg_value & 0x80) >> 7) == 1;
        let temp: u8 = &reg_value << 1;
        self.a = if self.flags.cy {
            (temp | 0x1).into()
        } else {
            (temp | 0x0).into()
        };
        self.flags.cy = carry_flag;
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

    fn rar(&mut self) {
        let reg_value: u8 = self.a.into();
        let carry_flag = (&reg_value & 0x1) == 1;
        let temp: u8 = &reg_value >> 1;
        self.a = if self.flags.cy {
            (temp | 0x80).into()
        } else {
            (temp | 0x0).into()
        };
        self.flags.cy = carry_flag;
    }

    fn shld(&mut self) {
        let mem_add = self.get_memory_reference();
        self.memory.ram[mem_add as usize] = self.l.into();
        self.memory.ram[(mem_add + 1) as usize] = self.h.into();
        self.pc += 2;
    }

    fn daa(&mut self) {
        let inc_6 = |val, flag| val + if val > 9 || flag { 6 } else { 0 };
        let chk_over = |val| ((val & 0xF0) >> 4) == 1u8;
        let split_bytes = |val: u8| ((val & 0xF), ((val & 0xF0) >> 4));
        let (lsbits, mut msbits) = split_bytes(self.a.into());
        let lsmod = inc_6(lsbits, self.flags.ac);
        if chk_over(lsmod) {
            msbits = msbits.wrapping_add(1);
        }
        let msmod = inc_6(msbits, self.flags.cy);
        self.flags.ac = chk_over(lsmod);
        self.flags.cy = chk_over(msmod);

        self.a = (((msmod & 0xF) << 4) | lsmod & 0xF).into();
    }

    fn lhld(&mut self) {
        let mem_add = self.get_memory_reference();
        self.h = self.memory.ram[mem_add as usize].into();
        self.l = self.memory.ram[(mem_add + 1) as usize].into();
        self.pc += 2;
    }

    fn cma(&mut self) {
        let val: u8 = self.a.into();
        let inverted: u8 = !val;
        self.a = inverted.into();
    }

    fn sta(&mut self) {
        let mem_add = self.get_memory_reference();
        self.memory.ram[mem_add as usize] = self.a.into();
        self.pc += 2;
    }

    fn stc(&mut self) {
        self.flags.cy = true;
    }

    fn lda(&mut self) {
        let mem_add = self.get_memory_reference();
        self.a = self.memory.ram[mem_add as usize].into();
        self.pc += 2;
    }

    fn cmc(&mut self) {
        self.flags.cy = !self.flags.cy;
    }

    fn hlt(&mut self) {
        // not sure how to implement yet
    }

    // Jump operations

    fn jmp(&mut self) {
        self.pc = self.get_memory_reference().into();
    }

    fn jnz(&mut self) {
        if self.flags.z {
            self.jmp();
        } else {
            self.pc += 2;
        }
    }

    // Call operations

    fn call_subroutine(&mut self) {
        let mem_ref = self.get_memory_reference();
        self.push_to_stack(self.pc.into());
        self.pc = mem_ref.into();
    }

    fn cnz(&mut self) {
        if self.flags.z {
            self.call_subroutine()
        } else {
            self.pc += 2
        }
    }

    // Return operations

    fn return_from_subroutine(&mut self) {
        let (msb, lsb) = self.pop_off_stack();
        self.pc = self.get_pair_value(msb, lsb).into();
    }

    fn rnz(&mut self) {
        if self.flags.z {
            self.return_from_subroutine();
        }
    }

    fn rz(&mut self) {
        if !self.flags.z {
            self.return_from_subroutine();
        }
    }

    fn rst_operation(&mut self, code: u8) {
        self.push_to_stack(self.pc.into());
        self.pc = (code & 0x38).into();
    }

    fn mov_b_operation(&mut self, code: u8) {
        self.b = self.get_reg_value(code).into();
    }

    fn mov_c_operation(&mut self, code: u8) {
        self.c = self.get_reg_value(code).into();
    }

    fn mov_d_operation(&mut self, code: u8) {
        self.d = self.get_reg_value(code).into();
    }

    fn mov_e_operation(&mut self, code: u8) {
        self.e = self.get_reg_value(code).into();
    }

    fn mov_h_operation(&mut self, code: u8) {
        self.h = self.get_reg_value(code).into();
    }

    fn mov_l_operation(&mut self, code: u8) {
        self.l = self.get_reg_value(code).into();
    }

    fn mov_a_operation(&mut self, code: u8) {
        self.a = self.get_reg_value(code).into();
    }

    fn mov_m_operation(&mut self, code: u8) {
        let mem_ref = self.get_reg_pair_value(self.h, self.l);
        self.memory.ram[mem_ref as usize] = self.get_reg_value(code).into();
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
                let (a, b) = Cpu::return_split_registers(self.sp.into());
                let (x, y) = self.update_register_pair_overflow(a, b, hl_reg, &overflowing_add_u16);
                self.h = x;
                self.l = y;
            }
            _ => panic!("Bug in opcode router"),
        }
    }

    fn ldax_operation(&mut self, code: u8) {
        match code {
            0x0A => {
                let val = self.get_reg_pair_value(self.b, self.c);
                self.a = self.memory.ram[val as usize].into();
            }
            0x1A => {
                let val = self.get_reg_pair_value(self.d, self.e);
                self.a = self.memory.ram[val as usize].into();
            }
            _ => panic!("Bug in opcode router"),
        }
    }

    fn dcx_operation(&mut self, code: u8) {
        match code {
            0x0B => {
                let (x, y) = self.update_register_pair(self.b, self.c, 1, &wrapping_sub);
                self.b = x.into();
                self.c = y.into();
            }
            0x1B => {
                let (x, y) = self.update_register_pair(self.d, self.e, 1, &wrapping_sub);
                self.b = x.into();
                self.c = y.into();
            }
            0x2B => {
                let (x, y) = self.update_register_pair(self.h, self.l, 1, &wrapping_sub);
                self.b = x.into();
                self.c = y.into();
            }
            0x3B => {
                let (a, b) = Cpu::return_split_registers(self.sp.into());
                let (x, y) = self.update_register_pair(a, b, 1, &wrapping_sub);
                self.b = x.into();
                self.c = y.into();
            }
            _ => panic!("Bug in opcode router"),
        }
    }

    fn update_register_with_overflow(
        &mut self,
        reg: Register,
        op: u8,
        f: &Fn(u8, u8) -> (u8, bool),
    ) -> Register {
        let val: u8 = reg.into();
        let (result, overflow) = f(val, op);
        self.set_flags(&result, reg.into(), op, overflow);
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
        self.set_flags(&result, reg.into(), op, false);
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
        let reg_val = self.get_reg_pair_value(msb, lsb);
        let (result, overflow) = f(op, reg_val);
        self.flags.cy = if overflow { true } else { false };
        Cpu::return_split_registers(result)
    }

    fn get_reg_value(self, code: u8) -> u8 {
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
        self.get_pair_value(msb.into(), lsb.into())
    }

    fn get_pair_value(self, msb: u8, lsb: u8) -> u16 {
        ((msb as u16) << 8) | lsb as u16
    }

    fn get_memory_reference(self) -> u16 {
        let low_adr: u16 = self.memory.ram[usize::from(self.pc + 1)].into();
        let high_adr: u16 = self.memory.ram[usize::from(self.pc + 2)].into();
        (high_adr << 8) | low_adr
    }

    fn get_next_byte(self) -> u8 {
        self.memory.ram[usize::from(self.pc + 1)]
    }

    fn push_to_stack(&mut self, val: u16) {
        let (msb, lsb) = Cpu::return_split_values(val);
        self.memory.ram[usize::from(self.sp - 1)] = msb;
        self.memory.ram[usize::from(self.sp - 2)] = lsb;
        self.sp -= 2;
    }

    fn pop_off_stack(&mut self) -> (u8, u8) {
        let lsb = self.memory.ram[usize::from(self.sp)].into();
        let msb = self.memory.ram[usize::from(self.sp + 1)].into();
        self.sp += 2;
        (msb, lsb)
    }

    fn return_split_registers(val: u16) -> (Register, Register) {
        let (x, y) = Cpu::return_split_values(val);
        (x.into(), y.into())
    }

    fn return_split_values(val: u16) -> (u8, u8) {
        (((val & 0xFF00) >> 8) as u8, (val & 0xFF) as u8)
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

fn wrapping_sub(val: u16, operand: u16) -> u16 {
    val.wrapping_sub(operand)
}

// Carries are inverted in subtraction
fn overflowing_sub_u8(val: u8, operand: u8) -> (u8, bool) {
    let (result, overflow) = val.overflowing_sub(operand);
    let overflow = if overflow { false } else { true };
    (result, overflow)
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
        let (b1, b2) = cpu.get_b3_vals(&v2, &v1);
        let (result, overflow) = v1.overflowing_add(v2);
        cpu.set_flags(&result, v2, v1, overflow);
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

    // opcode tests

    #[test]
    fn test_lxi_operation() {
        let mut cpu = Cpu::new();
        let opcode = 0x11;
        let rand_addr = 0x103;
        cpu.pc = (rand_addr as u16).into();
        cpu.memory.ram[rand_addr] = opcode;
        cpu.memory.ram[rand_addr + 1] = 0x3;
        cpu.memory.ram[rand_addr + 2] = 0x1;
        cpu.lxi_operation(opcode);

        assert_eq!(cpu.d, 0x1);
        assert_eq!(cpu.e, 0x3);
    }

    #[test]
    fn test_stax_operation() {
        let mut cpu = Cpu::new();
        let acc_val = get_random_number(0xFF);
        cpu.a = acc_val.into();
        cpu.b = (0x3F as u8).into();
        cpu.c = (0x16 as u8).into();
        cpu.stax_operation(0x02);

        assert_eq!(cpu.memory.ram[0x3F16], acc_val as u8);
    }

    #[test]
    fn test_inx_operation() {
        let mut cpu = Cpu::new();
        cpu.d = (0x38 as u8).into();
        cpu.e = (0xFF as u8).into();
        cpu.inx_operation(0x13);

        assert_eq!(cpu.d, 0x39);
        assert_eq!(cpu.e, 0x0);
    }

    #[test]
    fn test_inr_operation() {
        let mut cpu = Cpu::new();
        cpu.c = (0x99 as u8).into();
        cpu.inr_operation(0x0C);

        assert_eq!(cpu.c, 0x9A);
    }

    #[test]
    fn test_dcr_operation() {
        let mut cpu = Cpu::new();
        cpu.h = (0x3A as u8).into();
        cpu.l = (0x7C as u8).into();
        cpu.memory.ram[0x3A7C] = 0x40;
        cpu.dcr_operation(0x35);

        assert_eq!(cpu.memory.ram[0x3A7C], 0x3F);
    }

    #[test]
    fn test_mvi_operation() {
        let mut cpu = Cpu::new();
        cpu.d = (0xF4 as u8).into();
        cpu.memory.ram[0x1] = 0x36;
        cpu.mvi_operation(0x16);

        assert_eq!(cpu.d, 0x36);
    }

    #[test]
    fn test_dad_operation() {
        let mut cpu = Cpu::new();
        cpu.b = (0x33 as u8).into();
        cpu.c = (0x9F as u8).into();
        cpu.h = (0xA1 as u8).into();
        cpu.l = (0x7B as u8).into();
        cpu.dad_operation(0x09);

        assert_eq!(cpu.h, 0xD5);
        assert_eq!(cpu.l, 0x1A);
    }

    #[test]
    fn test_ldax_operation() {
        let mut cpu = Cpu::new();
        cpu.d = (0x93 as u8).into();
        cpu.e = (0x8B as u8).into();
        cpu.a = (0xFF as u8).into();
        let val: u8 = 0x34;
        cpu.memory.ram[0x938B as usize] = val;
        cpu.ldax_operation(0x1A);

        assert_eq!(cpu.a, val);
    }

    #[test]
    fn test_add_operation() {
        let mut cpu = Cpu::new();
        let val: u8 = 0x2E;
        cpu.b = val.into();
        let acc_val: u8 = 0x6C;
        cpu.a = acc_val.into();
        cpu.add_operation(0x80);

        assert_eq!(cpu.a, 0x9A);
    }

    #[test]
    fn test_adc_operation_not_set() {
        let mut cpu = Cpu::new();
        let val: u8 = 0x3D;
        cpu.b = val.into();
        let acc_val: u8 = 0x42;
        cpu.a = acc_val.into();
        cpu.adc_operation(0x88);

        assert_eq!(cpu.a, 0x7F);
    }

    #[test]
    fn test_adc_operation_set() {
        let mut cpu = Cpu::new();
        cpu.flags.cy = true;
        let val: u8 = 0x3D;
        cpu.b = val.into();
        let acc_val: u8 = 0x42;
        cpu.a = acc_val.into();
        cpu.adc_operation(0x88);

        assert_eq!(cpu.a, 0x80);
    }

    #[test]
    fn test_sub_operation() {
        let mut cpu = Cpu::new();
        cpu.flags.cy = true;
        let val: u8 = 0x3E;
        cpu.b = val.into();
        let acc_val: u8 = 0x3E;
        cpu.a = acc_val.into();
        cpu.sub_operation(0x80);

        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.flags.ac, true);
        assert_eq!(cpu.flags.cy, false);
        assert_eq!(cpu.flags.p, true);
        assert_eq!(cpu.flags.z, true);
    }

    #[test]
    fn test_sbb_operation_not_set() {
        let mut cpu = Cpu::new();
        let val: u8 = 0x2;
        cpu.l = val.into();
        let acc_val: u8 = 0x4;
        cpu.a = acc_val.into();
        cpu.sbb_operation(0x9D);

        assert_eq!(cpu.a, 0x2);
    }

    #[test]
    fn test_sbb_operation_set() {
        let mut cpu = Cpu::new();
        cpu.flags.cy = true;
        let val: u8 = 0x2;
        cpu.l = val.into();
        let acc_val: u8 = 0x4;
        cpu.a = acc_val.into();
        cpu.sbb_operation(0x9D);

        assert_eq!(cpu.a, 0x1);
        assert_eq!(cpu.flags.cy, false);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.p, false);
        assert_eq!(cpu.flags.ac, true);
    }

    #[test]
    fn test_ana_operation() {
        let mut cpu = Cpu::new();
        let reg_val: u8 = 0xF;
        let acc_val: u8 = 0xFC;
        cpu.a = acc_val.into();
        cpu.c = reg_val.into();
        cpu.ana_operation(0xA1);

        assert_eq!(cpu.a, 0xC);
    }

    #[test]
    fn test_xra_operation() {
        let mut cpu = Cpu::new();
        let acc_val: u8 = 0xF;
        cpu.a = acc_val.into();
        cpu.xra_operation(0xAF);

        assert_eq!(cpu.a, 0x0);
    }

    #[test]
    fn test_ora_operation() {
        let mut cpu = Cpu::new();
        let reg_val: u8 = 0x33;
        let acc_val: u8 = 0x0F;
        cpu.a = acc_val.into();
        cpu.c = reg_val.into();
        cpu.ora_operation(0xB1);

        assert_eq!(cpu.a, 0x3F);
    }

    #[test]
    fn test_cmp_operation() {
        let mut cpu = Cpu::new();
        let reg_val: u8 = 0x5;
        let acc_val: u8 = 0xA;
        cpu.a = acc_val.into();
        cpu.e = reg_val.into();
        cpu.cmp_operation(0xBB);

        assert_eq!(cpu.a, acc_val);
        assert_eq!(cpu.e, reg_val);
        assert_eq!(cpu.flags.cy, false);
        assert_eq!(cpu.flags.z, false);
    }

    #[test]
    fn test_rlc() {
        let mut cpu = Cpu::new();
        let val: u8 = 0xF2;
        cpu.a = val.into();
        cpu.rlc();

        assert_eq!(cpu.a, 0xE5);
    }

    #[test]
    fn test_rrc() {
        let mut cpu = Cpu::new();
        let val: u8 = 0xF2;
        cpu.a = val.into();
        cpu.rrc();

        assert_eq!(cpu.a, 0x79);
    }

    #[test]
    fn test_ral() {
        let mut cpu = Cpu::new();
        cpu.a = (0xB5 as u8).into();
        cpu.ral();

        assert_eq!(cpu.a, 0x6A);
    }

    #[test]
    fn test_rar() {
        let mut cpu = Cpu::new();
        cpu.a = (0x6A as u8).into();
        cpu.flags.cy = true;
        cpu.rar();

        assert_eq!(cpu.a, 0xB5);
    }

    #[test]
    fn test_shld() {
        let mut cpu = Cpu::new();
        cpu.h = (0xAE as u8).into();
        cpu.l = (0x29 as u8).into();
        let rand: u16 = get_random_number(0xFFFC);
        cpu.pc = rand.into();
        cpu.memory.ram[(rand + 1) as usize] = 0x0A;
        cpu.memory.ram[(rand + 2) as usize] = 0x01;
        cpu.shld();

        assert_eq!(cpu.memory.ram[0x10A], 0x29);
        assert_eq!(cpu.memory.ram[0x10B], 0xAE);
    }

    #[test]
    fn test_daa() {
        let mut cpu = Cpu::new();
        cpu.a = 0x9Bu8.into();
        cpu.daa();

        assert_eq!(cpu.a, 1);
    }

    #[test]
    fn test_lhld() {
        let mut cpu = Cpu::new();
        let rand: u16 = 0x10;
        cpu.pc = rand.into();
        cpu.memory.ram[(rand + 1) as usize] = 0xCB;
        cpu.memory.ram[(rand + 2) as usize] = 0x50;
        cpu.memory.ram[0x50CB] = 0xFF;
        cpu.memory.ram[0x50CC] = 0x03;
        cpu.lhld();

        assert_eq!(cpu.h, 0xFF);
        assert_eq!(cpu.l, 0x03);
    }

    #[test]
    fn test_cma() {
        let mut cpu = Cpu::new();
        cpu.a = 0x51u8.into();
        cpu.cma();

        assert_eq!(cpu.a, 0xAE);
    }

    #[test]
    fn test_sta() {
        let mut cpu = Cpu::new();
        let rand: u8 = get_random_number(0xFF) as u8;
        cpu.a = rand.into();
        let pc = get_random_number(0xFFFC);
        cpu.pc = pc.into();
        cpu.memory.ram[(pc + 1) as usize] = 0x23;
        cpu.memory.ram[(pc + 2) as usize] = 0xC8;
        cpu.sta();

        assert_eq!(cpu.memory.ram[0xC823], rand);
    }

    #[test]
    fn test_stc() {
        let mut cpu = Cpu::new();
        cpu.stc();

        assert_eq!(cpu.flags.cy, true);
    }

    #[test]
    fn test_lda() {
        let mut cpu = Cpu::new();
        let pc: u8 = get_random_number(0xFFFC) as u8;
        cpu.pc = pc.into();
        cpu.memory.ram[(pc + 1) as usize] = 0x39;
        cpu.memory.ram[(pc + 2) as usize] = 0xB2;
        cpu.memory.ram[0xB239] = 0xEE;
        cpu.lda();

        assert_eq!(cpu.a, 0xEE);
    }

    #[test]
    fn test_cmc() {
        let mut cpu = Cpu::new();
        cpu.flags.cy = true;
        cpu.cmc();

        assert_eq!(cpu.flags.cy, false);
        cpu.cmc();

        assert_eq!(cpu.flags.cy, true);
    }

    #[test]
    fn test_hlt() {
        unimplemented!()
    }

    #[test]
    fn test_mov_b() {
        let mut cpu = Cpu::new();
        cpu.c = 0x23u8.into();
        cpu.b = 0xAAu8.into();
        cpu.mov_b_operation(0x41);

        assert_eq!(cpu.b, 0x23);
    }

    #[test]
    fn test_mov_c() {
        let mut cpu = Cpu::new();
        cpu.d = 0x66u8.into();
        cpu.c = 0xE3u8.into();
        cpu.mov_c_operation(0x4A);

        assert_eq!(cpu.c, 0x66);
    }

    #[test]
    fn test_mov_d() {
        let mut cpu = Cpu::new();
        cpu.l = 0x01u8.into();
        cpu.h = 0xA9u8.into();
        cpu.memory.ram[0xA901] = 0x4C;
        cpu.d = 0x77u8.into();
        cpu.mov_d_operation(0x56);

        assert_eq!(cpu.d, 0x4C);
    }

    #[test]
    fn test_jmp() {
        let mut cpu = Cpu::new();
        let pc: u8 = get_random_number(0xFFF0) as u8;
        cpu.pc = pc.into();
        cpu.memory.ram[(pc + 1) as usize] = 0x00;
        cpu.memory.ram[(pc + 2) as usize] = 0x3E;
        cpu.jmp();

        assert_eq!(u16::from(cpu.pc), 0x3E00);
    }

    #[test]
    fn test_rnz_z_set() {
        let mut cpu = Cpu::new();
        let pc = get_random_number(0xFFFF);
        let sp = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.flags.z = true;
        cpu.sp = sp.into();
        let msb: u16 = cpu.memory.ram[(sp - 1) as usize].into();
        let lsb: u16 = cpu.memory.ram[(sp - 2) as usize].into();
        let result = (msb << 8) | lsb;
        cpu.rnz();

        assert_eq!(cpu.pc, result);
    }

    #[test]
    fn test_rnz_z_unset() {
        let mut cpu = Cpu::new();
        let pc = get_random_number(0xFFFF);
        let new_pc = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.sp = new_pc.into();
        cpu.rnz();

        assert_eq!(cpu.pc, pc);
    }

    #[test]
    fn test_rz_z_unset() {
        let mut cpu = Cpu::new();
        let pc = get_random_number(0xFFFF);
        let sp = get_random_number(0xFFFF);
        cpu.pc = pc.into();
        cpu.flags.z = false;
        cpu.sp = sp.into();
        let msb: u16 = cpu.memory.ram[(sp - 1) as usize].into();
        let lsb: u16 = cpu.memory.ram[(sp - 2) as usize].into();
        let result = (msb << 8) | lsb;
        cpu.rz();

        assert_eq!(cpu.pc, result);
    }

    #[test]
    fn test_rz_z_set() {
        let mut cpu = Cpu::new();
        let pc = get_random_number(0xFFFF);
        let new_pc = get_random_number(0xFFFF);
        cpu.flags.z = true;
        cpu.pc = pc.into();
        cpu.sp = new_pc.into();
        cpu.rz();

        assert_eq!(cpu.pc, pc);
    }

    #[test]
    fn test_pop_non_psw() {
        let mut cpu = Cpu::new();
        cpu.memory.ram[0x1239] = 0x3D;
        cpu.memory.ram[0x123A] = 0x93;
        cpu.sp = 0x1239u16.into();
        cpu.pop_operation(0xE1);

        assert_eq!(cpu.l, 0x3D);
        assert_eq!(cpu.h, 0x93);
    }

    #[test]
    fn test_pop_psw() {
        let mut cpu = Cpu::new();
        cpu.memory.ram[0x2C00] = 0xC3;
        cpu.memory.ram[0x2C01] = 0xFF;
        cpu.sp = 0x2C00u16.into();
        cpu.pop_operation(0xF1);

        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.flags.s, true);
        assert_eq!(cpu.flags.z, true);
        assert_eq!(cpu.flags.ac, false);
        assert_eq!(cpu.flags.p, false);
        assert_eq!(cpu.flags.cy, true);
    }

    #[test]
    fn test_jnz_zero_set() {
        let mut cpu = Cpu::new();
        cpu.flags.z = true;
        let pc = get_random_number(0xFFF0);
        cpu.pc = pc.into();
        cpu.memory.ram[(pc + 2) as usize] = 0xB9;
        cpu.memory.ram[(pc + 1) as usize] = 0x66;
        cpu.jnz();

        assert_eq!(cpu.pc, 0xB966);
    }

    #[test]
    fn test_jnz_zero_unset() {
        let mut cpu = Cpu::new();
        cpu.flags.z = false;
        let pc = get_random_number(0xFFF0);
        cpu.pc = pc.into();
        cpu.memory.ram[(pc + 2) as usize] = 0xB9;
        cpu.memory.ram[(pc + 1) as usize] = 0x66;
        cpu.jnz();

        assert_eq!(cpu.pc, pc + 2);
    }

    #[test]
    fn test_cnz_zero_set() {
        let mut cpu = Cpu::new();
        cpu.flags.z = true;
        let pc = get_random_number(0xFFF0);
        cpu.pc = pc.into();
        let sp = get_random_number(0xFFF0);
        cpu.sp = sp.into();
        cpu.memory.ram[(pc + 2) as usize] = 0x6E;
        cpu.memory.ram[(pc + 1) as usize] = 0x0D;
        cpu.memory.ram[(sp + 1) as usize] = 0x33;
        cpu.memory.ram[(sp + 2) as usize] = 0xA9;
        cpu.cnz();

        assert_eq!(cpu.pc, 0x6E0D);
    }

    #[test]
    fn test_cnz_zero_unset() {
        let mut cpu = Cpu::new();
        cpu.flags.z = false;
        let pc = get_random_number(0xFFF0);
        cpu.pc = pc.into();
        let sp = get_random_number(0xFFF0);
        cpu.sp = sp.into();
        cpu.memory.ram[(pc + 2) as usize] = 0x6E;
        cpu.memory.ram[(pc + 1) as usize] = 0x0D;
        cpu.memory.ram[(sp + 1) as usize] = 0x33;
        cpu.memory.ram[(sp + 2) as usize] = 0xA9;
        cpu.cnz();

        assert_eq!(cpu.pc, pc + 2);
    }

    #[test]
    fn test_push_operation() {
        let mut cpu = Cpu::new();
        cpu.d = 0x8Fu8.into();
        cpu.e = 0x9Du8.into();
        cpu.sp = 0x3A2Cu16.into();
        cpu.push_operation(0xD5);

        assert_eq!(cpu.sp, 0x3A2A);
        assert_eq!(cpu.memory.ram[0x3A2B], 0x8F);
        assert_eq!(cpu.memory.ram[0x3A2A], 0x9D);
    }

    #[test]
    fn test_push_operation_psw() {
        let mut cpu = Cpu::new();
        cpu.a = 0x1Fu8.into();
        cpu.sp = 0x502Au16.into();
        cpu.flags.cy = true;
        cpu.flags.z = true;
        cpu.flags.p = true;
        cpu.flags.s = false;
        cpu.flags.ac = false;
        cpu.push_operation(0xF5);

        assert_eq!(cpu.sp, 0x5028);
        assert_eq!(cpu.memory.ram[0x5029], 0x1F);
        assert_eq!(cpu.memory.ram[0x5028], 0x47);
    }

    #[test]
    fn test_adi() {
        let mut cpu = Cpu::new();
        cpu.a = 0x14u8.into();
        let pc = get_random_number(0xFFF0);
        cpu.pc = pc.into();
        cpu.memory.ram[(pc + 1) as usize] = 0x42u8;
        cpu.adi();

        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.flags.p, true);
        assert_eq!(cpu.flags.ac, false);
        assert_eq!(cpu.flags.cy, false);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.s, false);
    }

    #[test]
    fn test_rst_operation() {
        let mut cpu = Cpu::new();
        cpu.sp = get_random_number(0xFFF0).into();
        cpu.pc = get_random_number(0xFFF0).into();
        cpu.rst_operation(0xDF);

        assert_eq!(cpu.pc, 0x18);
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
