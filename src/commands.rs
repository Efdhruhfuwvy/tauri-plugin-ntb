use tauri::{command, Runtime};
use tauri::{LogicalPosition, Window};

use crate::models::*;
use crate::Result;

#[cfg(target_os = "linux")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
fn get_theme_path(theme: &String) -> Option<PathBuf> {
    use homedir::my_home;
    use std::fs::exists;
    use std::path::Path;
    let home = my_home().unwrap().unwrap();
    if exists(home.join(Path::new(&format!(".themes/{theme}")))).unwrap_or(false) {
        Some(home.join(Path::new(&format!(".themes/{theme}"))))
    } else if exists(home.join(Path::new(&format!(".local/share/themes/{theme}")))).unwrap_or(false)
    {
        Some(home.join(Path::new(&format!(".local/share/themes/{theme}"))))
    } else if exists(Path::new(&format!("/usr/share/themes/{theme}"))).unwrap_or(false) {
        Some(Path::new(&format!("/usr/share/themes/{theme}")).to_path_buf())
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
fn click_title_bar<R: Runtime>(window: Window<R>, dconf: &str, default: &str) {
    use dconf_rs::get_string;

    let value = get_string(dconf).unwrap_or(default.into());
    match if value.is_empty() { default } else { &value } {
        "minimize" => {
            let _ = window.minimize();
        }
        "toggle-maximize" => {
            if window.is_maximized().unwrap() {
                let _ = window.unmaximize();
            } else {
                let _ = window.maximize();
            }
        }
        _ => {}
    }
}

#[command]
pub(crate) async fn minimize<R: Runtime>(window: Window<R>) {
    let _ = window.minimize();
}

#[command]
pub(crate) async fn toggle_maximize<R: Runtime>(window: Window<R>) {
    if let Ok(maximized) = window.is_maximized() {
        if maximized {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[command]
pub(crate) async fn close<R: Runtime>(window: Window<R>) {
    let _ = window.close();
}

#[command]
pub(crate) async fn drag<R: Runtime>(window: Window<R>) {
    let _ = window.start_dragging();
}

#[command]
pub(crate) async fn move_by<R: Runtime>(window: Window<R>, x: i32, y: i32) {
    if let Ok(pos) = window.outer_position() {
        let pos: LogicalPosition<i32> = pos.to_logical(window.scale_factor().unwrap());
        let _ = window.set_position(LogicalPosition::new(pos.x + x, pos.y + y));
    }
}

#[command]
pub(crate) async fn get_drag_behavior() -> DragBehavior {
    #[cfg(target_os = "linux")]
    return DragBehavior::Threshold { threshold: 10 };
    #[cfg(not(target_os = "linux"))]
    DragBehavior::Immediate
}

#[command]
pub(crate) async fn is_maximized<R: Runtime>(window: Window<R>) -> bool {
    window.is_maximized().unwrap_or(false)
}

#[command]
pub(crate) async fn get_window_controls() -> Result<WindowControls> {
    #[cfg(desktop)]
    {
        #[cfg(target_os = "linux")]
        return {
            use dconf_rs::get_string;
            let mut result = WindowControls {
                left: vec![],
                right: vec![],
            };
            let button_layout = get_string("/org/gnome/desktop/wm/preferences/button-layout")
                .unwrap_or(":minimize,maximize,close".into());
            let func = |string: &str| {
                if string == "minimize" {
                    Some(WindowControl::Minimize)
                } else if string == "maximize" {
                    Some(WindowControl::Maximize)
                } else if string == "close" {
                    Some(WindowControl::Close)
                } else {
                    None
                }
            };
            result.left = button_layout.split(":").collect::<Vec<&str>>()[0]
                .split(",")
                .filter_map(func)
                .collect();
            result.right = button_layout.split(":").collect::<Vec<&str>>()[1]
                .split(",")
                .filter_map(func)
                .collect();
            Ok(result)
        };
        #[cfg(target_os = "windows")]
        return Ok(WindowControls {
            left: vec![],
            right: vec![
                WindowControl::Minimize,
                WindowControl::Maximize,
                WindowControl::Close,
            ],
        });
        #[cfg(target_os = "macos")]
        return Ok(WindowControls {
            left: vec![],
            right: vec![],
        });
        #[allow(unreachable_code)]
        return Ok(WindowControls {
            left: vec![],
            right: vec![
                WindowControl::Minimize,
                WindowControl::Maximize,
                WindowControl::Close,
            ],
        });
    }
    #[cfg(mobile)]
    {
        return Ok(WindowControls {
            left: vec![],
            right: vec![],
        });
    }
}

#[command]
pub(crate) async fn get_window_control_image(
    control: WindowControl,
    maximized: bool,
) -> WindowControlImage {
    #[cfg(target_os = "linux")]
    return {
        use linicon::lookup_icon;
        use std::fs::read_to_string;
        use xml_dom::level2::Document;
        use xml_dom::level2::Element;
        use xml_dom::level2::Node;
        use xml_dom::parser::read_xml;
        let control_name = if let WindowControl::Minimize = control {
            "minimize"
        } else if let WindowControl::Maximize = control {
            if maximized {
                "restore"
            } else {
                "maximize"
            }
        } else if let WindowControl::Close = control {
            "close"
        } else {
            ""
        }
        .into();

        let icon = lookup_icon(format!("window-{control_name}-symbolic"))
            .filter_map(|result| result.ok())
            .next();
        if let Some(icon) = icon {
            if let Ok(svg_xml) = read_to_string(&icon.path) {
                let svg_dom = read_xml(svg_xml).unwrap();
                let mut stack = vec![svg_dom.document_element().unwrap()];
                while stack.len() > 0 {
                    let mut node = stack.pop().unwrap();
                    node.set_attribute("color", "currentColor").unwrap();
                    node.set_attribute("fill", "currentColor").unwrap();
                    for node in node.child_nodes() {
                        stack.push(node);
                    }
                }
                return WindowControlImage::SVG {
                    svg: svg_dom.to_string(),
                };
            }
        }
        WindowControlImage::Text {
            font: "".into(),
            size: None,
            text: control_name,
        }
    };
    #[cfg(target_os = "windows")]
    return match control {
        WindowControl::Minimize => WindowControlImage::Text {
            font: "'Segoe Fluent Icons', 'Segoe MDL2 Assets'".into(),
            size: Some(10),
            text: "\u{e921}".into(),
        },
        WindowControl::Maximize => {
            if maximized {
                WindowControlImage::Text {
                    font: "'Segoe Fluent Icons', 'Segoe MDL2 Assets'".into(),
                    size: Some(10),
                    text: "\u{e923}".into(),
                }
            } else {
                WindowControlImage::Text {
                    font: "'Segoe Fluent Icons', 'Segoe MDL2 Assets'".into(),
                    size: Some(10),
                    text: "\u{e922}".into(),
                }
            }
        }
        WindowControl::Close => WindowControlImage::Text {
            font: "'Segoe Fluent Icons', 'Segoe MDL2 Assets'".into(),
            size: Some(10),
            text: "\u{e8bb}".into(),
        },
    };
    #[allow(unreachable_code)]
    match control {
        WindowControl::Minimize => WindowControlImage::SVG {
            svg: include_str!("assets/minimize.svg").into(),
        },
        WindowControl::Maximize => WindowControlImage::SVG {
            svg: if maximized {
                include_str!("assets/unmaximize.svg")
            } else {
                include_str!("assets/maximize.svg")
            }
            .into(),
        },
        WindowControl::Close => WindowControlImage::SVG {
            svg: include_str!("assets/close.svg").into(),
        },
    }
}

#[command]
#[allow(unused_variables, unreachable_code)]
pub(crate) async fn get_title_bar_css(dark: bool) -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        use dconf_rs::get_string;
        use gvdb::read::File;
        use regex::Captures;
        use regex::Regex;
        use std::fs::read_to_string;
        use std::path::Path;
        let theme = get_string("/org/gnome/desktop/interface/gtk-theme");
        if let Ok(theme) = theme {
            let theme_path = get_theme_path(&theme);
            if let Some(mut theme_path) = theme_path {
                if dark
                    != (theme_path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .ends_with("-dark"))
                {
                    if dark {
                        theme_path = Path::new(
                            format!(
                                "{}-dark",
                                theme_path.into_os_string().into_string().unwrap()
                            )
                            .as_str(),
                        )
                        .to_path_buf();
                    } else {
                        theme_path = Path::new::<String>(
                            &Regex::new("-dark$")
                                .unwrap()
                                .replace(
                                    theme_path.into_os_string().into_string().unwrap().as_str(),
                                    "",
                                )
                                .into(),
                        )
                        .to_path_buf();
                    };
                }
                let mut css = read_to_string(theme_path.join(Path::new(if dark {
                    "gtk-4.0/gtk-dark.css"
                } else {
                    "gtk-4.0/gtk.css"
                })))
                .unwrap_or("".into());
                css = Regex::new(r#"@import\s+url\s*\(\s*["']resource://(.*?)["']\s*\)\s*;"#)
                    .unwrap()
                    .replace_all(&css, |caps: &Captures| {
                        let file = File::from_file(
                            theme_path
                                .join(Path::new("gtk-4.0/gtk.gresource"))
                                .as_path(),
                        );
                        if let Ok(file) = file {
                            let resource_data =
                                file.hash_table().unwrap().get::<GResourceData>(&caps[1]);
                            if let Ok(resource_data) = resource_data {
                                String::from_utf8(resource_data.content).unwrap()
                            } else {
                                "".into()
                            }
                        } else {
                            "".into()
                        }
                    })
                    .into();
                return Ok(format!(
                    "{}{css}",
                    include_str!("css/linux.css")
                ));
            } else {
                return Ok(include_str!("css/linux.css").into());
            }
        } else {
            return Ok(include_str!("css/linux.css").into());
        };
    };
    #[cfg(target_os = "windows")]
    {
        return Ok(include_str!("css/windows.css").into());
    }
    Ok("".into())
}

#[command]
pub(crate) async fn double_click_title_bar<R: Runtime>(window: Window<R>) {
    #[cfg(target_os = "linux")]
    {
        click_title_bar(
            window,
            "/org/gnome/desktop/wm/preferences/action-double-click-titlebar",
            "toggle-maximize",
        );
    }
    #[cfg(not(target_os = "linux"))]
    {
        if window.is_maximized().unwrap() {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[command]
#[allow(unused_variables)]
pub(crate) async fn right_click_title_bar<R: Runtime>(window: Window<R>) {
    #[cfg(target_os = "linux")]
    {
        click_title_bar(
            window,
            "/org/gnome/desktop/wm/preferences/action-right-click-titlebar",
            "menu",
        );
    }
}

#[command]
#[allow(unused_variables)]
pub(crate) async fn middle_click_title_bar<R: Runtime>(window: Window<R>) {
    #[cfg(target_os = "linux")]
    {
        click_title_bar(
            window,
            "/org/gnome/desktop/wm/preferences/action-middle-click-titlebar",
            "lower",
        );
    }
}

#[command]
pub(crate) async fn show_snap_overlay() {
    #[cfg(target_os = "windows")]
    {
        use std::thread::sleep;
        use std::time::Duration;

        use enigo::{Direction, Enigo, Key, Keyboard};

        let mut enigo = Enigo::new(&Default::default()).unwrap();
        let _ = enigo.key(Key::Meta, Direction::Press);
        let _ = enigo.key(Key::Unicode('z'), Direction::Click);
        let _ = enigo.key(Key::Meta, Direction::Release);
        sleep(Duration::from_millis(500));
        let _ = enigo.key(Key::Alt, Direction::Click);
    }
}

#[command]
pub(crate) async fn hide_snap_overlay() {
    #[cfg(target_os = "windows")]
    {
        use enigo::{Direction, Enigo, Key, Keyboard};

        let mut enigo = Enigo::new(&Default::default()).unwrap();
        let _ = enigo.key(Key::Escape, Direction::Click);
    }
}
