use iced::Length::{Fill, FillPortion};
use iced::widget::{PickList, column, row};
use iced::widget::{button, pick_list, progress_bar, scrollable, table, text, space};
use iced::{Element, Length};
use iced::{Renderer, Theme};

mod helpers;
use helpers::region::{Estadual, Nacional, Regional};
use helpers::{Ftp, OptionItem, RegSeletor, Sim, Sinan};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .window(iced::window::Settings {
            max_size: Some(iced::Size::new(1024.0, 768.0)),
            min_size: Some(iced::Size::new(800.0, 400.0)),
            ..Default::default()
        })
        .run()
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
    rows: Vec<String>,
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

            reg_selected: None,
            est_selected: None,

            reg_sel: None,

            progress: 50.0,
            rows: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("RsBis")
    }

    fn view(&self) -> Element<'_, Message> {
        let ftp_pick: PickList<_, _, _, _> = pick_list(
            self.ftp_selector.clone(),
            self.ftp_selected,
            Message::SelectFtp,
        )
        .placeholder("SIM|SINAN|SINASC")
        .width(Fill);

        let db_pick: Element<'_, Message> = match self.ftp_selected {
            Some(Ftp::Sim) => pick_list(self.sim_selector.clone(), self.sim_selected, Message::Sim)
                .width(Fill)
                .into(),
            Some(Ftp::Sinan) => pick_list(
                self.sinan_selector.clone(),
                self.sinan_selected,
                Message::Sinan,
            )
            .width(Fill)
            .into(),
            None => {
                let empyt_opt: &[OptionItem] = &[];

                pick_list(empyt_opt, None::<OptionItem>, Message::OptionItem)
                    .placeholder("Nada Selecionado")
                    .width(Fill)
                    .into()
            }
        };

        let pick_reg_selector: PickList<'_, RegSeletor, Vec<RegSeletor>, RegSeletor, Message> =
            pick_list(
                RegSeletor::ALL.to_vec(),
                self.reg_selected,
                Message::RegSeletor,
            )
            .placeholder("Nacional|Estadual|Regional")
            .width(Fill);

        let pick_subreg: Element<'_, Message> = match self.reg_selected {
            Some(RegSeletor::Nacional) => {
                let nat: &[Nacional] = &[];
                pick_list(nat, Some(Nacional::Brasil), Message::Nacional)
                    .placeholder("BRASIL")
                    .width(Fill)
                    .into()
            }
            Some(RegSeletor::Estadual) => {
                let est = Estadual::ALL.to_vec();
                pick_list(est, self.est_selected, Message::Estadual)
                    .menu_height(200)
                    .placeholder("SELECIONE A UF")
                    .width(Fill)
                    .into()
            }
            Some(RegSeletor::Regional) => {
                let reg = Regional::ALL.to_vec();
                pick_list(reg, self.reg_sel, Message::Regional)
                    .placeholder("SELECIONE A REGIÃO")
                    .width(Fill)
                    .into()
            }
            None => {
                let empyt_opt: &[OptionItem] = &[];

                pick_list(empyt_opt, None::<OptionItem>, Message::OptionItem)
                    .placeholder("Nada Selecionado")
                    .width(Fill)
                    .into()
            }
        };

        let num_colunas = 50;

        let new_columns: Vec<table::Column<'_, '_, &str, Message, Theme, Renderer>> = (0
            ..num_colunas)
            .map(|i| {
                table::column(text(format!("Coluna {}", i + 1)), move |_row_data: &str| {
                    text(format!("Valor {}", i + 1))
                })
                .width(100.0)
            })
            .collect();

        let mytable: Element<'_, Message> = table(new_columns, vec![""; 50]).into();

        row![
            column![
                row![
                    column![ftp_pick, db_pick, pick_reg_selector, pick_subreg,]
                        .width(Length::FillPortion(1))
                        .padding(20)
                        .spacing(20),
                ],
                row![
                    button(text("Baixar dados").center())
                        .on_press(Message::Download)
                        .width(FillPortion(1)),
                    // space().height(20),
                    button(text("Pré-visualizar").center())
                        .on_press(Message::Visualizar)
                        .width(FillPortion(1)),
                ]
                .padding(20)
                .spacing(20),
                row![progress_bar(0.0..=100.0, self.progress),]
                    .padding(20)
                    .spacing(20),
            ]
            .width(Length::FillPortion(1)),
            column![scrollable(mytable).spacing(20).direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new(),
                horizontal: scrollable::Scrollbar::new()
            }).spacing(20),]
            .padding(8)
            .width(Length::FillPortion(1)),
        ]
        .into()
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

            Message::Nacional(data) => {
                println!("{}", data)
            }
            Message::Estadual(data) => {
                self.est_selected = Some(data);
            }
            Message::Regional(data) => self.reg_sel = Some(data),
            Message::OptionItem(_) => {}
        }
    }
}
