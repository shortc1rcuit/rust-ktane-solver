#[derive(Default)]
pub struct Edgework {
    pub batteries: u32,
    pub holders: u32,
    pub indicators: Vec<Indicator>,
    pub ports: Vec<Vec<Port>>,
    pub serial_num: String,
}

#[derive(Default, PartialEq)]
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
pub enum Port {
    DVI,
    Parallel,
    PS2,
    RCA,
    RJ45,
    Serial,
}

impl Edgework {
    pub fn label_exists(&self, label: Label) -> bool {
        self.indicators.iter().any(|ind| ind.label == label)
    }

    pub fn indicator_exists(&self, label: Label, lit: bool) -> bool {
        let indicator = Indicator { label, lit };
        self.indicators.contains(&indicator)
    }

    pub fn last_digit_serial(&self) -> u32 {
        let last = self.serial_num.chars().nth(5).unwrap();
        last.to_digit(10).unwrap()
    }
}
