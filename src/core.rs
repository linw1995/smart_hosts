use std::{
    collections::HashMap,
    io::{Seek, SeekFrom},
    net::Ipv4Addr,
};

use deku::prelude::*;
use tracing::trace;

#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(ctx = "names: &mut HashMap<String, u8>")]
pub struct DnsPacket {
    pub header: DnsHeader,
    #[deku(
        reader = "questions_read(deku::reader, header.qdcount, names)",
        writer = "questions_write(deku::writer, &self.questions, names)"
    )]
    pub questions: Vec<DnsQuestion>,
    #[deku(
        reader = "records_read(deku::reader, header.ancount, names)",
        writer = "records_write(deku::writer, &self.answers, names)"
    )]
    pub answers: Vec<DnsRecord>,
    #[deku(
        reader = "records_read(deku::reader, header.nscount, names)",
        writer = "records_write(deku::writer, &self.authorities, names)"
    )]
    pub authorities: Vec<DnsRecord>,
    #[deku(
        reader = "records_read(deku::reader, header.arcount, names)",
        writer = "records_write(deku::writer, &self.additional, names)"
    )]
    pub additional: Vec<DnsRecord>,
}

fn questions_read<R: std::io::Read + std::io::Seek>(
    reader: &mut Reader<R>,
    count: u16,
    names: &mut HashMap<String, u8>,
) -> Result<Vec<DnsQuestion>, DekuError> {
    let mut ans = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let q = DnsQuestion::from_reader_with_ctx(reader, names)?;
        ans.push(q);
    }
    Ok(ans)
}

fn questions_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    value: &Vec<DnsQuestion>,
    names: &mut HashMap<String, u8>,
) -> Result<(), DekuError> {
    for q in value {
        q.to_writer(writer, names)?;
    }
    Ok(())
}

fn records_read<R: std::io::Read + std::io::Seek>(
    reader: &mut Reader<R>,
    count: u16,
    names: &mut HashMap<String, u8>,
) -> Result<Vec<DnsRecord>, DekuError> {
    let mut ans = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let q = DnsRecord::from_reader_with_ctx(reader, names)?;
        ans.push(q);
    }
    Ok(ans)
}

fn records_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    value: &Vec<DnsRecord>,
    names: &mut HashMap<String, u8>,
) -> Result<(), DekuError> {
    for q in value {
        q.to_writer(writer, names)?;
    }
    Ok(())
}

/// DNS Header
#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct DnsHeader {
    /// Packet ID
    #[deku(bits = 16)]
    pub id: u16,

    /// Query or Reponse
    #[deku(bits = 1)]
    pub qr: bool,
    /// Operation code
    #[deku(bits = 4)]
    pub opcode: u8,
    /// Authoritative Answer
    #[deku(bits = 1)]
    pub aa: bool,
    /// Truncated Message
    #[deku(bits = 1)]
    pub tc: bool,
    /// Recursion Desired
    #[deku(bits = 1)]
    pub rd: bool,
    // Recursion Available
    #[deku(bits = 1)]
    pub ra: bool,
    /// Reserved
    #[deku(bits = 3)]
    pub z: u8,
    #[deku(bits = 4)]
    pub rcode: u8,

    /// Question Count
    #[deku(bits = 16)]
    pub qdcount: u16,
    /// Answer Count
    #[deku(bits = 16)]
    pub ancount: u16,
    /// Authority Count
    #[deku(bits = 16)]
    pub nscount: u16,
    /// Additional Count
    #[deku(bits = 16)]
    pub arcount: u16,
}

/// DNS Type
#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u16", endian = "big")]
pub enum DnsType {
    #[deku(id = 1)]
    A,
}

/// DNS Class
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u16", endian = "big")]
pub enum DnsClass {
    #[deku(id = 1)]
    In,
}

/// DNS Question
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(ctx = "names: &mut HashMap<String, u8>")]
pub struct DnsQuestion {
    #[deku(
        reader = "qname_read(deku::reader, names)",
        writer = "qname_write(deku::writer, &self.name, names)"
    )]
    pub name: String,
    pub r#type: DnsType,
    pub class: DnsClass,
}

/// DNS Record
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(ctx = "names: &mut HashMap<String, u8>")]
pub struct DnsRecord {
    #[deku(
        reader = "qname_read(deku::reader, names)",
        writer = "qname_write(deku::writer, &self.name, names)"
    )]
    pub name: String,
    pub r#type: DnsType,
    pub class: DnsClass,
    #[deku(endian = "big")]
    pub ttl: u32,
    #[deku(endian = "big")]
    pub len: u16,
    #[deku(bytes_read = "len", ctx = "*r#type")]
    pub data: Vec<DnsRData>,
}

