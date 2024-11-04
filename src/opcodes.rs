use core::panic;
pub enum OpCodeType{
    CALL(u8),
    DISPLAY(u8),
    FLOW(u8),
    COND(u8),
    CONST(u8),
    ASSIG(u8),
    BITOP(u8),
    MATH(u8),
    MEM(u8),
    RAND(u8),
    KEYOP(u8),
    TIMER(u8),
    SOUND(u8),
    BCD(u8),
}
pub struct OpCode {
    oc_type:OpCodeType,
    op_code:u16
}
pub fn check_op_code(op_code:&String)->bool{
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
pub fn parse_oc(op_code:String)->OpCode{
    if !check_op_code(&op_code){
        panic!("Invalid op code {}, hexadecimal token of length 4\n", op_code);
    }
    let oc_val:u16=0b0;
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

   }
    panic!("");
}

