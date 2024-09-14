use iced::widget::row;
use iced::{Length, Point};
use iced_node_editor::{connection, graph_container, node, Matrix};

use iced_runtime::command::Action;

use parse_rust::syn::__private::ToTokens;
use parse_rust::syn::{self, Item, ItemType};
use std::{path::PathBuf, str::FromStr};

use anyhow::{ensure, Ok, Result};
use iced::{
	executor,
	widget::{button, column, scrollable, text},
	Application, Command, Element, Settings, Theme,
};
use rfd::FileDialog;

type Ast = syn::File;

const BOLD_FONT: iced::Font = {
	let mut font = iced::Font::DEFAULT;
	font.weight = iced::font::Weight::Bold;
	font
};

const MONOSPACE_FONT: iced::Font = iced::Font::MONOSPACE;

#[derive(Debug, Clone)]
enum Message {
	LoadFile,
	FileLoaded(PathBuf),
	ScaleChanged(f32, f32, f32),
	TranslationChanged(f32, f32),
	MoveNode(usize, f32, f32),
	ToggleTextView,
}

impl From<Message> for Action<Message> {
	fn from(val: Message) -> Self {
		Action::Future(Box::pin(async { val }))
	}
}
struct NodeState {
	position: Point,
	text: String,
}

struct MainView {
	current_file: Option<PathBuf>,
	ast: Option<Ast>,
	matrix: Matrix,
	nodes: Vec<NodeState>,
	connections: Vec<(usize, usize)>,
	text_view: bool,
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
		let test_default_path = PathBuf::from_str("./ui/src/main.rs").unwrap();

		(
			Self {
				current_file: Some(test_default_path.clone()),
				ast: None,
				matrix: Matrix::identity(),
				nodes: Vec::new(),
				connections: Vec::new(),
				text_view: false,
			},
			Command::single(Message::FileLoaded(test_default_path).into()),
		)
	}

	fn title(&self) -> String {
		String::from("AST Viewer UI")
	}

	fn update(&mut self, _message: Message) -> Command<Message> {
		match _message {
			Message::LoadFile => {
				self.current_file = FileDialog::new().set_directory(".").pick_file();
				if let Some(path) = &self.current_file {
					return Command::single(Message::FileLoaded(path.to_path_buf()).into());
				}
			},
			Message::FileLoaded(path) => {
				self.ast = ast_from_path(&path).ok();
				if let Some(ast) = &self.ast {
					let mut x = 0;
					let mut y = 0;
					ast.items.iter().enumerate().for_each(|(idx, item)| {
						// Just chuck 'em all in a grid for now
						if x == 5 {
							x = 0
						}
						self.nodes.push(NodeState {
							position: Point::new((x * 225) as f32, (y * 100) as f32),
							text: item_to_string(item),
						});
						x += 1;
						y = idx / 4;
					})
				}
			},
			Message::ScaleChanged(x, y, scale) => {
				self.matrix = self
					.matrix
					.translate(-x, -y)
					.scale(if scale > 0.0 { 1.2 } else { 1.0 / 1.2 })
					.translate(x, y);
			},
			Message::TranslationChanged(x, y) => self.matrix = self.matrix.translate(x, y),
			Message::MoveNode(i, x, y) => {
				self.nodes[i].position = Point::new(
					self.nodes[i].position.x + x / self.matrix.get_scale(),
					self.nodes[i].position.y + y / self.matrix.get_scale(),
				);
			},
			Message::ToggleTextView => {
				self.text_view = !self.text_view;
			},
		}
		Command::none()
	}

	fn view(&self) -> Element<Message> {
		let title = text("AST Viewer UI").into();

		let loadbtn = button("Load File").on_press(Message::LoadFile).into();
		let text_view_toggle = button(match &self.text_view {
			true => "Hide Text View",
			false => "Show Text View",
		})
		.on_press(Message::ToggleTextView)
		.into();

		let file_path_display = text(match &self.current_file {
			Some(f) => f.to_string_lossy(),
			None => "No file loaded".into(),
		})
		.into();

		let text_view = scrollable(if let Some(ast) = &self.ast {
			let mut col = vec![];
			if let Some(shebang) = &ast.shebang {
				col.push(text(shebang.clone()).into());
			}

			let mut attrs = vec![text("Attributes: ").font(BOLD_FONT).into()];
			ast.attrs.clone().iter().for_each(|attr| {
				attrs.push(text(attr.to_token_stream()).into());
			});

			col.push(column(attrs).into());

			let mut items = vec![text("Items: ").font(BOLD_FONT).into()];
			ast.items.iter().for_each(|item| {
				items.push(text(item_to_string(item)).font(MONOSPACE_FONT).into());
			});

			col.push(column(items).into());

			column(col)
		} else {
			column(vec![text("No File Loaded").into()])
		})
		.direction(iced::widget::scrollable::Direction::Vertical(
			iced::widget::scrollable::Properties::new(),
		))
		.width(Length::Fill)
		.height(Length::Fill);

		let mut graph_content = Vec::new();

		for (i, n) in self.nodes.iter().enumerate() {
			graph_content.push(
				node(text(&n.text))
					.center_x()
					.center_y()
					.on_translate(move |p| Message::MoveNode(i, p.0, p.1))
					.width(Length::Fixed(200.0))
					.height(Length::Fixed(75.0))
					.position(n.position)
					.into(),
			);
		}

		for (_i, c) in self.connections.iter().enumerate() {
			graph_content.push(
				connection(
					Point::new(
						self.nodes[c.0].position.x + 200.0,
						self.nodes[c.0].position.y + 37.5,
					),
					Point::new(
						self.nodes[c.1].position.x,
						self.nodes[c.1].position.y + 37.5,
					),
				)
				.into(),
			);
		}

		let help_menu = row(vec![loadbtn, text_view_toggle]).spacing(6).into();

		let mut column_content = vec![title, help_menu, file_path_display];

		if self.text_view {
			column_content.push(text_view.into());
		};

		column([
			column(column_content).spacing(6).into(),
			graph_container(graph_content)
				.on_translate(|p| Message::TranslationChanged(p.0, p.1))
				.on_scale(|x, y, s| Message::ScaleChanged(x, y, s))
				.width(Length::Fill)
				.height(Length::Fill)
				.matrix(self.matrix)
				.into(),
		])
		.into()
	}
}

fn item_to_string(item: &Item) -> String {
	match item {
		Item::Fn(funcitem) => {
			format!("{:#?}", funcitem)
		},
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
		Item::Type(typeitem) => {
			// format!("{:#?}", typeitem)
			type_to_string(&typeitem)
		},
		Item::Enum(typeenum) => typeenum.to_token_stream().to_string(),
		Item::Struct(typestruct) => typestruct.to_token_stream().to_string(),
		Item::Impl(typeimpl) => {
			typeimpl.to_token_stream().to_string()
		},
		Item::Const(typeconst) => typeconst.to_token_stream().to_string(),
		_ => {
			eprintln!("{:?}", item);
			format!("{:#?}", item)
		},
	}
}

fn type_to_string(typ: &ItemType) -> String {
	typ.to_token_stream().to_string()
}

fn main() -> Result<()> {
	MainView::run(Settings::default())?;
	Ok(())
}
