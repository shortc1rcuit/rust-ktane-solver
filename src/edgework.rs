#[derive(Default)]
pub struct Edgework<'a> {
    pub batteries: u32,
    pub holders: u32,
    pub indicators: Vec<Indicator>,
    pub ports: Vec<Vec<Ports>>,
    pub serial_num: &'a str,
}

#[derive(Default)]
pub struct Indicator {
    pub label: Label,
    pub lit: bool,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, PartialEq)]
pub enum Label {
    #[default]
    BOB,
    CAR,
    CLR,
    FRK,
    FRQ,
    IND,
    MSA,
    NSA,
    SIG,
    SND,
    TRN,
}

pub enum Ports {
    DVI,
    Parallel,
    PS2,
    RJ45,
    Serial,
    RCA,
}
