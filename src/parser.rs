use core::panic;
use std::{fs, vec};
///Type of operation that code represents
#[derive( Debug, PartialEq, Eq )]
pub enum OpCodeType{
    CALL(u8),       //Responsible for calling machine code functions
    DISPLAY(u8),    //Manages the display
    FLOW(u8),       //Manages flow through jumps and line skips
    COND(u8),       //Evaluates conditional statements
    CONST(u8),      //Constant values
    ASSIG(u8),      //Register assignment
    BITOP(u8),      //Bit operations
    MATH(u8),       //Arithmetic
    MEM(u8),        //Memory management
    RAND(u8),       //PRNG
    KEYOP(u8),      //User Input
    TIMER(u8),      //Delay program
    SOUND(u8),      //Play sound
    BCD(u8),        //Fancy BCD things
}

///Specific op code identity
#[derive( Debug, PartialEq, Eq )]
pub enum OpCodeIdentity{
    CallMach,           //Call machine code routines
    ClrDisp,            //Clear display
    RetSub,             //Return from subroutine
    JumpAddr,           //Jump to addr
    CallSub,            //Call subroutine
    SkipEqRC,           //Skip if reg equal to const
    SkipNqRC,           //Skip if reg not equal to const
    SkipEqRR,           //Skip if reg equal to reg
    SetRC,              //Store const to reg
    AddNcRC,            //Add const to reg no carry flag
    SetRR,              //Set reg to reg
    OrRR,               //Bw or between regs
    AndRR,              //Bw and between regs
    XorRR,              //Bw xor between regs
    AddRR,              //Add reg to reg
    SubRRR,             //Sub reg from reg
    RshiftR,            //Rshift reg by 1
    SubLRR,             //Sub reg from reg but other order
    LshiftR,            //Lshift reg by 1
    SkipNqRR,           //Skip if reg not equal reg
    SetAddrRegC,        //Set addr reg to const
    JumpAddrCR,         //Jump to const + offset
    RandRC,             //PRNG
    DrawDispRRC,        //Draw sprite to disp
    SkipKeyPressedR,    //Skip if key stored in reg is pressed
    SkipNKeyPressedR,   //Skip if key stored in reg isn't pressed
    GetDelayR,          //Get value in delay timer
    AwaitGetKeyDownR,   //Wait for key then store in reg
    SetDelayR,          //Set delay timer to reg
    SetSoundR,          //Set sound timer to reg
    AddAddrRegR,        //Add reg to addr reg
    SetAddrRegSpriteR,  //Set addr reg to sprite for char in reg
    SetBcdR,            //BCD things
    DumpRegsToMemR,     //Dump V0-Reg to mem at addr const
    LoadRegsFromMemR,   //Load V0-reg from mem from addr const
}
///Holds an op code and metadata for it
#[derive( Debug, PartialEq, Eq )]
pub struct OpCode {
    pub oc_type:OpCodeType,
    pub oc_id:OpCodeIdentity,
    pub op_code:u16
}
fn check_op_code(op_code:&String)->bool{
   if (*op_code).len()!=4{
        return false;
    }
   for symbol in (*op_code).chars() {
       match symbol{
            '0'       => "",
            '1'       => "",
            '2'       => "",
            '3'       => "",
            '4'       => "",
            '5'       => "",
            '6'       => "",
            '7'       => "",
            '8'       => "",
            '9'       => "",
            'A' | 'a' => "",
            'B' | 'b' => "",
            'C' | 'c' => "",
            'D' | 'd' => "",
            'E' | 'e' => "",
            'F' | 'f' => "",
            _         => return false,
       };
   }
   return true;
}
fn get_oc_id(op_code:u16)->OpCodeIdentity{
    let w1:u16=(op_code&0xF000)>>12;
    let w2:u16=(op_code&0x0F00)>>8;
    let w3:u16=(op_code&0x00F0)>>4;
    let w4:u16=op_code&0x000F;
    if w1==0x0{
        if w2>=0x2{
            return OpCodeIdentity::CallMach;
        }
        if op_code==0x00E0{
            return OpCodeIdentity::ClrDisp;
        }
        if op_code==0x00EE{
            return OpCodeIdentity::RetSub;
        }
        panic!("");
    }
    if w1==0x1{return OpCodeIdentity::JumpAddr;}
    if w1==0x2{return OpCodeIdentity::CallSub;}
    if w1==0x3{return OpCodeIdentity::SkipEqRC;}
    if w1==0x4{return OpCodeIdentity::SkipNqRC;}
    if w1==0x5{
        if w4==0x0{
            return OpCodeIdentity::SkipEqRR;
        }
        panic!("");
    }
    if w1==0x6{return OpCodeIdentity::SetRC;}
    if w1==0x7{return OpCodeIdentity::AddNcRC;}
    if w1==0x8{
        if w4==0x0{
            return OpCodeIdentity::SetRR;
        }
        if w4==0x1{
            return OpCodeIdentity::OrRR;
        }
        if w4==0x2{
            return OpCodeIdentity::AndRR;
        }
        if w4==0x3{
            return OpCodeIdentity::XorRR;
        }
        if w4==0x4{
            return OpCodeIdentity::AddRR;
        }
        if w4==0x5{
            return OpCodeIdentity::SubRRR;
        }
        if w4==0x6{
            return OpCodeIdentity::RshiftR;
        }
        if w4==0x7{
            return OpCodeIdentity::SubLRR;
        }
        if w4==0xE{
            return OpCodeIdentity::LshiftR;
        }
        panic!("");
    }
    if w1==0x9{
        if w4==0x0{
            return OpCodeIdentity::SkipNqRR;
        }
        panic!("");
    }
    if w1==0xA{
        return OpCodeIdentity::SetAddrRegC;
    }
    if w1==0xB{
        return OpCodeIdentity::JumpAddrCR;
    }
    if w1==0xC{
        return OpCodeIdentity::RandRC;
    }
    if w1==0xD{
        return OpCodeIdentity::DrawDispRRC;
    }
    if w1==0xE{
        if w4==0xE&&w3==0x9{
            return OpCodeIdentity::SkipKeyPressedR;
        }
        if w4==0x1&&w3==0xA{
            return OpCodeIdentity::SkipNKeyPressedR;
        }
        panic!("")
    }
    if w1==0xF{
        let w34=(w3<<4)|w4;
        if w34==0x07{
            return OpCodeIdentity::GetDelayR;
        }
        if w34==0x0A{
            return OpCodeIdentity::AwaitGetKeyDownR;
        }
        if w34==0x15{
            return OpCodeIdentity::SetDelayR;
        }
        if w34==0x18{
            return OpCodeIdentity::SetSoundR;
        }
        if w34==0x1E{
            return OpCodeIdentity::AddAddrRegR;
        }
        if w34==0x29{
            return OpCodeIdentity::SetAddrRegSpriteR;
        }
        if w34==0x33{
            return OpCodeIdentity::SetBcdR;
        }
        if w34==0x55{
            return OpCodeIdentity::DumpRegsToMemR;
        }
        if w34==0x65{
            return OpCodeIdentity::LoadRegsFromMemR;
        }
        panic!("");
    }
    panic!("");
}
fn get_oc_type(op_code:u16)->OpCodeType{
    let w1:u16=(op_code&0xF000)>>12;
    let w2:u16=(op_code&0x0F00)>>8;
    let w3:u16=(op_code&0x00F0)>>4;
    let w4:u16=op_code&0x000F;
    if w1==0x0{
        if w2>=0x2{
            return OpCodeType::CALL(1);
        }
        if op_code==0x00E0{
            return OpCodeType::DISPLAY(1);
        }
        if op_code==0x00EE{
            return OpCodeType::FLOW(1);
        }
        panic!("");
    }
    if w1==0x1{return OpCodeType::FLOW(2);}
    if w1==0x2{return OpCodeType::FLOW(4);}
    if w1==0x3{return OpCodeType::COND(1);}
    if w1==0x4{return OpCodeType::COND(2);}
    if w1==0x5{
        if w4==0x0{
            return OpCodeType::COND(4);
        }
        panic!("");
    }
    if w1==0x6{return OpCodeType::CONST(1);}
    if w1==0x7{return OpCodeType::CONST(2);}
    if w1==0x8{
        if w4==0x0{
            return OpCodeType::ASSIG(1);
        }
        if w4==0x1{
            return OpCodeType::BITOP(1);
        }
        if w4==0x2{
            return OpCodeType::BITOP(2);
        }
        if w4==0x3{
            return OpCodeType::BITOP(4);
        }
        if w4==0x4{
            return OpCodeType::MATH(1);
        }
        if w4==0x5{
            return OpCodeType::MATH(2);
        }
        if w4==0x6{
            return OpCodeType::BITOP(8);
        }
        if w4==0x7{
            return OpCodeType::MATH(4);
        }
        if w4==0xE{
            return OpCodeType::BITOP(16);
        }
        panic!("");
    }
    if w1==0x9{
        if w4==0x0{
            return OpCodeType::COND(8);
        }
        panic!("");
    }
    if w1==0xA{
        return OpCodeType::MEM(1);
    }
    if w1==0xB{
        return OpCodeType::FLOW(8);
    }
    if w1==0xC{
        return OpCodeType::RAND(1);
    }
    if w1==0xD{
        return OpCodeType::DISPLAY(2);
    }
    if w1==0xE{
        if w4==0xE&&w3==0x9{
            return OpCodeType::KEYOP(1);
        }
        if w4==0x1&&w3==0xA{
            return OpCodeType::KEYOP(2);
        }
        panic!("")
    }
    if w1==0xF{
        let w34=(w3<<4)|w4;
        if w34==0x07{
            return OpCodeType::TIMER(1);
        }
        if w34==0x0A{
            return OpCodeType::KEYOP(4);
        }
        if w34==0x15{
            return OpCodeType::TIMER(2);
        }
        if w34==0x18{
            return OpCodeType::SOUND(1);
        }
        if w34==0x1E{
            return OpCodeType::MEM(2);
        }
        if w34==0x29{
            return OpCodeType::MEM(4);
        }
        if w34==0x33{
            return OpCodeType::MEM(8);
        }
        if w34==0x55{
            return OpCodeType::MEM(16);
        }
        if w34==0x65{
            return OpCodeType::MEM(32);
        }
        panic!("");
    }
    panic!("");
}
fn parse_oc(op_code:String)->OpCode{
    if !check_op_code(&op_code){
        panic!("Invalid op code {}, hexadecimal token of length 4\n", op_code);
    }
    let mut oc_val:u16=0b0;
    for (i,symbol) in (*op_code).chars().enumerate() {
        let symb_val:u16=match symbol{
            '0'       => 0x0,
            '1'       => 0x1,
            '2'       => 0x2,
            '3'       => 0x3,
            '4'       => 0x4,
            '5'       => 0x5,
            '6'       => 0x6,
            '7'       => 0x7,
            '8'       => 0x8,
            '9'       => 0x9,
            'A' | 'a' => 0xA,
            'B' | 'b' => 0xB,
            'C' | 'c' => 0xC,
            'D' | 'd' => 0xD,
            'E' | 'e' => 0xE,
            'F' | 'f' => 0xF,
            _ => 0x0,
        };
        oc_val|=symb_val<<(4*(3-i));
    }
    println!("{:#x}",oc_val);
    return OpCode { oc_type: get_oc_type(oc_val), oc_id:get_oc_id(oc_val), op_code: oc_val };
}
pub fn parse_file(fp:String)->Vec<OpCode>{
    let contents=fs::read_to_string(fp).expect("Error");
    let code_lines=contents.lines();
    let mut lns:Vec<OpCode>=Vec::new();
    for line in code_lines{
        lns.push(parse_oc(line.to_string()));
        println!("{}\n",line);
    }
    return lns;
}
pub fn parse_text(text:String)->Vec<OpCode>{
    let code_lines=text.lines();
    let mut lns:Vec<OpCode>=Vec::new();
    for line in code_lines{
        lns.push(parse_oc(line.to_string()));
        println!("{}\n",line);
    }
    return lns;
}
