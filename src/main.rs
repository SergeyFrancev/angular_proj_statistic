#[macro_use] extern crate cli_log;

fn main() -> Result<(), ng_stat::NgStatError>  {
    init_cli_log!("ng-stat");
    ng_stat::run()?;
    info!("bye");
    Ok(())
}
