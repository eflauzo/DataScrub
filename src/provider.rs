use egui_plot::{
    Arrows, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner,
    GridInput, GridMark, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint,
    PlotPoints, PlotResponse, Points, Polygon, Text, VLine,
};


use poll_promise::Promise;
use ehttp;
use log::info;
use log::Level;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct DataProvider{
    pub base_url: String,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Datasets {
    pub datasets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channels {
    pub channels: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelData{
    pub data: Vec<[f64; 2]>,
}

//use egui_plot::{PlotPoint};


impl DataProvider{
    pub fn load_data(&self, path:String, channel:String) -> Promise<ChannelData> {
        println!("XXX");
        let (sender, promise) = Promise::<ChannelData>::new();
        let fetch_url = format!("{}/data/{}/{}", self.base_url, path, channel); 
        let request = ehttp::Request::get(&fetch_url);
        ehttp::fetch(request, move |response| {
                info!("aXX 1");
                println!("aXXX1");
                let response_content = match response{
                    Ok(response_content) => response_content,
                    Err(e) => {
                        sender.send(  ChannelData{data:vec!()} );
                        return
                    }
                };

                info!("aXXX 2");
                println!("aXXX2");
                let text_content = response_content.text().unwrap();

                info!("aXXX 3");
                println!("aXXX3");
                let parsed_data = serde_json::from_str::<ChannelData>(&text_content);
                let data = match parsed_data{
                    Ok(data) => {
                        log::info!("adata {:?}", data);
                        println!("adata");
                        
                        //let mut result: Vec<PlotPoint> = vec![];
                        //for tuple in data.data.iter() {
                        //    result.push( PlotPoint{
                        //        x: tuple.0,
                        //        y: tuple.1,
                        //    });
                        //}
                        return  sender.send(data);
                    },
                    Err(e) => {
                        log::info!("didnt parsed {:?}", e);
                        println!("didnt parsed {:?}", e);
                        
                        sender.send(  ChannelData{data:vec!()} );
                        
                        
                        return
                    }
                };

        });
        return promise;
    }   

    pub fn list_channels(&self, path:String, filter:String) -> Promise<Channels> {
        
        let (sender, promise) = Promise::<Channels>::new();
        // self.base_url + "/list_channels/root/".to_owned(); //+ filter; 
            let fetch_url = format!("{}/list_channels/{}/{}", self.base_url, path, filter); 
            let request = ehttp::Request::get(&fetch_url);
            ehttp::fetch(request, move |response| {
                //ctx.forget_image(&prev_url);
                //ctx.request_repaint(); // wake up UI thread
                
                //response.text
                //let content_type = response.content_type().unwrap_or_default();
                //let got_text = response.text().to_owned();
                
                info!("XXX 1");
                let response_content = match response{
                    Ok(response_content) => response_content,
                    Err(e) => {
                        sender.send(  Channels{channels:vec!()} );
                        return
                    }
                };
                
                info!("XXX 2");
                let text_content = response_content.text().unwrap();
                //if let None = text_content {
                //    return;
                //}   
                
                info!("XXX 3");
                let parsed_channels = serde_json::from_str::<Channels>(&text_content);
                let channels = match parsed_channels{
                    Ok(channels) => {
                        log::info!("channsl {:?}", channels);
                        return  sender.send(channels);
                    },
                    Err(e) => {
                        log::info!("didnt parsed {:?}", e);
                        sender.send(  Channels{channels:vec!()} );
                        return
                    }
                };
                

                //let good_response = response?;
                //let p: Channels = serde_json::from_str(good_response.text())?;
                

                
                //let resource = response.map(|response| Resource::from_response(&ctx, response));
                //sender.send(resource);
            });
        return promise;
    }

    pub fn list_datasets(&self, filter:String) -> Promise<Datasets> {
        
        //! //("aaa");
        let (sender, promise) = Promise::<Datasets>::new();
        // self.base_url + "/list_channels/root/".to_owned(); //+ filter; 
            let fetch_url = format!("{}/list_datasets/{}", self.base_url, filter); 
            let request = ehttp::Request::get(&fetch_url);
            ehttp::fetch(request, move |response| {
                //ctx.forget_image(&prev_url);
                //ctx.request_repaint(); // wake up UI thread
                
                //response.text
                //let content_type = response.content_type().unwrap_or_default();
                //let got_text = response.text().to_owned();
                
                println!("XXX 1");
                let extracted_response_content = match response{
                    Ok(response_content) => response_content,
                    Err(e) => {
                        sender.send(  Datasets{datasets:vec!()} );
                        return;
                    }
                };
                
                println!("XXX 2");
                let text_content = extracted_response_content.text().unwrap();
                //if let None = text_content {
                //    return;
                //}   
                
                println!("XXX 3");
                let parsed_datasets = serde_json::from_str::<Datasets>(&text_content);
                let datasets = match parsed_datasets{
                    Ok(datasets) => {
                        println!("dataset {:?}", datasets);
                        return  sender.send(datasets);
                    },
                    Err(e) => {
                        println!("didnt parsed {:?}", e);
                        sender.send(  Datasets{datasets:vec!()} );
                        return
                    }
                };
                

                //let good_response = response?;
                //let p: Channels = serde_json::from_str(good_response.text())?;
                

                
                //let resource = response.map(|response| Resource::from_response(&ctx, response));
                //sender.send(resource);
            });
        return promise;
    }

}


/*
pub trait DataProvider{
    fn get_name(&self) -> String;
    fn list_channels(&self, path:String) -> Option<Vec<String>>;
    fn list_subcategories(&self, path: String) -> Option<Vec<String>>;
    fn get_range(&self, path: String, channel: String) -> Option<(f64, f64)>;
    fn get_data(&self, start: f64, end: f64) -> Option<PlotPoints>;
}



// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!



pub struct NativeDataProvider{
    pub url: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    channels: Option<Promise< Vec<String> >>,
}

impl DataProvider for NativeDataProvider {

    

    fn get_name(&self) -> String{
        return "".to_owned();
    }

    fn list_channels(&self, path:String) -> Option<Vec<String>> {
        
        if let None = self.channels{
            // we never listed channels
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get(&self.url);
            ehttp::fetch(request, move |response| {
                //ctx.forget_image(&prev_url);
                //ctx.request_repaint(); // wake up UI thread
                //let resource = response.map(|response| Resource::from_response(&ctx, response));
                //sender.send(resource);
            });
            self.channels = Some(promise);
        }else{

        }

        return self.channels; // We copy for now but not really care
    }
    
    fn list_subcategories(&self, path: String) -> Option<Vec<String>> {
        return None;
    }

    fn get_range(&self, path: String, channel: String) -> Option<(f64, f64)> {
        return None;
    }

    fn get_data(&self, start: f64, end: f64) -> Option<PlotPoints>{
        //return None;
        console_log::init_with_level(Level::Debug);
        info!("WAAAAZZZAAAAAP!!!");
        let request = ehttp::Request::get("http://127.0.0.1:8081/data/ch1");
        
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            //println!("Status code: {:?}", result.unwrap().status);
            info!("XXX {:?}", result.unwrap().text());
        });

        return None
    }
}

// trait ChannelDataProvider{
//     fn get_name() -> String;
//     fn get_range() -> (f64, f64);
//     fn get_data(start, end) -> ();
// }

 */