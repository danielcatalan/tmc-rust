pub trait Register {
    const ADDRESS: u8;
    fn get_address(&self) -> u8;
    fn get_bytes(&self) -> [u8; 4];
    fn from_bytes(data: [u8; 4]) -> Self;
}

pub trait ReadRegister: Register{

}

pub trait WriteRegister: Register{

}

