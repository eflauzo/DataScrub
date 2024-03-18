use crate::tab;
use super::channel_id::ChannelId;
use poll_promise::Promise;
use std::sync::Arc;
use super::provider::{ChannelData};

use egui;

use egui_plot::{
    Arrows, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner,
    GridInput, GridMark, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint,
    PlotPoints, PlotResponse, Points, Polygon, Text, VLine, 
};

pub struct ChannelContext{
    id: Arc<ChannelId>,
    data: Option<Arc<Promise<ChannelData>>>,
}


pub struct LogPlotTab{
    id: u32,
    channels: Vec<ChannelContext>,
}

impl crate::tab::TabRenderer for LogPlotTab{
    fn ui(&mut self, ui: &mut egui::Ui){
        //ui.label("Hello WOrld");

        let frame = egui::Frame::default().inner_margin(4.0);

        let (drop_area_response, dropped_payload) = ui.dnd_drop_zone::<ChannelId>(frame, |ui| {
            
            let mut plot = Plot::new(self.id)
            .legend(Legend::default())
            .y_axis_width(4)
            .show_axes(true)
            .link_axis(egui::Id::new(1),true, false)
            .show_grid(true);

            let plot_response = plot.show(ui, |plot_ui| {
                //plot_ui.line(self.circle());
                
                for channel_context in self.channels.iter() {
                    //channel_context.data.
                    let option_promise_data = channel_context.data.clone();
                    match option_promise_data {
                        Some(data_promise) => {
                            if let Some(data) = data_promise.ready() {
                                //plot_ui.line( PlotPoints{data} );
                                //Line::new();
                                //let d = vec![[0.0, 0.0]];
                                let x = PlotPoints::new(data.data.clone());
                                let mut l = Line::new(x)
                                .color(egui::Color32::from_rgb(120, 150, 100))
                                .name(&(channel_context).id.channel);
                                //l
                                plot_ui.line(l);
                            }
                        },
                        None    => {
                            println!("zzz");
                        },
                    };
                }
                
                //plot_ui.line(self.sin());
                //ui.put();
                
                //plot_ui.line(self.thingy());
            });
        
        
        });

        if let Some(dragged_payload) = dropped_payload {
            println!("Dropped {:?}", dragged_payload);
            self.channels.push(
                ChannelContext{
                    
                        id: dragged_payload.clone(),
                        data: Some(Arc::new(dragged_payload.data_source.load_data(dragged_payload.dataset.clone(), dragged_payload.channel.clone()))), //Option<Arc<Promise<ChannelData>>>,
                    
                }
            );
        }else{ 
            //println!("No drop");
        }

        
    }
}

static mut  LAST_LOG_PLOT_ID: u32 = 0;

pub fn create_log_plot_tab() -> tab::Tab {
    let mut id: u32 = 0;
    unsafe{
        LAST_LOG_PLOT_ID += 1;
        id = LAST_LOG_PLOT_ID;
    };
    tab::Tab{
        title: format!("Log Plot {id}").to_string(),
        renderer: Box::new(
            LogPlotTab{
                id: id,
                channels: vec![],
            }
        )
    }
}