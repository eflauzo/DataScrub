
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer, SurfaceIndex};
use egui::{Ui, WidgetText};

use crate::tab;
use crate::plot_tab;
use crate::data_navigator_tab;

use super::provider;
use std::borrow::Borrow;
use std::sync::Arc;
use poll_promise::Promise;

//mod tab;
//use self::TabType;


// First, let's pick a type that we'll use to attach some data to each tab.
// It can be any type.
//type Tab = String;

// To define the contents and properties of individual tabs, we implement the `TabViewer`
// trait. Only three things are mandatory: the `Tab` associated type, and the `ui` and
// `title` methods. There are more methods in `TabViewer` which you can also override.
//struct MyTabViewer;

struct MyTabViewer<'a> {
    added_nodes: &'a mut Vec<(SurfaceIndex, NodeIndex)>,
}


impl TabViewer for MyTabViewer<'_>  {
    // This associated type is used to attach some data to each tab.
    type Tab = tab::Tab;

    // Returns the current `tab`'s title.
    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title.as_str().into()
    }

    // Defines the contents of a given `tab`.
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        //ui.label(format!("Content of {tab}"));
        tab.renderer.ui(ui);
    }

    fn on_add(&mut self, surface: SurfaceIndex, node: NodeIndex) {
        //self.added_nodes.push((surface, node));
        println!("wazza");
        self.added_nodes.push((surface, node));

        //surface.
        
        //let main_surface = dock_state.main_surface_mut();

        //surface.

        //surface
        //let b = self.borrow_mut();
        
        /*
        let [_old_node, new_node] =
            surface.split_below(node, 0.50, vec![
                
                plot_tab::create_log_plot_tab(),
                
            ]);
        */

    }
}

// Here is a simple example of how you can manage a `DockState` of your application.
struct MyTabs {
    dock_state: DockState<tab::Tab>,
    counter: usize,
}

impl MyTabs {
    pub fn new(data_provider: Arc<provider::DataProvider>) -> Self {
        // Create a `DockState` with an initial tab "tab1" in the main `Surface`'s root node.
        //let tabs = ["tab1", "tab2", "tab3"].map(str::to_string).into_iter().collect();
        //let tabs = ["tab1"].map(str::to_string).into_iter().collect();
        //let dock_state = DockState::new(tabs);
        
        let mut dock_state = DockState::new(vec![
            data_navigator_tab::create_navigator_tab(data_provider),
        ]);

        let surface = dock_state.main_surface_mut();

        let [_old_node, first_log_node] =
            surface.split_right(NodeIndex::root(), 0.20, vec![
                
                plot_tab::create_log_plot_tab(),
                
            ]);
            
            

            let [_old_node, new_node] =
            surface.split_below(first_log_node, 0.50, vec![
                
                plot_tab::create_log_plot_tab(),
                
            ]);
        
        

        Self { dock_state, counter:3 }
    }

    fn ui(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        // Here we just display the `DockState` using a `DockArea`.
        // This is where egui handles rendering and all the integrations.
        //
        // We can specify a custom `Style` for the `DockArea`, or just inherit
        // all of it from egui.
        
        /*
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_add_buttons(true)
            .show_close_buttons(true)
            .show_window_close_buttons(false)
            .show_inside(ui, &mut MyTabViewer);
        */

        let mut added_nodes = Vec::new();
        DockArea::new(&mut self.dock_state)
            .show_add_buttons(true)
            .style({
               let mut style = Style::from_egui(ctx.style().as_ref());
               style.tab_bar.fill_tab_bar = true;
               style
            })
            .show(
                ctx,
                &mut MyTabViewer {
                    added_nodes: &mut added_nodes,
                },
            );
        
        
        added_nodes.drain(..).for_each(|(surface, node)| {
            self.dock_state.set_focused_node_and_surface((surface, node));
            self.dock_state.push_to_focused_leaf(plot_tab::create_log_plot_tab());
            //self.counter += 1;
        });
         
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DataScrubApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    my_tabs: MyTabs,
}

impl Default for DataScrubApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            my_tabs: MyTabs::new( Arc::new(provider::DataProvider{base_url: "http://127.0.0.1:8081".to_owned()}) ),
        }
    }
}

impl DataScrubApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for DataScrubApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        
        /*
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
         */

        egui::CentralPanel::default().show(ctx, |ui| {
            
            self.my_tabs.ui(ctx, ui);
            
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
