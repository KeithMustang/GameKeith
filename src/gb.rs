use std::fs;
use minifb::{Key, Window, WindowOptions};
pub struct GameKeith{
    pub memory: [u8;0xFFFF],
    //0000 3FFF 16KiB ROM bank 00 
    //4000 7FFF 16KiB ROM bank 01-NN
    //8000 9FFF 8Kib VRAM
    //A000 BFFF 8kib cartridge RAM 
    //
    pub af:u16,
    pub bc:u16,
    pub de:u16,
    pub hl:u16,
    pub sp:u16,
    pub pc:u16,
    pub instruction:u16,
    pub rom:Vec<u8>
}
impl GameKeith{
    fn cpuloop(&mut self){
        let mut instruction :u8 = self.memory[self.pc as usize];
        //run match statement in other file
    }
    fn operand(&mut self) ->bool {
        match self.instruction{
            0x00 => {self.pc = self.pc +1;},
            0x01 => {},
            ___ => false
        }; 

        false
    }
    fn graphics(&mut self){
        //add all layers to the window array return array
    }
}
          
