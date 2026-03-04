pub mod region {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Estadual {
        AC,
        AL,
        AP,
        AM,
        BA,
        CE,
        DF,
        ES,
        GO,
        MA,
        MT,
        MS,
        MG,
        PA,
        PB,
        PR,
        PE,
        PI,
        RJ,
        RN,
        RS,
        RO,
        RR,
        SC,
        SP,
        SE,
        TO,
    }


    impl Estadual {
        pub const ALL: [Estadual; 27] = [
            Estadual::AC,
            Estadual::AL,
            Estadual::AP,
            Estadual::AM,
            Estadual::BA,
            Estadual::CE,
            Estadual::DF,
            Estadual::ES,
            Estadual::GO,
            Estadual::MA,
            Estadual::MT,
            Estadual::MS,
            Estadual::MG,
            Estadual::PA,
            Estadual::PB,
            Estadual::PR,
            Estadual::PE,
            Estadual::PI,
            Estadual::RJ,
            Estadual::RN,
            Estadual::RS,
            Estadual::RO,
            Estadual::RR,
            Estadual::SC,
            Estadual::SP,
            Estadual::SE,
            Estadual::TO,
        ];
    }


    impl std::fmt::Display for Estadual {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Estadual::AC => "AC",
                Estadual::AL => "AL",
                Estadual::AP => "AP",
                Estadual::AM => "AM",
                Estadual::BA => "BA",
                Estadual::CE => "CE",
                Estadual::DF => "DF",
                Estadual::ES => "ES",
                Estadual::GO => "GO",
                Estadual::MA => "MA",
                Estadual::MT => "MT",
                Estadual::MS => "MS",
                Estadual::MG => "MG",
                Estadual::PA => "PA",
                Estadual::PB => "PB",
                Estadual::PR => "PR",
                Estadual::PE => "PE",
                Estadual::PI => "PI",
                Estadual::RJ => "RJ",
                Estadual::RN => "RN",
                Estadual::RS => "RS",
                Estadual::RO => "RO",
                Estadual::RR => "RR",
                Estadual::SC => "SC",
                Estadual::SP => "SP",
                Estadual::SE => "SE",
                Estadual::TO => "TO",
            })
        }
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Regional {
        Norte,
        Nordeste,
        Sul,
        Sudeste,
        CentroOeste,
    }

    impl Regional {
        pub const ALL: [Regional; 5] = [
            Regional::Norte,
            Regional::Nordeste,
            Regional::Sul,
            Regional::Sudeste,
            Regional::CentroOeste
        ];
    }

    impl std::fmt::Display for Regional {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Regional::Norte => "Norte",
                Regional::Nordeste => "Nordeste",
                Regional::Sul => "Sul",
                Regional::Sudeste => "Sudeste",
                Regional::CentroOeste => "Centro-Oeste",
            })
        }
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Nacional {
        Brasil
    }

    impl std::fmt::Display for Nacional {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Brasil")
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionItem { }


impl std::fmt::Display for OptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sim {
    NascidosVivos,
    NascidosMortos,
}

impl Sim {
    pub const ALL: [Sim; 2] = [Sim::NascidosMortos, Sim::NascidosVivos];
}

impl std::fmt::Display for Sim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Sim::NascidosVivos => "Nascidos Vivos",
            Sim::NascidosMortos => "Nascidos Mortos"
        } )
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sinan {
    AnimaisPenconhentos,
    AcidenteDeTrabalho,
    Toxinas,
}

impl Sinan {
    pub const ALL: [Sinan; 3] = [
        Sinan::AcidenteDeTrabalho,
        Sinan::AnimaisPenconhentos,
        Sinan::Toxinas
    ];
}

impl std::fmt::Display for Sinan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Sinan::AcidenteDeTrabalho => "Acidente de Trabalho",
            Sinan::AnimaisPenconhentos => "Animais Penconhentos",
            Sinan::Toxinas => "Toxinas"
        } )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ftp {
    Sim,
    Sinan,
}


impl Ftp {
    pub const ALL: [Ftp; 2] = [Ftp::Sim, Ftp::Sinan];
}


impl std::fmt::Display for Ftp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Ftp::Sim => "Sim",
            Ftp::Sinan => "Sinan"
        })
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegSeletor {
    Nacional,
    Estadual,
    Regional,
}

impl RegSeletor {
    pub const ALL: [RegSeletor; 3] = [
        RegSeletor::Nacional,
        RegSeletor::Estadual,
        RegSeletor::Regional
    ];
}

impl std::fmt::Display for RegSeletor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RegSeletor::Nacional => "Nacional",
            RegSeletor::Estadual => "Estadual",
            RegSeletor::Regional => "Regional",
        })
    }
}
