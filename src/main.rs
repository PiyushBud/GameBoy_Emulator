
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;



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
  f: u8,
  h: u8,
  l: u8,
}

struct FlagsRegister{
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}


impl Registers{

    /*
    Functions to create virtual 16 bit registers
    by combining 8 bit registers (af, bc, de, hi).
    */
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16);
    }

    fn set_af(&mut self, value: u16){
        self.a = (value & 0xFF00) >> 8 as u8;
        self.f = (value & 0xFF) as u8
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16);
    }

    fn set_bc(&mut self, value: u16){
        self.b = (value & 0xFF00) >> 8 as u8;
        self.c = (value & 0xFF) as u8
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16);
    }

    fn set_de(&mut self, value: u16){
        self.d = (value & 0xFF00) >> 8 as u8;
        self.e = (value & 0xFF) as u8
    }

    fn get_hi(&self) -> u16 {
        (self.h as u16) << 8 | (self.i as u16);
    }

    fn set_hi(&mut self, value: u16){
        self.h = (value & 0xFF00) >> 8 as u8;
        self.i = (value & 0xFF) as u8
    }
}

fn main() {
    println!("Hello, world!");
}
