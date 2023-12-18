const IMAGE_DATA: &'static [u8] = include_bytes!("../../assets/header.png");
const TITLE: &str = "
    ██╗  ██╗██████╗  █████╗ ███╗   ███╗██████╗ ██╗   ██╗███████╗
    ██║ ██╔╝██╔══██╗██╔══██╗████╗ ████║██╔══██╗██║   ██║██╔════╝
    █████╔╝ ██████╔╝███████║██╔████╔██║██████╔╝██║   ██║███████╗
    ██╔═██╗ ██╔══██╗██╔══██║██║╚██╔╝██║██╔═══╝ ██║   ██║╚════██║
    ██║  ██╗██║  ██║██║  ██║██║ ╚═╝ ██║██║     ╚██████╔╝███████║
    ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝      ╚═════╝ ╚══════╝
";

pub fn generate_header(term_width: usize) -> String {
    let art = img_to_ascii(IMAGE_DATA, 32, 16);

    return format!("{}{}\n",
                   center_txt(TITLE, term_width),
                   center_txt(art.as_str(), term_width)
    );
}

fn center_txt(art: &str, term_width: usize) -> String {
    let art_width = art.lines().map(|line| line.chars().count()).max().unwrap_or(0);
    let padding = if term_width > art_width {
        (term_width - art_width) / 2
    } else {
        0
    };

    return art.lines()
        .map(|line| format!("{:padding$}{}", "", line, padding = padding))
        .collect::<Vec<String>>()
        .join("\n");
}

fn img_to_ascii(img_data: &[u8], width: u32, height: u32) -> String {
    let ascii_chars = ['█', ' '];

    let img = image::load_from_memory(img_data)
        .expect("Error opening image");
    let formatted_img = img
        .resize_exact(width, height, image::imageops::FilterType::Nearest)
        .to_rgba8();

    let mut ascii_art = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = formatted_img.get_pixel(x, y);
            let avg = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;

            let value = if pixel[3] == 0 { 255 } else { avg };

            let index = (value as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
            ascii_art.push(ascii_chars[index]);
        }
        ascii_art.push('\n');
    }

    return ascii_art;
}
