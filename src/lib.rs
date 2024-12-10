#[macro_use]
pub mod macro_code;
use chrono::{Days, Local};
//use env_logger::{Builder, Logger};
use log::{Log, Metadata, Record, SetLoggerError, Level};
use rand::Rng;
//use once_cell::sync::OnceCell;
use std::{cell::{OnceCell, RefCell}, collections::HashMap, fs::{self, OpenOptions}, io::{BufWriter, Write}, path::{Path, PathBuf}, sync::{Mutex, RwLock}};
//use termcolor::{Color, ColorChoice, ColorSpec, WriteColor, StandardStream};
pub use log::{error, info, debug, warn, LevelFilter};
pub use fern::InitError;
use owo_colors::{OwoColorize, Style};
//static LOG: OnceCell<Mutex<VecDeque<CurrentLog>>> = OnceCell::with_value(Mutex::new(VecDeque::new()));

pub struct StructLogger {}

impl StructLogger  
{
    pub fn new_custom(level: log::LevelFilter, custom_logging_for_crates: Option<&[(&str, LevelFilter)]>) -> Result<(), fern::InitError>
    {
        let path = Path::new("logs");
        let _create_dir = fs::create_dir_all(&path);
        let mut config = fern::Dispatch::new()
        .level(level);
        if let Some(custom) = custom_logging_for_crates
        {
            for cl in custom
            {
                config = config.level_for(
                    format!("{}", cl.0),
                    cl.1,
                );
            }
        }
        //Делаем вывод для терминала и для файла, а файле ansi коды не нужны
        config
        .chain(
            fern::Dispatch::new()
            .format(|out, message, record| 
            {
                //let log_file = [log_name(), ".log".to_owned()].concat();
                let red_style = Style::new()
                .red();
    
                let green_style = Style::new()
                .green();
    
                let module_style = Style::new()
                .truecolor(24, 216, 152)
                .on_black();  
    
                let warn_style = Style::new()
                .truecolor(255, 153, 51);
    
                let debug_style = Style::new()
                .truecolor(255, 103, 51);
    
                let default_style = Style::new()
                .white()
                .on_black();
                let (record_level, color) = match record.level()
                {
                    Level::Error => ("❌ Ошибка", red_style),
                    Level::Info => ("💬 Информация", green_style),
                    Level::Warn => ("⚠ Предупреждение", warn_style),
                    Level::Debug => ("⚙ DEBUG", debug_style),
                    _ => ("❓ УРОВЕНЬ НЕ ОПРЕДЕЛЕН", default_style)
                };
                out.finish(format_args!(
                    "{}-[{}:{}]:{} -> {}", date_now().style(color), record.target().style(module_style), record.line().unwrap_or(0).style(module_style), record_level.style(color), record.args().style(color)
                ))
                
            }).chain(std::io::stdout()))
        .chain(
            fern::Dispatch::new()
            .format(|out, message, record| 
            {
                //let log_file = [log_name(), ".log".to_owned()].concat();
                let record_level = match record.level()
                {
                    Level::Error => "❌ Ошибка",
                    Level::Info => "💬 Информация",
                    Level::Warn => "⚠ Предупреждение",
                    Level::Debug => "⚙ DEBUG",
                    _ => "❓ УРОВЕНЬ НЕ ОПРЕДЕЛЕН"
                };
                out.finish(format_args!(
                    "{}-[{}:{}]:{} -> {}", date_now(), record.target(), record.line().unwrap_or(0), record_level, record.args()
                ))
            })
            .chain(
        {
                    //let time_now = Local::now().checked_add_days(Days::new(2)).unwrap();
                    //let log_file_name = time_now.format("%d-%m-%Y").to_string();
                    //let log_name = [log_name(), ".log".to_owned()].concat();
                    //let path = Path::new("logs").join(&log_name);
                    //let _create_dir = fs::create_dir_all(&path.parent().unwrap());
                   
                    fern::DateBased::new("logs/", "%Y-%m-%d.log")
                    //fern::log_reopen1(log_path().as_path(), op.unwrap())?
                    //fern::log_file(log_path())?
                }))
        .apply()?;
        Ok(())
    }


