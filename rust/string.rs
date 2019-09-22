use std::ops::Deref;
use std::{thread, time};
//方案一
fn get_new_strings(str1: Vec<String>, str2: Vec<String>) -> Vec<String> {
    let mut strs: Vec<String> = Vec::new();
    for s in &str1 {
        for v in &str2 {
            //let temp: String = s + &v;
            let temp: String = (s.deref()).to_string() + v;
            strs.push(temp);
        }
    }
    strs
}
//方案二
fn get_new_strs<'a>(str1: &Vec<&'a str>, str2: &Vec<&'a str>) -> Vec<String> {
    let mut strs: Vec<String> = Vec::new();
    for s in str1 {
        for v in str2 {
            let temp: String = s.to_string() + v;
            strs.push(temp);
        }
    }
    strs
}

fn main() {
    let sleep_seconds = std::time::Duration::from_secs(1000);
    let mut data = Vec::new();
    let data_0 = vec!["1", "2", "3"];
    let data_1 = vec!["a", "b", "c", "d"];
    let data_2 = vec!["e", "f", "g"];
    let data_3 = vec!["h", "i", "j", "k"];
    let data_4 = vec!["x", "y", "z"];
    data.push(data_1);
    data.push(data_2);
    data.push(data_3);
    data.push(data_4);
    let mut outdata: Vec<String> = data_0.iter().map(|x| x.to_string()).collect();
    for _dt in data {
        //Vec<String>=>Vec<&str>
        let temp = outdata.clone();
        let _temp: Vec<&str> = temp.iter().map(|x| x.deref()).collect();
        outdata = get_new_strs(&_temp, &_dt);
    }
    println!("outdata:{:?}, count:{:?}", outdata, outdata.len());
    thread::sleep(sleep_seconds);
}

