use crossbeam_channel::{Receiver, Sender, unbounded};
use resvg::{tiny_skia::Pixmap, usvg::{self, Transform}};
use rocket::tokio;
use tray_icon::{TrayIcon, TrayIconBuilder, TrayIconEvent, menu::{Menu, MenuEvent, MenuItem}};
use winit::{application::ApplicationHandler, event_loop::EventLoop};
use once_cell::sync::{OnceCell, Lazy};

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct QuitEvent {}

pub type QuitEventReceiver = Receiver<QuitEvent>;
type QuitEventHandler = Box<dyn Fn(QuitEvent) + Send + Sync + 'static>;

static QUIT_CHANNEL: Lazy<(Sender<QuitEvent>, QuitEventReceiver)> = Lazy::new(unbounded);
static QUIT_EVENT_HANDLER: OnceCell<Option<QuitEventHandler>> = OnceCell::new();

impl QuitEvent {
    pub fn receiver<'a>() -> &'a QuitEventReceiver {
        &QUIT_CHANNEL.1
    }

    pub fn set_event_handler<F: Fn(QuitEvent) + Send + Sync + 'static>(f: Option<F>) {
        if let Some(f) = f {
            let _ = QUIT_EVENT_HANDLER.set(Some(Box::new(f)));
        } else {
            let _ = QUIT_EVENT_HANDLER.set(None);
        }
    }

    pub fn send(event: QuitEvent) {
        if let Some(handler) = QUIT_EVENT_HANDLER.get_or_init(|| None) {
            handler(event);
        } else {
            let _ = QUIT_CHANNEL.0.send(event);
        }
    }
}

#[derive(Debug)]
pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    QuitEvent(QuitEvent)
}

pub struct Application {
    pub tray_icon: Option<TrayIcon>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            tray_icon: None
        }
    }

    pub fn new_tray_icon() -> TrayIcon {
        let bytes = include_bytes!("../assets/icon.svg");
        let tree = resvg::usvg::Tree::from_data(bytes, &usvg::Options::default()).expect("Failed to load SVG");

        let mut pixmap = Pixmap::new(1080, 1080).expect("Failed to create Pixmap");
        resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());

        let icon = tray_icon::Icon::from_rgba(pixmap.data().to_vec(), pixmap.width(), pixmap.height()).expect("Failed to read icon");
        TrayIconBuilder::new()
            .with_menu(Box::new(Self::new_tray_menu()))
            .with_tooltip("winit - awesome windowing lib")
            .with_icon(icon)
            .with_title("x")
            .build()
            .unwrap()
    }

    pub fn new_tray_menu() -> Menu {
        let menu = Menu::new();
        let close_button = MenuItem::with_id("close", "Close", true, None);
        if let Err(err) = menu.append(&close_button) {
            println!("{err:?}");
        }

        menu
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::<UserEvent>::with_user_event().build().expect("Failed to create EventLoop");

        let tray_icon_proxy = event_loop.create_proxy();
        TrayIconEvent::set_event_handler(Some(move |event| { let _ = tray_icon_proxy.send_event(UserEvent::TrayIconEvent(event)); }));
        
        let menu_proxy = event_loop.create_proxy();
        MenuEvent::set_event_handler(Some(move |event| { let _ = menu_proxy.send_event(UserEvent::MenuEvent(event)); }));

        let quit_proxy = event_loop.create_proxy();
        QuitEvent::set_event_handler(Some(move |event| { let _ = quit_proxy.send_event(UserEvent::QuitEvent(event)); }));
        
        let _menu_channel = MenuEvent::receiver();
        let _tray_channel = TrayIconEvent::receiver();
        
        #[cfg(target_os = "linux")]
        std::thread::spawn(|| {
            gtk::init().unwrap();
            let _tray_icon = Application::new_tray_icon();
            gtk::main();
        });

        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            QuitEvent::send(QuitEvent{});
        });

        if let Err(err) = event_loop.run_app(&mut self) {
            println!("Error: {:?}", err);
        }
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        // We create the icon once the event loop is actually running
        // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
        if winit::event::StartCause::Init == cause {
            #[cfg(not(target_os = "linux"))]
            {
                self.tray_icon = Some(Self::new_tray_icon());
            }

            // We have to request a redraw here to have the icon actually show up.
            // Winit only exposes a redraw method on the Window so we use core-foundation directly.
            #[cfg(target_os = "macos")]
            unsafe {
                use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};

                let rl = CFRunLoopGetMain().unwrap();
                CFRunLoopWakeUp(&rl);
            }
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
        UserEvent::MenuEvent(event) => {
            if event.id == "close" {
                event_loop.exit();
                return
            }

            println!("unhandled menu: {event:?}");
        },
        UserEvent::QuitEvent(_) => {
            event_loop.exit();
        }
        _ => {
            println!("unhandled event: {event:?}");
        }
        }
    }
}