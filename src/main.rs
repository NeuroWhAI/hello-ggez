mod imgui_wrapper;


use {
    ggez::{
        Context,
        ContextBuilder,
        GameResult,
        conf,
        graphics::{
            self,
            DrawMode,
        },
        event::{
            self,
            EventHandler,
            KeyCode,
            KeyMods,
            MouseButton,
        },
        nalgebra::{
            Point2,
        }
    },
    imgui_wrapper::ImGuiWrapper,
};


fn main() {
    let win_setup = conf::WindowSetup::default()
        .title("Hello ggez");
    let win_mode = conf::WindowMode::default()
        .dimensions(1024.0, 768.0)
        .fullscreen_type(conf::FullscreenType::Windowed);

    let mut config = conf::Conf::new();
    config.window_setup = win_setup;
    config.window_mode = win_mode;

    let (mut ctx, mut evt_loop) = ContextBuilder::new("hello-ggez", "NeuroWhAI")
        .conf(config)
        .build()
        .expect("Fail to build context");

    let mut state = State::new(&mut ctx);

    match event::run(&mut ctx, &mut evt_loop, &mut state) {
        Ok(_) => println!("Exited!"),
        Err(e) => println!("Error: {}", e),
    }
}


struct State {
    imgui_wrapper: ImGuiWrapper,
}

impl State {
    fn new(ctx: &mut Context) -> Self {
        let imgui_wrapper = ImGuiWrapper::new(ctx);

        State {
            imgui_wrapper,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let color = (0, 220, 0).into();
        let mesh = graphics::MeshBuilder::new()
            .circle(DrawMode::fill(), Point2::new(0.0, 0.0), 32.0, 1.0, color)
            .build(ctx)?;
        graphics::draw(ctx, &mesh, (Point2::new(100.0, 128.0), 0.0, graphics::WHITE))?;

        self.imgui_wrapper.render(ctx);

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        println!("{}, {}", x, y);
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::P => {
                self.imgui_wrapper.open_popup();
            }
            _ => (),
        }
    }
}
