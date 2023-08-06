use iced::{
	widget::{component, scrollable, text, Component},
	Element, Length, Renderer,
};

type Ast = syn::File;

pub struct AstView<Message> {
	text_representation: String,
	on_change: Box<dyn Fn() -> Message>,
}

pub fn ast_view<Message>(
	ast: Option<Ast>,
	on_change: impl Fn() -> Message + 'static,
) -> AstView<Message> {
	AstView::new(&ast, on_change)
}

#[derive(Debug, Clone)]
pub enum Event {
	AddAst,
}

impl<Message> AstView<Message> {
	pub fn new(ast: &Option<Ast>, on_change: impl Fn() -> Message + 'static) -> Self {
		Self {
			text_representation: match ast {
				Some(ast) => format!("{:#?}", ast),
				None => "No Ast Yet".into(),
			},
			on_change: Box::new(on_change),
		}
	}
}

impl<Message> Component<Message, Renderer> for AstView<Message> {
	type Event = Event;
	type State = ();

	fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
		match event {
			_ => None,
		}
	}

	fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
		scrollable(text(self.text_representation.clone()))
			.width(Length::Fill)
			.into()
	}
}

impl<'a, Message> From<AstView<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
{
	fn from(ast_view: AstView<Message>) -> Self {
		component(ast_view)
	}
}
