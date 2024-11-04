#[cfg(test)]
mod tests {
    use crate::parser::*;

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