    pub fn new_default() -> Result<(), fern::InitError>
    {
        Self::new_custom(log::LevelFilter::Debug, None)
    }
}

// impl Log for StructLogger 
// {
//     fn enabled(&self, metadata: &Metadata) -> bool 
//     {
//         self.logger.enabled(metadata)
//     }

//     fn log(&self, record: &Record) 
//     {
//         // Check if the record is matched by the logger before logging
//         if self.logger.matches(record) 
//         {

//             let log_file = [log_name(), ".log".to_owned()].concat();
//             let level = record.level().to_level_filter();
//             let mut stdout = StandardStream::stdout(ColorChoice::Always);
//             let (record_level, color, log_path) = match level
//             {
//                 LevelFilter::Error => ("❌ Ошибка", Color::Red, &log_file),
//                 LevelFilter::Info => ("💬 Информация", Color::Green, &log_file),
//                 LevelFilter::Warn => ("⚠ Предупреждение", Color::Rgb(255, 153, 51), &log_file),
//                 LevelFilter::Debug => ("⚙ DEBUG", Color::Rgb(255, 103, 51), &log_file),
//                                 _ => ("❓ УРОВЕНЬ НЕ ОПРЕДЕЛЕН", Color::Magenta, &log_file)
//             };
//             _ = stdout.set_color(ColorSpec::new().set_fg(Some(color)));

//             let output = format!("{}:{} -> {}", date_now(), record_level, record.args());
//             println!("{}", &output);
//             write_to_file(&output, log_path);
//             StructLogger::add(level, &output);
//             _ = stdout.reset();
//         }
//     }
//     fn flush(&self) {}
// }


fn date_now() -> String
{
    let time_now = Local::now();
    time_now.format("[%d-%m-%Y %H:%M:%S]").to_string()
}

// fn log_path() -> PathBuf
// {
   
//     let mut rng = rand::thread_rng();
//     let day: u64 = rng.gen_range(1..20); // generates a float betwe
//     let time_now = Local::now().checked_add_days(Days::new(day)).unwrap();
//     let name = time_now.format("%d-%m-%Y").to_string();
//     let log_name = [name, ".log".to_owned()].concat();
//     let path = Path::new("logs").join(&log_name);
//     let _create_dir = fs::create_dir_all(&path.parent().unwrap());
//     path
// }

fn write_to_file(s: &String, path : &str)
{
    let path = Path::new("logs").join(path);
    let _create_dir = fs::create_dir_all(&path.parent().unwrap());
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .expect(format!("немогу открыть файл {} для записи лога", &path.display()).as_str());
    let len = f.metadata().expect("не могу просмотреть метадату лог файла").len();
    if len > 1000000000
    {
        _ = fs::remove_file(&path);
        f = OpenOptions::new()
        .append(true)
        .create(true) 
        .open(&path)
        .expect(format!("немогу открыть файл {} для записи лога", &path.display()).as_str());
    }
    
    let mut f = BufWriter::new(f);
    f.write_all(s.as_bytes()).expect(format!("немогу записать строку {} в файл {}", s, &path.display()).as_str());
    f.write_all(b"\n").expect("ошибка записи новой строки в файл лога.");
}

#[cfg(test)]
mod test
{
    use log::{error, info, warn, debug};
    use crate::StructLogger;

    #[test]
    pub fn test_info()
    {
        StructLogger::new_default();
        debug!("{}", "дебаггер сообщает!");
        error!("{}", "Ошибка! ошииибка!!!");
        info!("{}", "В целях информации предлагаем вам ознакомиться");
        warn!("{}", "Предупреждаю в последний раз!");
    }

    #[test]
    pub fn test_info_2()
    {
        StructLogger::new_default();
        info!("{}", "Предупреждаю в последний раз1!");
        warn!("{}", "Предупреждаю в последний раз2!");
        error!("{}", "Предупреждаю в последний раз3!");
    }
}