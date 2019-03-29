use std::option::Option::None;
use std::sync::mpsc::Receiver;

pub use super::gui::Gui;

pub use super::options::Settings as Settings;

pub use super::render::Textures as Textures;
pub use super::render::Fonts as Fonts;
pub use super::render::Renders as Renders;

pub use super::world::World as World;

pub use super::options::WindowMode as WindowMode;

pub struct Application {
    delta_tick: f64,
    ups_max: u8,
    window_xscale: f32,
    window_yscale: f32,
    active: bool,

    glfw: glfw::Glfw,
    window: glfw::Window,

    settings: Settings,
    /*textures: Textures,
    //models: Models,
    fonts: Fonts,
    renders: Renders,*/

    screen: Gui,
    world: World
}

pub struct App {
    pub settings: Settings
}

impl Application {
    pub fn new(debug: bool, app: App) -> Application {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let mut settings = app.settings;

        settings.load(&mut glfw);
        settings.save();

        let (window, _) = Application::create_impl(&mut glfw, &settings).expect("Failed to create Window!");

        Application { window, settings, delta_tick: 0.0, ups_max: 0, window_xscale: 0.0, window_yscale: 0.0, active: false, glfw, screen: Gui::new(), world: World::new() }
    }

    pub fn create(&mut self) -> bool {


        true
    }

    fn create_impl(glfw: &mut glfw::Glfw, settings: &Settings) -> Option<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
        let instance =
            match settings.mode {
                WindowMode::WINDOWED => Application::create_windowed(glfw, settings),
                WindowMode::FULLSCREEN => Application::create_fullscreen(glfw, settings),
                WindowMode::BORDERLESS =>  Application::create_borderless(glfw, settings)
            };

        instance
    }

    fn create_windowed(glfw: &mut glfw::Glfw, settings: &Settings) -> Option<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
        if settings.window.width <= 0 || settings.window.height <= 0 {
            ()
        }

        glfw.window_hint(glfw::WindowHint::Decorated(true));
        glfw.window_hint(glfw::WindowHint::Resizable(true));
        glfw.window_hint(glfw::WindowHint::Focused(true));

        glfw.create_window(settings.window.width as u32, settings.window.height as u32, settings.title, glfw::WindowMode::Windowed)
    }

    fn create_fullscreen(glfw: &mut glfw::Glfw, settings: &Settings) -> Option<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
        let monitor = match settings.monitor {
            Some(ref monitor) => monitor,
            None => return None
        };

        let vidmode = match monitor.vidmode {
            Some(ref vidmode) => vidmode,
            None => return None
        };

        glfw.window_hint(glfw::WindowHint::RedBits(Some(vidmode.red_bits)));
        glfw.window_hint(glfw::WindowHint::GreenBits(Some(vidmode.green_bits)));
        glfw.window_hint(glfw::WindowHint::BlueBits(Some(vidmode.blue_bits)));
        glfw.window_hint(glfw::WindowHint::RefreshRate(Some(vidmode.refresh_rate)));

        glfw.window_hint(glfw::WindowHint::Decorated(false));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::Focused(true));

        monitor.glfw_monitor(glfw, |glfw, monitor| {
            glfw.create_window(vidmode.width, vidmode.height, settings.title, glfw::WindowMode::FullScreen(monitor))
        })
    }

    fn create_borderless(glfw: &mut glfw::Glfw, settings: &Settings) -> Option<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
        let monitor = match settings.monitor {
            Some(ref monitor) => monitor,
            None => return None
        };

        let vidmode = match monitor.vidmode {
            Some(ref vidmode) => vidmode,
            None => return None
        };

        glfw.window_hint(glfw::WindowHint::Decorated(false));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::Focused(true));

        glfw.window_hint(glfw::WindowHint::RedBits(Some(vidmode.red_bits)));
        glfw.window_hint(glfw::WindowHint::GreenBits(Some(vidmode.green_bits)));
        glfw.window_hint(glfw::WindowHint::BlueBits(Some(vidmode.blue_bits)));
        glfw.window_hint(glfw::WindowHint::RefreshRate(Some(vidmode.refresh_rate)));

        let window = glfw.create_window(vidmode.width, vidmode.height, settings.title, glfw::WindowMode::Windowed);

        match window {
            Some((mut window_unwrapped, receiver)) => {
                window_unwrapped.set_monitor(glfw::WindowMode::Windowed, monitor.pos.x, monitor.pos.y, vidmode.width, vidmode.height, Some(vidmode.refresh_rate));
                Some((window_unwrapped, receiver))
            },
            None => None
        }
    }

    pub fn destroy(mut self) -> bool {
        self.active = false;

        Application::destroy_impl(&self.settings);

        true
    }

    fn destroy_impl(settings: &Settings) -> bool {
        false
    }
}