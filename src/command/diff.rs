use crate::interface::Interface;

pub fn diff() -> anyhow::Result<()> {
    let interface = Interface::create_from_env()?;
    let (from_list, query_list) = interface.get_deleting_address()?;

    println!("フィルタ側のみに存在するfromのアドレス");

    for address in from_list.iter() {
        println!("- {}", address);
    }

    println!("フィルタ側のみに存在するqueryのアドレス");

    for address in query_list.iter() {
        println!("- {}", address);
    }
    Ok(())
}
