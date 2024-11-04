#[cfg(test)]
mod tests {
    use crate::opcodes::*;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    
    #[test]
    fn test_line() {
        let oc:OpCode=parse_oc("0FFF".to_owned());
        assert_eq!((oc.op_code,oc.oc_type,oc.oc_id),(0x0FFF,OpCodeType::CALL(1),OpCodeIdentity::CallMach));
    }
    #[test]
    fn test_text() {
        let ocs:Vec<OpCode>=parse_text("0FFF\n0222\nF355\n8AB3".to_owned());
        let cmpvec:Vec<OpCode>=vec![
            OpCode{oc_type:OpCodeType::CALL(1), oc_id:OpCodeIdentity::CallMach, op_code:0x0FFF},
            OpCode{oc_type:OpCodeType::CALL(1), oc_id:OpCodeIdentity::CallMach, op_code:0x0222},
            OpCode{oc_type:OpCodeType::MEM(16), oc_id:OpCodeIdentity::DumpRegsToMemR, op_code:0xF355},
            OpCode{oc_type:OpCodeType::BITOP(4), oc_id:OpCodeIdentity::XorRR, op_code:0x8AB3}
        ];
        assert_eq!(ocs,cmpvec);
    }


}
