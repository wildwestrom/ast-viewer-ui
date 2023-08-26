use parse_rust::syn;
use parse_rust::syn::__private::ToTokens;
use std::{path::PathBuf, str::FromStr};

use anyhow::{ensure, Ok, Result};
use iced::{
	executor,
	widget::{button, column, scrollable, text},
	Application, Command, Element, Settings, Theme,
};
use rfd::FileDialog;

type Ast = syn::File;

#[derive(Debug, Clone, Copy)]
enum Message {
	FileLoaded,
}

struct MainView {
	current_file: Option<PathBuf>,
	ast: Option<Ast>,
}

fn ast_from_path(file: &PathBuf) -> Result<Ast> {
	ensure!(file.is_file(), "{} is not a file.", file.to_string_lossy());
	let contents = std::fs::read_to_string(file)?;
	let ast = syn::parse_file(&contents)?;
	Ok(ast)
}

impl Application for MainView {
	type Executor = executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = Theme;

	fn new(_flags: ()) -> (Self, Command<Message>) {
		let test_default_path = PathBuf::from_str("ui/src/main.rs").unwrap();
		let test_default_ast = ast_from_path(&test_default_path).ok();
		(
			Self {
				current_file: Some(test_default_path),
				ast: test_default_ast,
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		String::from("AST Viewer UI")
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::FileLoaded => {
				self.current_file = FileDialog::new().set_directory(".").pick_file();
				if let Some(file) = &self.current_file {
					let ast = ast_from_path(file).ok();
					self.ast = ast;
				}
			},
		}
		Command::none()
	}

	fn view(&self) -> Element<Message> {
		let bold_font: iced::Font = iced::Font {
			weight: iced::font::Weight::Bold,
			family: iced::font::Family::default(),
			stretch: iced::font::Stretch::default(),
			monospaced: false,
		};

		let title = text("AST Viewer UI").into();
		let loadbtn = button("Load File").on_press(Message::FileLoaded).into();
		let curr_file_disp = text(match &self.current_file {
			Some(f) => f.to_string_lossy(),
			None => "No file loaded".into(),
		})
		.into();

		column(vec![title, loadbtn, curr_file_disp, {
			scrollable(if let Some(ast) = &self.ast {
				let mut col = vec![];
				if let Some(shebang) = &ast.shebang {
					col.push(text(shebang.clone()).into());
				}

				let mut attrs = vec![text("Attributes: ").font(bold_font).into()];
				ast.attrs.clone().iter().for_each(|attr| {
					attrs.push(text(attr.to_token_stream()).into());
				});

				col.push(column(attrs).into());

				let mut items = vec![text("Items: ").font(bold_font).into()];
				ast.items.iter().for_each(|item| {
					use syn::Item;
					items.push(
						text(match item {
							Item::Fn(funcitem) => funcitem.sig.to_token_stream().to_string(),
							Item::ExternCrate(exitem) => {
								format!("extern crate {} ", exitem.ident)
							},
							Item::Use(useitem) => {
								format!(
									"use {}{}",
									match useitem.leading_colon {
										Some(_) => "::",
										None => "",
									},
									useitem.tree.to_token_stream()
								)
							},
							Item::Mod(moddecl) => {
								format!("{} mod {}", moddecl.vis.to_token_stream(), moddecl.ident)
							},
							Item::Type(typeitem) => typeitem.to_token_stream().to_string(),
							_ => {
								eprintln!("{:?}", item);
								format!("{:#?}", item)
							},
						})
						.into(),
					);
				});

				col.push(column(items).into());

				column(col)
			} else {
				column(vec![text("No File Loaded").into()])
			})
			.direction(iced::widget::scrollable::Direction::Both {
				vertical: iced::widget::scrollable::Properties::new(),
				horizontal: iced::widget::scrollable::Properties::new(),
			})
			.width(iced::Length::Fill)
			.into()
		}])
		.spacing(6)
		.into()
	}
}

fn main() -> Result<()> {
	MainView::run(Settings::default())?;
	Ok(())
}
