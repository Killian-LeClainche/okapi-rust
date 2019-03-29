use super::super::util::math::{Size, Pos2 };

pub enum WindowMode {
    WINDOWED, FULLSCREEN, BORDERLESS
}

pub struct Monitor {
    pub name: String,
    pub size: Size,
    pub pos: Pos2,
    pub vidmode: Option<glfw::VidMode>,
    pub gamma_ramp: glfw::GammaRamp
}

impl Monitor {
    pub fn glfw_monitor<T, F>(&self, glfw: &mut glfw::Glfw, f: F) -> Option<T> where F: Fn(&mut glfw::Glfw, &glfw::Monitor) -> Option<T> {
        glfw.with_connected_monitors_mut(|glfw, monitors| {
            for m in monitors {
                if m.get_name().eq(&self.name) {
                    return f(glfw, m)
                }
            };

            glfw.with_primary_monitor_mut(|glfw, monitor| {
                match monitor {
                    Some(m) => f(glfw, m),
                    None => None
                }
            })
        })
    }
}

pub struct Settings {
    pub title: &'static str,
    pub mode: WindowMode,
    pub window: Size,
    pub monitors: Vec<Monitor>,
    pub monitor: Option<Monitor>
}

impl Monitor {
    pub fn new(monitor: &glfw::Monitor) -> Monitor {
        let name = monitor.get_name();
        let (width, height) = monitor.get_physical_size();
        let (x, y) = monitor.get_pos();
        let vidmode = monitor.get_video_mode();
        let gamma_ramp = monitor.get_gamma_ramp();

        Monitor { name, size: Size { width, height }, pos: Pos2 { x, y }, vidmode, gamma_ramp }
    }
}

impl Settings {

    pub fn new() -> Settings {
        Settings { title: "asdas", mode: WindowMode::WINDOWED, window: Size {width: 100, height: 100}, monitors: Vec::new(), monitor: None }
    }

    pub fn load(&mut self, glfw: &mut glfw::Glfw) {
        glfw.with_connected_monitors_mut(|_, monitors| {
            for m in monitors {
                self.monitors.push(Monitor::new(m))
            }
        });
    }

    pub fn save(&mut self) {

    }
}