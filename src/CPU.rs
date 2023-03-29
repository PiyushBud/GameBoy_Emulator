
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),

}

enum ArithmeticTarget{
    A, B, C, D, E, H, L, BC, DE, HL 
}

/*
Registers struct to emulate the CPU registers
of an 8 bit GameBoy.
*/ 
struct Registers{
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: FlagsRegister,
  h: u8,
  l: u8,
}

#[derive(Copy, Clone)]
struct FlagsRegister{
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

struct CPU{
    registers: Registers
}


impl Registers{

    /*
    Functions to create virtual 16 bit registers
    by combining 8 bit registers (af, bc, de, hi).
    */


    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (u8::from(self.f) as u16)
    }

    fn set_af(&mut self, value: u16){
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn set_de(&mut self, value: u16){
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16){
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister{
    fn from(byte: u8) -> Self {
        let zero: bool = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0x1) != 0;
        let subtract: bool = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0x1) != 0;
        let half_carry: bool = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0x1) != 0;
        let carry: bool = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0x1) != 0;

        FlagsRegister {zero, subtract, half_carry, carry}
    }
}

impl CPU {
    fn execute(&mut self, instruction: Instruction){
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Not a valid register.")
                }
            }

            Instruction::ADDHL(target) => {
                match target {
                    ArithmeticTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    _ => panic!("Not a valid register.")
                }
            }

            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Not a valid register.")
                }
            }

            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                                 self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Not a valid register.")

                }
            }

            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Not a valid register.")
                }
            }

            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.registers.a & self.registers.a;
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.registers.a & self.registers.b;
                    }
                    ArithmeticTarget::C => {
                        self.registers.a = self.registers.a & self.registers.c;
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.registers.a & self.registers.d;
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.registers.a & self.registers.e;
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.registers.a & self.registers.h;
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.registers.a & self.registers.l;
                    }
                    _ => panic!("Not a valid register.")
                }
            }

            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.registers.a | self.registers.a;
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.registers.a | self.registers.b;
                    }
                    ArithmeticTarget::C => {
                        self.registers.a = self.registers.a | self.registers.c;
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.registers.a | self.registers.d;
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.registers.a | self.registers.e;
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.registers.a | self.registers.h;
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.registers.a | self.registers.l;
                    }
                    _ => panic!("Not a valid register.")
                }
            }

            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.registers.a ^ self.registers.a;
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.registers.a ^ self.registers.b;
                    }
                    ArithmeticTarget::C => {
                        self.registers.a = self.registers.a ^ self.registers.c;
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.registers.a ^ self.registers.d;
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.registers.a ^ self.registers.e;
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.registers.a ^ self.registers.h;
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.registers.a ^ self.registers.l;
                    }
                    _ => panic!("Not a valid register.")
                }
            }
            _ => panic!("Not a valid instruction.")
        }
    }

    /* Add value u8 to register A. */
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;
        
        new_value
    }

    /* Add value u16 to register HL. */
    fn addhl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.f.carry = did_overflow;

        new_value
    }

    /* Add value u8 to register A and add the carry flag. */
    fn adc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;

        new_value + (did_overflow as u8)
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) - (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;

        new_value
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) - (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;

        new_value + (did_overflow as u8)
    }
}