#[macro_use]
pub mod macro_code;
use chrono::Local;
use env_logger::{Builder, Logger};
use log::{Log, Metadata, Record, SetLoggerError, Level};
use once_cell::sync::OnceCell;
use std::{cell::RefCell, collections::VecDeque, fs::{self, OpenOptions}, io::{BufWriter, Write}, path::Path, sync::Mutex};
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor, StandardStream};
pub use log::{error, info, debug, warn, LevelFilter};

static LOG: OnceCell<Mutex<VecDeque<CurrentLog>>> = OnceCell::with_value(Mutex::new(VecDeque::new()));

#[derive(Clone, Debug)]
pub struct CurrentLog
{
    level : LevelFilter,
    text: String
}
pub struct StructLogger 
{
    pub logger: Logger,
    pub level_filter: LevelFilter,
}

impl StructLogger  
{
    fn new(level : LevelFilter) -> StructLogger  
    {
        let mut builder = Builder::default();
        builder.filter_level(level);
        StructLogger  
        {
            logger: builder.build(),
            level_filter: level,
            
        }
    }

    pub fn init(level : LevelFilter) -> Result<(), SetLoggerError> 
    {
        let logger = Self::new(level);
        log::set_max_level(logger.level_filter);
        log::set_boxed_logger(Box::new(logger))
    }

    pub fn initialize_logger() -> bool
    {
        let logger = StructLogger::init(log::LevelFilter::Debug);
        if let Ok(_) = logger
        {
            return true;
        }
        else
        {
            println!("Ошибка инициализации логгера! {}", logger.err().unwrap());
            return false;
        }
    }
    pub fn get_current_log() -> VecDeque<CurrentLog>
    {
        let guard = LOG.get().unwrap().lock().unwrap();
        guard.clone()
    }
    pub fn add(level: LevelFilter, text: &str)
    {
        let mut guard = LOG.get().unwrap().lock().unwrap();
        if guard.len() > 1000
        {
            guard.pop_back();
            guard.push_front(CurrentLog{level, text: text.to_owned()});
        }
        else
        {
            guard.push_front(CurrentLog{level, text: text.to_owned()});
        }
    }
}

impl Log for StructLogger 
{
    fn enabled(&self, metadata: &Metadata) -> bool 
    {
        self.logger.enabled(metadata)
    }

    fn log(&self, record: &Record) 
    {
        // Check if the record is matched by the logger before logging
        if self.logger.matches(record) 
        {

            let log_file = [log_name(), ".log".to_owned()].concat();
            let level = record.level().to_level_filter();
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let (record_level, color, log_path) = match level
            {
                LevelFilter::Error => ("❌ Ошибка", Color::Red, &log_file),
                LevelFilter::Info => ("💬 Информация", Color::Green, &log_file),
                LevelFilter::Warn => ("⚠ Предупреждение", Color::Rgb(255, 153, 51), &log_file),
                LevelFilter::Debug => ("⚙ DEBUG", Color::Rgb(255, 103, 51), &log_file),
                                _ => ("❓ УРОВЕНЬ НЕ ОПРЕДЕЛЕН", Color::Magenta, &log_file)
            };
            _ = stdout.set_color(ColorSpec::new().set_fg(Some(color)));

            let output = format!("{}:{} -> {}", date_now(), record_level, record.args());
            println!("{}", &output);
            write_to_file(&output, log_path);
            StructLogger::add(level, &output);
            _ = stdout.reset();
        }
    }
    fn flush(&self) {}
}

fn date_now() -> String
{
    let time_now = Local::now();
    time_now.format("[%d-%m-%Y %H:%M:%S]").to_string()
}
fn log_name() -> String
{
    let time_now = Local::now();
    time_now.format("%d-%m-%Y").to_string()
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
    use log::{error, info, warn, debug};
    use crate::StructLogger;

    #[test]
    pub fn test_info()
    {
        StructLogger::init(log::LevelFilter::Debug).unwrap();
        debug!("{}", "дебаггер сообщает!");
        error!("{}", "Ошибка! ошииибка!!!");
        info!("{}", "В целях информации предлагаем вам ознакомиться");
        warn!("{}", "Предупреждаю в последний раз!");
    }

    #[test]
    pub fn test_info_2()
    {
        StructLogger::initialize_logger();
        info!("{}", "Предупреждаю в последний раз1!");
        warn!("{}", "Предупреждаю в последний раз2!");
        error!("{}", "Предупреждаю в последний раз3!");
        warn!("{:?}", StructLogger::get_current_log());
    }
}