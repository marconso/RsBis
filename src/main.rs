use iced::Element;
use iced::widget::{column, row, PickList};
use iced::widget::{button, text, pick_list};


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
}


impl App {
    fn new() -> Self {
        Self {
            ftp_selector: Ftp::ALL.to_vec(),
            ftp_selected: None,

            sim_selector: Vec::new(),
            sim_selected: None,

            sinan_selector: Vec::new(),
            sinan_selected: None,
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
           ).placeholder("Selecione uma base de dados...");

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
            None => text("Selecione uma base primeiro.").into()
        };

        column![
            ftp_pick,
            db_pick,
            row![
                button("Download").on_press(Message::Download),
                button("Visualizar").on_press(Message::Visualizar),
            ].spacing(10),
        ].padding(10).spacing(10).into()
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
                self.sim_selected = Some(data)
           }
           Message::Sinan(data) => {
                self.sinan_selected = Some(data)
           }
           Message::Download => {
                println!("Download")
            }
           Message::Visualizar => {
                println!("Visualizar")
            }
        }
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
