use iced::Vector;
use iced_native::event::{self, Event};
use iced_native::layout;
use iced_native::mouse;
use iced_native::overlay;
use iced_native::renderer;
use iced_native::touch;
use iced_native::widget::{tree, Operation, Tree};
use iced_native::{Clipboard, Element, Layout, Length, Point, Rectangle, Shell, Widget};

/// Emit messages on mouse events.
#[allow(missing_debug_implementations)]
pub struct MouseArea<'a, Message, Renderer> {
    content: Element<'a, Message, Renderer>,
    on_press: Option<Box<dyn Fn(f32, f32) -> Message>>,
}

impl<'a, Message, Renderer> MouseArea<'a, Message, Renderer> {
    /// The message to emit on a left button press.
    #[must_use]
    pub fn on_press(mut self, message_fn: Box<dyn Fn(f32, f32) -> Message>) -> Self {
        self.on_press = Some(message_fn);
        self
    }
}

/// Local state of the [`MouseArea`].
#[derive(Default)]
struct State {
    // TODO: Support on_mouse_enter and on_mouse_exit
}

impl<'a, Message, Renderer> MouseArea<'a, Message, Renderer> {
    /// Creates a [`MouseArea`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        MouseArea {
            content: content.into(),
            on_press: None,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for MouseArea<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        self.content
            .as_widget()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        ) {
            return event::Status::Captured;
        }

        update(self, &event, layout, cursor_position, shell)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            renderer_style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(&mut tree.children[0], layout, renderer)
    }
}

impl<'a, Message, Renderer> From<MouseArea<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(area: MouseArea<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(area)
    }
}

/// Processes the given [`Event`] and updates the [`State`] of an [`MouseArea`]
/// accordingly.
fn update<Message: Clone, Renderer>(
    widget: &mut MouseArea<'_, Message, Renderer>,
    event: &Event,
    layout: Layout<'_>,
    cursor_position: Point,
    shell: &mut Shell<'_, Message>,
) -> event::Status {
    if !layout.bounds().contains(cursor_position) {
        return event::Status::Ignored;
    }

    if let Some(message_fn) = widget.on_press.as_ref() {
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) = event
        {
            let layout_pos = layout.position();
            let layout_vec: Vector<f32> = Vector::new(layout_pos.x, layout_pos.y);
            let relative_pos = cursor_position - layout_vec;
            let bounds = layout.bounds();
            let (width, height) = (bounds.width, bounds.height);
            shell.publish(message_fn(relative_pos.x / width, relative_pos.y / height));

            return event::Status::Captured;
        }
    }

    event::Status::Ignored
}
