pub(crate) fn vdf_string(key: &str, value: &str) -> Vec<u8> {

    let mut out = vec![1u8];

    out.extend_from_slice(key.as_bytes());

    out.push(0);

    out.extend_from_slice(value.as_bytes());

    out.push(0);

    out

}

pub(crate) fn vdf_int(key: &str, value: u32) -> Vec<u8> {

    let mut out = vec![2u8];

    out.extend_from_slice(key.as_bytes());

    out.push(0);

    out.extend_from_slice(&value.to_le_bytes());

    out

}

pub(crate) fn find_subslice(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {

    if needle.is_empty() || from >= haystack.len() {

        return None;

    }

    haystack[from..]

        .windows(needle.len())

        .position(|window| window == needle)

        .map(|offset| offset + from)

}

// A digit run only counts as an entry index if it is bounded by 0x00 on both sides — a digit
// adjacent to anything else belongs to some other field, not an index.
pub(crate) fn digits_before(data: &[u8], end: usize) -> Option<usize> {

    if data.get(end)? != &0u8 {

        return None;

    }

    let mut start = end;

    while start > 0 && data[start - 1].is_ascii_digit() {

        start -= 1;

    }

    if start == end || start == 0 || data[start - 1] != 0u8 {

        return None;

    }

    Some(start)

}

// shortcuts.vdf holds every shortcut the user ever made by hand; a bad write loses all of them and
// Steam offers no undo. The rollback copy is written before the mutation or the mutation is refused.
pub(crate) fn backup_vdf(vdf_path: &str, original: &[u8]) -> Result<(), String> {

    let backup_path = format!("{}.bak-fixed", vdf_path);

    std::fs::write(&backup_path, original)

        .map_err(|error| format!("backup {}: {}", backup_path, error))

}

#[cfg(test)]
#[path = "vdf_tests.rs"]
mod vdf_tests;
