use crate::digits_before;
use crate::find_subslice;
use crate::vdf_string;

pub fn find_shortcut_appid(vdf_path: &str, app_name: &str) -> Option<u32> {

    let data = std::fs::read(vdf_path).ok()?;

    let name_marker = vdf_string("AppName", app_name);

    let found = find_subslice(&data, &name_marker, 0)?;

    let seg_start = found.saturating_sub(160);

    let seg = &data[seg_start..found];

    let marker = b"\x02appid\x00";

    let idx = find_subslice(seg, marker, 0)?;

    let value_start = idx + marker.len();

    if value_start + 4 > seg.len() {

        return None;

    }

    let bytes: [u8; 4] = [

        seg[value_start],

        seg[value_start + 1],

        seg[value_start + 2],

        seg[value_start + 3],

    ];

    Some(u32::from_le_bytes(bytes))

}

pub fn find_shortcut_appid_by_exe(vdf_path: &str, exe_path: &str) -> Option<u32> {

    let data = std::fs::read(vdf_path).ok()?;

    let exe_marker = vdf_string("Exe", &format!("\"{}\"", exe_path));

    let found = find_subslice(&data, &exe_marker, 0)?;

    let seg_start = found.saturating_sub(320);

    let seg = &data[seg_start..found];

    let marker = b"\x02appid\x00";

    let idx = find_subslice(seg, marker, 0)?;

    let value_start = idx + marker.len();

    if value_start + 4 > seg.len() {

        return None;

    }

    let bytes: [u8; 4] = [

        seg[value_start],

        seg[value_start + 1],

        seg[value_start + 2],

        seg[value_start + 3],

    ];

    Some(u32::from_le_bytes(bytes))

}

fn entry_index_at(data: &[u8], marker_start: usize) -> Option<u32> {

    let digits_end = marker_start.checked_sub(1)?;

    let start = digits_before(data, digits_end)?;

    std::str::from_utf8(&data[start..digits_end]).ok()?.parse().ok()

}

pub(crate) fn max_entry_index(data: &[u8]) -> u32 {

    let marker: &[u8] = b"\x02appid\x00";

    let mut max: u32 = 0;

    let mut pos = 0;

    while let Some(found) = find_subslice(data, marker, pos) {

        if let Some(index) = entry_index_at(data, found) {

            if index > max {

                max = index;

            }

        }

        pos = found + marker.len();

    }

    max

}

#[cfg(test)]
#[path = "shortcut_lookup_tests.rs"]
mod shortcut_lookup_tests;

pub fn find_shortcut_appids_by_exe(vdf_path: &str, exe_path: &str) -> Vec<u32> {

    let Ok(data) = std::fs::read(vdf_path) else {

        return Vec::new();

    };

    let exe_marker = vdf_string("Exe", &format!("\"{}\"", exe_path));

    let marker = b"\x02appid\x00";

    let mut appids: Vec<u32> = Vec::new();

    let mut from = 0;

    while let Some(found) = find_subslice(&data, &exe_marker, from) {

        let seg_start = found.saturating_sub(320);

        let seg = &data[seg_start..found];

        if let Some(idx) = find_subslice(seg, marker, 0) {

            let value_start = idx + marker.len();

            if value_start + 4 <= seg.len() {

                let bytes: [u8; 4] = [

                    seg[value_start],

                    seg[value_start + 1],

                    seg[value_start + 2],

                    seg[value_start + 3],

                ];

                let appid = u32::from_le_bytes(bytes);

                if !appids.contains(&appid) {

                    appids.push(appid);

                }

            }

        }

        from = found + exe_marker.len();

    }

    appids

}
