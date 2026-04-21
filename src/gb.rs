use std::fs;
use minifb::{Key, Window, WindowOptions};
pub struct GameKeith{
    pub memory: [u8;0xFFFF],
    //0000 3FFF 16KiB ROM bank 00 
    //4000 7FFF 16KiB ROM bank 01-NN
    //8000 9FFF 8Kib VRAM
    //A000 BFFF 8kib cartridge RAM 
    //
    pub a:u8,
    pub f: u8,
    pub b:u8,
    pub c:u8,
    pub d:u8,
    pub e:u8,
    pub hl:u16,
    pub s:u8,
    pub p:u8,
    pub pc:u16,
    pub instruction:u16,
    pub rom:Vec<u8>
}
impl GameKeith{
    fn operand(&mut self) {
        match self.instruction{
            0x00 => {self.pc +=1;},//nop
            0x01 => {self.pc +=1;
                    self.C = self.readmem(self.pc);
                    self.pc +=1;
                    self.b = self.readmem(self.pc;)},//LD BC n16
            0x02 =>{//write to memory[bc] from a
                    let temp =(self.b as u16 >> 8)|self.c;
                    self.writemem(temp, self.a);
                    self.pc +=1;}//LD BC A 
            0x03 =>{
                    self.c +=1;
                    if self.c = 0{self.b+=1;}}//inc bc 
            0x04 =>{self.b = self.alu_inc(self.b);}//inc B
            0x05 =>{self.b = self.alu_dec(self.b);}//dec B 
            0x06 =>{self.pc +=1; self.b = self.readmem(self.pc);}//LD B n8
            0x07 =>{self.a = self.alu_rlc(self.a); self.pc +=1;}//RLCA
            0x08 =>{let pc = self.pc; self.pc+=2;
                    let address =(self.pc as u16 >> 8)| pc as u16;
                    let sp =(self.s as u16 >> 8)| self.p as u16;
                    self.writemem(address,sp)
            }//ld [a16] SP
            0x09 =>{let bc =(self.b as u16 >> 8)| self.c as u16; self.alu_add16(bc);}//ADD HL BC -0HC

            ___ => {println!("{}not implemented",self.rom[self.pc])}
        };}

    fn cpuloop(&mut self){
        let mut instruction :u8 = self.memory[self.pc as usize];
        //run match statement in other file
    }

    fn readmem(&mut self, int:u16)->u8{
        return self.memory[int as usize]
        //if sholdnt be read then return FF 
    }
    fn writemem(&mut self, address:u16,int:u8){
        self.memory[address as usize] = int;
        //if memeroy should not be written to then dont
    }

    fn flag(&mut self,flag:u8,flaginput:bool){
        //take in flag input store bool
        if flaginput = true{
            match flag{
                0x04 =>self.f = self.f | 0b00001000,//carry flag
                0x05 =>self.f = self.f | 0b00010000,//half carry
                0x06 =>self.f = self.f | 0b00100000,//subtracton
                0x07 =>self.f = self.f | 0b01000000,//zero flag
                __ =>{println!("how did we get here?");}//error
            }}
        else{
            match flag{
                0x04 =>self.f = self.f & 0b11110111,//carry flag
                0x05 =>self.f = self.f & 0b11101111,//half carry
                0x06 =>self.f = self.f & 0b11011111,//subtracton
                0x07 =>self.f = self.f & 0b10111111,//zero flag
                __ =>{println!("how did we get here?");}//error
            }}
    }
    fn getflag(&mut self, flag:char) -> bool{
        let mut temp:u8 =0x00;
        match flag{
            C =>{temp = 0b00001000 & self.f;}
            H =>{temp = 0b00010000 & self.f;}
            N =>{temp = 0b00100000 & self.f;}
            Z =>{temp = 0b01000000 & self.f;}
            __ =>{println!("what kind of flag are you looking for?");}
        }
        if temp = 0x00{
            return false
        }
        else{
            return true
        }
    }

    fn alu_add(&mut self, b: u8, usec: bool) {
        let c = if usec && self.getflag(C) { 1 } else { 0 };
        let a = self.a;
        let r = a.wrapping_add(b).wrapping_add(c);
        self.flag(Z, r == 0);
        self.flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
        self.flag(N, false);
        self
            .flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
        self.a = r;
    }

