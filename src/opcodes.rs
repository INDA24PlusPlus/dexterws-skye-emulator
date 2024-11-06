#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub struct OpCode {
    pub oc_type: OpCodeType,
    pub oc_code: u16,
}

pub enum Data {
    NNN(u16),
    XNN(u8, u8),
    XYN(u8, u8, u8),
    XY(u8, u8),
    X(u8),
    Null,
}

impl Data {
    fn nnn(data: u16) -> Self {
        return Self::NNN(data & 0x0FFF);
    }

    fn xnn(data: u16) -> Self {
        return Self::XNN(((data & 0x0F00) >> 8) as u8, (data & 0x00FF) as u8);
    }

    fn xyn(data: u16) -> Self {
        return Self::XYN(((data & 0x0F00) >> 8) as u8, ((data & 0x00F0) >> 4) as u8, (data & 0x000F) as u8);
    }

    fn xy(data: u16) -> Self {
        return Self::XY(((data & 0x0F00) >> 8) as u8, ((data & 0x00F0) >> 4) as u8);
    }

    fn x(data: u16) -> Self {
        return Self::X(((data & 0x0F00) >> 8) as u8);
    }

    fn null() -> Self {
        return Self::Null;
    }
}

impl OpCode {
    pub fn get_data(&self) -> Data {
        use self::OpCodeType as OCT;
        match self.oc_type {
            OCT::CALL(0) | OCT::FLOW(1) | OCT::FLOW(2) | OCT::MEM(0) | OCT::FLOW(4)  => Data::nnn(self.oc_code),
            OCT::COND(0) | OCT::COND(1) | OCT::CONST(0) | OCT::CONST(1) | OCT::RAND(0) => Data::xnn(self.oc_code),
            OCT::DISPLAY(1) => Data::xyn(self.oc_code),
            OCT::COND(2) | OCT::ASSIG(0) | OCT::BITOP(0) | OCT::BITOP(1) | OCT::BITOP(2) | OCT::MATH(0) | OCT::MATH(1) | OCT::BITOP(4) | OCT::MATH(2) | OCT::BITOP(8) | OCT::COND(4) => Data::xy(self.oc_code),
            OCT::KEYOP(0) | OCT::KEYOP(1) | OCT::TIMER(0) | OCT::KEYOP(2) | OCT::TIMER(1) | OCT::SOUND(0) | OCT::MEM(1) | OCT::MEM(2) | OCT::BCD(0) | OCT::MEM(4) | OCT::MEM(8) => Data::x(self.oc_code),
            _ => Data::null(),
        }
    }
}
