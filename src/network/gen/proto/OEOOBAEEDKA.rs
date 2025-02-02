// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `OEOOBAEEDKA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OEOOBAEEDKA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OEOOBAEEDKA {
    // message fields
    // @@protoc_insertion_point(field:OEOOBAEEDKA.LBPJJFNLMFC)
    pub LBPJJFNLMFC: u32,
    // @@protoc_insertion_point(field:OEOOBAEEDKA.MHIAJFMPNDO)
    pub MHIAJFMPNDO: ::std::vec::Vec<super::AKFAEGPHHKK::AKFAEGPHHKK>,
    // @@protoc_insertion_point(field:OEOOBAEEDKA.CBCPDECNGOD)
    pub CBCPDECNGOD: u32,
    // @@protoc_insertion_point(field:OEOOBAEEDKA.status)
    pub status: ::protobuf::EnumOrUnknown<super::CNEIAFOAJLK::CNEIAFOAJLK>,
    // @@protoc_insertion_point(field:OEOOBAEEDKA.DOBCJFHJAAH)
    pub DOBCJFHJAAH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OEOOBAEEDKA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OEOOBAEEDKA {
    fn default() -> &'a OEOOBAEEDKA {
        <OEOOBAEEDKA as ::protobuf::Message>::default_instance()
    }
}

impl OEOOBAEEDKA {
    pub fn new() -> OEOOBAEEDKA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBPJJFNLMFC",
            |m: &OEOOBAEEDKA| { &m.LBPJJFNLMFC },
            |m: &mut OEOOBAEEDKA| { &mut m.LBPJJFNLMFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MHIAJFMPNDO",
            |m: &OEOOBAEEDKA| { &m.MHIAJFMPNDO },
            |m: &mut OEOOBAEEDKA| { &mut m.MHIAJFMPNDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CBCPDECNGOD",
            |m: &OEOOBAEEDKA| { &m.CBCPDECNGOD },
            |m: &mut OEOOBAEEDKA| { &mut m.CBCPDECNGOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &OEOOBAEEDKA| { &m.status },
            |m: &mut OEOOBAEEDKA| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DOBCJFHJAAH",
            |m: &OEOOBAEEDKA| { &m.DOBCJFHJAAH },
            |m: &mut OEOOBAEEDKA| { &mut m.DOBCJFHJAAH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OEOOBAEEDKA>(
            "OEOOBAEEDKA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OEOOBAEEDKA {
    const NAME: &'static str = "OEOOBAEEDKA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.LBPJJFNLMFC = is.read_uint32()?;
                },
                42 => {
                    self.MHIAJFMPNDO.push(is.read_message()?);
                },
                112 => {
                    self.CBCPDECNGOD = is.read_uint32()?;
                },
                120 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.DOBCJFHJAAH = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.LBPJJFNLMFC != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LBPJJFNLMFC);
        }
        for value in &self.MHIAJFMPNDO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CBCPDECNGOD != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.CBCPDECNGOD);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::CNEIAFOAJLK::CNEIAFOAJLK::ROGUE_TOURN_LAYER_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.status.value());
        }
        if self.DOBCJFHJAAH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DOBCJFHJAAH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LBPJJFNLMFC != 0 {
            os.write_uint32(3, self.LBPJJFNLMFC)?;
        }
        for v in &self.MHIAJFMPNDO {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.CBCPDECNGOD != 0 {
            os.write_uint32(14, self.CBCPDECNGOD)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::CNEIAFOAJLK::CNEIAFOAJLK::ROGUE_TOURN_LAYER_STATUS_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.DOBCJFHJAAH != 0 {
            os.write_uint32(1, self.DOBCJFHJAAH)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> OEOOBAEEDKA {
        OEOOBAEEDKA::new()
    }

    fn clear(&mut self) {
        self.LBPJJFNLMFC = 0;
        self.MHIAJFMPNDO.clear();
        self.CBCPDECNGOD = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::CNEIAFOAJLK::CNEIAFOAJLK::ROGUE_TOURN_LAYER_STATUS_NONE);
        self.DOBCJFHJAAH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OEOOBAEEDKA {
        static instance: OEOOBAEEDKA = OEOOBAEEDKA {
            LBPJJFNLMFC: 0,
            MHIAJFMPNDO: ::std::vec::Vec::new(),
            CBCPDECNGOD: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            DOBCJFHJAAH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OEOOBAEEDKA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OEOOBAEEDKA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OEOOBAEEDKA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OEOOBAEEDKA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OEOOBAEEDKA.proto\x1a\x11AKFAEGPHHKK.proto\x1a\x11CNEIAFOAJLK.prot\
    o\"\xc9\x01\n\x0bOEOOBAEEDKA\x12\x20\n\x0bLBPJJFNLMFC\x18\x03\x20\x01(\r\
    R\x0bLBPJJFNLMFC\x12.\n\x0bMHIAJFMPNDO\x18\x05\x20\x03(\x0b2\x0c.AKFAEGP\
    HHKKR\x0bMHIAJFMPNDO\x12\x20\n\x0bCBCPDECNGOD\x18\x0e\x20\x01(\rR\x0bCBC\
    PDECNGOD\x12$\n\x06status\x18\x0f\x20\x01(\x0e2\x0c.CNEIAFOAJLKR\x06stat\
    us\x12\x20\n\x0bDOBCJFHJAAH\x18\x01\x20\x01(\rR\x0bDOBCJFHJAAHb\x06proto\
    3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::AKFAEGPHHKK::file_descriptor().clone());
            deps.push(super::CNEIAFOAJLK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OEOOBAEEDKA::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