/// DNS Recrod Specific Data
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(ctx = "typ: DnsType", id = "typ")]
pub enum DnsRData {
    #[deku(id = "DnsType::A")]
    IP(#[deku(endian = "big")] Ipv4Addr),
}

/// Label
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Label {
    #[deku(update = "self.data.len()", bits = 8)]
    pub len: u8,
    #[deku(count = "len")]
    pub data: Vec<u8>,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Labels(#[deku(until = "|v: &Label| v.len == 0")] Vec<Label>);

/// Label Sequence
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum LabelSeq {
    #[deku(id = "192")]
    Jump(#[deku(bytes = 1)] u8),
    #[deku(id_pat = "_")]
    Lables(Labels),
}

impl std::str::FromStr for LabelSeq {
    type Err = DekuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let labels = s.split('.').map(|l| {
            let data = l.as_bytes().to_vec();
            let len = data.len() as u8;
            Label { len, data }
        });
        Ok(LabelSeq::Lables(Labels(labels.collect())))
    }
}

impl From<LabelSeq> for String {
    fn from(value: LabelSeq) -> Self {
        match value {
            LabelSeq::Lables(labels) => labels
                .0
                .into_iter()
                .map(|l| String::from_utf8(l.data).unwrap())
                .reduce(|acc, x| acc + "." + &x)
                .unwrap(),
            LabelSeq::Jump(_) => panic!("Jump not supported"),
        }
    }
}

fn qname_read<R: std::io::Read + std::io::Seek>(
    reader: &mut Reader<R>,
    _names: &mut HashMap<String, u8>,
) -> Result<String, DekuError> {
    // TODO: reduce jumping
    let mut jumped = 0;
    let original = reader.stream_position().unwrap();
    trace!("Original: {}", original);

    let mut value = LabelSeq::from_reader_with_ctx(reader, ())?;
    loop {
        match value {
            LabelSeq::Jump(offset) => {
                jumped += 1;
                reader.seek(SeekFrom::Start(offset as u64)).unwrap();
                value = LabelSeq::from_reader_with_ctx(reader, ())?;
            }
            LabelSeq::Lables(_) => {
                if jumped > 0 {
                    trace!("Jumped: {}", jumped);
                    reader.seek(SeekFrom::Start(original + 2)).unwrap();
                    trace!("Forward: {}", reader.stream_position().unwrap());
                }
                return Ok(value.into());
            }
        }
    }
}

fn qname_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    name: &str,
    names: &mut HashMap<String, u8>,
) -> Result<(), DekuError> {
    if let Some(&offset) = names.get(name) {
        trace!("Mark jumping to: {}", offset);
        LabelSeq::Jump(offset).to_writer(writer, ())?;
        return Ok(());
    }
    let offset = writer.stream_position().unwrap();
    names.insert(name.to_string(), offset as u8); // FIXME: offset maybe larger than u8
    let value: LabelSeq = name.parse().unwrap();
    value.to_writer(writer, ())
}

#[cfg(feature = "debug")]
#[ctor::ctor]
fn init() {
    crate::logging::setup_console_log();
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use tracing::debug;

    use super::*;

    fn hexdump_to_bytes(hexdump: &str) -> Vec<u8> {
        hexdump
            .split_whitespace()
            .map(|x| u8::from_str_radix(x, 16).unwrap())
            .collect()
    }

    #[test]
    fn label() {
        let data = vec![3, 119, 119, 119];
        let (_res, label) = Label::from_bytes((&data, 0)).unwrap();
        assert_eq!(String::from_utf8(label.data), Ok("www".to_string()));

        let data = vec![0];
        let (_res, label) = Label::from_bytes((&data, 0)).unwrap();
        assert_eq!(label.len, 0);
    }

    #[test]
    fn label_sequences() {
        let data: Vec<u8> = vec![
            3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0,
        ];
        let seq = LabelSeq::try_from(data.as_slice()).unwrap();

        match seq {
            LabelSeq::Jump(_) => unreachable!(),
            LabelSeq::Lables(lables) => {
                let names: Vec<String> = lables
                    .0
                    .into_iter()
                    .map(|l| String::from_utf8(l.data).unwrap())
                    .collect();
                assert_eq!(names, vec!["www", "google", "com", ""]);
            }
        }
    }

    #[test]
    fn label_sequences_converting() {
        let target: LabelSeq = "www.google.com.".parse().unwrap();
        assert_eq!(String::from(target), "www.google.com.".to_string());
    }

    #[test]
    fn label_sequences_jump() {
        let data: Vec<u8> = vec![192, 12];
        let seq = LabelSeq::try_from(data.as_slice()).unwrap();
        debug!("{:?}", seq);
        match seq {
            LabelSeq::Jump(offset) => assert_eq!(offset, 12),
            LabelSeq::Lables(_) => unreachable!(),
        }
    }

    #[test]
    fn dns_cls() {
        let raw = hexdump_to_bytes("00 01");
        let (_rest, cls) = DnsClass::from_bytes((&raw, 0)).unwrap();
        assert_eq!(cls, DnsClass::In);
    }

    #[test]
    fn question_section() {
        let raw: Vec<u8> = vec![3, 119, 119, 119, 0, 0, 1, 0, 1];
        let mut cursor = Cursor::new(raw);
        let mut reader = Reader::new(&mut cursor);
        let q = DnsQuestion::from_reader_with_ctx(&mut reader, &mut HashMap::new()).unwrap();
        debug!("{:?}", q);
    }

    #[test]
    fn answer_section() {
        let raw: Vec<u8> = vec![
            3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, // name
            0x00, 0x01, // type
            0x00, 0x01, // class
            0x00, 0x00, 0x01, 0x25, // ttl
            0x00, 0x04, // len
            0x8e, 0xfa, 0x45, 0xce, // address
        ];
        let mut cursor = Cursor::new(raw);
        let mut reader = Reader::new(&mut cursor);
        let a = DnsRecord::from_reader_with_ctx(&mut reader, &mut HashMap::new()).unwrap();
        debug!("{:?}", a);
    }

    #[test]
    fn parse_query() {
        let raw = hexdump_to_bytes(
            r#"
        29 7e 01 20 00 01 00 00  00 00 00 00 06 67 6f 6f
        67 6c 65 03 63 6f 6d 00  00 01 00 01
        "#,
        );
        let mut cursor = Cursor::new(raw);
        let mut reader = Reader::new(&mut cursor);

        let header = DnsHeader::from_reader_with_ctx(&mut reader, ()).unwrap();
        debug!("{:?}", header);

        reader.rewind().unwrap();
        let packet = DnsPacket::from_reader_with_ctx(&mut reader, &mut HashMap::new()).unwrap();
        debug!("{:?}", packet);
    }

    #[test]
    fn parse_response() {
        let raw = hexdump_to_bytes(
            r#"
        29 7e 81 80 00 01 00 01  00 00 00 00 06 67 6f 6f
        67 6c 65 03 63 6f 6d 00  00 01 00 01 c0 0c 00 01
        00 01 00 00 00 53 00 04  7f 00 00 01
        "#,
        );
        let mut cursor = Cursor::new(raw);
        let mut reader = Reader::new(&mut cursor);
        let header = DnsHeader::from_reader_with_ctx(&mut reader, ()).unwrap();
        debug!("{:?}", header);

        reader.rewind().unwrap();
        let packet = DnsPacket::from_reader_with_ctx(&mut reader, &mut HashMap::new()).unwrap();
        debug!("{:?}", packet);
    }

    #[test]
    fn response() {
        let packet = DnsPacket {
            header: DnsHeader {
                id: 0x297e,
                qr: true,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,
                ra: true,
                z: 0,
                rcode: 0,
                qdcount: 1,
                ancount: 1,
                nscount: 0,
                arcount: 0,
            },
            questions: vec![DnsQuestion {
                name: "google.com.".parse().unwrap(),
                r#type: DnsType::A,
                class: DnsClass::In,
            }],
            answers: vec![DnsRecord {
                name: "google.com.".parse().unwrap(),
                r#type: DnsType::A,
                class: DnsClass::In,
                ttl: 0x53,
                len: 4,
                data: vec![DnsRData::IP("127.0.0.1".parse().unwrap())],
            }],
            authorities: vec![],
            additional: vec![],
        };
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = Writer::new(&mut cursor);
        packet.to_writer(&mut writer, &mut HashMap::new()).unwrap();

        assert_eq!(
            cursor.into_inner().as_slice(),
            hexdump_to_bytes(
                r#"
        29 7e 81 80 00 01 00 01  00 00 00 00 06 67 6f 6f
        67 6c 65 03 63 6f 6d 00  00 01 00 01 c0 0c 00 01
        00 01 00 00 00 53 00 04  7f 00 00 01
            "#
            )
        )
    }
}
