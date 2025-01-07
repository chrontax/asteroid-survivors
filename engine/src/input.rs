use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::Key,
};

pub enum Input {
    Keyboard {
        key: Key,
        state: ElementState,
    },
    Cursor(PhysicalPosition<f64>),
    MouseButton {
        btn: MouseButton,
        state: ElementState,
    },
}

impl TryFrom<WindowEvent> for Input {
    type Error = ();

    fn try_from(value: WindowEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            WindowEvent::CursorMoved { position, .. } => Self::Cursor(position),
            WindowEvent::MouseInput { state, button, .. } => {
                Self::MouseButton { btn: button, state }
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state, logical_key, ..
                },
                ..
            } => Self::Keyboard {
                key: logical_key,
                state,
            },
            _ => Err(())?,
        })
    }
}
