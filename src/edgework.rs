#[derive(Default)]
pub struct Edgework {
    pub batteries: u32,
    pub holders: u32,
    pub indicators: Vec<Indicator>,
    pub ports: Vec<Vec<Ports>>,
    pub serial_num: String,
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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum Ports {
    DVI,
    Parallel,
    PS2,
    RCA,
    RJ45,
    Serial,
}
