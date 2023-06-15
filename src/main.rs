use std::process::Command;

use image::io::Reader as ImageReader;

use image::DynamicImage;

mod streamfunctions;
use streamfunctions::get_average_audio_levels;
use streamfunctions::get_stream_as_images;

use std::collections::HashMap;

#[tokio::main]
pub async fn main()  -> Result<(), Box<dyn std::error::Error>> {


    //a folder with the list of checks






    //MAIN LOOP
    loop {

        //get 10 streams that need to be checked

        //get the check results
        
        //put them back into the struct


        //get the structs that need attention called to them

        //create the notifications



    }


    let mut checker = Checker::new();

    checker.add_stream("test.webm".to_string(), "test".to_string() );


    let streamtocheck = checker.pop_stream_to_check().unwrap();

    println!("{}", streamtocheck);
    
    return Ok(());
}







pub enum CheckType{
    Volume(f32),
    Stillness(f32),
}


pub struct Check{
    stream: String,
    epochtime: u32,
    checktype: CheckType,
}

impl Check{




}


//the struct that manages the IPs check history, what needs to eb checked


pub struct Checker{

    //stream, nickname, check history
    streamstomonitor: Vec<String>,

    //stream to check history
    checkhistory: HashMap<String, Vec<Check> >,

}

impl Checker{

    pub fn new() -> Checker{
        Checker{
            streamstomonitor: Vec::new(),
            checkhistory: HashMap::new(),
        }
    }

    pub fn get_x_streams_for_checking(&self, x: usize) -> Vec<String>{

        //get x of the last checked streams


    }


    pub fn add_stream(&mut self, stream: String, nickname: String){
        self.streamstomonitor.push( (stream, nickname, CheckHistory::new()) );
    }

    //get the stream that is most in need of checking
    //right now, this is just gotten by getting the stream that was checked longest ago
    pub fn pop_stream_to_check(& self) -> Option<String>{

        let mut max = self.streamstomonitor.iter().max_by(|a, b| a.2.get_latest_check().cmp(&b.2.get_latest_check()));

        let max = match max{
            Some(x) => return Some( x.0.clone() ),
            None => return None,
        };
    }


}








// pub fn check(&self) -> (u32, CheckType){


//     // get_average_audio_levels("test.webm", 30.0).await;
    
//     // panic!("done");

//     // let images = get_stream_as_images("test.png", 10.0, 1.0).await;

//     // let mut count = 0;

//     // images.iter().for_each(|x|{
//     //     x.save(format!("tes{}.png", count)).unwrap();
//     //     count +=1;
//     // });


//     // for index in 0..images.len()-1{

//     //     let image_one = images[index].clone().into_rgb8();
//     //     let image_two = images[index+1].clone().into_rgb8();
//     //     let result = image_compare::rgb_hybrid_compare(&image_one, &image_two).expect("Images had different dimensions");

//     //     let imagetest = images[index].clone();

//     //     // let image_one = images[index].clone().into_luma8();
//     //     // let image_two = images[index+1].clone().into_luma8();
//     //     // let result = image_compare::gray_similarity_structure(&image_compare::Algorithm::MSSIMSimple, &image_one, &image_two).expect("Images had different dimensions");

//     //     println!("similarity {:?}", result.score);
//     // }

//     // panic!("done");

// }
