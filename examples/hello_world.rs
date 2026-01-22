use wgc::*;

fn main() -> anyhow::Result<()> {
    let item = match new_item_with_picker() {
        Ok(val) => val,
        Err(CaptureItemPickerFailed::NoItemSelected) => {
            eprintln!("No item selected");
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    println!("{item:?} {:?} {:?}", item.DisplayName(), item.Size());

    Ok(())
}
