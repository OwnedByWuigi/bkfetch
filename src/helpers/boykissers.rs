use std::collections::HashMap;
pub struct boykisser {
    pub text: String,
    pub lines: u16
}

pub fn get_boykisser(name: String) -> Option<boykisser> {
    let boykissers: HashMap<&str, Vec<u8>> = HashMap::from([
        ("withhighthighs", include_bytes!("../../boykissers/withhighthighs.txt").to_vec()),
        ("howyoulook", include_bytes!("../../boykissers/howyoulook.txt").to_vec()),
        #[cfg(target_os = "linux")]
        ("ahhhaah", include_bytes!("../../boykissers/ahhhaah.txt").to_vec()),
        ("cute", include_bytes!("../../boykissers/cute.txt").to_vec()),
        ("cutereversed", include_bytes!("../../boykissers/cutereversed.txt").to_vec()),
        ("cutie", include_bytes!("../../boykissers/cutie.txt").to_vec()),
        ("sad", include_bytes!("../../boykissers/sad.txt").to_vec()),
        ("sowhat", include_bytes!("../../boykissers/sowhat.txt").to_vec()),
        ("squinting", include_bytes!("../../boykissers/squinting.txt").to_vec()),
        ("thesilly_large", include_bytes!("../../boykissers/thesilly_large.txt").to_vec()),
        ("thesilly", include_bytes!("../../boykissers/thesilly.txt").to_vec()),
        ("typing", include_bytes!("../../boykissers/typing.txt").to_vec()),
        ("withhighthighsalt", include_bytes!("../../boykissers/withhighthighsalt.txt").to_vec()),
        ("yayyy", include_bytes!("../../boykissers/yayyy.txt").to_vec()),
        ("yippie", include_bytes!("../../boykissers/yippie.txt").to_vec()),
        ("youafurry", include_bytes!("../../boykissers/youafurry.txt").to_vec()),
        ("youlikeboys", include_bytes!("../../boykissers/youlikeboys.txt").to_vec()),
        ("youlikeboysfullbody", include_bytes!("../../boykissers/youlikeboysfullbody.txt").to_vec()),
    ]);

    let boykisser = String::from_utf8(
            boykissers.get(
            name.as_str()
        ).unwrap().to_vec()
    ).unwrap();

    Some(boykisser {
        text: boykisser.clone(),
        lines: boykisser.split("\n").count() as u16
    })
}
