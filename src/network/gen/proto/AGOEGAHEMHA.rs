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

//! Generated file from `AGOEGAHEMHA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AGOEGAHEMHA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AGOEGAHEMHA {
    // message fields
    // @@protoc_insertion_point(field:AGOEGAHEMHA.LAIOKLJNPFO)
    pub LAIOKLJNPFO: ::protobuf::MessageField<super::COFOFDFPLBE::COFOFDFPLBE>,
    // @@protoc_insertion_point(field:AGOEGAHEMHA.GFIAPFFEPFB)
    pub GFIAPFFEPFB: ::protobuf::MessageField<super::LHNPAKDHGFH::LHNPAKDHGFH>,
    // @@protoc_insertion_point(field:AGOEGAHEMHA.DAGLMONMMMH)
    pub DAGLMONMMMH: ::protobuf::MessageField<super::FJPJJEIJLLP::FJPJJEIJLLP>,
    // @@protoc_insertion_point(field:AGOEGAHEMHA.OGIONEOOFIN)
    pub OGIONEOOFIN: ::protobuf::MessageField<super::LMPECJMMBHP::LMPECJMMBHP>,
    // special fields
    // @@protoc_insertion_point(special_field:AGOEGAHEMHA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AGOEGAHEMHA {
    fn default() -> &'a AGOEGAHEMHA {
        <AGOEGAHEMHA as ::protobuf::Message>::default_instance()
    }
}

impl AGOEGAHEMHA {
    pub fn new() -> AGOEGAHEMHA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::COFOFDFPLBE::COFOFDFPLBE>(
            "LAIOKLJNPFO",
            |m: &AGOEGAHEMHA| { &m.LAIOKLJNPFO },
            |m: &mut AGOEGAHEMHA| { &mut m.LAIOKLJNPFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LHNPAKDHGFH::LHNPAKDHGFH>(
            "GFIAPFFEPFB",
            |m: &AGOEGAHEMHA| { &m.GFIAPFFEPFB },
            |m: &mut AGOEGAHEMHA| { &mut m.GFIAPFFEPFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FJPJJEIJLLP::FJPJJEIJLLP>(
            "DAGLMONMMMH",
            |m: &AGOEGAHEMHA| { &m.DAGLMONMMMH },
            |m: &mut AGOEGAHEMHA| { &mut m.DAGLMONMMMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LMPECJMMBHP::LMPECJMMBHP>(
            "OGIONEOOFIN",
            |m: &AGOEGAHEMHA| { &m.OGIONEOOFIN },
            |m: &mut AGOEGAHEMHA| { &mut m.OGIONEOOFIN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AGOEGAHEMHA>(
            "AGOEGAHEMHA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AGOEGAHEMHA {
    const NAME: &'static str = "AGOEGAHEMHA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LAIOKLJNPFO)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GFIAPFFEPFB)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DAGLMONMMMH)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OGIONEOOFIN)?;
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
        if let Some(v) = self.LAIOKLJNPFO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GFIAPFFEPFB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.DAGLMONMMMH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.OGIONEOOFIN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.LAIOKLJNPFO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.GFIAPFFEPFB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.DAGLMONMMMH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.OGIONEOOFIN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> AGOEGAHEMHA {
        AGOEGAHEMHA::new()
    }

    fn clear(&mut self) {
        self.LAIOKLJNPFO.clear();
        self.GFIAPFFEPFB.clear();
        self.DAGLMONMMMH.clear();
        self.OGIONEOOFIN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AGOEGAHEMHA {
        static instance: AGOEGAHEMHA = AGOEGAHEMHA {
            LAIOKLJNPFO: ::protobuf::MessageField::none(),
            GFIAPFFEPFB: ::protobuf::MessageField::none(),
            DAGLMONMMMH: ::protobuf::MessageField::none(),
            OGIONEOOFIN: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AGOEGAHEMHA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AGOEGAHEMHA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AGOEGAHEMHA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AGOEGAHEMHA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AGOEGAHEMHA.proto\x1a\x11COFOFDFPLBE.proto\x1a\x11FJPJJEIJLLP.prot\
    o\x1a\x11LHNPAKDHGFH.proto\x1a\x11LMPECJMMBHP.proto\"\xcd\x01\n\x0bAGOEG\
    AHEMHA\x12.\n\x0bLAIOKLJNPFO\x18\t\x20\x01(\x0b2\x0c.COFOFDFPLBER\x0bLAI\
    OKLJNPFO\x12.\n\x0bGFIAPFFEPFB\x18\r\x20\x01(\x0b2\x0c.LHNPAKDHGFHR\x0bG\
    FIAPFFEPFB\x12.\n\x0bDAGLMONMMMH\x18\x0f\x20\x01(\x0b2\x0c.FJPJJEIJLLPR\
    \x0bDAGLMONMMMH\x12.\n\x0bOGIONEOOFIN\x18\x01\x20\x01(\x0b2\x0c.LMPECJMM\
    BHPR\x0bOGIONEOOFINb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::COFOFDFPLBE::file_descriptor().clone());
            deps.push(super::FJPJJEIJLLP::file_descriptor().clone());
            deps.push(super::LHNPAKDHGFH::file_descriptor().clone());
            deps.push(super::LMPECJMMBHP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AGOEGAHEMHA::generated_message_descriptor_data());
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
