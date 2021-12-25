use log::error;
#[macro_export] macro_rules! log_err {
    ($e:expr, $msg:expr) => {
        match $e {
            Ok(val)=> val,
            Err(err) => {
                error!("{}",format!("{} : {}",$msg, err));
            }
        }
    };
}



