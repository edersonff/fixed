use crate::steam::backup_vdf;
use crate::steam::digits_before;
use crate::steam::find_subslice;
use crate::steam::vdf_string;

fn skip_cstring(data: &[u8], pos: usize) -> Option<usize> {

    let offset = data[pos..].iter().position(|&byte| byte == 0)?;

    Some(pos + offset + 1)

}

// A generic binary-VDF object walker: given the position right after an object's opening
// key-terminator, it consumes fields (nested object, string, int) until the matching 0x08 that
// closes THIS object, so it works regardless of how many fields an entry carries.
fn skip_vdf_object(data: &[u8], mut pos: usize) -> Option<usize> {

    loop {

        let tag = *data.get(pos)?;

        pos += 1;

        match tag {

            0x08 => return Some(pos),

            0x00 => {

                pos = skip_cstring(data, pos)?;

                pos = skip_vdf_object(data, pos)?;

            }

            0x01 => {

                pos = skip_cstring(data, pos)?;

                pos = skip_cstring(data, pos)?;

            }

            0x02 => {

                pos = skip_cstring(data, pos)?;

                pos = pos.checked_add(4)?;

                if pos > data.len() {

                    return None;

                }

            }

            _ => return None,

        }

    }

}

fn entry_header_start(data: &[u8], appid_marker_pos: usize) -> Option<usize> {

    let digits_end = appid_marker_pos.checked_sub(1)?;

    let start = digits_before(data, digits_end)?;

    start.checked_sub(1)

}

// Steam expects the shortcut indices to be a contiguous 0..N key space; leaving a gap after a
// delete is undefined for its own writer, so every remaining entry is renumbered in place.
fn renumber_entries(data: &[u8]) -> Vec<u8> {

    let marker: &[u8] = b"\x02appid\x00";

    let mut out = Vec::with_capacity(data.len());

    let mut cursor = 0usize;

    let mut index = 0u32;

    let mut pos = 0usize;

    while let Some(found) = find_subslice(data, marker, pos) {

        if let Some(digits_end) = found.checked_sub(1) {

            if let Some(start) = digits_before(data, digits_end) {

                out.extend_from_slice(&data[cursor..start]);

                out.extend_from_slice(index.to_string().as_bytes());

                cursor = digits_end;

                index += 1;

            }

        }

        pos = found + marker.len();

    }

    out.extend_from_slice(&data[cursor..]);

    out

}

pub fn remove_steam_shortcut(vdf_path: &str, app_name: &str) -> Result<bool, String> {

    let data = std::fs::read(vdf_path).map_err(|error| format!("read {}: {}", vdf_path, error))?;

    let name_marker = vdf_string("AppName", app_name);

    let Some(found) = find_subslice(&data, &name_marker, 0) else {

        return Ok(false);

    };

    let seg_start = found.saturating_sub(160);

    let appid_marker: &[u8] = b"\x02appid\x00";

    let Some(relative) = find_subslice(&data[seg_start..found], appid_marker, 0) else {

        return Ok(false);

    };

    let appid_pos = seg_start + relative;

    let Some(header_start) = entry_header_start(&data, appid_pos) else {

        return Ok(false);

    };

    let Some(entry_end) = skip_vdf_object(&data, appid_pos) else {

        return Ok(false);

    };

    let mut trimmed = Vec::with_capacity(data.len());

    trimmed.extend_from_slice(&data[..header_start]);

    trimmed.extend_from_slice(&data[entry_end..]);

    let renumbered = renumber_entries(&trimmed);

    backup_vdf(vdf_path, &data)?;

    std::fs::write(vdf_path, &renumbered).map_err(|error| format!("write {}: {}", vdf_path, error))?;

    Ok(true)

}

#[cfg(test)]
#[path = "steam_remove_tests.rs"]
mod steam_remove_tests;
