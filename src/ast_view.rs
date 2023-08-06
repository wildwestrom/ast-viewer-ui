use iced::{
	widget::{scrollable, text},
	Element, Length, Renderer,
};

type Ast = syn::File;

pub struct AstView {
	text_representation: String,
}

pub fn ast_view(ast: Option<Ast>) -> AstView {
	AstView::new(ast)
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
	AddAst,
}

impl AstView {
	pub fn new(ast: Option<Ast>) -> Self {
		Self {
			text_representation: match ast {
				Some(ast) => format!("{:#?}", ast),
				None => "No Ast Yet".into(),
			},
		}
	}
}

impl AstView {
	pub fn view(&self) -> Element<Message> {
		scrollable(text(self.text_representation.clone()))
			.width(Length::Fill)
			.height(Length::Fill)
			.into()
	}
}

impl<'a, Message> From<AstView> for Element<'a, Message, Renderer>
where
	Message: 'a,
{
	fn from(ast_view: AstView) -> Self {
		ast_view.into()
	}
}
