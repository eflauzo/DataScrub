
//pub trait BaseTab{
//    fn get_tab_title(&self) -> String;
//}

//use 
use std::boxed::Box;
use egui::Ui;

pub trait TabRenderer{
    fn ui(&mut self, ui: &mut Ui);
}



pub struct Tab{
    pub title: String,
    pub renderer: Box<dyn TabRenderer>
}

pub struct HelloWorldTab{

}

impl TabRenderer for HelloWorldTab{
    fn ui(&mut self, ui: &mut Ui){

        ui.label("Hello WOrld 1");
    }
}

pub fn create_hello_world_tab() -> Tab { 
    Tab{
        title: "tab1".to_string(),
        renderer: Box::new(HelloWorldTab{})
    }
}

//impl BaseTab{
//    fn get_tab_title(&self) -> String {
//        return self.title;
//    }
//}

//pub type TabType = dyn BaseTab;