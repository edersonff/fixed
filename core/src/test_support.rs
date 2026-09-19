use std::sync::Mutex;

// HOME is process-wide state; tests across different modules that redirect it to a temp
// directory run on separate threads under `cargo test` and corrupt each other without this lock.
pub(crate) static HOME_ENV_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn empty_shortcuts_vdf() -> Vec<u8> {

    let mut data = Vec::new();

    data.push(0);

    data.extend_from_slice(b"shortcuts");

    data.push(0);

    data.push(8);

    data.push(8);

    data

}

// Production code writes binary VDF but never needs to parse it back, so this walker lives in
// test code only.
pub(crate) enum VdfNode {

    Map(String, Vec<VdfNode>),

    Leaf(String),

}

fn node_key(node: &VdfNode) -> &str {

    match node {

        VdfNode::Map(key, _) => key,

        VdfNode::Leaf(key) => key,

    }

}

fn read_cstring(data: &[u8], pos: &mut usize) -> String {

    let start = *pos;

    while data[*pos] != 0 {

        *pos += 1;

    }

    let value = String::from_utf8_lossy(&data[start..*pos]).into_owned();

    *pos += 1;

    value

}

fn parse_map(data: &[u8], pos: &mut usize) -> Vec<VdfNode> {

    let mut nodes = Vec::new();

    loop {

        let tag = data[*pos];

        *pos += 1;

        match tag {

            0x08 => return nodes,

            0x00 => {

                let key = read_cstring(data, pos);

                let children = parse_map(data, pos);

                nodes.push(VdfNode::Map(key, children));

            }

            0x01 => {

                let key = read_cstring(data, pos);

                read_cstring(data, pos);

                nodes.push(VdfNode::Leaf(key));

            }

            0x02 => {

                let key = read_cstring(data, pos);

                *pos += 4;

                nodes.push(VdfNode::Leaf(key));

            }

            other => panic!("unexpected vdf tag {other}"),

        }

    }

}

pub(crate) fn top_level_keys(data: &[u8]) -> Vec<String> {

    let mut pos = 0usize;

    parse_map(data, &mut pos).iter().map(|node| node_key(node).to_string()).collect()

}

pub(crate) fn keys_under(data: &[u8], key: &str) -> Vec<String> {

    let mut pos = 0usize;

    let nodes = parse_map(data, &mut pos);

    nodes.iter()

        .find_map(|node| match node {

            VdfNode::Map(found_key, children) if found_key.as_str() == key => Some(children),

            _ => None,

        })

        .map(|children| children.iter().map(|node| node_key(node).to_string()).collect())

        .unwrap_or_default()

}
