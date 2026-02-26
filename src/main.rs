use iced::{Element, Length};
use iced::widget::{column, row, PickList};
use iced::widget::{button, table, text, pick_list, progress_bar};


fn main() -> iced::Result {
    iced::application(
        App::new,
        App::update,
        App::view,
    ).title(App::title).run()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Download,
    Visualizar,
    OptionItem(OptionItem),
    RegSeletor(RegSeletor),
    Nacional(Nacional),
    Estadual(Estadual),
    Regional(Regional),
    SelectFtp(Ftp),
    Sim(Sim),
    Sinan(Sinan),
}


struct App {
    ftp_selector: Vec<Ftp>,
    ftp_selected: Option<Ftp>,

    sim_selector: Vec<Sim>,
    sim_selected: Option<Sim>,

    sinan_selector: Vec<Sinan>,
    sinan_selected: Option<Sinan>,

    reg_selected: Option<RegSeletor>,
    est_selected: Option<Estadual>,
    reg_sel: Option<Regional>,


    progress: f32,
}


impl App{
    fn new() -> Self {
        Self {
            ftp_selector: Ftp::ALL.to_vec(),
            ftp_selected: None,

            sim_selector: Vec::new(),
            sim_selected: None,

            sinan_selector: Vec::new(),
            sinan_selected: None,

            reg_selected: None,
            est_selected: None,

            reg_sel: None,

            progress: 50.0,
        }
    }

    fn title(&self) -> String {
        String::from("RsBis")
    }

    fn view(&self) -> Element<'_, Message> {
        let ftp_pick: PickList<_, _, _, _> = pick_list(
               self.ftp_selector.clone(),
               self.ftp_selected,
               Message::SelectFtp
           ).placeholder("SIM|SINAN|SINASC");

        let db_pick: Element<'_, Message> = match self.ftp_selected {
            Some(Ftp::Sim) => {
                pick_list(
                   self.sim_selector.clone(),
                   self.sim_selected,
                   Message::Sim
                ).into()
            }
            Some(Ftp::Sinan) => {
                pick_list(
                   self.sinan_selector.clone(),
                   self.sinan_selected,
                   Message::Sinan
                ).into()
            }
            None =>  {
                let empyt_opt: &[OptionItem] = &[];

                pick_list(
                    empyt_opt,
                    None::<OptionItem>,
                    Message::OptionItem
                ).placeholder("Nada Selecionado").into()
            }
        };

        let pick_reg_selector: PickList<'_, RegSeletor, Vec<RegSeletor>, RegSeletor, Message> = pick_list(
            RegSeletor::ALL.to_vec(),
            self.reg_selected,
            Message::RegSeletor
        ).placeholder("Nacional|Estadual|Regional");

        let pick_subreg: Element<'_, Message> = match self.reg_selected {
            Some(RegSeletor::Nacional) => {
                let nat: &[Nacional] = &[];
                pick_list(
                   nat,
                   Some(Nacional::Brasil),
                   Message::Nacional
                ).placeholder("BRASIL").into()
            }
            Some(RegSeletor::Estadual) => {
                let est = Estadual::ALL.to_vec();
                pick_list(
                   est,
                   self.est_selected,
                   Message::Estadual
                ).placeholder("SELECIONE A UF").into()
            }
            Some(RegSeletor::Regional) => {
                let reg = Regional::ALL.to_vec();
                pick_list(
                   reg,
                   self.reg_sel,
                   Message::Regional
                ).placeholder("SELECIONE A REGIÃO").into()
            }
            None =>  {
                let empyt_opt: &[OptionItem] = &[];

                pick_list(
                    empyt_opt,
                    None::<OptionItem>,
                    Message::OptionItem
                ).placeholder("Nada Selecionado").into()
            }
        };

        column![
            row![
                column![
                    ftp_pick,
                    db_pick,
                ].padding(10).spacing(10),

                column![
                    pick_reg_selector,
                    pick_subreg,
                ].padding(10).spacing(10),
            ],
                row![
                    button("Baixar dados").on_press(Message::Download),
                    button("Pré-visualizar").on_press(Message::Visualizar),
                ].padding(10).spacing(10),

            row![
                progress_bar(0.0..=100.0, self.progress).length(Length::FillPortion(1)),
            ].padding(10).spacing(10)
        ].into()
    }

    fn update(&mut self, message: Message) {
        match message {
           Message::SelectFtp(data) => {
                self.ftp_selected = Some(data);
                if data == Ftp::Sim {
                    self.sim_selector.clear();
                    self.sim_selector = Sim::ALL.to_vec();
                    self.sim_selected = Some(Sim::ALL[0]);
                    self.sinan_selector.clear();
                    self.sinan_selected = None;
                } else if data == Ftp::Sinan {
                    self.sinan_selector.clear();
                    self.sinan_selector = Sinan::ALL.to_vec();
                    self.sinan_selected = Some(Sinan::ALL[0]);
                    self.sim_selector.clear();
                    self.sim_selected = None;
                }
           }
           Message::Sim(data) => {
                self.sim_selected = Some(data);
           }
           Message::Sinan(data) => {
                self.sinan_selected = Some(data);
           }
           Message::Download => {
                if self.sim_selected.is_some() {
                    println!("Baixando {}", self.sim_selected.unwrap())
                } else if self.sinan_selected.is_some() {
                    println!("Baixando {}", self.sinan_selected.unwrap())
                }
           }
           Message::Visualizar => {
                if self.sim_selected.is_some() {
                    println!("Visualizar {}", self.sim_selected.unwrap())
                } else if self.sinan_selected.is_some() {
                    println!("Visualizar {}", self.sinan_selected.unwrap())
                }
           }
           Message::RegSeletor(data) => {
                self.reg_selected = Some(data);
            }

           Message::Nacional(data) => { println!("{}", data) }
           Message::Estadual(data) => {self.est_selected = Some(data);}
           Message::Regional(data) => { self.reg_sel = Some(data)}
           Message::OptionItem(_) => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OptionItem { }


impl std::fmt::Display for OptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sim {
    NascidosVivos,
    NascidosMortos,
}

impl Sim {
    const ALL: [Sim; 2] = [Sim::NascidosMortos, Sim::NascidosVivos];
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
enum Sinan {
    AnimaisPenconhentos,
    AcidenteDeTrabalho,
    Toxinas,
}

impl Sinan {
    const ALL: [Sinan; 3] = [
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
enum Ftp {
    Sim,
    Sinan,
}


impl Ftp {
    const ALL: [Ftp; 2] = [Ftp::Sim, Ftp::Sinan];
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
enum RegSeletor {
    Nacional,
    Estadual,
    Regional,
}

impl RegSeletor {
    const ALL: [RegSeletor; 3] = [
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Nacional {
    Brasil
}

impl std::fmt::Display for Nacional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Brasil")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Regional {
    Norte,
    Nordeste,
    Sul,
    Sudeste,
    CentroOeste,
}

impl Regional {
    const ALL: [Regional; 5] = [
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
enum Estadual {
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
    const ALL: [Estadual; 27] = [
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
