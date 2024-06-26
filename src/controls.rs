
#[derive(Default, Clone, Copy)]
pub struct InputState {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,

    pub m: bool,

    pub space: bool,
    pub l_ctrl: bool,
    pub escape: bool,
    pub l_shift: bool,

    pub cursor_did_move: bool,
    pub cursor_x: f64,
    pub cursor_y: f64,
}

impl InputState {
    pub fn update_from_event(&mut self, event: &glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                self.escape = true;
            },
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Release, _) => {
                self.escape = false;
            },
            glfw::WindowEvent::Key(glfw::Key::LeftControl, _, glfw::Action::Press, _) => {
                self.l_ctrl = true;
            },
            glfw::WindowEvent::Key(glfw::Key::LeftControl, _, glfw::Action::Release, _) => {
                self.l_ctrl = false;
            },
            glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Press, _) => {
                self.l_shift = true;
            },
            glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Release, _) => {
                self.l_shift = false;
            },
            glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
                self.space = true;
            },
            glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Release, _) => {
                self.space = false;
            },
            glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Press, _) => {
                self.w = true;
            },
            glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Release, _) => {
                self.w = false;
            },
            glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Press, _) => {
                self.a = true;
            },
            glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Release, _) => {
                self.a = false;
            },
            glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Press, _) => {
                self.s = true;
            },
            glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Release, _) => {
                self.s = false;
            },
            glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Press, _) => {
                self.d = true;
            },
            glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Release, _) => {
                self.d = false;
            },
            glfw::WindowEvent::Key(glfw::Key::M, _, glfw::Action::Press, _) => {
                self.m = true;
            },
            glfw::WindowEvent::Key(glfw::Key::M, _, glfw::Action::Release, _) => {
                self.m = false;
            },
            glfw::WindowEvent::CursorPos(x, y) => {
                self.cursor_did_move = true;
                self.cursor_x = *x;
                self.cursor_y = *y;
            }
            _ => {},
        }
    }
}
