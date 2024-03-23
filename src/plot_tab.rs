use crate::tab;
use super::channel_id::ChannelId;
use poll_promise::Promise;
use std::borrow::Borrow;
use std::sync::Arc;
use super::provider::{ChannelData};

use egui;
use random_color::color_dictionary::{ColorDictionary, ColorInformation};
use random_color::{Color, Luminosity, RandomColor};
//use super::channel_id::ChannelId;

use egui::{emath, vec2, Color32, Frame, Id, Pos2, Rect, Sense, Shape, Stroke, Vec2, Widget};
use egui_plot::{
    Arrows, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner,
    GridInput, GridMark, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint,
    PlotPoints, PlotResponse, Points, Polygon, Text, VLine, 
};

pub struct ChannelContext{
    id: Arc<ChannelId>,
    data: Option<Arc<Promise<ChannelData>>>,
    color: [f32; 3],
}


pub struct LogPlotTab{
    id: u32,
    channels: Vec<ChannelContext>,
}

impl LogPlotTab {
    fn has_channel(&self, id: &ChannelId) -> bool {
        for channel_context in self.channels.iter() {
            if ( *channel_context.id == *id){ 
                return true;
            }
        }
        return false;
    }
}


struct Header<'a> {
    channels: &'a mut Vec <ChannelContext>,
    plot_id: u32,
}

impl egui::Widget for &mut Header<'_>{
    

    //pub fn display(&mut self, ui: &mut egui::Ui){
        fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        
        let head_w = ui.max_rect().width(); //50.0;
        let head_h = ui.max_rect().height();
        
        /*
        let (mut response, painter) = ui.allocate_painter(Vec2{x:head_w, y:head_h}, Sense::click());


         let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let stroke_green = Stroke::new(1.0, Color32::from_rgb(25, 200, 100));
        let stroke_red = Stroke::new(1.0, Color32::from_rgb(200, 50, 50));
        let stroke_blue = Stroke::new(1.0, Color32::from_rgb(50, 50, 250));
        
        let control_point_radius = 8.0;

        let point0 = Pos2{x: 0.0 , y: 0.0};
        let point1 = Pos2{x: head_w , y: head_h};

        let point_in_screen = to_screen.transform_pos(point0);
        painter.add( Shape::circle_stroke(point_in_screen, control_point_radius, stroke_green) );

        let point_in_screen = to_screen.transform_pos(point1);
        painter.add( Shape::circle_stroke(point_in_screen, control_point_radius, stroke_red) );


        //Shape::
        
        
        let pts = vec!(
            to_screen.transform_pos(Pos2{x: 0.0, y: 0.0}),
            to_screen.transform_pos(Pos2{x: head_w, y: head_h}),
        );
        println!("pts:{:?}", pts);
        painter.add(Shape::line(
            pts, stroke_blue
        ));        
        */
        ui.vertical(|ui| {
            let mut kill_item: Option<usize> = None;
            for (i, channel_context) in self.channels.iter_mut().enumerate() {
            //
            //for i in 0..self.channels.len()
                
                //let ch_id = channel_context.id.borrow();
                // TODO is bad:
                //let new_id = Arc::try_unwrap(channel_context.id).unwrap();

                //} ;



                let item_id = egui::Id::new(("plot", channel_context.id.channel.clone(), self.plot_id));
                
                    ui.horizontal(|ui| {
                        
                        //let mut color: [f32; 3] = [1.0 , 0.0, 0.0];
                        ui.color_edit_button_rgb(&mut channel_context.color);
                        ui.dnd_drag_source(item_id, ChannelId{
                            data_source: channel_context.id.data_source.clone(),
                            dataset: channel_context.id.dataset.clone(),
                            channel: channel_context.id.channel.clone(),
                        }, |ui| {
                            ui.label(channel_context.id.channel.to_owned());
                        });
                        if (ui.button("x").clicked()){
                            println!("Kill {}",channel_context.id.channel);
                            kill_item = Some(i);

                        }
                    });
                
            };
            
            if let Some(need_kill) = kill_item {
                self.channels.remove(need_kill);
            }

            //ui.label("XXX2: ");
            //ui.label("XXX3: ");
            //ui.label("XXX4: ")
        }).response

        //response
    }
}



impl crate::tab::TabRenderer for LogPlotTab{
    fn ui(&mut self, ui: &mut egui::Ui){
        //ui.label("Hello WOrld");

        let frame = egui::Frame::default().inner_margin(4.0);

        let mut start_offset = 0.0;
        let (drop_area_response, dropped_payload) = ui.dnd_drop_zone::<ChannelId>(frame, |ui| {
            
            let mut plot = Plot::new(self.id)
            //.legend(Legend::default())
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
                                .color(egui::Color32::from_rgb(
                                    (channel_context.color[0] * 255.0) as u8,
                                    (channel_context.color[1] * 255.0) as u8,
                                    (channel_context.color[2] * 255.0) as u8,
                                ))
                                //.color(egui::Color32::from_rgb(
                                //        channel_context.color[0], 
                                //        channel_context.color[1], 
                                //        channel_context.color[2])
                                //)
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
            
            let mut plot_rect = plot_response.response.rect;
            start_offset = plot_rect.min.x;
        
        });

        

        let mut r = drop_area_response.rect;
        r.set_left(start_offset + 10.0);
        r.set_top(r.top() + 10.0);
        //let mut left = r.left()
        //left = *left + 5.0;

        r.set_width(100.0);

        let mut h = Header{channels: &mut self.channels, plot_id:self.id};

        ui.put(r, &mut h);
        

        


        if let Some(dragged_payload) = dropped_payload {
            println!("Dropped {:?}", dragged_payload);

            let cl = RandomColor::new()
                //.hue(Color::Blue) // Optional
                
                //.luminosity(Luminosity::High) // Optional
                //.seed(42) // Optional
                //.alpha(1.0) // Optional
                .dictionary(ColorDictionary::new())
                .to_rgb_array();

            let new_id = dragged_payload.clone();
            if (!self.has_channel(&new_id)){

                self.channels.push(
                    
                    ChannelContext{
                        
                            id: new_id,
                            data: Some(Arc::new(dragged_payload.data_source.load_data(dragged_payload.dataset.clone(), dragged_payload.channel.clone()))), //Option<Arc<Promise<ChannelData>>>,
                            color: [(cl[0] as f32 / 255.0), (cl[1] as f32 / 255.0) ,(cl[2] as f32 / 255.0)]
                    }
                );
            };
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