    fn alu_sub(&mut self, b: u8, usec: bool) {
        let c = if usec && self.getflag(C) { 1 } else { 0 };
        let a = self.a;
        let r = a.wrapping_sub(b).wrapping_sub(c);
        self.flag(Z, r == 0);
        self.flag(H, (a & 0x0F) < (b & 0x0F) + c);
        self.flag(N, true);
        self.flag(C, (a as u16) < (b as u16) + (c as u16));
        self.a = r;
    }

    fn alu_and(&mut self, b: u8) {
        let r = self.a & b;
        self.flag(Z, r == 0);
        self.flag(H, true);
        self.flag(C, false);
        self.flag(N, false);
        self.a = r;
    }

    fn alu_or(&mut self, b: u8) {
        let r = self.a | b;
        self.flag(Z, r == 0);
        self.flag(C, false);
        self.flag(H, false);
        self.flag(N, false);
        self.a = r;
    }

    fn alu_xor(&mut self, b: u8) {
        let r = self.a ^ b;
        self.flag(Z, r == 0);
        self.flag(C, false);
        self.flag(H, false);
        self.flag(N, false);
        self.a = r;
    }

    fn alu_cp(&mut self, b: u8) {
        let r = self.a;
        self.alu_sub(b, false);
        self.a = r;
    }

    fn alu_inc(&mut self, a: u8) -> u8 {
        let r = a.wrapping_add(1);
        self.flag(Z, r == 0);
        self.flag(H, (a & 0x0F) + 1 > 0x0F);
        self.flag(N, false);
                    
        return r;
    }

    fn alu_dec(&mut self, a: u8) -> u8 {
        let r = a.wrapping_sub(1);
        self.flag(Z, r == 0);
        self.flag(H, (a & 0x0F) == 0);
        self.flag(N, true);
        return r;
    }

    fn alu_add16(&mut self, b: u16) {
        let a = self.hl();//hl register u16
        let r = a.wrapping_add(b);
        self.flag(H, (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF);
        self.flag(N, false);
        self.flag(C, a > 0xFFFF - b);
        self.sethl(r);
    }

    fn alu_add16imm(&mut self, a: u16) -> u16 {
        let b = self.fetchbyte() as i8 as i16 as u16;
        self.flag(N, false);
        self.flag(Z, false);
        self.flag(H, (a & 0x000F) + (b & 0x000F) > 0x000F);
        self.flag(C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
        return a.wrapping_add(b);
    }

    fn alu_swap(&mut self, a: u8) -> u8 {
        self.flag(Z, a == 0);
        self.flag(C, false);
        self.flag(H, false);
        self.flag(N, false);
        (a >> 4) | (a << 4)
    }

    fn alu_srflagupdate(&mut self, r: u8, c: bool) {
        self.flag(H, false);
        self.flag(N, false);
        self.flag(Z, r == 0);
        self.flag(C, c);
    }

    fn alu_rlc(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if c { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_rl(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if self.getflag(C) { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_rrc(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if c { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_rr(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if self.getflag(C) { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_sla(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = a << 1;
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_sra(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (a & 0x80);
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_srl(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = a >> 1;
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_bit(&mut self, a: u8, b: u8) {
        let r = a & (1 << (b as u32)) == 0;
        self.flag(N, false);
        self.flag(H, true);
        self.flag(Z, r);
    }

    fn alu_daa(&mut self) {
        let mut a = self.a;
        let mut adjust = if self.getflag(C) { 0x60 } else { 0x00 };
        if self.getflag(H) {
            adjust |= 0x06;
        };
        if !self.getflag(N) {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.flag(C, adjust >= 0x60);
        self.flag(H, false);
        self.flag(Z, a == 0);
        self.a = a;
    }

    fn cpu_jr(&mut self) {
        let n = self.fetchbyte() as i8;
        self.pc = ((self.pc as u32 as i32) + (n as i32)) as u16;
    }
    fn graphics(&mut self){
        //add all layers to the window array return array
    }
}
          
