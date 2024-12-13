#[macro_use]
pub mod macro_code;
use chrono::{Days, Local};
//use env_logger::{Builder, Logger};
use log::{Log, Metadata, Record, SetLoggerError, Level};
//use once_cell::sync::OnceCell;
use std::{cell::{OnceCell, RefCell}, collections::HashMap, fs::{self, OpenOptions}, io::{BufWriter, Write}, path::{Path, PathBuf}, sync::{Mutex, RwLock}};
//use termcolor::{Color, ColorChoice, ColorSpec, WriteColor, StandardStream};
pub use log::{error, info, debug, warn, LevelFilter};
pub use fern::InitError;
use fern::colors::{Color, ColoredLevelConfig};
//use owo_colors::{OwoColorize, Style};
//static LOG: OnceCell<Mutex<VecDeque<CurrentLog>>> = OnceCell::with_value(Mutex::new(VecDeque::new()));

pub struct StructLogger {}

impl StructLogger  
{
    ///for custom loggint on other crates use `custom_logging_for_crates` with `crate_name`, `log_level` params
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
                // let colors_line = ColoredLevelConfig::new()
                // .error(Color::Red)
                // .warn(Color::Yellow)
                // // we actually don't need to specify the color for debug and info, they are white by default
                // .info(Color::White)
                // .debug(Color::BrightBlue)
                // // depending on the terminals color scheme, this is the same as the background color
                // .trace(Color::BrightBlack);
                //let log_file = [log_name(), ".log".to_owned()].concat();
                // let red_style = Style::new()
                // .red();
    
                // let green_style = Style::new()
                // .green();
    
                // let module_style = Style::new()
                // .truecolor(24, 216, 152)
                // .on_black();  
    
                // let warn_style = Style::new()
                // .truecolor(255, 153, 51);
    
                // let debug_style = Style::new()
                // .truecolor(255, 103, 51);
    
                // let default_style = Style::new()
                // .white()
                // .on_black();
            
                
                //let color = colors_line.get_color(&record.level()).to_fg_str();
                // let (record_level, color) = match record.level()
                // {
                //     Level::Error => ("❌ Ошибка", red_style),
                //     Level::Info => ("💬 Информация", green_style),
                //     Level::Warn => ("⚠ Предупреждение", warn_style),
                //     Level::Debug => ("⚙ DEBUG", debug_style),
                //     _ => ("❓ УРОВЕНЬ НЕ ОПРЕДЕЛЕН", default_style)
                // };
                
                let (prefix, color) = match record.level()
                {
                    Level::Error => ("❌ Ошибка", Color::Red),
                    Level::Info => ("💬 Информация", Color::White),
                    Level::Warn => ("⚠ Предупреждение", Color::TrueColor { r: 254, g: 145, b: 109 }),
                    Level::Debug => ("⚙ DEBUG", Color::TrueColor { r: 129, g: 35, b: 3 }),
                    Level::Trace => ("Trace:", Color::BrightBlack),
                    _ =>            ("❓ УРОВЕНЬ НЕ ОПРЕДЕЛЕН", Color::BrightGreen)
                };
                out.finish(format_args!(
                    "\x1B[{}m{}-[{}:{}]:{} -> {}\x1B[0m", color.to_fg_str(), date_now(), record.target(), record.line().unwrap_or(0), prefix, record.args()
                ))
                
            }).chain(std::io::stdout()))
        .chain(
            fern::Dispatch::new()
            .format(|out, message, record| 
            {
                //let log_file = [log_name(), ".log".to_owned()].concat();
                let prefix = match record.level()
                {
                    Level::Error => "❌ Ошибка",
                    Level::Info => "💬 Информация",
                    Level::Warn => "⚠ Предупреждение",
                    Level::Debug => "⚙ DEBUG",
                    _ => "❓ УРОВЕНЬ НЕ ОПРЕДЕЛЕН"
                };
                out.finish(format_args!(
                    "{}-[{}:{}]:{} -> {}", date_now(), record.target(), record.line().unwrap_or(0), prefix, record.args()
                ))
                // "{}-[{}:{}]:{} -> {}", date_now().style(color), record.target().style(module_style), record.line().unwrap_or(0).style(module_style), record_level.style(color), record.args().style(color)
            })
            .chain(
            {
                fern::DateBased::new("logs/", "%Y-%m-%d.log")
            }))
        .apply()?;
        Ok(())
    }


    pub fn new_default() -> Result<(), fern::InitError>
    {
        Self::new_custom(log::LevelFilter::Debug, None)
    }
}


fn date_now() -> String
{
    let time_now = Local::now();
    time_now.format("[%d-%m-%Y %H:%M:%S]").to_string()
}


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
    use log::{debug, error, info, trace, warn};
    use crate::StructLogger;

    #[test]
    pub fn test_info()
    {
        StructLogger::new_default();
        debug!("{}", "дебаггер сообщает!");
        error!("{}", "Ошибка! ошииибка!!!");
        info!("{}", "В целях информации предлагаем вам ознакомиться");
        warn!("{}", "Предупреждаю в последний раз!");
        trace!("{}", "просто трейс... 123")
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