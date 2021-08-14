pub const IMG_SIZE: u32 = 256;

pub mod middle_img {
    pub const W: u32 = 149;
    pub const H: u32 = 211;
    pub const X: u32 = 31;
    pub const Y: u32 = 16;
}

pub mod anilist {
    pub const QUERY: &str = "
        query ($search: String){ 
            Media(search: $search, type:ANIME) { 
                id 
                coverImage {
                    extraLarge
                }
            }
        }
        ";
}
