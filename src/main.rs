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
    imgui::{
        Ui,
        im_str,
    },
    imgui_wrapper::{
        ImGuiWrapper,
        ImGuiBlueprint,
    },
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


struct DemoGui {
    popup_visible: bool,
}

impl DemoGui {
    fn new() -> Self {
        DemoGui {
            popup_visible: false,
        }
    }

    fn open_popup(&mut self) {
        self.popup_visible = true;
    }

    fn close_popup(&mut self) {
        self.popup_visible = false;
    }
}

impl ImGuiBlueprint for DemoGui {
    fn render(&mut self, ui: &Ui) {
        // Window
        ui.window(im_str!("Hello world"))
            .size([300.0, 600.0], imgui::Condition::FirstUseEver)
            .position([100.0, 100.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));

                if ui.small_button(im_str!("small button")) {
                    println!("Small button clicked");
                }
            });

        // Popup
        ui.popup(im_str!("popup"), || {
            if ui.menu_item(im_str!("popup menu item 1")).build() {
                println!("popup menu item 1 clicked");
            }

            if ui.menu_item(im_str!("popup menu item 2")).build() {
                println!("popup menu item 2 clicked");
            }
        });

        // Menu bar
        ui.main_menu_bar(|| {
            ui.menu(im_str!("Menu 1")).build(|| {
                if ui.menu_item(im_str!("Item 1.1")).build() {
                    println!("item 1.1 inside menu bar clicked");
                }

                ui.menu(im_str!("Item 1.2")).build(|| {
                    if ui.menu_item(im_str!("Item 1.2.1")).build() {
                        println!("item 1.2.1 inside menu bar clicked");
                    }
                    if ui.menu_item(im_str!("Item 1.2.2")).build() {
                        println!("item 1.2.2 inside menu bar clicked");
                    }
                });
            });

            ui.menu(im_str!("Menu 2")).build(|| {
                if ui.menu_item(im_str!("Item 2.1")).build() {
                    println!("item 2.1 inside menu bar clicked");
                }
            });
        });

        if self.popup_visible {
            ui.open_popup(im_str!("popup"));
        }
    }
}


struct State {
    imgui_wrapper: ImGuiWrapper,
    gui: DemoGui,
}

impl State {
    fn new(ctx: &mut Context) -> Self {
        let imgui_wrapper = ImGuiWrapper::new(ctx);
        let gui = DemoGui::new();

        State {
            imgui_wrapper,
            gui,
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

        self.imgui_wrapper.render(ctx, &mut self.gui);

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

        self.gui.close_popup();
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
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::P {
            self.gui.open_popup();
        }
    }
}
