



use crate::tab;

use egui;
use super::channel_id::ChannelId;
use super::splitter;
use super::provider;
use std::sync::Arc;
use poll_promise::Promise;

pub struct DataNavigatorTab{
    data_provider: Arc<provider::DataProvider>,
    selected_dataset: String,
    dataset_filter: String,
    channel_filter: String,
    option_datasets_promise: Option<Arc<Promise<provider::Datasets>>>,
    option_channels_promise: Option<Arc<Promise<provider::Channels>>>
}

impl crate::tab::TabRenderer for DataNavigatorTab{

    fn ui(&mut self, ui: &mut egui::Ui){

             splitter::Splitter::new("some_plot_split", splitter::SplitterAxis::Vertical)
             .min_size(250.0)
             .default_pos(2.0 / 3.0)
             .show(ui, |ui_a, ui_b| {
                
                ui_a.label("Dataset: ");

                ui_a.horizontal(|ui_a| {
                    ui_a.label("Filter: ");

                    ui_a.text_edit_singleline(&mut self.dataset_filter);

                    let mut need_reload_datasets = false;
                    if ui_a.button("Go!").clicked() {
                        //self.value += 1.0;
                        //log::info!("get datasets");
                        need_reload_datasets = true;
                    }
                    
                    if (self.option_datasets_promise.is_none()){
                        need_reload_datasets = true;
                    }

                    if (need_reload_datasets){
                        println!("aaa");
                        let x = Some(Arc::new(self.data_provider.list_datasets("all".to_owned())));
                        self.option_datasets_promise = x;
                    }


                });

                egui::ScrollArea::vertical()
                .id_source("first_scroll_area")
                .show(ui_a, |ui| {
                    
                    let option_promise_datasets = self.option_datasets_promise.clone();
                                        match option_promise_datasets {
                                            // The division was valid
                                            Some(datasets_promise) => {
                                                if let Some(datasets) = datasets_promise.ready() {
                                                    
                                                    //let mut selected: i32 = 0;
                                                    for dataset_name in datasets.datasets.iter() {
                                                        //ui.label(format!("A My label # {i}"));
                                                        
                                                        let this_label_text = dataset_name.clone(); //format!("");
                                                        if ui.add(egui::SelectableLabel::new(self.selected_dataset == this_label_text, this_label_text.clone())).clicked() {
                                                            //my_enum = Enum::First
                                                            if (self.selected_dataset != this_label_text){
                                                                self.selected_dataset = this_label_text.clone();
                                                                // force reload channels
                                                                self.option_channels_promise = None;
                                                                println!("invalidated channels");
                                                            }
                                                        }

                                                    }
                                                
                                                }else{
                                                //info!("promise is not ready");
                                                    println!("promise not ready");
                                                }
                                            },
                                            None    => {
                                                println!("No pending promise");
                                            },


                                            };
                                        
                    
                
                });


                ui_b.label("Channels: ");

                let mut need_reload_channels = false;

                ui_b.horizontal(|ui| {
                    ui.label("Filter: ");

                    ui.text_edit_singleline(&mut self.channel_filter);

                    
                    if ui.button("Go!").clicked() {
                        //self.value += 1.0;
                        //log::info!("get datasets");
                        need_reload_channels = true;
                    }
                });

                if (self.option_channels_promise.is_none()){
                    if self.selected_dataset.len() > 0 {
                        need_reload_channels = true;
                    }
                }

                if (need_reload_channels){
                    println!("aaa");
                    let x = Some(Arc::new(self.data_provider.list_channels(self.selected_dataset.clone(), "all".to_owned())));
                    self.option_channels_promise = x;
                }


                egui::ScrollArea::vertical()
                .id_source("second_scroll_area")
                .show(ui_b, |ui| {
                    

                    
                    //for i in 0..100 {
                    //    ui.label(format!("B My label # {i}       "));
                    //}


                    let option_promise_channels = self.option_channels_promise.clone();
                    match option_promise_channels {
                        // The division was valid
                        Some(channels_promise) => {
                            if let Some(channels) = channels_promise.ready() {
                                
                                //let mut selected: i32 = 0;
                                for channel_name in channels.channels.iter() {
                                    //ui.label(format!("A My label # {i}"));
                                    
                                    let this_label_text = channel_name.clone(); //format!("");
                                    
                                    let item_location: i32 = 44;
                                                                    let ch_id = ChannelId{
                                                                        data_source: self.data_provider.clone(),//"".to_owned(),
                                                                        dataset: self.selected_dataset.clone(), //"".to_owned(),
                                                                        channel: channel_name.clone(),
                                                                    };
                                                                    let item_id = egui::Id::new(("hotdog", channel_name));
                    
                                                                    ui.dnd_drag_source(item_id, ch_id, |ui| {

                                    ui.label(this_label_text);
                                                                    });

                                    //if ui.add(egui::SelectableLabel::new(self.selected_dataset == this_label_text, this_label_text.clone())).clicked() {
                                        //my_enum = Enum::First
                                    //    if (self.selected_dataset != this_label_text){
                                     //       self.selected_dataset = this_label_text.clone();
                                            // force reload channels
                                     //       self.option_channels_promise = None;
                                     //       println!("invalidated channels");
                                     //   }
                                    
                                    }

                                
                            
                            }else{
                            //info!("promise is not ready");
                                println!("promise not ready");
                            }
                        },
                        None    => {
                            println!("No pending promise");
                        },


                        };
                
                });

             });
     

        
        /*
        egui::ScrollArea::vertical().show(ui, |ui| {
                
                for i in 0..100 {
                    ui.label(format!("2 My label # {i}"));
                }
            
        });
        */ 

    }
}

pub fn create_navigator_tab(data_provider_param: Arc<provider::DataProvider>) -> tab::Tab { 
    tab::Tab{
        title: "data navigator".to_string(),
        renderer: Box::new(
            DataNavigatorTab{
                data_provider: data_provider_param,
                selected_dataset: "".to_string(),
                dataset_filter: "".to_string(),
                channel_filter: "".to_string(),
                option_datasets_promise: None,
                option_channels_promise: None
            }
        )
    }
}