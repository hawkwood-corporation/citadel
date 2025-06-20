use crate::sections::*;

struct Page {
    slug: String,
    title: String,
    sections: Vec<Section>,  // Which sections this page needs
    // page-specific data
}