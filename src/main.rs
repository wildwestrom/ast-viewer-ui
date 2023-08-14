use std::{path::PathBuf, str::FromStr};

use anyhow::{ensure, Ok, Result};
use iced::{
	executor,
	widget::{scrollable,button, column, text},
	Application, Command, Element, Length, Settings, Theme,
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
	ast_text_representation: String,
}

fn ast_from_path(file: &PathBuf) -> Result<Ast> {
	ensure!(file.is_file(), "{} is not a file.", file.to_string_lossy());
	let contents = std::fs::read_to_string(file)?;
	let ast = syn::parse_file(&contents)?;
	Ok(ast)
}

fn map_ast_to_text(maybe_ast: Option<Ast>) -> String {
	match maybe_ast {
		Some(ast) => format!("{:#?}", ast),
		None => "No Ast Yet".into(),
	}
}

impl Application for MainView {
	type Executor = executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = Theme;

	fn new(_flags: ()) -> (Self, Command<Message>) {
		let test_default_path = PathBuf::from_str("./test-inputs/quicksort.rs").unwrap();
		let test_default_ast = ast_from_path(&test_default_path).ok();
		(
			Self {
				current_file: Some(test_default_path),
				ast: test_default_ast.clone(),
				ast_text_representation: map_ast_to_text(test_default_ast),
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
					self.ast = ast.clone();
					self.ast_text_representation = map_ast_to_text(ast);
				}
			},
		}
		Command::none()
	}

	fn view(&self) -> Element<Message> {
		let title = text("AST Viewer UI").into();
		let loadbtn = button("Load File").on_press(Message::FileLoaded).into();
		let curr_file_disp = text(match &self.current_file {
			Some(f) => f.to_string_lossy(),
			None => "No file loaded".into(),
		})
		.into();

		let ast_view = scrollable(text(self.ast_text_representation.clone()))
			.width(Length::Fill)
			.height(Length::Fill)
			.into();
	

		column(vec![title, loadbtn, curr_file_disp, ast_view])
			.width(Length::Fill)
			.into()
	}
}

fn main() -> Result<()> {
	MainView::run(Settings::default())?;
	Ok(())
}