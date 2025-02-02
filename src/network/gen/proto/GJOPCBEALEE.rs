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

//! Generated file from `GJOPCBEALEE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GJOPCBEALEE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GJOPCBEALEE {
    // message fields
    // @@protoc_insertion_point(field:GJOPCBEALEE.CKLHHOLMBOO)
    pub CKLHHOLMBOO: ::std::vec::Vec<super::FMGCOGMJKCM::FMGCOGMJKCM>,
    // @@protoc_insertion_point(field:GJOPCBEALEE.PPMKDMINBNH)
    pub PPMKDMINBNH: u32,
    // @@protoc_insertion_point(field:GJOPCBEALEE.FILMAOEBILH)
    pub FILMAOEBILH: u32,
    // @@protoc_insertion_point(field:GJOPCBEALEE.HGMDMODJIKO)
    pub HGMDMODJIKO: u32,
    // @@protoc_insertion_point(field:GJOPCBEALEE.NOPLLNLNMEH)
    pub NOPLLNLNMEH: u32,
    // @@protoc_insertion_point(field:GJOPCBEALEE.level)
    pub level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GJOPCBEALEE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GJOPCBEALEE {
    fn default() -> &'a GJOPCBEALEE {
        <GJOPCBEALEE as ::protobuf::Message>::default_instance()
    }
}

impl GJOPCBEALEE {
    pub fn new() -> GJOPCBEALEE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CKLHHOLMBOO",
            |m: &GJOPCBEALEE| { &m.CKLHHOLMBOO },
            |m: &mut GJOPCBEALEE| { &mut m.CKLHHOLMBOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPMKDMINBNH",
            |m: &GJOPCBEALEE| { &m.PPMKDMINBNH },
            |m: &mut GJOPCBEALEE| { &mut m.PPMKDMINBNH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FILMAOEBILH",
            |m: &GJOPCBEALEE| { &m.FILMAOEBILH },
            |m: &mut GJOPCBEALEE| { &mut m.FILMAOEBILH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HGMDMODJIKO",
            |m: &GJOPCBEALEE| { &m.HGMDMODJIKO },
            |m: &mut GJOPCBEALEE| { &mut m.HGMDMODJIKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NOPLLNLNMEH",
            |m: &GJOPCBEALEE| { &m.NOPLLNLNMEH },
            |m: &mut GJOPCBEALEE| { &mut m.NOPLLNLNMEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &GJOPCBEALEE| { &m.level },
            |m: &mut GJOPCBEALEE| { &mut m.level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GJOPCBEALEE>(
            "GJOPCBEALEE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GJOPCBEALEE {
    const NAME: &'static str = "GJOPCBEALEE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    self.CKLHHOLMBOO.push(is.read_message()?);
                },
                56 => {
                    self.PPMKDMINBNH = is.read_uint32()?;
                },
                72 => {
                    self.FILMAOEBILH = is.read_uint32()?;
                },
                104 => {
                    self.HGMDMODJIKO = is.read_uint32()?;
                },
                8 => {
                    self.NOPLLNLNMEH = is.read_uint32()?;
                },
                120 => {
                    self.level = is.read_uint32()?;
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
        for value in &self.CKLHHOLMBOO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PPMKDMINBNH != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.PPMKDMINBNH);
        }
        if self.FILMAOEBILH != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FILMAOEBILH);
        }
        if self.HGMDMODJIKO != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.HGMDMODJIKO);
        }
        if self.NOPLLNLNMEH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.NOPLLNLNMEH);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.CKLHHOLMBOO {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.PPMKDMINBNH != 0 {
            os.write_uint32(7, self.PPMKDMINBNH)?;
        }
        if self.FILMAOEBILH != 0 {
            os.write_uint32(9, self.FILMAOEBILH)?;
        }
        if self.HGMDMODJIKO != 0 {
            os.write_uint32(13, self.HGMDMODJIKO)?;
        }
        if self.NOPLLNLNMEH != 0 {
            os.write_uint32(1, self.NOPLLNLNMEH)?;
        }
        if self.level != 0 {
            os.write_uint32(15, self.level)?;
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

    fn new() -> GJOPCBEALEE {
        GJOPCBEALEE::new()
    }

    fn clear(&mut self) {
        self.CKLHHOLMBOO.clear();
        self.PPMKDMINBNH = 0;
        self.FILMAOEBILH = 0;
        self.HGMDMODJIKO = 0;
        self.NOPLLNLNMEH = 0;
        self.level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GJOPCBEALEE {
        static instance: GJOPCBEALEE = GJOPCBEALEE {
            CKLHHOLMBOO: ::std::vec::Vec::new(),
            PPMKDMINBNH: 0,
            FILMAOEBILH: 0,
            HGMDMODJIKO: 0,
            NOPLLNLNMEH: 0,
            level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GJOPCBEALEE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GJOPCBEALEE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GJOPCBEALEE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GJOPCBEALEE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GJOPCBEALEE.proto\x1a\x11FMGCOGMJKCM.proto\"\xdb\x01\n\x0bGJOPCBEA\
    LEE\x12.\n\x0bCKLHHOLMBOO\x18\x04\x20\x03(\x0b2\x0c.FMGCOGMJKCMR\x0bCKLH\
    HOLMBOO\x12\x20\n\x0bPPMKDMINBNH\x18\x07\x20\x01(\rR\x0bPPMKDMINBNH\x12\
    \x20\n\x0bFILMAOEBILH\x18\t\x20\x01(\rR\x0bFILMAOEBILH\x12\x20\n\x0bHGMD\
    MODJIKO\x18\r\x20\x01(\rR\x0bHGMDMODJIKO\x12\x20\n\x0bNOPLLNLNMEH\x18\
    \x01\x20\x01(\rR\x0bNOPLLNLNMEH\x12\x14\n\x05level\x18\x0f\x20\x01(\rR\
    \x05levelb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::FMGCOGMJKCM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GJOPCBEALEE::generated_message_descriptor_data());
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
